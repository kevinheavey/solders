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
    collections::hash_map::DefaultHasher,
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
mod mymod;
pub use mymod::double;

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
    let mymodule = PyModule::new(_py, "mymod")?;
    mymodule.add_function(wrap_pyfunction!(double, m)?)?;
    _py.import("sys")?
        .getattr("modules")?
        .set_item("solders.mymod", mymodule)?;
    m.add_submodule(mymodule)?;
    m.add_class::<Pubkey>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<Signature>()?;
    m.add_class::<AccountMeta>()?;
    m.add_class::<Instruction>()?;
    m.add_class::<CompiledInstruction>()?;
    m.add_class::<SolderHash>()?;
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<Transaction>()?;
    m.add_class::<SystemProgram>()?;
    m.add_class::<Sysvar>()?;
    m.add_class::<Presigner>()?;
    m.add("ParseHashError", _py.get_type::<ParseHashError>())?;
    m.add("BincodeError", _py.get_type::<BincodeError>())?;
    m.add("SignerError", _py.get_type::<SignerError>())?;
    m.add("SanitizeError", _py.get_type::<SanitizeError>())?;
    Ok(())
}
