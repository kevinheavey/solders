#[macro_export]
macro_rules! error_message {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
        #[pyclass(module = "solders.rpc.errors", subclass)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[pyo3(get)]
            message: String,
        }

        transaction_status_boilerplate!($name);

        #[richcmp_eq_only]
        #[common_methods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(message: String) -> Self {
                message.into()
            }
        }
    };
    ($name:ident, $data_type:ty) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
        #[pyclass(module = "solders.rpc.errors", subclass)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[pyo3(get)]
            message: String,
            #[pyo3(get)]
            data: $data_type,
        }

        transaction_status_boilerplate!($name);

        #[richcmp_eq_only]
        #[common_methods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(message: String, data: $data_type) -> Self {
                (message, data).into()
            }
        }
    };
}
