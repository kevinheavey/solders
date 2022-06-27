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
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
    };
}

macro_rules! pyclass_boilerplate_with_default {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
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

pyclass_boilerplate!(
/// Configuration object for ``getSignatureStatuses``.
///
/// Args:
///     search_transaction_history:  If True, a Solana node will search its ledger cache for any signatures not found in the recent status cache
    => RpcSignatureStatusConfig
);

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

pyclass_boilerplate_with_default!(
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
    => RpcSendTransactionConfig
);

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

pyclass_boilerplate_with_default!(
    /// Accounts configuration for ``simulateTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned Account data
    ///     addresses (Sequence[str]): An array of accounts to return, as base-58 encoded strings.
    => RpcSimulateTransactionAccountsConfig
);

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

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionAccountsConfig: The default instance.
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
    ///     >>> from solders.account_decoder import UiAccountEncoding
    ///     >>> RpcSimulateTransactionAccountsConfig(UiAccountEncoding.Base64, []).to_json()
    ///     '{"encoding":"base64","addresses":[]}'
    pub fn to_json(&self) -> String {
        to_json(self)
    }
}

rpc_config_impls!(RpcSimulateTransactionAccountsConfig);

pyclass_boilerplate_with_default!(
    /// Configuration object for ``simulateTransaction``.
    ///
    /// Args:
    ///     sig_verify (bool): If True the transaction signatures will be verified
    ///         (conflicts with ``replace_recent_blockhash``).
    ///     replace_recent_blockhash (bool): If True the transaction recent blockhash
    ///         will be replaced with the most recent blockhash
    ///         (conflicts with ``sig_verify``).
    ///     commitment (Optional[CommitmentConfig]): Commitment level at which to simulate the transaction.
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     accounts (Optional[RpcSimulateTransactionAccountsConfig]): Accounts configuration object.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    => RpcSimulateTransactionConfig
);

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

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self::default()
    }

    /// Serialize as a JSON string.
    ///
    /// Example:
    ///
    ///     >>> from solders.rpc.config import RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig
    ///     >>> from solders.account_decoder import UiAccountEncoding
    ///     >>> from solders.commitment_config import CommitmentLevel, CommitmentConfig
    ///     >>> accounts_config = RpcSimulateTransactionAccountsConfig(UiAccountEncoding.Base64, [])
    ///     >>> config = RpcSimulateTransactionConfig(sig_verify=True, replace_recent_blockhash=False, accounts=accounts_config)
    ///     >>> config.to_json()
    ///     '{"sigVerify":true,"replaceRecentBlockhash":false,"encoding":null,"accounts":{"encoding":"base64","addresses":[]},"minContextSlot":null}'
    pub fn to_json(&self) -> String {
        to_json(self)
    }
}

rpc_config_impls!(RpcSimulateTransactionConfig);

pyclass_boilerplate_with_default!(/// Foo
=> RpcRequestAirdropConfig);

rpc_config_impls!(RpcRequestAirdropConfig);

#[common_magic_methods]
#[pymethods]
impl RpcRequestAirdropConfig {
    #[new]
    fn new(recent_blockhash: Option<&str>, commitment: Option<CommitmentConfig>) -> Self {
        Self(rpc_config::RpcRequestAirdropConfig {
            recent_blockhash: recent_blockhash.map(String::from),
            commitment: commitment.map(|c| c.into()),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcRequestAirdropConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self::default()
    }
}

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionConfig>()?;
    config_mod.add_class::<RpcRequestAirdropConfig>()?;
    Ok(config_mod)
}
