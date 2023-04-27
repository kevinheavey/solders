use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solders_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

use solders_rpc_config_macros::pyclass_boilerplate_with_default;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getAccountInfo``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned account data.
    ///     data_slice (Optiona;[UiDataSliceConfig]): Limit the returned account data.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcAccountInfoConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcAccountInfoConfig {
    #[new]
    pub fn new(
        encoding: Option<UiAccountEncoding>,
        data_slice: Option<UiDataSliceConfig>,
        commitment: Option<CommitmentLevel>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcAccountInfoConfig {
            encoding: encoding.map(Into::into),
            data_slice: data_slice.map(|d| d.into()),
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcEpochConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.0.encoding.map(Into::into)
    }

    #[getter]
    pub fn data_slice(&self) -> Option<UiDataSliceConfig> {
        self.0.data_slice.map(|d| d.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}
