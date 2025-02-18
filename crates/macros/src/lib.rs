//! A collection of attribute macros to reduce boilerplate in the
//! [solders](https://github.com/kevinheavey/solders) project.
//!
//! These macros make some very specific assumptions about the structs
//! they're applied to, so they're unlikely to be useful for other projects.
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ItemEnum, ItemImpl};

/// Add a `__hash__` to the impl using the `PyHash` trait.
///
/// # Example
///
/// ```rust
/// use solders_macros::pyhash;
///
/// #[derive(Debug)]
/// struct Foo(u8);
///
/// #[pyhash]
/// impl Foo {
///   pub fn pyhash(&self) -> u64 {  // Fake implementation in place of `PyHash`.
///      self.0.into()
///   }
/// }
///
/// let foo = Foo(3);
/// assert_eq!(3, foo.__hash__());
///
/// ```
#[proc_macro_attribute]
pub fn pyhash(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __hash__(&self) -> u64 {
        solders_traits_core::PyHash::pyhash(self)
    }};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpFull` trait.
///
/// # Example
///
/// ```rust
/// use solders_macros::richcmp_full;
///
///
/// #[derive(Debug)]
/// struct Foo(u8);
///
/// #[richcmp_full]
/// impl Foo {
///   pub fn richcmp(&self, other: &Self, op: CompareOp) -> bool {  // Fake implementation in place of `RichcmpFull`.
///      true
///   }
/// }
///
/// let foo = Foo(3);
/// assert_eq!(true, foo.__richcmp__(&foo, CompareOp::Eq));
///
/// ```
#[proc_macro_attribute]
pub fn richcmp_full(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        solders_traits_core::RichcmpFull::richcmp(self, other, op)
    }};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpEqualityOnly` trait.
#[proc_macro_attribute]
pub fn richcmp_eq_only(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> pyo3::prelude::PyResult<bool> {
        solders_traits_core::RichcmpEqualityOnly::richcmp(self, other, op)
    }};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpSigner` trait.
#[proc_macro_attribute]
pub fn richcmp_signer(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: crate::signer::Signer, op: pyo3::basic::CompareOp) -> pyo3::prelude::PyResult<bool> {
        solders_traits::RichcmpSigner::richcmp(self, other, op)
    }};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

fn add_core_methods(ast: &mut ItemImpl) {
    let mut methods = vec![
        ImplItem::Verbatim(quote! {pub fn __bytes__<'a>(&self) -> Vec<u8>  {
            solders_traits_core::CommonMethodsCore::pybytes(self)
        }}),
        ImplItem::Verbatim(quote! { pub fn __str__(&self) -> String {
            solders_traits_core::CommonMethodsCore::pystr(self)
        } }),
        ImplItem::Verbatim(quote! { pub fn __repr__(&self) -> String {
            solders_traits_core::CommonMethodsCore::pyrepr(self)
        } }),
    ];
    if !ast.items.iter().any(|item| match item {
        ImplItem::Method(m) => m.sig.ident == "from_bytes",
        _ => false,
    }) {
        let from_bytes = ImplItem::Verbatim(quote! {
            /// Deserialize from bytes.
            ///
            /// Args:
            ///     data (bytes): the serialized object.
            ///
            /// Returns: the deserialized object.
            ///
            #[staticmethod]
            pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
                <Self as solders_traits_core::CommonMethodsCore>::py_from_bytes(data)
            }
        });
        methods.push(from_bytes);
    };
    ast.items.extend_from_slice(&methods);
}

/// Add `__bytes__`, `__str__`, and `__repr__` using the `CommonMethodsCore` trait.
///
/// Also add `from_bytes` if not already defined.
#[proc_macro_attribute]
pub fn common_methods_core(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    add_core_methods(&mut ast);
    TokenStream::from(ast.to_token_stream())
}

/// Add `__bytes__`, `__str__`, `__repr__`, `to_json` and `from_json` using the `CommonMethods` trait.
///
/// Also add `from_bytes` if not already defined.
#[proc_macro_attribute]
pub fn common_methods(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    add_core_methods(&mut ast);
    let methods = vec![
        ImplItem::Verbatim(quote! {
        /// Convert to a JSON string.
        pub fn to_json(&self) -> String {
            solders_traits_core::CommonMethods::py_to_json(self)
        } }),
        ImplItem::Verbatim(quote! {
        /// Build from a JSON string.
        #[staticmethod] pub fn from_json(raw: &str) -> PyResult<Self> {
            <Self as solders_traits_core::CommonMethods>::py_from_json(raw)
        } }),
    ];
    ast.items.extend_from_slice(&methods);
    TokenStream::from(ast.to_token_stream())
}

/// Add `__bytes__`, `__str__`, `__repr__`, and `to_json` using the `CommonMethods` trait.
///
/// Also add `from_bytes` if not already defined.
#[proc_macro_attribute]
pub fn common_methods_ser_only(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    add_core_methods(&mut ast);
    let methods = vec![ImplItem::Verbatim(quote! {
    /// Convert to a JSON string.
    pub fn to_json(&self) -> String {
        solders_traits_core::CommonMethodsSerOnly::py_to_json(self)
    } })];
    ast.items.extend_from_slice(&methods);
    TokenStream::from(ast.to_token_stream())
}

/// Add `__bytes__`, `__str__`, `__repr__`, `__reduce__`, `to_json`, `from_json`, `from_bytes` and `__richcmp__` using the `CommonMethodsRpcResp` trait.
#[proc_macro_attribute]
pub fn common_methods_rpc_resp(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let methods = vec![
        ImplItem::Verbatim(quote! {pub fn __bytes__(&self) -> Vec<u8>  {
            CommonMethodsRpcResp::pybytes(self)
        }}),
        ImplItem::Verbatim(quote! { pub fn __str__(&self) -> String {
            CommonMethodsRpcResp::pystr(self)
        } }),
        ImplItem::Verbatim(quote! { pub fn __repr__(&self) -> String {
            CommonMethodsRpcResp::pyrepr(self)
        } }),
        ImplItem::Verbatim(quote! {
        /// Convert to a JSON string.
        pub fn to_json(&self) -> String {
            CommonMethodsRpcResp::py_to_json(self)
        } }),
        ImplItem::Verbatim(quote! {
        /// Build from a JSON string.
        ///
        /// Args:
        ///     raw (str): The RPC JSON response (can be an error response).
        ///
        /// Returns:
        ///     Either the deserialized object or ``RPCError``.
        ///
        #[staticmethod]
        pub fn from_json(raw: &str) -> PyResult<Resp<Self>> {
            <Self as CommonMethodsRpcResp>::py_from_json(raw)
        } }),
        ImplItem::Verbatim(quote! {
            /// Deserialize from bytes.
            ///
            /// Args:
            ///     data (bytes): the serialized object.
            ///
            /// Returns: the deserialized object.
            ///
            #[staticmethod]
            pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
                <Self as CommonMethodsRpcResp>::py_from_bytes(data)
            }
        }),
        ImplItem::Verbatim(
            quote! {pub fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> pyo3::prelude::PyResult<bool> {
                solders_traits_core::RichcmpEqualityOnly::richcmp(self, other, op)
            }},
        ),
    ];
    ast.items.extend_from_slice(&methods);
    TokenStream::from(ast.to_token_stream())
}

/// Add an `id` getter to an RPC request object.
///
/// By convention, assumes the `id` lives at `self.base.id`.
#[proc_macro_attribute]
pub fn rpc_id_getter(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {
    /// int: The ID of the RPC request.
    #[getter]
    pub fn id(&self) -> u64 {
        self.base.id
    }};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add mappings to and from another enum that has the exact same fields.
///
/// # Example
///
/// ```rust
/// use solders_macros::enum_original_mapping;
///
/// #[derive(PartialEq, Debug)]
/// pub enum Foo {
///   A,
///   B
/// }
/// #[enum_original_mapping(Foo)]
/// #[derive(PartialEq, Debug)]
/// pub enum Bar {
///   A,
///   B,
/// }
///
/// let a = Bar::A;
/// let b = Foo::B;
/// assert_eq!(Foo::from(a), Foo::A);
/// assert_eq!(Bar::from(b), Bar::B);
///
#[proc_macro_attribute]
pub fn enum_original_mapping(original: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_stream = proc_macro2::TokenStream::from(item.clone());
    let ast = parse_macro_input!(item as ItemEnum);
    let enum_name = ast.ident;
    let orig = parse_macro_input!(original as Ident);
    let variant_names: Vec<Ident> = ast.variants.into_iter().map(|v| v.ident).collect();
    let from_impl = quote! {
        impl From<#orig> for #enum_name {
            fn from(left: #orig) -> Self {
                match left {
                    #(#orig::#variant_names => Self::#variant_names),*,
                    _ => panic!("Unrecognized variant: {:?}", left)
                }
            }
        }

        impl From<#enum_name> for #orig {
            fn from(left: #enum_name) -> Self {
                match left {
                    #(#enum_name::#variant_names => Self::#variant_names),*
                }
            }
        }
    };
    new_stream.extend(from_impl);
    TokenStream::from(new_stream)
}

/// Impl IntoPyObject for an ADT where each variant is a newtype.
///
/// # Example
///
/// ```rust
/// use solders_macros::EnumIntoPy;
///
/// #[derive(PartialEq, Debug, EnumIntoPy)]
/// pub enum Foo {
///   A(u8),
///   B(u8)
/// }
///
#[proc_macro_derive(EnumIntoPy)]
pub fn enum_into_py(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemEnum);
    let enum_name = ast.ident;
    let variant_names: Vec<Ident> = ast.variants.into_iter().map(|v| v.ident).collect();
    let into_py_impl = quote! {
        impl<'py> IntoPyObject<'py> for #enum_name {
            type Target = PyAny; // the Python type
            type Output = Bound<'py, Self::Target>; // in most cases this will be `Bound`
            type Error = std::convert::Infallible;

            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                Ok(match self {
                    #(Self::#variant_names(x) => ::pyo3::conversion::IntoPyObjectExt::into_bound_py_any(&x, py).unwrap()),*,
                })
            }
        }
    };
    into_py_impl.to_token_stream().into()
}
