use crate::tmp_account_decoder::UiDataSliceConfig as UiDataSliceConfigOriginal;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solders_macros::richcmp_eq_only;

use crate::RichcmpEqualityOnly;

/// Configuration object for limiting returned account data.
///
/// Args:
///     offset (int): Skip this many bytes at the beginning of the data.
///     length (int): Return only this many bytes.
///
#[pyclass(module = "solders.account_decoder")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiDataSliceConfig(UiDataSliceConfigOriginal);

#[richcmp_eq_only]
#[pymethods]
impl UiDataSliceConfig {
    #[new]
    fn new(offset: usize, length: usize) -> Self {
        Self(UiDataSliceConfigOriginal { offset, length })
    }

    #[getter]
    pub fn offset(&self) -> usize {
        self.0.offset
    }

    #[getter]
    pub fn length(&self) -> usize {
        self.0.length
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

impl From<UiDataSliceConfigOriginal> for UiDataSliceConfig {
    fn from(u: UiDataSliceConfigOriginal) -> Self {
        Self(u)
    }
}

impl From<UiDataSliceConfig> for UiDataSliceConfigOriginal {
    fn from(u: UiDataSliceConfig) -> Self {
        u.0
    }
}

impl RichcmpEqualityOnly for UiDataSliceConfig {}

/// Encoding options for account data.
#[pyclass(module = "solders.account_decoder")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum UiAccountEncoding {
    Binary, // Legacy. Retained for RPC backwards compatibility
    Base58,
    Base64,
    JsonParsed,
    #[serde(rename = "base64+zstd")]
    Base64Zstd,
}

pub fn create_account_decoder_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "account_decoder")?;
    m.add_class::<UiDataSliceConfig>()?;
    m.add_class::<UiAccountEncoding>()?;
    Ok(m)
}
