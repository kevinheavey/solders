use solders_traits::{
    handle_py_value_err, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};
use std::fmt::Display;

use crate::tmp_account_decoder::{
    ParsedAccount as ParsedAccountOriginal, UiDataSliceConfig as UiDataSliceConfigOriginal,
    UiTokenAmount as UiTokenAmountOriginal,
};
use derive_more::{From, Into};
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solders_macros::{common_methods, richcmp_eq_only};

/// Configuration object for limiting returned account data.
///
/// Args:
///     offset (int): Skip this many bytes at the beginning of the data.
///     length (int): Return only this many bytes.
///
#[pyclass(module = "solders.account_decoder")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
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
        format!("{self:#?}")
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, From, Into)]
#[pyclass(module = "solders.account_decoder")]
pub struct ParsedAccount(ParsedAccountOriginal);

impl RichcmpEqualityOnly for ParsedAccount {}
impl Display for ParsedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
pybytes_general_via_bincode!(ParsedAccount);
py_from_bytes_general_via_bincode!(ParsedAccount);
solders_traits::common_methods_default!(ParsedAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ParsedAccount {
    #[new]
    pub fn new(program: &str, parsed: &PyAny, space: u64) -> PyResult<Self> {
        let value = handle_py_value_err(depythonize::<Value>(parsed))?;
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
    pub fn parsed(&self, py: Python<'_>) -> PyResult<PyObject> {
        handle_py_value_err(pythonize(py, &self.0.parsed))
    }

    #[getter]
    pub fn space(&self) -> u64 {
        self.0.space
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, From, Into)]
#[pyclass(module = "solders.account_decoder")]
pub struct UiTokenAmount(UiTokenAmountOriginal);

impl RichcmpEqualityOnly for UiTokenAmount {}
impl Display for UiTokenAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
pybytes_general_via_bincode!(UiTokenAmount);
py_from_bytes_general_via_bincode!(UiTokenAmount);
solders_traits::common_methods_default!(UiTokenAmount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTokenAmount {
    #[pyo3(
        signature = (ui_amount, decimals, amount, ui_amount_string)
    )]
    #[new]
    pub fn new(
        ui_amount: Option<f64>,
        decimals: u8,
        amount: String,
        ui_amount_string: String,
    ) -> PyResult<Self> {
        Ok(UiTokenAmountOriginal {
            ui_amount,
            decimals,
            amount,
            ui_amount_string,
        }
        .into())
    }

    #[getter]
    pub fn ui_amount(&self) -> Option<f64> {
        self.0.ui_amount
    }

    #[getter]
    pub fn decimals(&self) -> u8 {
        self.0.decimals
    }

    #[getter]
    pub fn amount(&self) -> String {
        self.0.amount.clone()
    }

    #[getter]
    pub fn ui_amount_string(&self) -> String {
        self.0.ui_amount_string.clone()
    }
}

pub fn create_account_decoder_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "account_decoder")?;
    m.add_class::<UiDataSliceConfig>()?;
    m.add_class::<UiAccountEncoding>()?;
    m.add_class::<ParsedAccount>()?;
    m.add_class::<UiTokenAmount>()?;
    Ok(m)
}
