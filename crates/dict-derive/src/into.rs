use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_quote, spanned::Spanned, Data, DeriveInput, Field};

fn map_fields(field: Field) -> TokenStream {
    let ident = match &field.ident {
        Some(i) => i,
        None => {
            return syn::Error::new(field.span(), "Unnamed fields are not supported")
                .to_compile_error()
        }
    };

    let name = ident.to_string();

    quote_spanned! {field.ty.span()=>
        dict.set_item(#name, self.#ident)?;
    }
}

pub fn into_impl(ast: DeriveInput) -> TokenStream {
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

    let field_setters = struct_data.fields.into_iter().map(map_fields);

    let name = ast.ident;
    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut impl_generics = ast.generics.clone();
    impl_generics.params.insert(0, parse_quote!('py));
    let (impl_generics, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_generics ::pyo3::IntoPyObject<'py> for #name #ty_generics #where_clause {
            type Target = ::pyo3::types::PyDict;
            type Output = ::pyo3::Bound<'py, ::pyo3::types::PyDict>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<Self::Output> {
                use ::pyo3::types::PyDict;
                let dict = PyDict::new(py);
                #(#field_setters)*
                ::std::result::Result::Ok(dict)
            }
        }
    }
}
