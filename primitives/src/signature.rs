use std::str::FromStr;

use crate::Pubkey;
use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::{ParseSignatureError, Signature as SignatureOriginal, SIGNATURE_BYTES};
use solders_macros::{common_methods, pyhash, richcmp_full};

use solders_traits::{
    handle_py_value_err, impl_display, pybytes_general_via_slice, CommonMethodsCore,
    PyFromBytesGeneral, PyHash, RichcmpFull,
};

#[pyclass(module = "solders.signature", subclass)]
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
    Serialize,
    Deserialize,
    From,
    Into,
)]
/// The ``Signature`` object is a wrapper around a raw bytes signature, typically
/// returned by :meth:`~solders.keypair.Keypair.sign_message` or other similar methods.
///
/// Args:
///     signature_bytes (bytes): The raw signature bytes.
///
pub struct Signature(SignatureOriginal);

#[pyhash]
#[richcmp_full]
#[common_methods]
#[pymethods]
impl Signature {
    #[classattr]
    pub const LENGTH: usize = SIGNATURE_BYTES;

    #[new]
    pub fn new(signature_bytes: [u8; Self::LENGTH]) -> Self {
        SignatureOriginal::new(&signature_bytes).into()
    }

    #[staticmethod]
    /// Create a random siganture.
    ///
    /// Returns:
    ///     Signature: The random signature.
    ///
    pub fn new_unique() -> Self {
        SignatureOriginal::new_unique().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default signature object.
    ///
    /// Returns:
    ///     Signature: The default signature.
    ///
    /// Example:
    ///     >>> from solders.signature import Signature
    ///     >>> Signature.default()
    ///     Signature(
    ///         1111111111111111111111111111111111111111111111111111111111111111,
    ///     )
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    /// Retrieve a signature from a base58-encoded string.
    ///
    /// Args:
    ///     s (str): base58-encoded signature.
    ///
    /// Returns:
    ///     Signature: The decoded signature.
    ///
    /// Example:
    ///     >>> from solders.signature import Signature
    ///     >>> from solders.keypair import Keypair
    ///     >>> sig = Keypair().sign_message(bytes([0]))
    ///     >>> assert Signature.from_string(str(sig)) == sig
    ///
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_value_err(Self::from_str(s))
    }

    /// Check if the signature is a valid signature created by the given pubkey on the given message.
    ///
    /// Args:
    ///     pubkey: The pubkey that is supposed to have signed the message.
    ///     message (bytes): The message in bytes.
    ///
    /// Returns:
    ///     bool: True if verfiication is successful.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.signature import Signature
    ///     >>> kp = Keypair()
    ///     >>> msg = b"macaroni"
    ///     >>> sig = kp.sign_message(msg)
    ///     >>> sig.verify(kp.pubkey(), msg)
    ///     True
    ///
    pub fn verify(&self, pubkey: Pubkey, message_bytes: &[u8]) -> bool {
        self.0.verify(pubkey.as_ref(), message_bytes)
    }

    #[allow(clippy::wrong_self_convention)]
    /// Returns this signature as a byte array.
    ///
    /// Returns:
    ///     list[int]: the signature as a list of 64 u8 ints.
    ///
    /// Example:
    ///      >>> from solders.signature import Signature
    ///      >>> assert Signature.default().to_bytes_array() == [0] * 64
    ///
    pub fn to_bytes_array(&self) -> [u8; 64] {
        self.0.into()
    }

    #[staticmethod]
    /// Construct from ``bytes``. Equivalent to ``Signature.__init__`` but included for the sake of consistency.
    ///
    /// Args:
    ///     raw_bytes (bytes): the signature bytes.
    ///
    /// Returns:
    ///     Signature: a ``Signature`` object.
    ///
    pub fn from_bytes(raw_bytes: [u8; Self::LENGTH]) -> PyResult<Self> {
        Self::py_from_bytes(&raw_bytes)
    }
}

impl PyHash for Signature {}
impl PyFromBytesGeneral for Signature {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        Ok(SignatureOriginal::new(raw).into())
    }
}
solders_traits::common_methods_default!(Signature);
impl RichcmpFull for Signature {}
pybytes_general_via_slice!(Signature);
impl_display!(Signature);

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<SignatureOriginal> for Signature {
    fn as_ref(&self) -> &SignatureOriginal {
        &self.0
    }
}

impl FromStr for Signature {
    type Err = ParseSignatureError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SignatureOriginal::from_str(s).map(Signature::from)
    }
}

pub(crate) fn originals_into_solders(sigs: Vec<SignatureOriginal>) -> Vec<Signature> {
    sigs.into_iter().map(Signature::from).collect()
}

pub(crate) fn solders_into_originals(sigs: Vec<Signature>) -> Vec<SignatureOriginal> {
    sigs.into_iter().map(SignatureOriginal::from).collect()
}
