use pyo3::prelude::*;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;

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
