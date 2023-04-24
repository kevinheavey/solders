use crate::{pubkey::Pubkey, signature::Signature};
use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::signer::{
    keypair::{
        keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as KeypairOriginal,
    },
    Signer as SignerTrait,
};
use solders_macros::{common_methods, pyhash, richcmp_signer};

use solders_traits::{
    handle_py_value_err, impl_display, impl_signer_hash, CommonMethods, CommonMethodsCore,
    PyBytesGeneral, PyFromBytesGeneral, PyHash, RichcmpSigner, SignerTraitWrapper,
    ToSignerOriginal,
};

mod keypair_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use solana_sdk::signer::keypair::Keypair as KeypairOriginal;

    pub fn serialize<S>(kp: &KeypairOriginal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&kp.to_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeypairOriginal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = Vec::deserialize(deserializer)?;
        KeypairOriginal::from_bytes(&b).map_err(serde::de::Error::custom)
    }
}

#[pyclass(module = "solders.keypair", subclass)]
#[derive(PartialEq, Debug, Serialize, Deserialize, From, Into)]
/// A vanilla Ed25519 key pair.
///
/// Calling ``Keypair()`` creates a new, random ``Keypair``.
///
/// Example:
///     >>> from solders.keypair import Keypair
///     >>> assert Keypair() != Keypair()
///
pub struct Keypair(#[serde(with = "keypair_serde")] pub KeypairOriginal);

#[pyhash]
#[richcmp_signer]
#[common_methods]
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
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> kp = Keypair()
    ///     >>> assert kp == Keypair.from_bytes(bytes(kp))
    ///
    #[staticmethod]
    pub fn from_bytes(raw_bytes: [u8; Self::LENGTH]) -> PyResult<Self> {
        Self::py_from_bytes(&raw_bytes)
    }

    /// Returns this ``Keypair`` as a byte array.
    ///
    /// Returns:
    ///     list[int]: the keypair as a list of 64 u8 ints.
    ///
    /// Example:
    ///      >>> from solders.keypair import Keypair
    ///      >>> raw_bytes = bytes([1] * 64)
    ///      >>> assert Keypair.from_bytes(raw_bytes).to_bytes_array() == list(raw_bytes)
    ///
    pub fn to_bytes_array(&self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
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
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> raw_bytes = bytes([0] * 64)
    ///     >>> base58_str = "1" * 64
    ///     >>> kp = Keypair.from_base58_string(base58_str)
    ///     >>> assert kp == Keypair.from_bytes(raw_bytes)
    ///     >>> assert str(kp) == base58_str
    ///     
    pub fn from_base58_string(s: &str) -> Self {
        KeypairOriginal::from_base58_string(s).into()
    }
    /// Gets this ``Keypair``'s secret key.
    ///
    /// Returns:
    ///     bytes: The secret key in 32 bytes.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> kp = Keypair()
    ///     >>> assert kp.secret() == bytes(kp)[:32]
    ///
    pub fn secret(&self) -> &[u8] {
        self.0.secret().as_ref()
    }

    #[pyo3(name = "pubkey")]
    /// Get this keypair's :class:`~solders.pubkey.Pubkey`.
    ///
    /// Returns:
    ///     Pubkey: the pubkey of this keypair.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> seed_bytes = bytes([0] * 32)
    ///     >>> pubkey_bytes = bytes([1] * 32)
    ///     >>> kp = Keypair.from_bytes(seed_bytes + pubkey_bytes)
    ///     >>> assert kp.pubkey() == Pubkey(pubkey_bytes)
    ///
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    /// Sign a mesage with this keypair, producing an Ed25519 signature over the provided message bytes.
    ///
    /// Args:
    ///     message (bytes): The message to sign.
    ///
    /// Returns:
    ///     Signature: The Ed25519 signature.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> seed = bytes([1] * 32)
    ///     >>> keypair = Keypair.from_seed(seed)
    ///     >>> msg = b"hello"
    ///     >>> sig = keypair.sign_message(msg)
    ///     >>> bytes(sig).hex()
    ///     'e1430c6ebd0d53573b5c803452174f8991ef5955e0906a09e8fdc7310459e9c82a402526748c3431fe7f0e5faafbf7e703234789734063ee42be17af16438d08'
    ///
    pub fn py_sign_message(&self, message: &[u8]) -> Signature {
        self.sign_message(message).into()
    }

    #[staticmethod]
    /// Generate a keypair from a 32-byte seed.
    ///
    /// Args:
    ///     seed (bytes): 32-byte seed.
    /// Returns:
    ///     Keypair: The generated keypair.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> seed_bytes = bytes([0] * 32)
    ///     >>> from_seed = Keypair.from_seed(seed_bytes)
    ///     >>> from_bytes = Keypair.from_bytes(seed_bytes + bytes(from_seed.pubkey()))
    ///     >>> assert from_seed == from_bytes
    ///
    pub fn from_seed(seed: [u8; 32]) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed(&seed))
    }

    #[staticmethod]
    /// Generate a keypair from a seed phrase and passphrase.
    ///
    /// Args:
    ///     seed_phrase (string): Secret seed phrase.
    ///     passphrase (string): Passphrase.
    ///
    /// Example:
    ///     >>> from pybip39 import Mnemonic, Seed
    ///     >>> from solders.keypair import Keypair
    ///     >>> mnemonic = Mnemonic()
    ///     >>> passphrase = "42"
    ///     >>> seed = Seed(mnemonic, passphrase)
    ///     >>> expected_keypair = Keypair.from_seed(bytes(seed)[:32])
    ///     >>> keypair = Keypair.from_seed_phrase_and_passphrase(mnemonic.phrase, passphrase)
    ///     >>> assert keypair.pubkey() == expected_keypair.pubkey()
    ///
    pub fn from_seed_phrase_and_passphrase(seed_phrase: &str, passphrase: &str) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed_phrase_and_passphrase(
            seed_phrase,
            passphrase,
        ))
    }

    #[pyo3(name = "is_interactive")]
    /// Whether the impelmentation requires user interaction to sign.
    ///
    /// Returns:
    ///     bool: Always ``False`` for this class.
    ///
    pub fn py_is_interactive(&self) -> bool {
        self.is_interactive()
    }
}

impl_signer_hash!(Keypair);
impl PyBytesGeneral for Keypair {
    fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.to_bytes_array())
    }
}
impl PyHash for Keypair {}

impl PyFromBytesGeneral for Keypair {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        handle_py_value_err(KeypairOriginal::from_bytes(raw))
    }
}

impl CommonMethodsCore for Keypair {
    fn pystr(&self) -> String {
        self.0.to_base58_string()
    }
}
impl CommonMethods<'_> for Keypair {}

impl RichcmpSigner for Keypair {}

impl_display!(Keypair);

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
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

impl ToSignerOriginal for Keypair {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.clone().0)
    }
}

impl SignerTraitWrapper for Keypair {}
