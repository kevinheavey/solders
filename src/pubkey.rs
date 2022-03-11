use std::{hash::Hash, str::FromStr};

use crate::calculate_hash;
use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};
use solana_sdk::pubkey::Pubkey as PubkeyOriginal;

#[pyclass]
#[derive(PartialEq, PartialOrd, Debug, Default, Hash)]
pub struct Pubkey(pub PubkeyOriginal);

#[pymethods]
impl Pubkey {
    #[classattr]
    const LENGTH: u8 = 32;

    #[new]
    pub fn new(pubkey_bytes: &[u8]) -> Self {
        Self(PubkeyOriginal::new(pubkey_bytes))
    }

    #[staticmethod]
    pub fn new_unique() -> Self {
        Self(PubkeyOriginal::new_unique())
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        match PubkeyOriginal::from_str(s) {
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
        Self(PubkeyOriginal::create_with_seed(&from_public_key.0, seed, &program_id.0).unwrap())
    }

    #[staticmethod]
    pub fn create_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> Self {
        Self(
            PubkeyOriginal::create_program_address(&seeds[..], &program_id.0)
                .expect("Failed to create program address. This is extremely unlikely."),
        )
    }

    #[staticmethod]
    pub fn find_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> (Self, u8) {
        let (pubkey, nonce) = PubkeyOriginal::find_program_address(&seeds[..], &program_id.0);
        (Self(pubkey), nonce)
    }

    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    #[pyo3(name = "to_string")]
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
        format!("{:#?}", self)
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

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}
