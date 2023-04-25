use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_common::RpcSimulateTransactionResult;
use solders_rpc_errors_common::error_message;
use solders_traits_core::transaction_status_boilerplate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct SendTransactionPreflightFailure {
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    result: RpcSimulateTransactionResult,
}

transaction_status_boilerplate!(SendTransactionPreflightFailure);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl SendTransactionPreflightFailure {
    #[new]
    pub fn new(message: String, result: RpcSimulateTransactionResult) -> Self {
        (message, result).into()
    }
}

error_message!(
    SendTransactionPreflightFailureMessage,
    RpcSimulateTransactionResult
);
