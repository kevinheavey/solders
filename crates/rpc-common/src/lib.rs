use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use solana_account_decoder_client_types::UiAccount;
use solana_rpc_client_types::response::RpcSimulateTransactionResult as RpcSimulateTransactionResultOriginal;
use solana_transaction_error::TransactionError as TransactionErrorOriginal;
use solana_transaction_status_client_types::UiInnerInstructions as UiInnerInstructionsOriginal;
use solders_account::Account;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_response_data_boilerplate::response_data_boilerplate;
use solders_rpc_responses_common::RpcBlockhash;
use solders_transaction_error::TransactionErrorType;
use solders_transaction_return_data::TransactionReturnData;
use solders_transaction_status::{
    UiInnerInstructions, UiLoadedAddresses, UiTransactionTokenBalance,
};

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSimulateTransactionResult(RpcSimulateTransactionResultOriginal);

response_data_boilerplate!(RpcSimulateTransactionResult);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionResult {
    #[pyo3(signature = (err=None, logs=None, accounts=None, units_consumed=None, return_data=None, inner_instructions=None, replacement_blockhash=None, fee=None, loaded_accounts_data_size=None, pre_balances=None, post_balances=None, pre_token_balances=None, post_token_balances=None, loaded_addresses=None))]
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        err: Option<TransactionErrorType>,
        logs: Option<Vec<String>>,
        accounts: Option<Vec<Option<Account>>>,
        units_consumed: Option<u64>,
        return_data: Option<TransactionReturnData>,
        inner_instructions: Option<Vec<UiInnerInstructions>>,
        replacement_blockhash: Option<RpcBlockhash>,
        fee: Option<u64>,
        loaded_accounts_data_size: Option<u32>,
        pre_balances: Option<Vec<u64>>,
        post_balances: Option<Vec<u64>>,
        pre_token_balances: Option<Vec<UiTransactionTokenBalance>>,
        post_token_balances: Option<Vec<UiTransactionTokenBalance>>,
        loaded_addresses: Option<UiLoadedAddresses>,
    ) -> Self {
        let accounts_underlying: Option<Vec<Option<UiAccount>>> = accounts.map(|accs| {
            accs.into_iter()
                .map(|maybe_acc| maybe_acc.map(UiAccount::from))
                .collect()
        });
        let inner_instructions_underlying: Option<Vec<UiInnerInstructionsOriginal>> =
            inner_instructions.map(|ixns| ixns.into_iter().map(Into::into).collect());
        let err_orig = err.map(TransactionErrorOriginal::from);
        Self(RpcSimulateTransactionResultOriginal {
            err: err_orig.map(Into::into),
            logs,
            accounts: accounts_underlying,
            units_consumed,
            return_data: return_data.map(Into::into),
            inner_instructions: inner_instructions_underlying,
            replacement_blockhash: replacement_blockhash.map(Into::into),
            fee,
            loaded_accounts_data_size,
            pre_balances,
            post_balances,
            pre_token_balances: pre_token_balances
                .map(|x| x.into_iter().map(|y| y.into()).collect()),
            post_token_balances: post_token_balances
                .map(|x| x.into_iter().map(|y| y.into()).collect()),
            loaded_addresses: loaded_addresses.map(Into::into),
        })
    }

    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        let orig = self.0.err.clone().map(TransactionErrorOriginal::from);
        orig.map(Into::into)
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

    #[getter]
    pub fn inner_instructions(&self) -> Option<Vec<UiInnerInstructions>> {
        self.0
            .inner_instructions
            .clone()
            .map(|ixns| ixns.into_iter().map(Into::into).collect())
    }

    #[getter]
    pub fn replacement_blockhash(&self) -> Option<RpcBlockhash> {
        self.0.replacement_blockhash.clone().map(Into::into)
    }

    #[getter]
    pub fn fee(&self) -> Option<u64> {
        self.0.fee
    }
    #[getter]
    pub fn loaded_accounts_data_size(&self) -> Option<u32> {
        self.0.loaded_accounts_data_size
    }
    #[getter]
    pub fn pre_balances(&self) -> Option<Vec<u64>> {
        self.0.pre_balances.clone()
    }
    #[getter]
    pub fn post_balances(&self) -> Option<Vec<u64>> {
        self.0.post_balances.clone()
    }
    #[getter]
    pub fn pre_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        let orig = self.0.pre_token_balances.clone();
        orig.map(|x| x.into_iter().map(Into::into).collect())
    }
    #[getter]
    pub fn post_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        let orig = self.0.post_token_balances.clone();
        orig.map(|x| x.into_iter().map(Into::into).collect())
    }
    #[getter]
    pub fn loaded_addresses(&self) -> Option<UiLoadedAddresses> {
        self.0.loaded_addresses.clone().map(Into::into)
    }
}
