use std::str::FromStr;

use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};
use solana_sdk::signature::Signature as SignatureOriginal;

use crate::richcmp_type_error;

#[pyclass]
#[derive(PartialEq, Debug, Default)]
pub struct Signature(pub SignatureOriginal);

#[pymethods]
impl Signature {
    #[new]
    pub fn new(signature_slice: &[u8]) -> Self {
        Self(SignatureOriginal::new(signature_slice))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        Self(SignatureOriginal::new_unique())
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        match SignatureOriginal::from_str(s) {
            Ok(val) => Ok(Self(val)),
            Err(val) => Err(PyValueError::new_err(val.to_string())),
        }
    }

    pub fn verify(&self, pubkey_bytes: &[u8], message_bytes: &[u8]) -> bool {
        self.0.verify(pubkey_bytes, message_bytes)
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
    }

    #[pyo3(name = "to_string")]
    pub fn string(&self) -> String {
        self.0.to_string()
    }

    pub fn __str__(&self) -> String {
        self.string()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self == other),
            CompareOp::Ne => Ok(self != other),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}
