use pyo3::{basic::CompareOp, prelude::*};
use solana_sdk::instruction::{
    AccountMeta as AccountMetaOriginal, Instruction as InstructionOriginal,
};

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
#[derive(PartialEq, Debug, Clone)]
pub struct AccountMeta(pub AccountMetaOriginal);
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

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Instruction(InstructionOriginal);

#[pymethods]
impl Instruction {
    #[new]
    pub fn new(program_id: &Pubkey, data: &[u8], accounts: Vec<AccountMeta>) -> Self {
        let underlying_accounts: Vec<AccountMetaOriginal> =
            accounts.into_iter().map(|x| x.0).collect();
        let underlying =
            InstructionOriginal::new_with_bytes(program_id.0, data, underlying_accounts);
        Self(underlying)
    }

    /// Pubkey of the program that executes this instruction.
    #[getter]
    pub fn program_id(&self) -> Pubkey {
        Pubkey(self.0.program_id)
    }

    /// Opaque data passed to the program for its own interpretation.
    #[getter]
    pub fn data(&self) -> Vec<u8> {
        self.0.clone().data
    }

    /// Metadata describing accounts that should be passed to the program.
    #[getter]
    pub fn accounts(&self) -> Vec<AccountMeta> {
        self.0
            .accounts
            .clone()
            .into_iter()
            .map(AccountMeta)
            .collect()
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
