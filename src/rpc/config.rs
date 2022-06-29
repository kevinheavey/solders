use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_macros::{common_methods, richcmp_eq_only};

use crate::{
    account_decoder::{UiAccountEncoding, UiDataSliceConfig},
    commitment_config::{CommitmentConfig, CommitmentLevel},
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    transaction_status::{TransactionDetails, UiTransactionEncoding},
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};

use super::filter::RpcFilterType;

macro_rules! rpc_config_impls {
    ($ident:ident) => {
        pybytes_general_via_bincode!($ident);
        py_from_bytes_general_via_bincode!($ident);
        impl_display!($ident);
        impl RichcmpEqualityOnly for $ident {}
        impl CommonMethods<'_> for $ident {}
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

macro_rules! pyclass_boilerplate {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
        rpc_config_impls!($name);
    };
}

macro_rules! pyclass_boilerplate_with_default {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
        rpc_config_impls!($name);
    };
}

pyclass_boilerplate!(
/// Configuration object for ``getSignatureStatuses``.
///
/// Args:
///     search_transaction_history:  If True, a Solana node will search its ledger cache for any signatures not found in the recent status cache
    => RpcSignatureStatusConfig
);

#[common_methods]
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
}

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

#[common_methods]
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
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Accounts configuration for ``simulateTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned Account data
    ///     addresses (Sequence[str]): An array of accounts to return, as base-58 encoded strings.
    => RpcSimulateTransactionAccountsConfig
);

#[common_methods]
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
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn addresses(&self) -> Vec<String> {
        self.0.addresses.clone()
    }
}

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

#[common_methods]
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

    #[getter]
    pub fn sig_verify(&self) -> bool {
        self.0.sig_verify
    }

    #[getter]
    pub fn replace_recent_blockhash(&self) -> bool {
        self.0.replace_recent_blockhash
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn accounts(&self) -> Option<RpcSimulateTransactionAccountsConfig> {
        self.0.accounts.clone().map(|a| a.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``requestAirdrop``.
    /// 
    /// Args:
    ///     recent_blockhash (Optional[str]): The ID of a recent ledger entry.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    /// 
=> RpcRequestAirdropConfig);

#[common_methods]
#[pymethods]
impl RpcRequestAirdropConfig {
    #[new]
    pub fn new(recent_blockhash: Option<&str>, commitment: Option<CommitmentConfig>) -> Self {
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
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> Option<String> {
        self.0.recent_blockhash.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getLeaderSchedule``.
    /// 
    /// Args:
    ///     identity (Optional[str]): Validator identity, as a base-58 encoded string
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    /// 
=> RpcLeaderScheduleConfig);

#[common_methods]
#[pymethods]
impl RpcLeaderScheduleConfig {
    #[new]
    pub fn new(identity: Option<&str>, commitment: Option<CommitmentConfig>) -> Self {
        Self(rpc_config::RpcLeaderScheduleConfig {
            identity: identity.map(String::from),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<String> {
        self.0.identity.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Range object for ``RpcBlockProductionConfig``.
    /// 
    /// Args:
    ///     first_slot (int): First slot in the range
    ///     last_slot (Optional[int]): Last slot in the range.
    /// 
=> RpcBlockProductionConfigRange);

#[common_methods]
#[pymethods]
impl RpcBlockProductionConfigRange {
    #[new]
    pub fn new(first_slot: u64, last_slot: Option<u64>) -> Self {
        Self(rpc_config::RpcBlockProductionConfigRange {
            first_slot,
            last_slot,
        })
    }

    #[getter]
    pub fn first_slot(&self) -> u64 {
        self.0.first_slot
    }

    #[getter]
    pub fn last_slot(&self) -> Option<u64> {
        self.0.last_slot
    }
}

/// Configuration object for ``getBlockProduction``.
///
/// Args:
///     identity (Optional[str]): Validator identity, as a base-58 encoded string
///     range (Optional[RpcBlockProductionConfigRange]): Slot range to query. Current epoch if ``None``.
///     commitment (Optional[CommitmentConfig]): Bank state to query.
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcBlockProductionConfig(rpc_config::RpcBlockProductionConfig);

impl PartialEq for RpcBlockProductionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0.identity == other.0.identity
            && self.0.range == other.0.range
            && self.0.commitment == other.0.commitment
    }
}

rpc_config_impls!(RpcBlockProductionConfig);

#[common_methods]
#[pymethods]
impl RpcBlockProductionConfig {
    #[new]
    pub fn new(
        identity: Option<&str>,
        range: Option<RpcBlockProductionConfigRange>,
        commitment: Option<CommitmentConfig>,
    ) -> Self {
        Self(rpc_config::RpcBlockProductionConfig {
            identity: identity.map(String::from),
            range: range.map(|r| r.into()),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<String> {
        self.0.identity.clone()
    }

    #[getter]
    pub fn range(&self) -> Option<RpcBlockProductionConfigRange> {
        self.0.range.clone().map(|r| r.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockProductionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getVoteAccounts``.
    /// 
    /// Args:
    ///     vote_pubkey (Optional[str]): Validator vote address, as a base-58 encoded string
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     keep_unstaked_delinquents (Optional[bool]): Do not filter out delinquent validators with no stake.
    ///     delinquent_slot_distance (Optional[int]): Specify the number of slots behind the tip that a validator
    ///         must fall to be considered delinquent.
    ///         NOTE: For the sake of consistency between ecosystem products, it is not recommended that
    ///         this argument be specified.
    /// 
    => RpcGetVoteAccountsConfig);

#[common_methods]
#[pymethods]
impl RpcGetVoteAccountsConfig {
    #[new]
    pub fn new(
        vote_pubkey: Option<&str>,
        commitment: Option<CommitmentConfig>,
        keep_unstaked_delinquents: Option<bool>,
        delinquent_slot_distance: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcGetVoteAccountsConfig {
            vote_pubkey: vote_pubkey.map(String::from),
            commitment: commitment.map(|c| c.into()),
            keep_unstaked_delinquents,
            delinquent_slot_distance,
        })
    }

    #[getter]
    pub fn vote_pubkey(&self) -> Option<String> {
        self.0.vote_pubkey.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn keep_unstaked_delinquents(&self) -> Option<bool> {
        self.0.keep_unstaked_delinquents
    }

    #[getter]
    pub fn delinquent_slot_distance(&self) -> Option<u64> {
        self.0.delinquent_slot_distance
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcGetVoteAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

/// Filter for ``getLargestAccounts``.
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcLargestAccountsFilter {
    Circulating,
    NonCirculating,
}

impl From<RpcLargestAccountsFilter> for rpc_config::RpcLargestAccountsFilter {
    fn from(f: RpcLargestAccountsFilter) -> Self {
        match f {
            RpcLargestAccountsFilter::Circulating => {
                rpc_config::RpcLargestAccountsFilter::Circulating
            }
            RpcLargestAccountsFilter::NonCirculating => {
                rpc_config::RpcLargestAccountsFilter::NonCirculating
            }
        }
    }
}

impl From<rpc_config::RpcLargestAccountsFilter> for RpcLargestAccountsFilter {
    fn from(f: rpc_config::RpcLargestAccountsFilter) -> Self {
        match f {
            rpc_config::RpcLargestAccountsFilter::Circulating => {
                RpcLargestAccountsFilter::Circulating
            }
            rpc_config::RpcLargestAccountsFilter::NonCirculating => {
                RpcLargestAccountsFilter::NonCirculating
            }
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getLargestAccounts``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     filter (Optional[RpcLargestAccountsFilter]): Filter results by account type.
    ///
    => RpcLargestAccountsConfig
);

#[common_methods]
#[pymethods]
impl RpcLargestAccountsConfig {
    #[new]
    pub fn new(
        commitment: Option<CommitmentConfig>,
        filter: Option<RpcLargestAccountsFilter>,
    ) -> Self {
        Self(rpc_config::RpcLargestAccountsConfig {
            commitment: commitment.map(|c| c.into()),
            filter: filter.map(|f| f.into()),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcLargestAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn filter(&self) -> Option<RpcLargestAccountsFilter> {
        self.0.filter.clone().map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getSupply``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     exclude_non_circulating_accounts_list (bool): Exclude non circulating accounts list from response.
    ///
    => RpcSupplyConfig
);

#[common_methods]
#[pymethods]
impl RpcSupplyConfig {
    #[new]
    pub fn new(
        commitment: Option<CommitmentConfig>,
        exclude_non_circulating_accounts_list: bool,
    ) -> Self {
        Self(rpc_config::RpcSupplyConfig {
            commitment: commitment.map(|c| c.into()),
            exclude_non_circulating_accounts_list,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSupplyConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn exclude_non_circulating_accounts_list(&self) -> bool {
        self.0.exclude_non_circulating_accounts_list
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object containing epoch information.
    ///
    /// Args:
    ///     epoch (Optional[int]): Epoch is a unit of time a given leader schedule is honored, some number of Slots.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcEpochConfig
);

#[common_methods]
#[pymethods]
impl RpcEpochConfig {
    #[new]
    pub fn new(
        epoch: Option<u64>,
        commitment: Option<CommitmentConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcEpochConfig {
            epoch,
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
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn epoch(&self) -> Option<u64> {
        self.0.epoch
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getAccountInfo``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned account data.
    ///     data_slice (Optiona;[UiDataSliceConfig]): Limit the returned account data
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcAccountInfoConfig
);

#[common_methods]
#[pymethods]
impl RpcAccountInfoConfig {
    #[new]
    pub fn new(
        encoding: Option<UiAccountEncoding>,
        data_slice: Option<UiDataSliceConfig>,
        commitment: Option<CommitmentConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcAccountInfoConfig {
            encoding: encoding.map(|e| e.into()),
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
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn data_slice(&self) -> Option<UiDataSliceConfig> {
        self.0.data_slice.map(|d| d.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getProgramAccounts``.
    ///
    /// Args:
    ///     filters (Optional[Sequence[int | Memcmp]]): Filter results using various filter objects; account must meet all filter criteria to be included in results.
    ///     account_config (RpcAccountInfoConfig): Account info config.
    ///     with_context (Optional[bool]): Wrap the result in an RpcResponse JSON object.
    ///
    => RpcProgramAccountsConfig
);

#[common_methods]
#[pymethods]
impl RpcProgramAccountsConfig {
    #[new]
    pub fn new(
        filters: Option<Vec<RpcFilterType>>,
        account_config: RpcAccountInfoConfig,
        with_context: Option<bool>,
    ) -> Self {
        Self(rpc_config::RpcProgramAccountsConfig {
            filters: filters.map(|v| v.into_iter().map(|f| f.into()).collect()),
            account_config: account_config.into(),
            with_context,
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
    pub fn filters(&self) -> Option<Vec<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.0.filters.clone().map(|v| {
            v.into_iter()
                .map(|f| RpcFilterType::from(f).into_py(py))
                .collect()
        })
    }

    #[getter]
    pub fn with_context(&self) -> Option<bool> {
        self.0.with_context
    }
}

/// Fieldless filters for ``logsSubscribe``.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum RpcTransactionLogsFilter {
    All,
    AllWithVotes,
}

/// ``mentions`` filter for ``logsSubscribe``.
///
/// Args:
///     pubkey (str): Subscribe to all transactions that mention the provided Pubkey (as base-58 encoded string).
///
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct RpcTransactionLogsFilterMentions(Vec<String>);

#[richcmp_eq_only]
#[pymethods]
impl RpcTransactionLogsFilterMentions {
    #[new]
    pub fn new(pubkey: &str) -> Self {
        Self(vec![pubkey.to_string()])
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    #[getter]
    pub fn pubkey(&self) -> String {
        self.0[0].clone()
    }
}

impl RichcmpEqualityOnly for RpcTransactionLogsFilterMentions {}

#[derive(FromPyObject)]
pub enum TransactionLogsFilterWrapper {
    Plain(RpcTransactionLogsFilter),
    Mentions(RpcTransactionLogsFilterMentions),
}

impl From<TransactionLogsFilterWrapper> for rpc_config::RpcTransactionLogsFilter {
    fn from(w: TransactionLogsFilterWrapper) -> Self {
        match w {
            TransactionLogsFilterWrapper::Plain(f) => match f {
                RpcTransactionLogsFilter::All => rpc_config::RpcTransactionLogsFilter::All,
                RpcTransactionLogsFilter::AllWithVotes => {
                    rpc_config::RpcTransactionLogsFilter::AllWithVotes
                }
            },
            TransactionLogsFilterWrapper::Mentions(m) => {
                rpc_config::RpcTransactionLogsFilter::Mentions(m.0)
            }
        }
    }
}

impl From<rpc_config::RpcTransactionLogsFilter> for TransactionLogsFilterWrapper {
    fn from(f: rpc_config::RpcTransactionLogsFilter) -> Self {
        match f {
            rpc_config::RpcTransactionLogsFilter::All => Self::Plain(RpcTransactionLogsFilter::All),
            rpc_config::RpcTransactionLogsFilter::AllWithVotes => {
                Self::Plain(RpcTransactionLogsFilter::AllWithVotes)
            }
            rpc_config::RpcTransactionLogsFilter::Mentions(v) => {
                Self::Mentions(RpcTransactionLogsFilterMentions(v))
            }
        }
    }
}

pyclass_boilerplate!(
    /// Configuration object for ``logsSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///
    => RpcTransactionLogsConfig
);

#[common_methods]
#[pymethods]
impl RpcTransactionLogsConfig {
    #[new]
    pub fn new(commitment: Option<CommitmentConfig>) -> Self {
        Self(rpc_config::RpcTransactionLogsConfig {
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }
}

/// ``mint`` filter for ``getTokenAccountsBy*`` methods.
///
/// Args:
///     mint (str):  Pubkey of the specific token Mint to limit accounts to, as base-58 encoded string.
///
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct RpcTokenAccountsFilterMint(String);

#[richcmp_eq_only]
#[pymethods]
impl RpcTokenAccountsFilterMint {
    #[new]
    pub fn new(mint: &str) -> Self {
        Self(mint.to_string())
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    #[getter]
    pub fn mint(&self) -> String {
        self.0.clone()
    }
}

impl RichcmpEqualityOnly for RpcTokenAccountsFilterMint {}

/// ``programId`` filter for ``getTokenAccountsBy*`` methods.
///
/// Args:
///     program_id (str):   Pubkey of the Token program that owns the accounts, as base-58 encoded string.
///
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct RpcTokenAccountsFilterProgramId(String);

#[richcmp_eq_only]
#[pymethods]
impl RpcTokenAccountsFilterProgramId {
    #[new]
    pub fn new(program_id: &str) -> Self {
        Self(program_id.to_string())
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    #[getter]
    pub fn program_id(&self) -> String {
        self.0.clone()
    }
}

impl RichcmpEqualityOnly for RpcTokenAccountsFilterProgramId {}

#[derive(FromPyObject)]
pub enum RpcTokenAccountsFilterWrapper {
    Mint(RpcTokenAccountsFilterMint),
    ProgramId(RpcTokenAccountsFilterProgramId),
}

impl From<RpcTokenAccountsFilterWrapper> for rpc_config::RpcTokenAccountsFilter {
    fn from(w: RpcTokenAccountsFilterWrapper) -> Self {
        match w {
            RpcTokenAccountsFilterWrapper::Mint(m) => rpc_config::RpcTokenAccountsFilter::Mint(m.0),
            RpcTokenAccountsFilterWrapper::ProgramId(p) => {
                rpc_config::RpcTokenAccountsFilter::ProgramId(p.0)
            }
        }
    }
}

impl From<rpc_config::RpcTokenAccountsFilter> for RpcTokenAccountsFilterWrapper {
    fn from(f: rpc_config::RpcTokenAccountsFilter) -> Self {
        match f {
            rpc_config::RpcTokenAccountsFilter::Mint(s) => {
                RpcTokenAccountsFilterWrapper::Mint(RpcTokenAccountsFilterMint(s))
            }
            rpc_config::RpcTokenAccountsFilter::ProgramId(s) => {
                RpcTokenAccountsFilterWrapper::ProgramId(RpcTokenAccountsFilterProgramId(s))
            }
        }
    }
}

impl IntoPy<PyObject> for RpcTokenAccountsFilterWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            RpcTokenAccountsFilterWrapper::Mint(m) => m.0.into_py(py),
            RpcTokenAccountsFilterWrapper::ProgramId(m) => m.0.into_py(py),
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``signatureSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     enable_received_notification (Optional[bool]): Enable received notification.
    => RpcSignatureSubscribeConfig
);

#[common_methods]
#[pymethods]
impl RpcSignatureSubscribeConfig {
    #[new]
    fn new(
        commitment: Option<CommitmentConfig>,
        enable_received_notification: Option<bool>,
    ) -> Self {
        rpc_config::RpcSignatureSubscribeConfig {
            commitment: commitment.map(|c| c.into()),
            enable_received_notification,
        }
        .into()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn enable_received_notification(&self) -> Option<bool> {
        self.0.enable_received_notification
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSignatureSubscribeConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

/// Filter for ``blockSubscribe``.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub enum RpcBlockSubscribeFilter {
    All,
}

/// ``mentions`` filter for ``blockSubscribe``.
///
/// Args:
///     pubkey (str): Return only transactions that mention the provided public key (as base-58 encoded string)
///
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct RpcBlockSubscribeFilterMentions(String);

#[richcmp_eq_only]
#[pymethods]
impl RpcBlockSubscribeFilterMentions {
    #[new]
    pub fn new(pubkey: &str) -> Self {
        Self(pubkey.to_string())
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    #[getter]
    pub fn pubkey(&self) -> String {
        self.0.clone()
    }
}

impl RichcmpEqualityOnly for RpcBlockSubscribeFilterMentions {}

#[derive(FromPyObject)]
pub enum RpcBlockSubscribeFilterWrapper {
    All(RpcBlockSubscribeFilter),
    MentionsAccountOrProgram(RpcBlockSubscribeFilterMentions),
}

impl From<RpcBlockSubscribeFilterWrapper> for rpc_config::RpcBlockSubscribeFilter {
    fn from(w: RpcBlockSubscribeFilterWrapper) -> Self {
        match w {
            RpcBlockSubscribeFilterWrapper::All(_) => Self::All,
            RpcBlockSubscribeFilterWrapper::MentionsAccountOrProgram(p) => {
                Self::MentionsAccountOrProgram(p.0)
            }
        }
    }
}

impl From<rpc_config::RpcBlockSubscribeFilter> for RpcBlockSubscribeFilterWrapper {
    fn from(f: rpc_config::RpcBlockSubscribeFilter) -> Self {
        match f {
            rpc_config::RpcBlockSubscribeFilter::All => Self::All(RpcBlockSubscribeFilter::All),
            rpc_config::RpcBlockSubscribeFilter::MentionsAccountOrProgram(p) => {
                Self::MentionsAccountOrProgram(RpcBlockSubscribeFilterMentions(p))
            }
        }
    }
}

impl IntoPy<PyObject> for RpcBlockSubscribeFilterWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::All(m) => m.into_py(py),
            Self::MentionsAccountOrProgram(m) => m.into_py(py),
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``blockSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     transaction_details (Optional[TransactionDetails]): Level of transaction detail to return.
    ///     show_rewards (Optional[bool]): Whether to populate the ``rewards`` array.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcBlockSubscribeConfig
);

#[common_methods]
#[pymethods]
impl RpcBlockSubscribeConfig {
    #[new]
    fn new(
        commitment: Option<CommitmentConfig>,
        encoding: Option<UiTransactionEncoding>,
        transaction_details: Option<TransactionDetails>,
        show_rewards: Option<bool>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcBlockSubscribeConfig {
            commitment: commitment.map(|c| c.into()),
            encoding: encoding.map(|e| e.into()),
            transaction_details: transaction_details.map(|t| t.into()),
            show_rewards,
            max_supported_transaction_version,
        }
        .into()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn transaction_details(&self) -> Option<TransactionDetails> {
        self.0.transaction_details.map(|t| t.into())
    }

    #[getter]
    pub fn show_rewards(&self) -> Option<bool> {
        self.0.show_rewards
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockSubscribeConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getSignaturesForAddress``.
    ///
    /// Args:
    ///     before (Optional[str]): Start searching backwards from this transaction signature (base58-encoded).
    ///     until (Optional[str]): Search until this transaction signature (base58-encoded).
    ///     limit (Optional[int]): Maximum transaction signatures to return (between 1 and 1,000, default: 1,000).
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSignaturesForAddressConfig
);

#[common_methods]
#[pymethods]
impl RpcSignaturesForAddressConfig {
    #[new]
    fn new(
        before: Option<String>,
        until: Option<String>,
        limit: Option<usize>,
        commitment: Option<CommitmentConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        rpc_config::RpcSignaturesForAddressConfig {
            before,
            until,
            limit,
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        }
        .into()
    }

    #[getter]
    pub fn before(&self) -> Option<String> {
        self.0.before.clone()
    }

    #[getter]
    pub fn until(&self) -> Option<String> {
        self.0.until.clone()
    }

    #[getter]
    pub fn limit(&self) -> Option<usize> {
        self.0.limit
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
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

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getBlock``.
    ///
    /// Args:
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     transaction_details (Optional[TransactionDetails]): Level of transaction detail to return.
    ///     rewards (Optional[bool]): Whether to populate the ``rewards`` array.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcBlockConfig
);

#[common_methods]
#[pymethods]
impl RpcBlockConfig {
    #[new]
    pub fn new(
        encoding: Option<UiTransactionEncoding>,
        transaction_details: Option<TransactionDetails>,
        rewards: Option<bool>,
        commitment: Option<CommitmentConfig>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcBlockConfig {
            encoding: encoding.map(|e| e.into()),
            transaction_details: transaction_details.map(|t| t.into()),
            rewards,
            commitment: commitment.map(|c| c.into()),
            max_supported_transaction_version,
        }
        .into()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn transaction_details(&self) -> Option<TransactionDetails> {
        self.0.transaction_details.map(|t| t.into())
    }

    #[getter]
    pub fn rewards(&self) -> Option<bool> {
        self.0.rewards
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Create a new instance for only showing rewards.
    #[staticmethod]
    pub fn rewards_only() -> Self {
        rpc_config::RpcBlockConfig::rewards_only().into()
    }

    /// Create a new instance for only showing rewards, with a specified commitment level.
    #[staticmethod]
    pub fn rewards_with_commitment(commitment: Option<CommitmentConfig>) -> Self {
        rpc_config::RpcBlockConfig::rewards_with_commitment(commitment.map(|c| c.into())).into()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcTransactionConfig
);

#[common_methods]
#[pymethods]
impl RpcTransactionConfig {
    #[new]
    pub fn new(
        encoding: Option<UiTransactionEncoding>,
        commitment: Option<CommitmentConfig>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcTransactionConfig {
            encoding: encoding.map(|e| e.into()),
            commitment: commitment.map(|c| c.into()),
            max_supported_transaction_version,
        }
        .into()
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }
}

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionConfig>()?;
    config_mod.add_class::<RpcRequestAirdropConfig>()?;
    config_mod.add_class::<RpcLeaderScheduleConfig>()?;
    config_mod.add_class::<RpcBlockProductionConfigRange>()?;
    config_mod.add_class::<RpcGetVoteAccountsConfig>()?;
    config_mod.add_class::<RpcLargestAccountsFilter>()?;
    config_mod.add_class::<RpcSupplyConfig>()?;
    config_mod.add_class::<RpcEpochConfig>()?;
    config_mod.add_class::<RpcAccountInfoConfig>()?;
    config_mod.add_class::<RpcProgramAccountsConfig>()?;
    config_mod.add_class::<RpcTransactionLogsFilter>()?;
    config_mod.add_class::<RpcTransactionLogsFilterMentions>()?;
    config_mod.add_class::<RpcTransactionLogsConfig>()?;
    config_mod.add_class::<RpcTokenAccountsFilterMint>()?;
    config_mod.add_class::<RpcTokenAccountsFilterProgramId>()?;
    config_mod.add_class::<RpcSignatureSubscribeConfig>()?;
    config_mod.add_class::<RpcBlockSubscribeFilter>()?;
    config_mod.add_class::<RpcBlockSubscribeFilterMentions>()?;
    config_mod.add_class::<RpcBlockSubscribeConfig>()?;
    config_mod.add_class::<RpcSignaturesForAddressConfig>()?;
    config_mod.add_class::<RpcBlockConfig>()?;
    config_mod.add_class::<RpcTransactionConfig>()?;
    Ok(config_mod)
}
