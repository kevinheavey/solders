use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_config;

#[pyclass(module = "solders.rpc.config", subclass)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcSignatureStatusConfig(rpc_config::RpcSignatureStatusConfig);

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

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSignatureStatusConfig
    ///     >>> RpcSignatureStatusConfig(True).to_json()
    ///     '{"searchTransactionHistory":true}'
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    Ok(config_mod)
}
