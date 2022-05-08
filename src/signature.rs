use std::{fmt, str::FromStr};

use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::signature::{Signature as SignatureOriginal, SIGNATURE_BYTES};

use crate::{calculate_hash, handle_py_value_err, RichcmpFull};

#[pyclass]
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Signature(SignatureOriginal);

#[pymethods]
impl Signature {
    #[classattr]
    pub const LENGTH: usize = SIGNATURE_BYTES;

    #[new]
    pub fn new(signature_slice: &[u8]) -> Self {
        SignatureOriginal::new(signature_slice).into()
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        SignatureOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_value_err(SignatureOriginal::from_str(s))
    }

    pub fn verify(&self, pubkey_bytes: [u8; 32], message_bytes: &[u8]) -> bool {
        self.0.verify(&pubkey_bytes, message_bytes)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes_array(&self) -> [u8; 64] {
        self.0.into()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
    }

    #[pyo3(name = "to_string")]
    pub fn string(&self) -> String {
        self.to_string()
    }

    pub fn __str__(&self) -> String {
        self.string()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        self.richcmp(other, op)
    }

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}

impl RichcmpFull for Signature {}

impl From<SignatureOriginal> for Signature {
    fn from(sig: SignatureOriginal) -> Self {
        Self(sig)
    }
}

impl From<Signature> for SignatureOriginal {
    fn from(sig: Signature) -> Self {
        sig.0
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<SignatureOriginal> for Signature {
    fn as_ref(&self) -> &SignatureOriginal {
        &self.0
    }
}
