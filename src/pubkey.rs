use std::{fmt, hash::Hash, str::FromStr};

use crate::{calculate_hash, to_py_value_err, RichcmpFull};
use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::pubkey::{Pubkey as PubkeyOriginal, PUBKEY_BYTES};
#[pyclass]
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Clone)]
pub struct Pubkey(pub PubkeyOriginal);

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
        PubkeyOriginal::from_str(s).map_or_else(|e| Err(to_py_value_err(e)), |v| Ok(v.into()))
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
        PubkeyOriginal::create_with_seed(&from_public_key.0, seed, &program_id.0)
            .map_or_else(|e| Err(to_py_value_err(e)), |v| Ok(v.into()))
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

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
