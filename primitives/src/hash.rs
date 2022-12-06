use std::str::FromStr;

use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::hash::{
    hash, Hash as HashOriginal, ParseHashError as ParseHashErrorOriginal, HASH_BYTES,
};
use solders_macros::{common_methods, pyhash, richcmp_full};

use solders_traits::{
    handle_py_err, impl_display, pybytes_general_via_slice, CommonMethodsCore, PyFromBytesGeneral,
    PyHash, RichcmpFull,
};

#[pyclass(module = "solders.hash", subclass)]
/// A SHA-256 hash, most commonly used for blockhashes.
///
/// Args:
///     hash_bytes (bytes): the hashed bytes.
///
#[derive(
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Deserialize,
    Serialize,
    From,
    Into,
)]
pub struct Hash(HashOriginal);

#[pyhash]
#[richcmp_full]
#[common_methods]
#[pymethods]
impl Hash {
    #[classattr]
    pub const LENGTH: usize = HASH_BYTES;

    #[new]
    pub fn new(hash_bytes: [u8; HASH_BYTES]) -> Self {
        HashOriginal::new_from_array(hash_bytes).into()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    /// Create a ``Hash`` from a base-58 string.
    ///
    /// Args:
    ///     s (str): The base-58 encoded string
    ///
    /// Returns:
    ///     Hash: a ``Hash`` object.
    ///
    /// Example:
    ///
    ///     >>> from solders.hash import Hash
    ///     >>> Hash.from_string("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM")
    ///     Hash(
    ///         4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM,
    ///     )
    ///
    pub fn new_from_string(s: &str) -> PyResult<Self> {
        handle_py_err(HashOriginal::from_str(s))
    }

    #[staticmethod]
    /// Create a unique Hash for tests and benchmarks.
    ///
    /// Returns:
    ///     Hash: a ``Hash`` object.
    pub fn new_unique() -> Self {
        HashOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// The default ``Hash`` object.
    ///
    /// Returns:
    ///     Hash: a ``Hash`` object.
    /// Example:
    ///     >>> from solders.hash import Hash
    ///     >>> Hash.default()
    ///     Hash(
    ///         11111111111111111111111111111111,
    ///     )
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    #[allow(clippy::self_named_constructors)]
    /// Return a Sha256 hash for the given data.
    ///
    /// Args:
    ///     val (bytes): the data to hash.
    ///
    /// Returns:
    ///     Hash: a ``Hash`` object.
    ///
    /// Example:
    ///     >>> from solders.hash import Hash
    ///     >>> Hash.hash(b"foo")
    ///     Hash(
    ///         3yMApqCuCjXDWPrbjfR5mjCPTHqFG8Pux1TxQrEM35jj,
    ///     )
    pub fn hash(val: &[u8]) -> Self {
        hash(val).into()
    }

    #[staticmethod]
    /// Construct from ``bytes``. Equivalent to ``Hash.__init__`` but included for the sake of consistency.
    ///
    /// Args:
    ///     raw_bytes (bytes): the hashed bytes.
    ///
    /// Returns:
    ///     Hash: a ``Hash`` object.
    ///
    pub fn from_bytes(raw_bytes: [u8; HASH_BYTES]) -> PyResult<Self> {
        Self::py_from_bytes(&raw_bytes)
    }
}

impl PyFromBytesGeneral for Hash {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        Ok(HashOriginal::new(raw).into())
    }
}

pybytes_general_via_slice!(Hash);
solders_traits::common_methods_default!(Hash);

impl RichcmpFull for Hash {}

impl PyHash for Hash {}

impl AsRef<HashOriginal> for Hash {
    fn as_ref(&self) -> &HashOriginal {
        &self.0
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl FromStr for Hash {
    type Err = ParseHashErrorOriginal;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HashOriginal::from_str(s).map(Hash::from)
    }
}

impl_display!(Hash);
