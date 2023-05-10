use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_status::TransactionConfirmationStatus as TransactionConfirmationStatusOriginal;
use solders_macros::enum_original_mapping;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(TransactionConfirmationStatusOriginal)]
#[pyclass(module = "solders.transaction_status")]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}
