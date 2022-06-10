use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_macros::pyrepr;

use crate::{commitment_config::CommitmentLevel, transaction_status::UiTransactionEncoding};

fn to_json(obj: &impl Serialize) -> String {
    serde_json::to_string(obj).unwrap()
}

#[pyclass(module = "solders.rpc.config", subclass)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcSignatureStatusConfig(rpc_config::RpcSignatureStatusConfig);

#[pyrepr]
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

#[pyclass(module = "solders.rpc.config", subclass)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RpcSendTransactionConfig(rpc_config::RpcSendTransactionConfig);

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

    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self(rpc_config::RpcSendTransactionConfig::default())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSendTransactionConfig
    ///     >>> RpcSendTransactionConfig.default().to_json()
    ///     '{"skipPreflight":false,"preflightCommitment":null,"encoding":null,"maxRetries":null,"minContextSlot":null}'
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RpcSimulateTransactionAccountsConfig(rpc_config::RpcSimulateTransactionAccountsConfig);

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    Ok(config_mod)
}
