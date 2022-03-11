use std::str::FromStr;

use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::hash::{hash, Hash as HashOriginal, HASH_BYTES};

use crate::{calculate_hash, richcmp_type_error, to_py_value_err};

#[pyclass]
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Hash(HashOriginal);

#[pymethods]
impl Hash {
    #[new]
    pub fn new(hash_bytes: [u8; HASH_BYTES]) -> Self {
        Self(HashOriginal::new_from_array(hash_bytes))
    }

    #[pyo3(name = "to_string")]
    pub fn string(&self) -> String {
        self.0.to_string()
    }

    pub fn __str__(&self) -> String {
        self.string()
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_string(s: &str) -> PyResult<Self> {
        HashOriginal::from_str(s).map_or_else(|e| Err(to_py_value_err(e)), |v| Ok(Self(v)))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        Self(HashOriginal::new_unique())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
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

    #[staticmethod]
    #[allow(clippy::self_named_constructors)]
    pub fn hash(val: &[u8]) -> Self {
        Self(hash(val))
    }

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}
