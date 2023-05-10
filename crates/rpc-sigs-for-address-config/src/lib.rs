use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_signature::Signature;
use std::str::FromStr;

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getSignaturesForAddress``.
    ///
    /// Args:
    ///     before (Optional[Signature]): Start searching backwards from this transaction signature.
    ///     until (Optional[Signature]): Search until this transaction signature.
    ///     limit (Optional[int]): Maximum transaction signatures to return (between 1 and 1,000, default: 1,000).
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSignaturesForAddressConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSignaturesForAddressConfig {
    #[new]
    fn new(
        before: Option<&Signature>,
        until: Option<&Signature>,
        limit: Option<usize>,
        commitment: Option<CommitmentLevel>,
        min_context_slot: Option<u64>,
    ) -> Self {
        rpc_config::RpcSignaturesForAddressConfig {
            before: before.map(|sig| sig.to_string()),
            until: until.map(|sig| sig.to_string()),
            limit,
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        }
        .into()
    }

    #[getter]
    pub fn before(&self) -> Option<Signature> {
        self.0
            .before
            .clone()
            .map(|s| Signature::from_str(&s).unwrap())
    }

    #[getter]
    pub fn until(&self) -> Option<Signature> {
        self.0
            .until
            .clone()
            .map(|s| Signature::from_str(&s).unwrap())
    }

    #[getter]
    pub fn limit(&self) -> Option<usize> {
        self.0.limit
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSignaturesForAddressConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}
