use pyo3::{exceptions::PyValueError, prelude::*, pyclass::CompareOp, types::PyBytes};
use solana_sdk::signer::{
    keypair::{
        keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as KeypairOriginal,
    },
    Signer,
};

use crate::{pubkey::Pubkey, signature::Signature, to_py_value_err, RichcmpEqualityOnly};

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Keypair(KeypairOriginal);

#[pymethods]
impl Keypair {
    /// Constructs a new, random `Keypair` using `OsRng`
    #[new]
    pub fn new() -> Self {
        KeypairOriginal::new().into()
    }

    /// Recovers a `Keypair` from a byte array
    #[staticmethod]
    pub fn from_bytes(raw_bytes: &[u8]) -> PyResult<Self> {
        KeypairOriginal::from_bytes(raw_bytes)
            .map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
    }

    /// Returns this `Keypair` as a byte array
    pub fn to_bytes_array(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn __bytes__<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.to_bytes_array().as_slice())
    }

    /// Recovers a `Keypair` from a base58-encoded string
    #[staticmethod]
    pub fn from_base58_string(s: &str) -> Self {
        KeypairOriginal::from_base58_string(s).into()
    }
    /// Gets this `Keypair`'s secret key
    pub fn secret(&self) -> &[u8] {
        self.0.secret().as_ref()
    }

    pub fn to_base58_string(&self) -> String {
        self.0.to_base58_string()
    }

    pub fn __str__(&self) -> String {
        self.to_base58_string()
    }

    pub fn pubkey(&self) -> Pubkey {
        self.0.pubkey().into()
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature {
        self.0.sign_message(message).into()
    }

    #[staticmethod]
    pub fn from_seed(seed: &[u8]) -> PyResult<Self> {
        keypair_from_seed(seed).map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
    }

    #[staticmethod]
    pub fn from_seed_phrase_and_passphrase(seed_phrase: &str, passphrase: &str) -> PyResult<Self> {
        keypair_from_seed_phrase_and_passphrase(seed_phrase, passphrase)
            .map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
    }

    pub fn __hash__(&self) -> PyResult<isize> {
        // call `hash((class_name, bytes(obj)))`
        Python::with_gil(|py| {
            let builtins = PyModule::import(py, "builtins")?;
            let arg1 = "Keypair";
            let arg2 = self.__bytes__(py);
            builtins.getattr("hash")?.call1(((arg1, arg2),))?.extract()
        })
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        self.richcmp(other, op)
    }
}

impl RichcmpEqualityOnly for Keypair {}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

impl From<KeypairOriginal> for Keypair {
    fn from(keypair: KeypairOriginal) -> Self {
        Self(keypair)
    }
}
