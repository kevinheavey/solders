use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_status_client_types::TransactionConfirmationStatus as TransactionConfirmationStatusOriginal;
use solders_macros::enum_original_mapping;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(TransactionConfirmationStatusOriginal)]
#[pyclass(module = "solders.transaction_status", eq, eq_int)]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}
