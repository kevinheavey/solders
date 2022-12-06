use crate::{pubkey::Pubkey, signature::Signature};
use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::signer::{null_signer::NullSigner as NullSignerOriginal, Signer as SignerTrait};
use solders_macros::{common_methods, pyhash, richcmp_signer};

use solders_traits::{
    impl_display, impl_signer_hash, CommonMethodsCore, PyBytesGeneral, PyFromBytesGeneral, PyHash,
    RichcmpSigner, SignerTraitWrapper, ToSignerOriginal,
};

mod null_signer_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use solana_sdk::{
        pubkey::Pubkey as PubkeyOriginal, signature::Signer,
        signer::null_signer::NullSigner as NullSignerOriginal,
    };

    pub fn serialize<S>(ns: &NullSignerOriginal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&ns.try_pubkey().unwrap().to_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NullSignerOriginal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = Vec::deserialize(deserializer)?;
        let pubkey = PubkeyOriginal::new(&b);
        Ok(NullSignerOriginal::new(&pubkey))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.null_signer", subclass)]
/// A signer implementation that always produces :meth:`solders.signature.Signature.default()`.
/// Used as a placeholder for absentee signers whose 'Pubkey` is required to construct
/// the transaction.
///
/// Args:
///     pubkey (Pubkey): The pubkey of the signer.
///
pub struct NullSigner(#[serde(with = "null_signer_serde")] pub NullSignerOriginal);

#[pyhash]
#[richcmp_signer]
#[common_methods]
#[pymethods]
impl NullSigner {
    #[new]
    pub fn new(pubkey: &Pubkey) -> Self {
        NullSignerOriginal::new(pubkey.as_ref()).into()
    }

    #[pyo3(name = "pubkey")]
    /// Return the pubkey of the signer.
    ///
    /// Returns:
    ///     Pubkey: The signer's pubkey.
    ///
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    /// Simply returns :meth:`solders.signature.Signature.default()`.
    ///
    /// Returns:
    ///     Signature: The default signature.
    ///
    pub fn py_sign_message(&self, message: &[u8]) -> Signature {
        self.try_sign_message(message).unwrap().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default null signer.
    ///
    /// Returns:
    ///     NullSigner: The default null signer.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    /// Deserialize a serialized ``NullSigner`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``NullSigner``.
    ///
    /// Returns:
    ///     NullSigner: The deserialized ``NullSigner``.
    fn from_bytes(data: [u8; Pubkey::LENGTH]) -> PyResult<Self> {
        Self::py_from_bytes(&data)
    }
}

impl_display!(NullSigner);
impl_signer_hash!(NullSigner);
impl PyHash for NullSigner {}

impl PyBytesGeneral for NullSigner {
    fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        self.py_pubkey().pybytes(py)
    }
}

impl PyFromBytesGeneral for NullSigner {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        Ok(Self::new(&Pubkey::from_bytes(raw)?))
    }
}

solders_traits::common_methods_default!(NullSigner);

impl ToSignerOriginal for NullSigner {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.0.clone())
    }
}

impl SignerTraitWrapper for NullSigner {}

impl RichcmpSigner for NullSigner {}
