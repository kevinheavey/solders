use bincode::serialize;
use ed25519_dalek::SECRET_KEY_LENGTH;
use pyo3::{
    basic::CompareOp,
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::PyBytes,
};
use solana_sdk::{
    pubkey::{bytes_are_curve_point, Pubkey as OldPubkey},
    short_vec::{decode_shortu16_len, ShortU16},
    signature::{Signature as OldSignature, Signer},
    signer::keypair::{
        keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as OldKeypair,
    },
};
use std::error::Error;
use std::str::FromStr;

fn to_py_value_err<T: Error>(err: T) -> PyErr {
    PyValueError::new_err(err.to_string())
}

// #[derive(Debug)]
// pub struct SignatureError(OldSignatureError);

// impl std::convert::From<SignatureError> for PyErr {
//     fn from(err: SignatureError) -> PyErr {
//         PyValueError::new_err(err.0.to_string())
//     }
// }

/// Check if _bytes s is a valid point on curve or not.
#[pyfunction]
fn is_on_curve(_bytes: &[u8]) -> bool {
    bytes_are_curve_point(_bytes)
}

/// Return the serialized length.
#[pyfunction]
fn encode_length(value: u16) -> Vec<u8> {
    serialize(&ShortU16(value)).unwrap()
}

/// Return the decoded value and how many bytes it consumed.
#[pyfunction]
fn decode_length(raw_bytes: &[u8]) -> PyResult<(usize, usize)> {
    if raw_bytes == b"" {
        return Ok((0, 0));
    }
    let res = decode_shortu16_len(raw_bytes);
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err(PyValueError::new_err("Could not decode value.")),
    }
}
#[pyclass]
#[derive(PartialEq, PartialOrd, Debug, Default, Display)]
pub struct Pubkey(OldPubkey);

#[pymethods]
impl Pubkey {
    #[classattr]
    const LENGTH: u8 = 32;

    #[new]
    pub fn new(pubkey_bytes: &[u8]) -> Self {
        Self(OldPubkey::new(pubkey_bytes))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        Self(OldPubkey::new_unique())
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        match OldPubkey::from_str(s) {
            Ok(val) => Ok(Self(val)),
            Err(val) => Err(PyValueError::new_err(val.to_string())),
        }
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn create_with_seed(from_public_key: &Self, seed: &str, program_id: &Self) -> Self {
        Self(OldPubkey::create_with_seed(&from_public_key.0, seed, &program_id.0).unwrap())
    }

    #[staticmethod]
    pub fn create_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> Self {
        Self(
            OldPubkey::create_program_address(&seeds[..], &program_id.0)
                .expect("Failed to create program address. This is extremely unlikely."),
        )
    }

    #[staticmethod]
    pub fn find_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> (Self, u8) {
        let (pubkey, nonce) = OldPubkey::find_program_address(&seeds[..], &program_id.0);
        (Self(pubkey), nonce)
    }

    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    pub fn string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Lt => self < other,
            CompareOp::Gt => self > other,
            CompareOp::Le => self <= other,
            CompareOp::Ge => self >= other,
        }
    }

    pub fn __hash__(&self) -> PyResult<isize> {
        // call `hash((class_name, bytes(obj)))`
        Python::with_gil(|py| {
            let builtins = PyModule::import(py, "builtins")?;
            let arg1 = "Pubkey";
            let arg2 = self.to_bytes();
            builtins.getattr("hash")?.call1(((arg1, arg2),))?.extract()
        })
    }
}

#[pyclass]
pub struct Signature(OldSignature);

#[pymethods]
impl Signature {
    #[new]
    pub fn new(signature_slice: &[u8]) -> Self {
        Self(OldSignature::new(signature_slice))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        Self(OldSignature::new_unique())
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        match OldSignature::from_str(s) {
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
}

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Keypair(OldKeypair);

#[pymethods]
impl Keypair {
    /// Constructs a new, random `Keypair` using `OsRng`
    #[new]
    pub fn new() -> Self {
        Self(OldKeypair::new())
    }

    /// Recovers a `Keypair` from a byte array
    #[staticmethod]
    pub fn from_bytes(raw_bytes: &[u8]) -> PyResult<Self> {
        let res = OldKeypair::from_bytes(raw_bytes);
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
        Self(OldKeypair::from_base58_string(s))
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

fn richcmp_type_error(op: &str) -> PyErr {
    let msg = format!("{} not supported by Keypair", op);
    PyTypeError::new_err(msg)
}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn solders(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_length, m)?)?;
    m.add_function(wrap_pyfunction!(decode_length, m)?)?;
    m.add_function(wrap_pyfunction!(is_on_curve, m)?)?;
    m.add_class::<Pubkey>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<Signature>()?;
    Ok(())
}
