use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding as UiAccountEncodingOriginal;

#[pyclass(module = "solders.account_decoder")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiAccountEncoding {
    Binary, // Legacy. Retained for RPC backwards compatibility
    Base58,
    Base64,
    JsonParsed,
    Base64Zstd,
}

impl From<UiAccountEncodingOriginal> for UiAccountEncoding {
    fn from(e: UiAccountEncodingOriginal) -> Self {
        match e {
            UiAccountEncodingOriginal::Binary => Self::Binary,
            UiAccountEncodingOriginal::Base64 => Self::Base64,
            UiAccountEncodingOriginal::Base58 => Self::Base58,
            UiAccountEncodingOriginal::JsonParsed => Self::JsonParsed,
            UiAccountEncodingOriginal::Base64Zstd => Self::Base64Zstd,
        }
    }
}

impl From<UiAccountEncoding> for UiAccountEncodingOriginal {
    fn from(e: UiAccountEncoding) -> Self {
        match e {
            UiAccountEncoding::Binary => Self::Binary,
            UiAccountEncoding::Base64 => Self::Base64,
            UiAccountEncoding::Base58 => Self::Base58,
            UiAccountEncoding::JsonParsed => Self::JsonParsed,
            UiAccountEncoding::Base64Zstd => Self::Base64Zstd,
        }
    }
}
