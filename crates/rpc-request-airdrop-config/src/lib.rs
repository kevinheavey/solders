use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solders_commitment_config::CommitmentLevel;
use solders_hash::Hash as SolderHash;
use solders_macros::{common_methods, richcmp_eq_only};

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

pyclass_boilerplate_with_default!(
    /// Configuration object for ``requestAirdrop``.
    /// 
    /// Args:
    ///     recent_blockhash (Optional[Hash]): The ID of a recent ledger entry.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    /// 
=> RpcRequestAirdropConfig);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcRequestAirdropConfig {
    #[new]
    pub fn new(recent_blockhash: Option<SolderHash>, commitment: Option<CommitmentLevel>) -> Self {
        Self(rpc_config::RpcRequestAirdropConfig {
            recent_blockhash: recent_blockhash.map(|h| h.to_string()),
            commitment: commitment.map(|c| c.into()),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcRequestAirdropConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> Option<SolderHash> {
        self.0
            .recent_blockhash
            .clone()
            .map(|s| SolderHash::from_str(&s).unwrap())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }
}
