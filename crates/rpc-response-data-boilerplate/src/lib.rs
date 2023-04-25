#[macro_export]
macro_rules! response_data_boilerplate {
    ($name:ident) => {
        impl solders_traits_core::RichcmpEqualityOnly for $name {}
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        solders_traits_core::pybytes_general_via_bincode!($name);
        solders_traits_core::py_from_bytes_general_via_bincode!($name);
        solders_traits_core::common_methods_default!($name);
    };
}
