use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::instruction::AccountMeta as AccountMetaOriginal;

use crate::{pubkey::Pubkey, richcmp_type_error};

/// Describes a single account read or written by a program during instruction
/// execution.
///
/// When constructing an [`Instruction`], a list of all accounts that may be
/// read or written during the execution of that instruction must be supplied.
/// Any account that may be mutated by the program during execution, either its
/// data or metadata such as held lamports, must be writable.
///
/// Note that because the Solana runtime schedules parallel transaction
/// execution around which accounts are writable, care should be taken that only
/// accounts which actually may be mutated are specified as writable.
#[pyclass]
#[derive(PartialEq, Debug)]
pub struct AccountMeta(AccountMetaOriginal);
#[pymethods]
impl AccountMeta {
    /// Construct metadata for an account.
    #[new]
    pub fn new(pubkey: &Pubkey, is_signer: bool, is_writable: bool) -> Self {
        let underlying_pubkey = pubkey.0;
        if is_writable {
            Self(AccountMetaOriginal::new(underlying_pubkey, is_signer))
        } else {
            Self(AccountMetaOriginal::new_readonly(
                underlying_pubkey,
                is_signer,
            ))
        }
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey(self.0.pubkey)
    }

    #[getter]
    pub fn is_signer(&self) -> bool {
        self.0.is_signer
    }

    #[getter]
    pub fn is_writable(&self) -> bool {
        self.0.is_writable
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
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
