use std::{fmt, hash::Hash, str::FromStr};

use crate::{calculate_hash, handle_py_err, handle_py_value_err, PyErrWrapper, RichcmpFull};
use pyo3::{basic::CompareOp, create_exception, exceptions::PyException, prelude::*};
use solana_sdk::pubkey::{
    Pubkey as PubkeyOriginal, PubkeyError as PubkeyErrorOriginal, PUBKEY_BYTES,
};

create_exception!(
    solders,
    PubkeyError,
    PyException,
    "Umbrella error for the ``Pubkey`` object."
);

impl From<PubkeyErrorOriginal> for PyErrWrapper {
    fn from(e: PubkeyErrorOriginal) -> Self {
        Self(PubkeyError::new_err(e.to_string()))
    }
}
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

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn __bytes__(&self) -> &[u8] {
        self.as_ref()
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
