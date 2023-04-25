use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto, TryFromInto};

use solders_account::Account;
use solders_account_decoder::tmp_account_decoder::UiAccount;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_response_data_boilerplate::response_data_boilerplate;
use solders_transaction_error::TransactionErrorType;
use solders_transaction_status::{
    tmp_transaction_status::UiTransactionReturnData, TransactionReturnData,
};

// the one in solana_client doesn't derive Eq
// TODO: latest does
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSimulateTransactionResult {
    #[serde(default)]
    #[pyo3(get)]
    pub err: Option<TransactionErrorType>,
    #[serde(default)]
    #[pyo3(get)]
    pub logs: Option<Vec<String>>,
    #[serde_as(as = "Option<Vec<Option<TryFromInto<UiAccount>>>>")]
    #[serde(default)]
    #[pyo3(get)]
    pub accounts: Option<Vec<Option<Account>>>,
    #[serde(default)]
    #[pyo3(get)]
    pub units_consumed: Option<u64>,
    #[serde_as(as = "Option<FromInto<UiTransactionReturnData>>")]
    #[serde(default)]
    #[pyo3(get)]
    pub return_data: Option<TransactionReturnData>,
}

response_data_boilerplate!(RpcSimulateTransactionResult);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionResult {
    #[new]
    pub fn new(
        err: Option<TransactionErrorType>,
        logs: Option<Vec<String>>,
        accounts: Option<Vec<Option<Account>>>,
        units_consumed: Option<u64>,
        return_data: Option<TransactionReturnData>,
    ) -> Self {
        Self {
            err,
            logs,
            accounts,
            units_consumed,
            return_data,
        }
    }
}
