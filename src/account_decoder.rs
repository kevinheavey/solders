use std::{fmt::Display, str::FromStr};

use crate::{
    tmp_account_decoder::{
        ParsedAccount as ParsedAccountOriginal, UiDataSliceConfig as UiDataSliceConfigOriginal,
    },
    to_py_err, CommonMethods,
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solders_macros::{common_methods, richcmp_eq_only};

use crate::{
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode, PyBytesBincode,
    PyFromBytesBincode, RichcmpEqualityOnly,
};

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[pyclass(module = "solders.account_decoder")]
pub struct ParsedAccount(ParsedAccountOriginal);

impl RichcmpEqualityOnly for ParsedAccount {}
impl Display for ParsedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pybytes_general_via_bincode!(ParsedAccount);
py_from_bytes_general_via_bincode!(ParsedAccount);
impl<'a> CommonMethods<'a> for ParsedAccount {}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ParsedAccount {
    #[new]
    pub fn new(program: &str, parsed: &str, space: u64) -> PyResult<Self> {
        let value = Value::from_str(parsed).map_err(to_py_err)?;
        Ok(ParsedAccountOriginal {
            program: program.to_owned(),
            parsed: value,
            space,
        }
        .into())
    }

    #[getter]
    pub fn program(&self) -> String {
        self.0.program.clone()
    }

    #[getter]
    pub fn parsed(&self) -> String {
        self.0.parsed.to_string()
    }

    #[getter]
    pub fn space(&self) -> u64 {
        self.0.space
    }
}

impl From<ParsedAccountOriginal> for ParsedAccount {
    fn from(p: ParsedAccountOriginal) -> Self {
        Self(p)
    }
}

impl From<ParsedAccount> for ParsedAccountOriginal {
    fn from(p: ParsedAccount) -> Self {
        p.0
    }
}

pub fn create_account_decoder_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "account_decoder")?;
    m.add_class::<UiDataSliceConfig>()?;
    m.add_class::<UiAccountEncoding>()?;
    m.add_class::<ParsedAccount>()?;
    Ok(m)
}
