use std::{fmt, hash::Hash, str::FromStr};

use crate::{calculate_hash, handle_py_value_err, RichcmpFull};
use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::pubkey::{Pubkey as PubkeyOriginal, PUBKEY_BYTES};
/// A public key.
///
/// Args:
///      pubkey_bytes (bytes): The pubkey in bytes.
///
/// Example:
///     >>> from solders import Pubkey
///     >>> pubkey = Pubkey(bytes([1] * 32))
///     >>> str(pubkey)
///     '4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi'
///     >>> bytes(pubkey).hex()
///     '0101010101010101010101010101010101010101010101010101010101010101'
///
#[pyclass]
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Clone)]
pub struct Pubkey(PubkeyOriginal);

#[pymethods]
impl Pubkey {
    #[classattr]
    const LENGTH: usize = PUBKEY_BYTES;

    #[new]
    pub fn new(pubkey_bytes: [u8; PUBKEY_BYTES]) -> Self {
        PubkeyOriginal::new_from_array(pubkey_bytes).into()
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        PubkeyOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_value_err(PubkeyOriginal::from_str(s))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn create_with_seed(
        from_public_key: &Self,
        seed: &str,
        program_id: &Self,
    ) -> PyResult<Self> {
        handle_py_value_err(PubkeyOriginal::create_with_seed(
            &from_public_key.0,
            seed,
            &program_id.0,
        ))
    }

    #[staticmethod]
    pub fn create_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> Self {
        PubkeyOriginal::create_program_address(&seeds[..], &program_id.0)
            .expect("Failed to create program address. This is extremely unlikely.")
            .into()
    }

    #[staticmethod]
    pub fn find_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> (Self, u8) {
        let (pubkey, nonce) = PubkeyOriginal::find_program_address(&seeds[..], &program_id.0);
        (pubkey.into(), nonce)
    }

    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    #[pyo3(name = "to_string")]
    pub fn string(&self) -> String {
        self.to_string()
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __bytes__(&self) -> &[u8] {
        self.to_bytes()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        self.richcmp(other, op)
    }

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}

impl RichcmpFull for Pubkey {}

impl From<PubkeyOriginal> for Pubkey {
    fn from(pubkey: PubkeyOriginal) -> Self {
        Self(pubkey)
    }
}

impl From<&PubkeyOriginal> for Pubkey {
    fn from(pubkey: &PubkeyOriginal) -> Self {
        Self(*pubkey)
    }
}

impl From<&Pubkey> for PubkeyOriginal {
    fn from(pubkey: &Pubkey) -> Self {
        pubkey.0
    }
}

impl From<Pubkey> for PubkeyOriginal {
    fn from(pubkey: Pubkey) -> Self {
        pubkey.0
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<[u8]> for Pubkey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<PubkeyOriginal> for Pubkey {
    fn as_ref(&self) -> &PubkeyOriginal {
        &self.0
    }
}
