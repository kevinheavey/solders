use pyo3::{prelude::*, pyclass::CompareOp};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{presigner::Presigner as PresignerOriginal, Signer as SignerTrait, SignerError},
};

use crate::{handle_py_value_err, Pubkey, RichcmpEqOnlyPrecalculated, Signature, Signer};

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(module = "solders", subclass)]
pub struct Presigner(pub PresignerOriginal);

#[pymethods]
impl Presigner {
    #[new]
    pub fn new(pubkey: &Pubkey, signature: &Signature) -> Self {
        PresignerOriginal::new(pubkey.as_ref(), signature.as_ref()).into()
    }

    #[pyo3(name = "pubkey")]
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    pub fn py_sign_message(&self, message: &[u8]) -> PyResult<Signature> {
        handle_py_value_err(self.try_sign_message(message))
    }

    fn __richcmp__(&self, other: Signer, op: CompareOp) -> PyResult<bool> {
        let other_eq = match other {
            Signer::KeypairWrapper(kp) => kp.0 == self.0,
            Signer::PresignerWrapper(ps) => ps.0 == self.0,
        };
        self.richcmp(other_eq, op)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

impl From<PresignerOriginal> for Presigner {
    fn from(signer: PresignerOriginal) -> Self {
        Self(signer)
    }
}

impl SignerTrait for Presigner {
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

impl RichcmpEqOnlyPrecalculated for Presigner {}
