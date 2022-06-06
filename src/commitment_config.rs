use std::str::FromStr;

use pyo3::{create_exception, exceptions::PyException, prelude::*};
use serde::{Deserialize, Serialize};
use solana_sdk::commitment_config::{
    CommitmentConfig as CommitmentConfigOriginal, CommitmentLevel as CommitmentLevelOriginal,
    ParseCommitmentLevelError as ParseCommitmentLevelErrorOriginal,
};

use crate::{handle_py_err, PyErrWrapper};

create_exception!(
    solders,
    ParseCommitmentLevelError,
    PyException,
    "Raised when an error is encountered converting a string into a ``CommitmentConfig``."
);

impl From<ParseCommitmentLevelErrorOriginal> for PyErrWrapper {
    fn from(e: ParseCommitmentLevelErrorOriginal) -> Self {
        Self(ParseCommitmentLevelError::new_err(e.to_string()))
    }
}

#[pyclass(module = "solders.commitment_config")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommitmentLevel {
    /// The highest slot of the heaviest fork processed by the node. Ledger state at this slot is
    /// not derived from a confirmed or finalized block, but if multiple forks are present, is from
    /// the fork the validator believes is most likely to finalize.
    Processed,

    /// The highest slot that has been voted on by supermajority of the cluster, ie. is confirmed.
    /// Confirmation incorporates votes from gossip and replay. It does not count votes on
    /// descendants of a block, only direct votes on that block, and upholds "optimistic
    /// confirmation" guarantees in release 1.3 and onwards.
    Confirmed,

    /// The highest slot having reached max vote lockout, as recognized by a supermajority of the
    /// cluster.
    Finalized,
}

#[pymethods]
impl CommitmentLevel {
    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_err(CommitmentLevelOriginal::from_str(s))
    }
}

impl From<CommitmentLevelOriginal> for CommitmentLevel {
    #![allow(deprecated)]
    fn from(c: CommitmentLevelOriginal) -> Self {
        match c {
            CommitmentLevelOriginal::Processed => Self::Processed,
            CommitmentLevelOriginal::Confirmed => Self::Confirmed,
            CommitmentLevelOriginal::Finalized => Self::Finalized,
            CommitmentLevelOriginal::Max => Self::Finalized,
            CommitmentLevelOriginal::Recent => Self::Processed,
            CommitmentLevelOriginal::Root => Self::Finalized,
            CommitmentLevelOriginal::Single => Self::Confirmed,
            CommitmentLevelOriginal::SingleGossip => Self::Confirmed,
        }
    }
}

impl From<CommitmentLevel> for CommitmentLevelOriginal {
    fn from(c: CommitmentLevel) -> Self {
        match c {
            CommitmentLevel::Processed => Self::Processed,
            CommitmentLevel::Confirmed => Self::Confirmed,
            CommitmentLevel::Finalized => Self::Finalized,
        }
    }
}

#[pyclass(module = "solders.commitment_config", subclass)]
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommitmentConfig(CommitmentConfigOriginal);

#[pymethods]
impl CommitmentConfig {
    #[new]
    pub fn new(commitment: CommitmentLevel) -> Self {
        CommitmentConfigOriginal {
            commitment: commitment.into(),
        }
        .into()
    }

    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_err(CommitmentConfigOriginal::from_str(s))
    }

    #[staticmethod]
    pub fn processed() -> Self {
        CommitmentConfigOriginal::processed().into()
    }

    #[staticmethod]
    pub fn confirmed() -> Self {
        CommitmentConfigOriginal::confirmed().into()
    }

    #[staticmethod]
    pub fn finalized() -> Self {
        CommitmentConfigOriginal::finalized().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    pub fn is_finalized(&self) -> bool {
        self.0.is_finalized()
    }

    pub fn is_confirmed(&self) -> bool {
        self.0.is_confirmed()
    }

    pub fn is_at_least_confirmed(&self) -> bool {
        self.0.is_at_least_confirmed()
    }
}

impl From<CommitmentConfigOriginal> for CommitmentConfig {
    fn from(c: CommitmentConfigOriginal) -> CommitmentConfig {
        Self(c)
    }
}
