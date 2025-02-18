use derive_more::{From, Into};
use pyo3::{exceptions::PyValueError, prelude::*};
use serde::{Deserialize, Serialize};
use solders_macros::{common_methods, pyhash, richcmp_signer};
use solders_pubkey::Pubkey;
use solders_signature::Signature;
use {
    solana_derivation_path::DerivationPath,
    solana_keypair::{
        keypair_from_seed, keypair_from_seed_phrase_and_passphrase,
        seed_derivable::keypair_from_seed_and_derivation_path, Keypair as KeypairOriginal,
    },
    solana_signer::Signer as SignerTrait,
};

use solders_traits::{impl_signer_hash, RichcmpSigner, SignerTraitWrapper, ToSignerOriginal};
use solders_traits_core::{
    handle_py_value_err, impl_display, CommonMethods, CommonMethodsCore, PyBytesGeneral,
    PyFromBytesGeneral, PyHash,
};

pub mod null_signer;
pub mod presigner;
pub mod signer;

mod keypair_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use solana_keypair::Keypair as KeypairOriginal;

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
    ///      >>> raw_bytes = b'\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x8a\x88\xe3\xddt\t\xf1\x95\xfdR\xdb-<\xba]r\xcag\t\xbf\x1d\x94\x12\x1b\xf3t\x88\x01\xb4\x0fo\\'
    ///      >>> assert Keypair.from_bytes(raw_bytes).to_bytes() == raw_bytes
    ///
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
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
    ///     >>> raw_bytes = b'\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x8a\x88\xe3\xddt\t\xf1\x95\xfdR\xdb-<\xba]r\xcag\t\xbf\x1d\x94\x12\x1b\xf3t\x88\x01\xb4\x0fo\\'
    ///     >>> base58_str = "2AXDGYSE4f2sz7tvMMzyHvUfcoJmxudvdhBcmiUSo6iuCXagjUCKEQF21awZnUGxmwD4m9vGXuC3qieHXJQHAcT"
    ///     >>> kp = Keypair.from_base58_string(base58_str)
    ///     >>> assert kp == Keypair.from_bytes(raw_bytes)
    ///     >>> assert str(kp) == base58_str
    ///     
    pub fn from_base58_string(s: &str) -> PyResult<Self> {
        let mut buf = [0u8; 64];
        five8::decode_64(s, &mut buf)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))?;
        Self::py_from_bytes_general(&buf)
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
    ///     >>> pubkey_bytes = b";j'\xbc\xce\xb6\xa4-b\xa3\xa8\xd0*o\rse2\x15w\x1d\xe2C\xa6:\xc0H\xa1\x8bY\xda)"
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
    /// Generate a keypair from a 32-byte seed and derivation path..
    ///
    /// Args:
    ///     seed (bytes): 32-byte seed.
    ///     dpath (str): derivation path.
    /// Returns:
    ///     Keypair: The generated keypair.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> seed_bytes = bytes([0] * 64)
    ///     >>> account_index = 0
    ///     >>> derivation_path = f"m/44'/501'/0'/{account_index}'"
    ///     >>> from_seed = Keypair.from_seed_and_derivation_path(seed_bytes, derivation_path)
    ///
    pub fn from_seed_and_derivation_path(seed: [u8; 64], dpath: &str) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed_and_derivation_path(
            &seed,
            Some(DerivationPath::from_absolute_path_str(dpath).unwrap()),
        ))
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
    fn pybytes_general(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
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
        Self::from_bytes(self.to_bytes()).unwrap()
    }
}

impl ToSignerOriginal for Keypair {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.clone().0)
    }
}

impl SignerTraitWrapper for Keypair {}
