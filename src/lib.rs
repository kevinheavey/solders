use bincode::serialize;
use instruction::CompiledInstruction;
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    pyclass::CompareOp,
};
use solana_sdk::{
    pubkey::bytes_are_curve_point,
    short_vec::{decode_shortu16_len, ShortU16},
};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
};
mod pubkey;
pub use pubkey::Pubkey;
mod signature;
pub use signature::Signature;
mod keypair;
pub use keypair::Keypair;
mod instruction;
pub use instruction::{AccountMeta, Instruction};
mod hash;
pub use hash::Hash as SolderHash;

fn to_py_value_err(err: &impl ToString) -> PyErr {
    PyValueError::new_err(err.to_string())
}

// #[derive(Debug)]
// pub struct SignatureError(SignatureOriginalError);

// impl std::convert::From<SignatureError> for PyErr {
//     fn from(err: SignatureError) -> PyErr {
//         PyValueError::new_err(err.0.to_string())
//     }
// }

/// Check if _bytes s is a valid point on curve or not.
#[pyfunction]
fn is_on_curve(_bytes: &[u8]) -> bool {
    bytes_are_curve_point(_bytes)
}

/// Return the serialized length.
#[pyfunction]
fn encode_length(value: u16) -> Vec<u8> {
    serialize(&ShortU16(value)).unwrap()
}

/// Return the decoded value and how many bytes it consumed.
#[pyfunction]
fn decode_length(raw_bytes: &[u8]) -> PyResult<(usize, usize)> {
    if raw_bytes == b"" {
        return Ok((0, 0));
    }
    let res = decode_shortu16_len(raw_bytes);
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err(PyValueError::new_err("Could not decode value.")),
    }
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
    m.add_function(wrap_pyfunction!(encode_length, m)?)?;
    m.add_function(wrap_pyfunction!(decode_length, m)?)?;
    m.add_function(wrap_pyfunction!(is_on_curve, m)?)?;
    m.add_class::<Pubkey>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<Signature>()?;
    m.add_class::<AccountMeta>()?;
    m.add_class::<Instruction>()?;
    m.add_class::<CompiledInstruction>()?;
    m.add_class::<SolderHash>()?;
    Ok(())
}
