use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_quote, spanned::Spanned, Data, DeriveInput, Field, Ident};

fn is_option(ty: &syn::Type) -> bool {
    let path = match *ty {
        syn::Type::Path(ref p) if p.qself.is_none() => &p.path,
        _ => return false,
    };

    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .any(|x| x.as_str() == "Option")
}

fn map_extraction(field: Field) -> TokenStream {
    let ident = match &field.ident {
        Some(i) => i,
        None => {
            return syn::Error::new(field.span(), "Unnamed fields are not supported")
                .to_compile_error()
        }
    };

    let name = ident.to_string();

    let function = if is_option(&field.ty) {
        Ident::new("extract_optional", field.ty.span())
    } else {
        Ident::new("extract_required", field.ty.span())
    };

    quote_spanned! {ident.span()=>
        #ident: #function(dict, #name)?
    }
}

fn extraction_functions() -> TokenStream {
    quote! {
        fn map_exception(name: &str, e: PyErr) -> PyErr {
            PyErr::new::<PyTypeError, _>(format!("Unable to convert key: {}. Error: {}", name, e))
        }

        fn extract_required<'a, T>(dict: &Bound<'a, PyDict>, name: &str) -> PyResult<T>
        where
            T: FromPyObject<'a>,
        {
            match PyDictMethods::get_item(dict, name)? {
                Some(v) => FromPyObject::extract_bound(&v)
                    .map_err(|err| map_exception(name, err)),
                None => Err(PyErr::new::<PyValueError, _>(format!(
                    "Missing required key: {}",
                    name
                ))),
            }
        }

        fn extract_optional<'a, T>(dict: &Bound<'a, PyDict>, name: &str) -> PyResult<std::option::Option<T>>
        where
            T: FromPyObject<'a>,
        {
            match PyDictMethods::get_item(dict, name)? {
                Some(v) => FromPyObject::extract_bound(&v)
                    .map_err(|err| map_exception(name, err)),
                None => Ok(None),
            }
        }
    }
}

pub fn from_impl(ast: DeriveInput) -> TokenStream {
    let struct_data = match ast.data {
        Data::Struct(s) => s,
        Data::Enum(e) => {
            return syn::Error::new(e.enum_token.span, "Deriving enums is not supported")
                .to_compile_error();
        }
        Data::Union(u) => {
            return syn::Error::new(u.union_token.span, "Deriving unions is not supported")
                .to_compile_error();
        }
    };

    let extractions = struct_data.fields.into_iter().map(map_extraction);

    let name = ast.ident;
    let mut impl_generics = ast.generics.clone();

    let lifetimes_count = impl_generics.lifetimes().count();

    if lifetimes_count > 1 {
        return syn::Error::new(
            impl_generics.span(),
            "Deriving structs with more than one lifetime is not supported",
        )
        .to_compile_error();
    } else if lifetimes_count == 0 {
        impl_generics.params.push(parse_quote!('source));
    };

    let lifetime = impl_generics
        .lifetimes()
        .next()
        .map(|lt| lt.lifetime.to_token_stream())
        .unwrap();

    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (impl_generics, _, _) = impl_generics.split_for_impl();

    let functions = extraction_functions();

    quote! {
        impl #impl_generics ::pyo3::FromPyObject<#lifetime> for #name #ty_generics #where_clause {
            fn extract_bound(
                obj: &::pyo3::prelude::Bound<#lifetime, ::pyo3::types::PyAny>,
            ) -> ::pyo3::PyResult<Self> {
                use ::pyo3::{FromPyObject, PyErr, PyResult};
                use ::pyo3::exceptions::{PyTypeError, PyValueError};
                use ::pyo3::prelude::Bound;
                use ::pyo3::types::{PyAnyMethods, PyDict, PyDictMethods};
                let dict = obj.downcast::<PyDict>().map_err(|_| {
                    PyErr::new::<PyTypeError, _>("Invalid type to convert, expected dict")
                })?;

                #functions

                ::std::result::Result::Ok(#name {
                    #(#extractions),*
                })
            }
        }
    }
}
