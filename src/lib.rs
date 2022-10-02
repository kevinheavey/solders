//! These docstrings are written for Python users.
//!
//! If you're viewing them on docs.rs, the formatting won't make much sense.
use account::create_account_mod;
use address_lookup_table_account::create_address_lookup_table_account_mod;
use bincode::ErrorKind;
use commitment_config::{CommitmentConfig, CommitmentLevel};
use pyo3::{
    create_exception,
    exceptions::{PyException, PyTypeError, PyValueError},
    prelude::*,
    pyclass::CompareOp,
    types::PyBytes,
};
use rpc::{create_rpc_mod, requests::SerdeJSONError};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::Instruction as InstructionOriginal,
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{Signer as SignerTrait, SignerError as SignerErrorOriginal},
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt,
    hash::{Hash, Hasher},
};
use system_program::create_system_program_mod;
use sysvar::create_sysvar_mod;
use transaction_status::create_transaction_status_mod;
pub mod pubkey;
use pubkey::Pubkey;
pub mod signer;
use signer::{Signer, SignerError};
pub mod signature;
use signature::Signature;
pub mod keypair;
use keypair::Keypair;
pub mod instruction;
use instruction::{AccountMeta, CompiledInstruction, Instruction};
pub mod hash;
use hash::{Hash as SolderHash, ParseHashError};
pub mod message;
use message::create_message_mod;
pub mod transaction;
use transaction::create_transaction_mod;
pub mod presigner;
use presigner::Presigner;
pub mod null_signer;
use null_signer::NullSigner;
pub mod account_decoder;
use account_decoder::create_account_decoder_mod;
pub mod account;
pub mod address_lookup_table_account;
pub mod commitment_config;
pub mod epoch_schedule;
pub mod rpc;
pub mod system_program;
pub mod sysvar;
mod tmp_account_decoder;
mod tmp_transaction_status;
pub mod transaction_status;
use epoch_schedule::create_epoch_schedule_mod;

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

create_exception!(
    solders,
    BincodeError,
    PyException,
    "Raised when the Rust bincode library returns an error during (de)serialization."
);

create_exception!(
    solders,
    CborError,
    PyException,
    "Raised when the Rust cbor library returns an error during (de)serialization."
);

impl From<Box<ErrorKind>> for PyErrWrapper {
    fn from(e: Box<ErrorKind>) -> Self {
        Self(BincodeError::new_err(e.to_string()))
    }
}

impl From<serde_cbor::Error> for PyErrWrapper {
    fn from(e: serde_cbor::Error) -> Self {
        Self(CborError::new_err(e.to_string()))
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

fn calculate_hash<T>(t: &T) -> u64
where
    T: Hash + ?Sized,
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait ToSignerOriginal {
    fn to_inner(&self) -> Box<dyn SignerTrait>;
}

pub trait SignerTraitWrapper: ToSignerOriginal {
    fn pubkey(&self) -> PubkeyOriginal {
        self.to_inner().pubkey()
    }
    fn try_pubkey(&self) -> Result<PubkeyOriginal, SignerErrorOriginal> {
        self.to_inner().try_pubkey()
    }
    fn sign_message(&self, message: &[u8]) -> SignatureOriginal {
        self.to_inner().sign_message(message)
    }
    fn try_sign_message(&self, message: &[u8]) -> Result<SignatureOriginal, SignerErrorOriginal> {
        self.to_inner().try_sign_message(message)
    }
    fn is_interactive(&self) -> bool {
        self.to_inner().is_interactive()
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

pub trait RichcmpSigner: SignerTraitWrapper {
    fn richcmp(&self, other: impl SignerTraitWrapper, op: CompareOp) -> PyResult<bool> {
        let eq_val = self.pubkey() == other.pubkey();
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

macro_rules! impl_display {
    ($ident:ident) => {
        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

pub(crate) use impl_display;

macro_rules! impl_signer_hash {
    ($ident:ident) => {
        #[allow(clippy::derive_hash_xor_eq)]
        impl std::hash::Hash for $ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.pubkey().hash(state);
            }
        }
    };
}

pub(crate) use impl_signer_hash;
pub trait PyHash: Hash {
    fn pyhash(&self) -> u64 {
        calculate_hash(self)
    }
}

pub trait PyBytesSlice: AsRef<[u8]> {
    fn pybytes_slice<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.as_ref())
    }
}

macro_rules! pybytes_general_for_pybytes_slice {
    ($ident:ident) => {
        impl crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                self.pybytes_slice(py)
            }
        }
    };
}

pub(crate) use pybytes_general_for_pybytes_slice;

macro_rules! pybytes_general_for_pybytes_bincode {
    ($ident:ident) => {
        impl crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                self.pybytes_bincode(py)
            }
        }
    };
}

pub(crate) use pybytes_general_for_pybytes_bincode;

macro_rules! pybytes_general_for_pybytes_cbor {
    ($ident:ident) => {
        impl crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                self.pybytes_cbor(py)
            }
        }
    };
}
pub(crate) use pybytes_general_for_pybytes_cbor;

pub trait PyBytesBincode: Serialize {
    fn pybytes_bincode<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &bincode::serialize(self).unwrap())
    }
}

pub trait PyBytesCbor: Serialize + std::marker::Sized {
    fn pybytes_cbor<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &serde_cbor::to_vec(self).unwrap())
    }
}

pub trait PyBytesGeneral {
    fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes;
}

macro_rules! pybytes_general_via_slice {
    ($ident:ident) => {
        impl crate::PyBytesSlice for $ident {}
        crate::pybytes_general_for_pybytes_slice!($ident);
    };
}

pub(crate) use pybytes_general_via_slice;

macro_rules! pybytes_general_via_bincode {
    ($ident:ident) => {
        impl crate::PyBytesBincode for $ident {}
        crate::pybytes_general_for_pybytes_bincode!($ident);
    };
}
pub(crate) use pybytes_general_via_bincode;

macro_rules! pybytes_general_via_cbor {
    ($ident:ident) => {
        impl crate::PyBytesCbor for $ident {}
        crate::pybytes_general_for_pybytes_cbor!($ident);
    };
}
pub(crate) use pybytes_general_via_cbor;

macro_rules! py_from_bytes_general_for_py_from_bytes_bincode {
    ($ident:ident) => {
        impl crate::PyFromBytesGeneral for $ident {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                Self::py_from_bytes_bincode(raw)
            }
        }
    };
}
pub(crate) use py_from_bytes_general_for_py_from_bytes_bincode;

macro_rules! py_from_bytes_general_for_py_from_bytes_cbor {
    ($ident:ident) => {
        impl crate::PyFromBytesGeneral for $ident {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                Self::py_from_bytes_cbor(raw)
            }
        }
    };
}
pub(crate) use py_from_bytes_general_for_py_from_bytes_cbor;

macro_rules! py_from_bytes_general_via_bincode {
    ($ident:ident) => {
        impl crate::PyFromBytesBincode<'_> for $ident {}
        crate::py_from_bytes_general_for_py_from_bytes_bincode!($ident);
    };
}

pub(crate) use py_from_bytes_general_via_bincode;
pub trait PyFromBytesBincode<'b>: Deserialize<'b> {
    fn py_from_bytes_bincode(raw: &'b [u8]) -> PyResult<Self> {
        let deser = bincode::deserialize::<Self>(raw);
        handle_py_err(deser)
    }
}

macro_rules! py_from_bytes_general_via_cbor {
    ($ident:ident) => {
        impl crate::PyFromBytesCbor<'_> for $ident {}
        crate::py_from_bytes_general_for_py_from_bytes_cbor!($ident);
    };
}

pub(crate) use py_from_bytes_general_via_cbor;

pub trait PyFromBytesCbor<'b>: Deserialize<'b> {
    fn py_from_bytes_cbor(raw: &'b [u8]) -> PyResult<Self> {
        let deser = serde_cbor::from_slice::<Self>(raw);
        handle_py_err(deser)
    }
}

pub trait PyFromBytesGeneral: Sized {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self>;
}

pub trait CommonMethods<'a>:
    fmt::Display
    + fmt::Debug
    + PyBytesGeneral
    + PyFromBytesGeneral
    + IntoPy<PyObject>
    + Clone
    + Serialize
    + Deserialize<'a>
{
    fn pybytes<'b>(&self, py: Python<'b>) -> &'b PyBytes {
        self.pybytes_general(py)
    }

    fn pystr(&self) -> String {
        self.to_string()
    }
    fn pyrepr(&self) -> String {
        format!("{:#?}", self)
    }

    fn py_from_bytes(raw: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes_general(raw)
    }

    fn pyreduce(&self) -> PyResult<(PyObject, PyObject)> {
        let cloned = self.clone();
        Python::with_gil(|py| {
            let constructor = cloned.into_py(py).getattr(py, "from_bytes")?;
            Ok((constructor, (self.pybytes(py).to_object(py),).to_object(py)))
        })
    }

    fn py_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn py_from_json(raw: &'a str) -> PyResult<Self> {
        serde_json::from_str(raw).map_err(to_py_err)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn solders(py: Python, m: &PyModule) -> PyResult<()> {
    let hash_mod = PyModule::new(py, "hash")?;
    hash_mod.add_class::<SolderHash>()?;
    hash_mod.add("ParseHashError", py.get_type::<ParseHashError>())?;
    let instruction_mod = PyModule::new(py, "instruction")?;
    instruction_mod.add_class::<AccountMeta>()?;
    instruction_mod.add_class::<Instruction>()?;
    instruction_mod.add_class::<CompiledInstruction>()?;
    let pubkey_mod = PyModule::new(py, "pubkey")?;
    pubkey_mod.add_class::<Pubkey>()?;
    let keypair_mod = PyModule::new(py, "keypair")?;
    keypair_mod.add_class::<Keypair>()?;
    let signature_mod = PyModule::new(py, "signature")?;
    signature_mod.add_class::<Signature>()?;
    let message_mod = create_message_mod(py)?;
    let null_signer_mod = PyModule::new(py, "null_signer")?;
    null_signer_mod.add_class::<NullSigner>()?;
    let transaction_mod = create_transaction_mod(py)?;
    let system_program_mod = create_system_program_mod(py)?;
    let sysvar_mod = create_sysvar_mod(py)?;
    let presigner_mod = PyModule::new(py, "presigner")?;
    presigner_mod.add_class::<Presigner>()?;
    let errors_mod = PyModule::new(py, "errors")?;
    errors_mod.add("BincodeError", py.get_type::<BincodeError>())?;
    errors_mod.add("SignerError", py.get_type::<SignerError>())?;
    errors_mod.add("CborError", py.get_type::<CborError>())?;
    errors_mod.add("SerdeJSONError", py.get_type::<SerdeJSONError>())?;
    let rpc_mod = create_rpc_mod(py)?;
    let commitment_config_mod = PyModule::new(py, "commitment_config")?;
    commitment_config_mod.add_class::<CommitmentConfig>()?;
    commitment_config_mod.add_class::<CommitmentLevel>()?;
    let transaction_status_mod = create_transaction_status_mod(py)?;
    let account_decoder_mod = create_account_decoder_mod(py)?;
    let account_mod = create_account_mod(py)?;
    let epoch_schedule_mod = create_epoch_schedule_mod(py)?;
    let address_lookup_table_account_mod = create_address_lookup_table_account_mod(py)?;
    let submodules = [
        errors_mod,
        hash_mod,
        instruction_mod,
        keypair_mod,
        message_mod,
        null_signer_mod,
        presigner_mod,
        pubkey_mod,
        signature_mod,
        transaction_mod,
        system_program_mod,
        sysvar_mod,
        rpc_mod,
        commitment_config_mod,
        transaction_status_mod,
        account_decoder_mod,
        account_mod,
        address_lookup_table_account_mod,
        epoch_schedule_mod,
    ];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("solders.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        m.add_submodule(submod)?;
    }
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
