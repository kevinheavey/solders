use std::str::FromStr;

use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::hash::{hash, Hash as HashOriginal, HASH_BYTES};

use crate::{calculate_hash, to_py_value_err, RichcmpFull};

#[pyclass]
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Hash(HashOriginal);

#[pymethods]
impl Hash {
    #[new]
    pub fn new(hash_bytes: [u8; HASH_BYTES]) -> Self {
        HashOriginal::new_from_array(hash_bytes).into()
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
        HashOriginal::from_str(s).map_or_else(|e| Err(to_py_value_err(e)), |v| Ok(v.into()))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        HashOriginal::new_unique().into()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        self.richcmp(other, op)
    }

    #[staticmethod]
    #[allow(clippy::self_named_constructors)]
    pub fn hash(val: &[u8]) -> Self {
        hash(val).into()
    }

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}

impl RichcmpFull for Hash {}

impl From<HashOriginal> for Hash {
    fn from(h: HashOriginal) -> Self {
        Self(h)
    }
}
