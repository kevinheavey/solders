#[macro_export]
macro_rules! rpc_config_impls {
    ($ident:ident) => {
        pybytes_general_via_cbor!($ident);
        py_from_bytes_general_via_cbor!($ident);
        impl_display!($ident);
        impl RichcmpEqualityOnly for $ident {}
        solders_traits_core::common_methods_default!($ident);
        impl From<rpc_config::$ident> for $ident {
            fn from(c: rpc_config::$ident) -> Self {
                Self(c)
            }
        }
        impl From<$ident> for rpc_config::$ident {
            fn from(c: $ident) -> Self {
                c.0
            }
        }
    };
}

#[macro_export]
macro_rules! pyclass_boilerplate {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
        rpc_config_impls!($name);
    };
}

#[macro_export]
macro_rules! pyclass_boilerplate_with_default {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
        $crate::rpc_config_impls!($name);
    };
}
