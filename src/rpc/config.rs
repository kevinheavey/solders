use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_macros::common_magic_methods;

use crate::{
    account_decoder::UiAccountEncoding,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    transaction_status::UiTransactionEncoding,
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};

fn to_json(obj: &impl Serialize) -> String {
    serde_json::to_string(obj).unwrap()
}

macro_rules! pyclass_boilerplate {
    ($ident:ident) => {
        #[pyclass(module = "solders.rpc.config", subclass)]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $ident(rpc_config::$ident);
    };
}

macro_rules! rpc_config_impls {
    ($ident:ident) => {
        pybytes_general_via_bincode!($ident);
        py_from_bytes_general_via_bincode!($ident);
        impl_display!($ident);
        impl RichcmpEqualityOnly for $ident {}
        impl CommonMethods for $ident {}
        impl From<rpc_config::$ident> for $ident {
            fn from(c: rpc_config::$ident) -> Self {
                Self(c)
            }
        }
        impl From<$ident> for rpc_config::$ident {
            fn from(c: $ident) -> Self {
                c.0
            }
        }
    };
}

#[pyclass(module = "solders.rpc.config", subclass)]
/// Configuration object for ``getSignatureStatuses``.
///
/// Args:
///     search_transaction_history:  If True, a Solana node will search its ledger cache for any signatures not found in the recent status cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcSignatureStatusConfig(rpc_config::RpcSignatureStatusConfig);

#[common_magic_methods]
#[pymethods]
impl RpcSignatureStatusConfig {
    #[new]
    pub fn new(search_transaction_history: bool) -> Self {
        Self(rpc_config::RpcSignatureStatusConfig {
            search_transaction_history,
        })
    }

    #[getter]
    pub fn search_transaction_history(&self) -> bool {
        self.0.search_transaction_history
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSignatureStatusConfig
    ///     >>> RpcSignatureStatusConfig(True).to_json()
    ///     '{"searchTransactionHistory":true}'
    pub fn to_json(&self) -> String {
        to_json(self)
    }
}

rpc_config_impls!(RpcSignatureStatusConfig);

#[pyclass(module = "solders.rpc.config", subclass)]
/// Configuration object for ``sendTransaction``.
///
/// Args:
///     skip_preflight (bool):  If true, skip the preflight transaction checks.
///     preflight_commitment (Optional[CommitmentLevel]): Commitment level to use for preflight.
///     encoding: (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
///     max_retries: (Optional[int]): Maximum number of times for the RPC node to retry sending
///         the transaction to the leader. If this parameter not provided, the RPC node will
///         retry the transaction until it is finalized or until the blockhash expires.
///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
///
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RpcSendTransactionConfig(rpc_config::RpcSendTransactionConfig);

#[common_magic_methods]
#[pymethods]
impl RpcSendTransactionConfig {
    #[new]
    pub fn new(
        skip_preflight: bool,
        preflight_commitment: Option<CommitmentLevel>,
        encoding: Option<UiTransactionEncoding>,
        max_retries: Option<usize>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSendTransactionConfig {
            skip_preflight,
            preflight_commitment: preflight_commitment.map(CommitmentLevelOriginal::from),
            encoding: encoding.map(UiTransactionEncodingOriginal::from),
            max_retries,
            min_context_slot,
        })
    }

    #[getter]
    pub fn skip_preflight(&self) -> bool {
        self.0.skip_preflight
    }

    #[getter]
    pub fn preflight_commitment(&self) -> Option<CommitmentLevel> {
        self.0.preflight_commitment.map(|p| p.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn max_retries(&self) -> Option<usize> {
        self.0.max_retries
    }

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
    fn new_default() -> Self {
        Self::default()
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSendTransactionConfig
    ///     >>> RpcSendTransactionConfig.default().to_json()
    ///     '{"skipPreflight":false,"preflightCommitment":null,"encoding":null,"maxRetries":null,"minContextSlot":null}'
    ///
    pub fn to_json(&self) -> String {
        to_json(self)
    }
}

rpc_config_impls!(RpcSendTransactionConfig);

#[pyclass(module = "solders.rpc.config", subclass)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RpcSimulateTransactionAccountsConfig(rpc_config::RpcSimulateTransactionAccountsConfig);

#[common_magic_methods]
#[pymethods]
impl RpcSimulateTransactionAccountsConfig {
    #[new]
    pub fn new(encoding: Option<UiAccountEncoding>, addresses: Vec<String>) -> Self {
        Self(rpc_config::RpcSimulateTransactionAccountsConfig {
            encoding: encoding.map(|x| x.into()),
            addresses,
        })
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self::default()
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSimulateTransactionAccountsConfig
    ///     >>> RpcSimulateTransactionAccountsConfig.default().to_json()
    ///     '{"encoding":null,"addresses":[]}'
    pub fn to_json(&self) -> String {
        to_json(self)
    }
}

rpc_config_impls!(RpcSimulateTransactionAccountsConfig);

#[pyclass(module = "solders.rpc.config", subclass)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RpcSimulateTransactionConfig(rpc_config::RpcSimulateTransactionConfig);

#[common_magic_methods]
#[pymethods]
impl RpcSimulateTransactionConfig {
    #[new]
    fn new(
        sig_verify: bool,
        replace_recent_blockhash: bool,
        commitment: Option<CommitmentConfig>,
        encoding: Option<UiTransactionEncoding>,
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSimulateTransactionConfig {
            sig_verify,
            replace_recent_blockhash,
            commitment: commitment.map(|c| c.into()),
            encoding: encoding.map(|e| e.into()),
            accounts: accounts.map(|a| a.into()),
            min_context_slot,
        })
    }
}

rpc_config_impls!(RpcSimulateTransactionConfig);

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionConfig>()?;
    Ok(config_mod)
}
