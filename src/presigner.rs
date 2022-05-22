use pyo3::{prelude::*, pyclass::CompareOp};
use solana_sdk::signer::{presigner::Presigner as PresignerOriginal, Signer as SignerTrait};

use crate::{
    handle_py_err, Pubkey, RichcmpSigner, Signature, Signer, SignerTraitWrapper, ToSignerOriginal,
};

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
        handle_py_err(self.try_sign_message(message))
    }

    fn __richcmp__(&self, other: Signer, op: CompareOp) -> PyResult<bool> {
        self.richcmp(other, op)
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

impl ToSignerOriginal for Presigner {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.0.clone())
    }
}

impl SignerTraitWrapper for Presigner {}

impl RichcmpSigner for Presigner {}
