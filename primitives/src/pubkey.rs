use std::{hash::Hash, str::FromStr};

use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey as PubkeyOriginal, PUBKEY_BYTES};
use solders_macros::{common_methods, pyhash, richcmp_full};
use solders_traits::{
    handle_py_err, handle_py_value_err, pybytes_general_via_slice, CommonMethodsCore,
    PyFromBytesGeneral, PyHash, RichcmpFull,
};

/// A public key.
///
/// Args:
///      pubkey_bytes (bytes): The pubkey in bytes.
///
/// Example:
///     >>> from solders.pubkey import Pubkey
///     >>> pubkey = Pubkey(bytes([1] * 32))
///     >>> str(pubkey)
///     '4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi'
///     >>> bytes(pubkey).hex()
///     '0101010101010101010101010101010101010101010101010101010101010101'
///
#[pyclass(module = "solders.pubkey", subclass)]
#[derive(
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Default,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    From,
    Into,
)]
pub struct Pubkey(pub PubkeyOriginal);

#[pyhash]
#[richcmp_full]
#[common_methods]
#[pymethods]
impl Pubkey {
    #[classattr]
    pub const LENGTH: usize = PUBKEY_BYTES;

    #[new]
    pub fn new(pubkey_bytes: [u8; PUBKEY_BYTES]) -> Self {
        PubkeyOriginal::new_from_array(pubkey_bytes).into()
    }

    #[staticmethod]
    /// Unique pubkey for tests and benchmarks.
    ///
    /// Returns:
    ///     Pubkey: Randomly generated pubkey.
    pub fn new_unique() -> Self {
        PubkeyOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    /// Retrieve a pubkey from a base58 string.
    ///
    /// Returns:
    ///     Pubkey: the pubkey obtained from the base58 string.
    ///
    /// Example:
    ///
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> Pubkey.from_string("BPFLoader1111111111111111111111111111111111")
    ///     Pubkey(
    ///         BPFLoader1111111111111111111111111111111111,
    ///     )
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_value_err(PubkeyOriginal::from_str(s))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default pubkey.
    ///
    /// Returns:
    ///     Pubkey: The default pubkey.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> Pubkey.default()
    ///     Pubkey(
    ///         11111111111111111111111111111111,
    ///     )
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    /// Derive a pubkey from another key, a seed, and a program ID.
    ///
    /// Args:
    ///     base (Pubkey): The other pubkey to use.
    ///     seed (str): The seed string
    ///     program_id (Pubkey): The program ID.
    ///
    /// Returns:
    ///     Pubkey: The derived pubkey.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> default_public_key = Pubkey.default()
    ///     >>> Pubkey.create_with_seed(default_public_key, "limber chicken: 4/45", default_public_key)
    ///     Pubkey(
    ///         9h1HyLCW5dZnBVap8C5egQ9Z6pHyjsh5MNy83iPqqRuq,
    ///     )
    ///
    pub fn create_with_seed(base: &Self, seed: &str, program_id: &Self) -> PyResult<Self> {
        handle_py_err(PubkeyOriginal::create_with_seed(
            &base.0,
            seed,
            &program_id.0,
        ))
    }

    #[staticmethod]
    /// Derive a program address from seeds and a program ID.
    ///
    /// Args:
    ///     seeds (Sequence[bytes]): The seeds to use.
    ///     program_id (Pubkey): The program ID.
    ///
    /// Returns:
    ///     Pubkey: The derived program address.
    ///
    /// Example:
    ///
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> program_id = Pubkey.from_string("BPFLoader1111111111111111111111111111111111")
    ///     >>> Pubkey.create_program_address([b"", bytes([1])], program_id)
    ///     Pubkey(
    ///         3gF2KMe9KiC6FNVBmfg9i267aMPvK37FewCip4eGBFcT,
    ///     )
    ///
    pub fn create_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> Self {
        PubkeyOriginal::create_program_address(&seeds, &program_id.0)
            .expect("Failed to create program address. This is extremely unlikely.")
            .into()
    }

    #[staticmethod]
    /// Find a valid `program derived address <https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses>`_ and its corresponding bump seed.
    ///
    /// Program derived addresses (PDAs) are account keys that only the program,
    /// ``program_id``, has the authority to sign. The address is of the same form
    /// as a Solana ``Pubkey``, except they are ensured to not be on the ed25519
    /// curve and thus have no associated private key. When performing
    /// cross-program invocations the program can "sign" for the key by calling
    /// ``invoke_signed`` and passing the same seeds used to generate the
    /// address, along with the calculated *bump seed*, which this function
    /// returns as the second tuple element. The runtime will verify that the
    /// program associated with this address is the caller and thus authorized
    /// to be the signer.
    ///
    /// The ``seeds`` are application-specific, and must be carefully selected to
    /// uniquely derive accounts per application requirements. It is common to
    /// use static strings and other pubkeys as seeds.
    ///
    /// Because the program address must not lie on the ed25519 curve, there may
    /// be seed and program id combinations that are invalid. For this reason,
    /// an extra seed (the bump seed) is calculated that results in a
    /// point off the curve. The bump seed must be passed as an additional seed
    /// when calling ``invoke_signed``.
    ///
    ///
    /// **Warning**: Because of the way the seeds are hashed there is a potential
    /// for program address collisions for the same program id.  The seeds are
    /// hashed sequentially which means that seeds {"abcdef"}, {"abc", "def"},
    /// and {"ab", "cd", "ef"} will all result in the same program address given
    /// the same program id. Since the chance of collision is local to a given
    /// program id, the developer of that program must take care to choose seeds
    /// that do not collide with each other. For seed schemes that are susceptible
    /// to this type of hash collision, a common remedy is to insert separators
    /// between seeds, e.g. transforming {"abc", "def"} into {"abc", "-", "def"}.
    ///
    /// Args:
    ///     seeds (Sequence[bytes]): The seeds to use.
    ///     program_id (Pubkey): The program ID.
    ///
    /// Returns:
    ///     Pubkey: The PDA.
    ///
    /// Example:
    ///
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> program_id = Pubkey.from_string("BPFLoader1111111111111111111111111111111111")
    ///     >>> program_address, nonce = Pubkey.find_program_address([b""], program_id)
    ///     >>> program_address
    ///     Pubkey(
    ///         EXWkUCz3YJU9TDVk39ogA4TwoVsUi75ZDhH6yT7acPgQ,
    ///     )
    ///     >>> nonce
    ///     255
    ///
    pub fn find_program_address(seeds: Vec<&[u8]>, program_id: &Self) -> (Self, u8) {
        let (pubkey, nonce) = PubkeyOriginal::find_program_address(&seeds, &program_id.0);
        (pubkey.into(), nonce)
    }

    /// Check that the pubkey is on the ed25519 curve.
    ///
    /// Returns:
    ///     bool: `True` if the pubkey is on the curve.
    ///
    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    #[staticmethod]
    /// Construct from ``bytes``. Equivalent to ``Pubkey.__init__`` but included for the sake of consistency.
    ///
    /// Args:
    ///     raw (bytes): the pubkey bytes.
    ///
    /// Returns:
    ///     Pubkey: a ``Pubkey`` object.
    ///
    pub fn from_bytes(raw: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(raw)
    }
}

impl RichcmpFull for Pubkey {}
impl PyHash for Pubkey {}

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

impl std::fmt::Display for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pybytes_general_via_slice!(Pubkey);
impl PyFromBytesGeneral for Pubkey {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        Ok(PubkeyOriginal::new(raw).into())
    }
}
solders_traits::common_methods_default!(Pubkey);

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

impl FromStr for Pubkey {
    type Err = ParsePubkeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PubkeyOriginal::from_str(s).map(Pubkey::from)
    }
}
