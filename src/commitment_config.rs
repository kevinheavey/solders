use std::str::FromStr;

use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::commitment_config::{
    CommitmentConfig as CommitmentConfigOriginal, CommitmentLevel as CommitmentLevelOriginal,
};

use solders_traits::handle_py_err;

/// RPC request `commitment <https://docs.solana.com/developing/clients/jsonrpc-api#configuring-state-commitment>`_ options.
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

impl Default for CommitmentLevel {
    fn default() -> Self {
        CommitmentLevelOriginal::default().into()
    }
}

#[pymethods]
impl CommitmentLevel {
    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_err(CommitmentLevelOriginal::from_str(s))
    }

    /// Create a new default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
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

/// Wrapper object for ``CommitmentLevel``.
///
/// Args:
///     commitment (CommitmentLevel): Bank state to query.
#[pyclass(module = "solders.commitment_config", subclass)]
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
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

    #[getter]
    pub fn commitment(&self) -> CommitmentLevel {
        self.0.commitment.into()
    }

    /// Create from a string.
    #[staticmethod]
    #[pyo3(name = "from_string")]
    pub fn new_from_str(s: &str) -> PyResult<Self> {
        handle_py_err(CommitmentConfigOriginal::from_str(s))
    }

    /// Create a new instance with ``processed`` commitment.
    #[staticmethod]
    pub fn processed() -> Self {
        CommitmentConfigOriginal::processed().into()
    }

    /// Create a new instance with ``confirmed`` commitment.
    #[staticmethod]
    pub fn confirmed() -> Self {
        CommitmentConfigOriginal::confirmed().into()
    }

    /// Create a new instance with ``finalized`` commitment.
    #[staticmethod]
    pub fn finalized() -> Self {
        CommitmentConfigOriginal::finalized().into()
    }

    /// Create a new default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Check if using ``finalized`` commitment.
    pub fn is_finalized(&self) -> bool {
        self.0.is_finalized()
    }

    /// Check if using ``confirmed`` commitment.
    pub fn is_confirmed(&self) -> bool {
        self.0.is_confirmed()
    }

    /// Check if using at least ``confirmed`` commitment.
    pub fn is_at_least_confirmed(&self) -> bool {
        self.0.is_at_least_confirmed()
    }
}

impl From<CommitmentLevel> for CommitmentConfig {
    fn from(c: CommitmentLevel) -> Self {
        CommitmentConfig::new(c)
    }
}

impl From<CommitmentLevel> for CommitmentConfigOriginal {
    fn from(c: CommitmentLevel) -> Self {
        CommitmentConfig::from(c).into()
    }
}

impl From<CommitmentConfig> for CommitmentLevel {
    fn from(c: CommitmentConfig) -> Self {
        c.commitment()
    }
}

impl From<CommitmentLevelOriginal> for CommitmentConfig {
    fn from(c: CommitmentLevelOriginal) -> Self {
        CommitmentLevel::from(c).into()
    }
}

impl From<CommitmentConfigOriginal> for CommitmentLevel {
    fn from(c: CommitmentConfigOriginal) -> Self {
        CommitmentConfig::from(c).into()
    }
}
