use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};
use solders_transaction_status_enums::UiTransactionEncoding;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``sendTransaction``.
    ///
    /// Args:
    ///     skip_preflight (bool):  If true, skip the preflight transaction checks.
    ///     preflight_commitment (Optional[CommitmentLevel]): Commitment level to use for preflight checks.
    ///     max_retries: (Optional[int]): Maximum number of times for the RPC node to retry sending
    ///         the transaction to the leader. If this parameter not provided, the RPC node will
    ///         retry the transaction until it is finalized or until the blockhash expires.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSendTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSendTransactionConfig {
    #[new]
    #[pyo3(signature = (skip_preflight=false, preflight_commitment=None, max_retries=None, min_context_slot=None))]
    pub fn new(
        skip_preflight: bool,
        preflight_commitment: Option<CommitmentLevel>,
        max_retries: Option<usize>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSendTransactionConfig {
            skip_preflight,
            preflight_commitment: preflight_commitment.map(CommitmentLevelOriginal::from),
            encoding: Some(UiTransactionEncodingOriginal::Base64),
            max_retries,
            min_context_slot,
        })
    }

    /// bool:  If true, skip the preflight transaction checks.
    #[getter]
    pub fn skip_preflight(&self) -> bool {
        self.0.skip_preflight
    }

    /// Optional[CommitmentLevel]: Commitment level to use for preflight checks.
    #[getter]
    pub fn preflight_commitment(&self) -> Option<CommitmentLevel> {
        self.0.preflight_commitment.map(|p| p.into())
    }

    /// UiTransactionEncoding: Encoding used for the transaction data.
    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    /// Optional[int]: Maximum number of times for the RPC node to retry sending the transaction to the leader.
    #[getter]
    pub fn max_retries(&self) -> Option<usize> {
        self.0.max_retries
    }

    /// Optional[int]: The minimum slot that the request can be evaluated at.
    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSendTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}
