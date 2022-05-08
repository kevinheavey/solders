use pyo3::{prelude::*, pyclass::CompareOp, types::PyBytes};
use solana_sdk::signer::{presigner::Presigner as PresignerOriginal, Signer};

use crate::{handle_py_value_err, richcmp_type_error, Pubkey, RichcmpEqualityOnly, Signature};

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass]
pub struct Presigner(PresignerOriginal);

#[pymethods]
impl Presigner {
    #[new]
    pub fn new(pubkey: &Pubkey, signature: &Signature) -> Self {
        PresignerOriginal::new(pubkey.as_ref(), signature.as_ref()).into()
    }

    pub fn pubkey(&self) -> Pubkey {
        self.0.pubkey().into()
    }

    pub fn sign_message(&self, message: &[u8]) -> PyResult<Signature> {
        handle_py_value_err(self.0.try_sign_message(message))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self == other),
            CompareOp::Ne => Ok(self != other),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}

impl From<PresignerOriginal> for Presigner {
    fn from(signer: PresignerOriginal) -> Self {
        Self(signer)
    }
}
