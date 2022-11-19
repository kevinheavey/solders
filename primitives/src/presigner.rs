use crate::{pubkey::Pubkey, signature::Signature};
use pyo3::prelude::*;
use solana_sdk::signer::{presigner::Presigner as PresignerOriginal, Signer as SignerTrait};
use solders_macros::{pyhash, richcmp_signer};

use solders_traits::{
    handle_py_err, impl_display, impl_signer_hash, PyHash, RichcmpSigner, SignerTraitWrapper,
    ToSignerOriginal,
};

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(module = "solders.presigner", subclass)]
/// A signer that represents a :class:`~solders.signature.Signature` that has been
/// constructed externally. Performs a signature verification against the
/// expected message upon ``sign()`` requests to affirm its relationship to
/// the ``message`` bytes.
///
/// Args:
///     pubkey (Pubkey): The pubkey of the signer.
///     signature (Signature): The signature created by signing the message.
///     
pub struct Presigner(pub PresignerOriginal);

#[pyhash]
#[richcmp_signer]
#[pymethods]
impl Presigner {
    #[new]
    pub fn new(pubkey: &Pubkey, signature: &Signature) -> Self {
        PresignerOriginal::new(pubkey.as_ref(), signature.as_ref()).into()
    }

    #[pyo3(name = "pubkey")]
    /// Get this signer's :class:`~solders.pubkey.Pubkey`.
    ///
    /// Returns:
    ///     Pubkey: The pubkey of the presigner.
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
    /// Verifies the signature of the presigner and returns it if valid.
    ///
    /// Returns:
    ///     Signature: The signature assigned to this object.
    ///
    /// Raises:
    ///     SignerError: if the signature is invalid.
    ///
    /// Example:
    ///
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.presigner import Presigner
    ///     >>> keypair = Keypair.from_seed(bytes([0] * 32))
    ///     >>> pubkey = keypair.pubkey()
    ///     >>> data = bytes([1])
    ///     >>> sig = keypair.sign_message(data)
    ///     >>> presigner = Presigner(pubkey, sig)
    ///     >>> assert presigner.pubkey() == pubkey
    ///     >>> assert presigner.sign_message(data) == sig
    ///
    pub fn py_sign_message(&self, message: &[u8]) -> PyResult<Signature> {
        handle_py_err(self.try_sign_message(message))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default presigner.
    ///
    /// Returns:
    ///     Presigner: The default presigner.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl_display!(Presigner);
impl_signer_hash!(Presigner);
impl PyHash for Presigner {}

impl From<PresignerOriginal> for Presigner {
    fn from(signer: PresignerOriginal) -> Self {
        Self(signer)
    }
}

impl ToSignerOriginal for Presigner {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.0.clone())
    }
}

impl SignerTraitWrapper for Presigner {}
impl RichcmpSigner for Presigner {}
