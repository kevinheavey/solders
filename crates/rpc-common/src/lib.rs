use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use solana_account_decoder::UiAccount;
use solana_rpc_client_api::response::RpcSimulateTransactionResult as RpcSimulateTransactionResultOriginal;
use solana_transaction_status::UiInnerInstructions as UiInnerInstructionsOriginal;
use solders_account::Account;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_response_data_boilerplate::response_data_boilerplate;
use solders_rpc_responses_common::RpcBlockhash;
use solders_transaction_error::TransactionErrorType;
use solders_transaction_return_data::TransactionReturnData;
use solders_transaction_status::UiInnerInstructions;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSimulateTransactionResult(RpcSimulateTransactionResultOriginal);

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
        inner_instructions: Option<Vec<UiInnerInstructions>>,
        replacement_blockhash: Option<RpcBlockhash>,
    ) -> Self {
        let accounts_underlying: Option<Vec<Option<UiAccount>>> = accounts.map(|accs| {
            accs.into_iter()
                .map(|maybe_acc| maybe_acc.map(UiAccount::from))
                .collect()
        });
        let inner_instructions_underlying: Option<Vec<UiInnerInstructionsOriginal>> =
            inner_instructions.map(|ixns| ixns.into_iter().map(Into::into).collect());
        Self(RpcSimulateTransactionResultOriginal {
            err: err.map(Into::into),
            logs,
            accounts: accounts_underlying,
            units_consumed,
            return_data: return_data.map(Into::into),
            inner_instructions: inner_instructions_underlying,
            replacement_blockhash: replacement_blockhash.map(Into::into),
        })
    }

    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.0.err.clone().map(Into::into)
    }

    #[getter]
    pub fn logs(&self) -> Option<Vec<String>> {
        self.0.logs.clone()
    }

    #[getter]
    pub fn accounts(&self) -> Option<Vec<Option<Account>>> {
        self.0.accounts.clone().map(|accs| {
            accs.into_iter()
                .map(|maybe_acc| maybe_acc.map(|acc| Account::try_from(acc).unwrap()))
                .collect()
        })
    }

    #[getter]
    pub fn units_consumed(&self) -> Option<u64> {
        self.0.units_consumed
    }

    #[getter]
    pub fn return_data(&self) -> Option<TransactionReturnData> {
        self.0.return_data.clone().map(Into::into)
    }
}
