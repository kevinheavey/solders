use pyo3::{prelude::*, pyclass::CompareOp, types::PyBytes};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{
        keypair::{
            keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as KeypairOriginal,
        },
        Signer as SignerTrait, SignerError,
    },
};

use crate::{
    handle_py_value_err, pubkey::Pubkey, signature::Signature, RichcmpEqOnlyPrecalculated, Signer,
};

#[pyclass(module = "solders", subclass)]
#[derive(PartialEq, Debug)]
/// A vanilla Ed25519 key pair.
///
/// Calling ``Keypair()`` creates a new, random ``Keypair``.
///
/// Example::
///     from solders.keypair import Keypair
///     
///     assert Keypair() != Keypair()
///
pub struct Keypair(pub KeypairOriginal);

#[pymethods]
impl Keypair {
    #[classattr]
    /// The length of a keypair in bytes.
    const LENGTH: usize = 64;
    #[new]
    /// Constructs a new, random ``Keypair`` using ``OsRng``
    pub fn new() -> Self {
        KeypairOriginal::new().into()
    }

    /// Recovers a ``Keypair`` from bytes.
    ///
    /// Args:
    ///     raw_bytes (bytes): a 64-byte keypair.
    ///
    /// Returns:
    ///     Keypair: a keypair object.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     kp = Keypair()
    ///     assert kp == Keypair.from_bytes(bytes(kp))
    ///
    #[staticmethod]
    pub fn from_bytes(raw_bytes: [u8; Self::LENGTH]) -> PyResult<Self> {
        handle_py_value_err(KeypairOriginal::from_bytes(&raw_bytes))
    }

    /// Returns this ``Keypair`` as a byte array.
    ///
    /// Returns:
    ///     list[int]: the keypair as a list of 64 u8 ints.
    ///
    /// Example::
    ///      from solders.keypair import Keypair
    ///      raw_bytes = bytes([1] * 64)
    ///      assert Keypair.from_bytes(raw_bytes).to_bytes_array() == list(raw_bytes)
    ///
    pub fn to_bytes_array(&self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    pub fn __bytes__<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.to_bytes_array().as_slice())
    }

    #[staticmethod]
    /// Recovers a ``Keypair`` from a base58-encoded string.
    ///
    /// Args:
    ///     s (str): The base58-encoded string.
    ///
    /// Returns:
    ///     Keypair: a keypair oject.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     
    ///     raw_bytes = bytes([0] * 64)
    ///     base58_str = "1" * 64
    ///     assert Keypair.from_base58_string(base58_str) == Keypair.from_bytes(raw_bytes)
    ///     
    pub fn from_base58_string(s: &str) -> Self {
        KeypairOriginal::from_base58_string(s).into()
    }
    /// Gets this ``Keypair``'s secret key.
    ///
    /// Returns:
    ///     bytes: The secret key in 32 bytes.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     
    ///     kp = Keypair.from_bytes(bytes([1] * 64))
    ///     assert kp.secret() == bytes([1] * 32)
    ///
    pub fn secret(&self) -> &[u8] {
        self.0.secret().as_ref()
    }

    /// Return this keypair as a base58-encoded string.
    ///
    /// Returns:
    ///     str: the base58-encoded string.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> Keypair.from_bytes([1] * 64).to_base58_string()
    ///     '2AXDGYSE4f2sz7tvMMzyHvUfcoJmxudvdhBcmiUSo6ijwfYmfZYsKRxboQMPh3R4kUhXRVdtSXFXMheka4Rc4P2'
    pub fn to_base58_string(&self) -> String {
        self.0.to_base58_string()
    }

    pub fn __str__(&self) -> String {
        self.to_base58_string()
    }

    #[pyo3(name = "pubkey")]
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    pub fn py_sign_message(&self, message: &[u8]) -> Signature {
        self.sign_message(message).into()
    }

    #[staticmethod]
    pub fn from_seed(seed: [u8; 32]) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed(&seed))
    }

    #[staticmethod]
    pub fn from_seed_phrase_and_passphrase(seed_phrase: &str, passphrase: &str) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed_phrase_and_passphrase(
            seed_phrase,
            passphrase,
        ))
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

    fn __richcmp__(&self, other: Signer, op: CompareOp) -> PyResult<bool> {
        let other_eq = match other {
            Signer::KeypairWrapper(kp) => kp.0 == self.0,
            Signer::PresignerWrapper(ps) => ps.0 == self.0,
        };
        self.richcmp(other_eq, op)
    }

    #[pyo3(name = "is_interactive")]
    pub fn py_is_interactive(&self) -> bool {
        self.is_interactive()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

impl RichcmpEqOnlyPrecalculated for Keypair {}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

impl From<KeypairOriginal> for Keypair {
    fn from(keypair: KeypairOriginal) -> Self {
        Self(keypair)
    }
}

impl From<Keypair> for KeypairOriginal {
    fn from(k: Keypair) -> Self {
        k.0
    }
}

impl AsRef<KeypairOriginal> for Keypair {
    fn as_ref(&self) -> &KeypairOriginal {
        &self.0
    }
}

impl Clone for Keypair {
    fn clone(&self) -> Self {
        Self::from_bytes(self.to_bytes_array()).unwrap()
    }
}

impl SignerTrait for Keypair {
    fn pubkey(&self) -> PubkeyOriginal {
        self.0.pubkey()
    }
    fn try_pubkey(&self) -> Result<PubkeyOriginal, SignerError> {
        self.0.try_pubkey()
    }
    fn sign_message(&self, message: &[u8]) -> SignatureOriginal {
        self.0.sign_message(message)
    }
    fn try_sign_message(&self, message: &[u8]) -> Result<SignatureOriginal, SignerError> {
        self.0.try_sign_message(message)
    }
    fn is_interactive(&self) -> bool {
        self.0.is_interactive()
    }
}
