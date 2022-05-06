use std::{fmt, str::FromStr};

use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::hash::{hash, Hash as HashOriginal, HASH_BYTES};

use crate::{calculate_hash, handle_py_value_err, RichcmpFull};

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
        self.to_string()
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
        handle_py_value_err(HashOriginal::from_str(s))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        HashOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_ref()
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

impl From<Hash> for HashOriginal {
    fn from(h: Hash) -> HashOriginal {
        h.0
    }
}

impl AsRef<HashOriginal> for Hash {
    fn as_ref(&self) -> &HashOriginal {
        &self.0
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
