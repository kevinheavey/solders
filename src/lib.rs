use bincode::serialize;
use ed25519_dalek::SignatureError as OldSignatureError;
use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};
use solana_sdk::{
    pubkey::{bytes_are_curve_point, Pubkey},
    short_vec::{decode_shortu16_len, ShortU16},
    signer::keypair::Keypair as OldKeypair,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct SignatureError(OldSignatureError);

impl std::convert::From<SignatureError> for PyErr {
    fn from(err: SignatureError) -> PyErr {
        PyValueError::new_err(err.0.to_string())
    }
}

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
#[derive(PartialEq, PartialOrd, Debug, Default)]
pub struct PublicKey(Pubkey);

#[pymethods]
impl PublicKey {
    #[classattr]
    #[pyo3(name = "LENGTH")]
    fn length() -> u8 {
        32
    }

    #[new]
    pub fn new(pubkey_bytes: &[u8]) -> Self {
        PublicKey(Pubkey::new(pubkey_bytes))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        PublicKey(Pubkey::new_unique())
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    pub fn new_from_str(s: &str) -> Self {
        PublicKey(Pubkey::from_str(s).expect("Failed to parse pubkey."))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        PublicKey::default()
    }

    #[staticmethod]
    pub fn create_with_seed(
        from_public_key: &PublicKey,
        seed: &str,
        program_id: &PublicKey,
    ) -> PublicKey {
        PublicKey(Pubkey::create_with_seed(&from_public_key.0, seed, &program_id.0).unwrap())
    }

    #[staticmethod]
    pub fn create_program_address(seeds: Vec<&[u8]>, program_id: &PublicKey) -> PublicKey {
        PublicKey(
            Pubkey::create_program_address(&seeds[..], &program_id.0)
                .expect("Failed to create program address."),
        )
    }

    #[staticmethod]
    pub fn find_program_address(seeds: Vec<&[u8]>, program_id: &PublicKey) -> (PublicKey, u8) {
        let (pubkey, nonce) = Pubkey::find_program_address(&seeds[..], &program_id.0);
        (PublicKey(pubkey), nonce)
    }

    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __bytes__(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn __richcmp__(&self, other: &PublicKey, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Lt => self < other,
            CompareOp::Gt => self > other,
            CompareOp::Le => self <= other,
            CompareOp::Ge => self >= other,
        }
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
        Keypair(OldKeypair::new())
    }

    /// Recovers a `Keypair` from a byte array
    #[staticmethod]
    pub fn from_bytes(raw_bytes: &[u8]) -> Result<Self, SignatureError> {
        let res = OldKeypair::from_bytes(raw_bytes);
        match res {
            Ok(val) => Ok(Keypair(val)),
            Err(val) => Err(SignatureError(val)),
        }
    }
}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn solder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_on_curve, m)?)?;
    m.add_class::<PublicKey>()?;
    m.add_function(wrap_pyfunction!(encode_length, m)?)?;
    m.add_function(wrap_pyfunction!(decode_length, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_is_on_curve() {
    //     let res = is_on_curve(b"\xc1M\xce\x1e\xa4\x86<\xf1\xbc\xfc\x12\xf4\xf2\xe2Y\xf4\x8d\xe4V\xb7\xf9\xd4\\!{\x04\x89j\x1f\xfeA\xdc");
    //     assert!(res);
    // }

    #[test]
    fn test_equality() {
        let left = PublicKey::default();
        let right = PublicKey::default();
        assert_eq!(left, right);
    }

    #[test]
    fn test_decode_length() {
        let bytes = &[0x0];
        let len: u16 = 0x0;
        let left = decode_length(bytes).unwrap();
        let right = (usize::from(len), bytes.len());
        assert_eq!(left, right);
    }

    #[test]
    fn test_decode_length_max_u16() {
        let bytes = &[0xff, 0xff, 0x03];
        let len: u16 = 0xffff;
        let left = decode_length(bytes).unwrap();
        let right = (usize::from(len), bytes.len());
        assert_eq!(left, right);
    }

    #[test]
    fn test_decode_length_empty_bytes() {
        let bytes = b"";
        println!("bytes: {:?}", bytes);
        let len: u16 = 0x0;
        let left = decode_length(bytes).unwrap();
        let right = (usize::from(len), bytes.len());
        assert_eq!(left, right);
    }
}
