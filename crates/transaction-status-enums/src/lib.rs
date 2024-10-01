use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_status::{
    TransactionDetails as TransactionDetailsOriginal,
    UiTransactionEncoding as UiTransactionEncodingOriginal,
};
use solders_macros::enum_original_mapping;

/// Levels of transaction detail to return in RPC requests.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.transaction_status")]
pub enum TransactionDetails {
    Full,
    Signatures,
    #[serde(rename = "none")]
    None_,
    Accounts,
}

impl Default for TransactionDetails {
    fn default() -> Self {
        Self::Full
    }
}

impl From<TransactionDetails> for TransactionDetailsOriginal {
    fn from(value: TransactionDetails) -> Self {
        match value {
            TransactionDetails::Full => Self::Full,
            TransactionDetails::Signatures => Self::Signatures,
            TransactionDetails::None_ => Self::None,
            TransactionDetails::Accounts => Self::Accounts,
        }
    }
}

impl From<TransactionDetailsOriginal> for TransactionDetails {
    fn from(value: TransactionDetailsOriginal) -> Self {
        match value {
            TransactionDetailsOriginal::Full => Self::Full,
            TransactionDetailsOriginal::Signatures => Self::Signatures,
            TransactionDetailsOriginal::None => Self::None_,
            TransactionDetailsOriginal::Accounts => Self::Accounts,
        }
    }
}

/// Encoding options for transaction data.
#[pyclass(module = "solders.transaction_status")]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(UiTransactionEncodingOriginal)]
pub enum UiTransactionEncoding {
    Binary, // Legacy. Retained for RPC backwards compatibility
    Base64,
    Base58,
    Json,
    JsonParsed,
}

impl Default for UiTransactionEncoding {
    fn default() -> Self {
        Self::Base64
    }
}
