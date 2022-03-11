use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*, types::PyBytes};
use solana_sdk::signer::{
    keypair::{
        keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as KeypairOriginal,
    },
    Signer,
};

use crate::{pubkey::Pubkey, richcmp_type_error, signature::Signature};

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Keypair(KeypairOriginal);

#[pymethods]
impl Keypair {
    /// Constructs a new, random `Keypair` using `OsRng`
    #[new]
    pub fn new() -> Self {
        Self(KeypairOriginal::new())
    }

    /// Recovers a `Keypair` from a byte array
    #[staticmethod]
    pub fn from_bytes(raw_bytes: &[u8]) -> PyResult<Self> {
        let res = KeypairOriginal::from_bytes(raw_bytes);
        match res {
            Ok(val) => Ok(Keypair(val)),
            Err(val) => Err(PyValueError::new_err(val.to_string())),
        }
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
        Self(KeypairOriginal::from_base58_string(s))
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
        Pubkey(self.0.pubkey())
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature {
        Signature(self.0.sign_message(message))
    }

    #[staticmethod]
    pub fn from_seed(seed: &[u8]) -> PyResult<Self> {
        match keypair_from_seed(seed) {
            Ok(val) => Ok(Keypair(val)),
            Err(val) => Err(PyValueError::new_err(val.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_seed_phrase_and_passphrase(seed_phrase: &str, passphrase: &str) -> PyResult<Self> {
        match keypair_from_seed_phrase_and_passphrase(seed_phrase, passphrase) {
            Ok(val) => Ok(Keypair(val)),
            Err(val) => Err(PyValueError::new_err(val.to_string())),
        }
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

    pub fn __hash__(&self) -> PyResult<isize> {
        // call `hash((class_name, bytes(obj)))`
        Python::with_gil(|py| {
            let builtins = PyModule::import(py, "builtins")?;
            let arg1 = "Keypair";
            let arg2 = self.__bytes__(py);
            builtins.getattr("hash")?.call1(((arg1, arg2),))?.extract()
        })
    }
}
