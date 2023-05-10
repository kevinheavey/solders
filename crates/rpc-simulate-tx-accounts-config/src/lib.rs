use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solders_account_decoder::UiAccountEncoding;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_pubkey::Pubkey;

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

pyclass_boilerplate_with_default!(
    /// Accounts configuration for ``simulateTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned Account data
    ///     addresses (Sequence[Pubkey]): An array of accounts to return.
    => RpcSimulateTransactionAccountsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionAccountsConfig {
    #[new]
    pub fn new(addresses: Vec<Pubkey>, encoding: Option<UiAccountEncoding>) -> Self {
        Self(rpc_config::RpcSimulateTransactionAccountsConfig {
            encoding: encoding.map(Into::into),
            addresses: addresses.iter().map(|a| a.to_string()).collect(),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.0
            .addresses
            .clone()
            .iter()
            .map(|s| Pubkey::from_str(s).unwrap())
            .collect()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.0.encoding.map(Into::into)
    }
}
