use std::{fmt, str::FromStr};

use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::signature::{Signature as SignatureOriginal, SIGNATURE_BYTES};

use crate::{calculate_hash, handle_py_value_err, Pubkey, RichcmpFull};

#[pyclass(module = "solders.signature", subclass)]
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// The ``Signature`` object is a wrapper around a raw bytes signature, typically
/// returned by :meth:`~solders.keypair.Keypair.sign_message` or other similar methods.
///
/// Args:
///     signature_bytes (bytes): The raw signature bytes.
///
pub struct Signature(SignatureOriginal);

#[pymethods]
impl Signature {
    #[classattr]
    pub const LENGTH: usize = SIGNATURE_BYTES;

    #[new]
    pub fn new(signature_bytes: &[u8]) -> Self {
        SignatureOriginal::new(signature_bytes).into()
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
        handle_py_value_err(SignatureOriginal::from_str(s))
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

    pub fn __bytes__(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        self.richcmp(other, op)
    }

    pub fn __hash__(&self) -> u64 {
        calculate_hash(self)
    }
}

impl RichcmpFull for Signature {}

impl From<SignatureOriginal> for Signature {
    fn from(sig: SignatureOriginal) -> Self {
        Self(sig)
    }
}

impl From<Signature> for SignatureOriginal {
    fn from(sig: Signature) -> Self {
        sig.0
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
