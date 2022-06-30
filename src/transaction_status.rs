use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_status::{
    TransactionDetails as TransactionDetailsOriginal,
    UiTransactionEncoding as UiTransactionEncodingOriginal,
};

/// Encoding options for transaction data.
#[pyclass(module = "solders.transaction_status")]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiTransactionEncoding {
    Binary, // Legacy. Retained for RPC backwards compatibility
    Base64,
    Base58,
    Json,
    JsonParsed,
}

impl From<UiTransactionEncodingOriginal> for UiTransactionEncoding {
    fn from(e: UiTransactionEncodingOriginal) -> Self {
        match e {
            UiTransactionEncodingOriginal::Binary => Self::Binary,
            UiTransactionEncodingOriginal::Base64 => Self::Base64,
            UiTransactionEncodingOriginal::Base58 => Self::Base58,
            UiTransactionEncodingOriginal::Json => Self::Json,
            UiTransactionEncodingOriginal::JsonParsed => Self::JsonParsed,
        }
    }
}

impl From<UiTransactionEncoding> for UiTransactionEncodingOriginal {
    fn from(e: UiTransactionEncoding) -> Self {
        match e {
            UiTransactionEncoding::Binary => Self::Binary,
            UiTransactionEncoding::Base64 => Self::Base64,
            UiTransactionEncoding::Base58 => Self::Base58,
            UiTransactionEncoding::Json => Self::Json,
            UiTransactionEncoding::JsonParsed => Self::JsonParsed,
        }
    }
}

/// Levels of transaction detail to return in RPC requests.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub enum TransactionDetails {
    Full,
    Signatures,
    None_,
}

impl Default for TransactionDetails {
    fn default() -> Self {
        TransactionDetailsOriginal::default().into()
    }
}

impl From<TransactionDetailsOriginal> for TransactionDetails {
    fn from(d: TransactionDetailsOriginal) -> Self {
        match d {
            TransactionDetailsOriginal::Full => Self::Full,
            TransactionDetailsOriginal::Signatures => Self::Signatures,
            TransactionDetailsOriginal::None => Self::None_,
        }
    }
}

impl From<TransactionDetails> for TransactionDetailsOriginal {
    fn from(d: TransactionDetails) -> Self {
        match d {
            TransactionDetails::Full => Self::Full,
            TransactionDetails::Signatures => Self::Signatures,
            TransactionDetails::None_ => Self::None,
        }
    }
}

pub fn create_transaction_status_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "transaction_status")?;
    m.add_class::<TransactionDetails>()?;
    m.add_class::<UiTransactionEncoding>()?;
    Ok(m)
}
