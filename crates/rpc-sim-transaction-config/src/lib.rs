use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solana_transaction_status_client_types::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_rpc_simulate_tx_accounts_config::RpcSimulateTransactionAccountsConfig;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};
use solders_transaction_status_enums::UiTransactionEncoding;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``simulateTransaction``.
    ///
    /// Args:
    ///     sig_verify (bool): If True the transaction signatures will be verified
    ///         (conflicts with ``replace_recent_blockhash``).
    ///     replace_recent_blockhash (bool): If True the transaction recent blockhash
    ///         will be replaced with the most recent blockhash
    ///         (conflicts with ``sig_verify``).
    ///     commitment (Optional[CommitmentLevel]): Commitment level at which to simulate the transaction.
    ///     accounts (Optional[RpcSimulateTransactionAccountsConfig]): Accounts configuration object.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    => RpcSimulateTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionConfig {
    #[new]
    #[pyo3(signature = (sig_verify=false, replace_recent_blockhash=false, commitment=None, accounts=None, min_context_slot=None, inner_instructions=false))]
    fn new(
        sig_verify: bool,
        replace_recent_blockhash: bool,
        commitment: Option<CommitmentLevel>,
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
        min_context_slot: Option<u64>,
        inner_instructions: bool,
    ) -> Self {
        Self(rpc_config::RpcSimulateTransactionConfig {
            sig_verify,
            replace_recent_blockhash,
            commitment: commitment.map(|c| c.into()),
            encoding: Some(UiTransactionEncodingOriginal::Base64),
            accounts: accounts.map(|a| a.into()),
            min_context_slot,
            inner_instructions,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn sig_verify(&self) -> bool {
        self.0.sig_verify
    }

    #[getter]
    pub fn replace_recent_blockhash(&self) -> bool {
        self.0.replace_recent_blockhash
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    #[getter]
    pub fn accounts(&self) -> Option<RpcSimulateTransactionAccountsConfig> {
        self.0.accounts.clone().map(|a| a.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    #[getter]
    pub fn inner_instructions(&self) -> bool {
        self.0.inner_instructions
    }
}
