use bincode::ErrorKind;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyTypeError, PyValueError},
    prelude::*,
    pyclass::CompareOp,
};
use solana_sdk::{
    instruction::Instruction as InstructionOriginal, pubkey::Pubkey as PubkeyOriginal,
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};
mod pubkey;
pub use pubkey::Pubkey;
mod signer;
pub use signer::{Signer, SignerError};
mod signature;
pub use signature::Signature;
mod keypair;
pub use keypair::Keypair;
mod instruction;
pub use instruction::{AccountMeta, CompiledInstruction, Instruction};
mod hash;
pub use hash::{Hash as SolderHash, ParseHashError};
mod message;
pub use message::{Message, MessageHeader};
mod transaction;
pub use transaction::{SanitizeError, Transaction};
mod system_program;
pub use system_program::SystemProgram;
mod sysvar;
pub use sysvar::Sysvar;
mod presigner;
pub use presigner::Presigner;

struct PyErrWrapper(PyErr);

impl From<PyErrWrapper> for PyErr {
    fn from(e: PyErrWrapper) -> Self {
        e.0
    }
}

fn to_py_err<T: Into<PyErrWrapper>>(e: T) -> PyErr {
    let wrapped: PyErrWrapper = e.into();
    wrapped.into()
}

fn handle_py_err<T: Into<P>, E: ToString + Into<PyErrWrapper>, P>(
    res: Result<T, E>,
) -> PyResult<P> {
    res.map_or_else(|e| Err(to_py_err(e)), |v| Ok(v.into()))
}

fn to_py_value_err(err: &impl ToString) -> PyErr {
    PyValueError::new_err(err.to_string())
}

fn handle_py_value_err<T: Into<P>, E: ToString, P>(res: Result<T, E>) -> PyResult<P> {
    res.map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
}

create_exception!(solders, BincodeError, PyException);

impl From<Box<ErrorKind>> for PyErrWrapper {
    fn from(e: Box<ErrorKind>) -> Self {
        Self(BincodeError::new_err(e.to_string()))
    }
}

fn convert_optional_pubkey(pubkey: Option<&Pubkey>) -> Option<&PubkeyOriginal> {
    pubkey.map(|p| p.as_ref())
}

fn convert_instructions(instructions: Vec<Instruction>) -> Vec<InstructionOriginal> {
    instructions
        .into_iter()
        .map(solana_sdk::instruction::Instruction::from)
        .collect()
}

fn richcmp_type_error(op: &str) -> PyErr {
    let msg = format!("{} not supported.", op);
    PyTypeError::new_err(msg)
}

fn calculate_hash(t: &impl Hash) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait RichcmpEqOnlyPrecalculated: PartialEq {
    fn richcmp(&self, eq_val: bool, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(eq_val),
            CompareOp::Ne => Ok(!eq_val),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}

pub trait RichcmpEqualityOnly: PartialEq {
    fn richcmp(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
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

pub trait RichcmpFull: PartialEq + PartialOrd {
    fn richcmp(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Lt => self < other,
            CompareOp::Gt => self > other,
            CompareOp::Le => self <= other,
            CompareOp::Ge => self >= other,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn solders(_py: Python, m: &PyModule) -> PyResult<()> {
    let hash_mod = PyModule::new(_py, "hash")?;
    hash_mod.add_class::<SolderHash>()?;
    hash_mod.add("ParseHashError", _py.get_type::<ParseHashError>())?;
    let instruction_mod = PyModule::new(_py, "instruction")?;
    instruction_mod.add_class::<AccountMeta>()?;
    instruction_mod.add_class::<Instruction>()?;
    instruction_mod.add_class::<CompiledInstruction>()?;
    let pubkey_mod = PyModule::new(_py, "pubkey")?;
    pubkey_mod.add_class::<Pubkey>()?;
    let keypair_mod = PyModule::new(_py, "keypair")?;
    keypair_mod.add_class::<Keypair>()?;
    let signature_mod = PyModule::new(_py, "signature")?;
    signature_mod.add_class::<Signature>()?;
    let message_mod = PyModule::new(_py, "message")?;
    message_mod.add_class::<Message>()?;
    message_mod.add_class::<MessageHeader>()?;
    let transaction_mod = PyModule::new(_py, "transaction")?;
    transaction_mod.add_class::<Transaction>()?;
    transaction_mod.add("SanitizeError", _py.get_type::<SanitizeError>())?;
    let system_program_mod = PyModule::new(_py, "system_program")?;
    system_program_mod.add_class::<SystemProgram>()?;
    let sysvar_mod = PyModule::new(_py, "sysvar")?;
    sysvar_mod.add_class::<Sysvar>()?;
    let presigner_mod = PyModule::new(_py, "presigner")?;
    presigner_mod.add_class::<Presigner>()?;
    let errors_mod = PyModule::new(_py, "errors")?;
    errors_mod.add("BincodeError", _py.get_type::<BincodeError>())?;
    errors_mod.add("SignerError", _py.get_type::<SignerError>())?;
    let submodules = vec![
        hash_mod,
        instruction_mod,
        pubkey_mod,
        keypair_mod,
        signature_mod,
        message_mod,
        transaction_mod,
        system_program_mod,
        sysvar_mod,
        presigner_mod,
        errors_mod,
    ];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("solders.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = _py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        m.add_submodule(submod)?;
    }
    Ok(())
}
