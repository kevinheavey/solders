#![allow(deprecated)]
use crate::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    message::Message,
    py_from_bytes_general_via_cbor, pybytes_general_via_cbor, to_py_err,
    transaction::Transaction,
    CommonMethods, Pubkey, PyBytesCbor, PyErrWrapper, PyFromBytesCbor, RichcmpEqualityOnly,
};
use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::PyTuple,
    PyTypeInfo,
};
extern crate base64;
use crate::rpc::tmp_config::{
    RpcBlockSubscribeFilter, RpcTokenAccountsFilter, RpcTransactionLogsFilter,
};
use camelpaste::paste;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr, FromInto};
use solana_sdk::{
    message::Message as MessageOriginal, transaction::Transaction as TransactionOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only, rpc_id_getter};

use crate::{Signature, SolderHash};

use super::config::{
    RpcAccountInfoConfig, RpcBlockConfig, RpcBlockProductionConfig, RpcBlockSubscribeConfig,
    RpcBlockSubscribeFilterWrapper, RpcContextConfig, RpcEpochConfig, RpcGetVoteAccountsConfig,
    RpcLargestAccountsFilter, RpcLeaderScheduleConfig, RpcProgramAccountsConfig,
    RpcRequestAirdropConfig, RpcSendTransactionConfig, RpcSignatureStatusConfig,
    RpcSignatureSubscribeConfig, RpcSignaturesForAddressConfig, RpcSimulateTransactionConfig,
    RpcSupplyConfig, RpcTokenAccountsFilterWrapper, RpcTransactionConfig, RpcTransactionLogsConfig,
    TransactionLogsFilterWrapper,
};

create_exception!(
    solders,
    SerdeJSONError,
    PyException,
    "Raised when an error is encountered during JSON (de)serialization."
);

macro_rules! rpc_impl_display {
    ($ident:ident) => {
        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_json())
            }
        }
    };
}

macro_rules! request_boilerplate {
    ($name:ident) => {
        rpc_impl_display!($name);
        impl CommonMethods<'_> for $name {
            fn py_to_json(&self) -> String {
                let wrapped = Body::from(self.clone());
                serde_json::to_string(&wrapped).unwrap()
            }

            fn py_from_json(raw: &str) -> PyResult<Self> {
                let parsed = serde_json::from_str::<Body>(raw).map_err(to_py_err)?;
                match parsed {
                    Body::$name(x) => Ok(x),
                    _ => Err(PyValueError::new_err(format!(
                        "Deserialized to wrong type: {:?}",
                        parsed
                    ))),
                }
            }
        }
        impl RichcmpEqualityOnly for $name {}
        pybytes_general_via_cbor!($name);
        py_from_bytes_general_via_cbor!($name);
        impl From<$name> for Body {
            fn from(r: $name) -> Self {
                Self::$name(r)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UnsubscribeParams((u64,));

macro_rules! unsubscribe_def {
    ($name:ident) => {
        paste! {
        #[doc = "``" $name:camel "`` request.

Args:
    subscription_id (int): ID of subscription to cancel
    id (Optional[int]): Request ID.

Example:
     >>> from solders.rpc.requests import " $name "
     >>> " $name "(1, 2).to_json()
     '{\"method\":\"" $name:camel "\",\"jsonrpc\":\"2.0\",\"id\":2,\"params\":[1]}'
"]
                #[pyclass(module = "solders.rpc.requests")]
                #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
                pub struct $name {
                    #[serde(flatten)]
                    base: RequestBase,
                    params: UnsubscribeParams,
                }

                #[richcmp_eq_only]
                #[common_methods]
                #[rpc_id_getter]
                #[pymethods]
                impl $name {
                    #[new]
                    fn new(subscription_id: u64, id: Option<u64>) -> Self {
                        let params = UnsubscribeParams((subscription_id,));
                        let base = RequestBase::new(id);
                        Self { base, params }
                    }

                    /// int: ID of subscription to cancel
                    #[getter]
                    fn subscription_id(&self) -> u64 {
                        self.params.0 .0
                    }
                }

                request_boilerplate!($name);}
    };
}

macro_rules! zero_param_req_def {
    ($name:ident) => {
        paste! {
        #[doc = "``" $name:camel "`` request.

Args:
    id (Optional[int]): Request ID.

Example:
     >>> from solders.rpc.requests import " $name "
     >>> " $name "(123).to_json()
     '{\"method\":\"" $name:camel "\",\"jsonrpc\":\"2.0\",\"id\":123}'
"]
                #[pyclass(module = "solders.rpc.requests")]
                #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
                pub struct $name {
                    #[serde(flatten)]
                    base: RequestBase,
                }

                #[richcmp_eq_only]
                #[common_methods]
                #[rpc_id_getter]
                #[pymethods]
                impl $name {
                    #[new]
                    fn new(id: Option<u64>) -> Self {
                        let base = RequestBase::new(id);
                        Self { base }
                    }
                }

                request_boilerplate!($name);}
    };
}

unsubscribe_def!(AccountUnsubscribe);
unsubscribe_def!(BlockUnsubscribe);
unsubscribe_def!(LogsUnsubscribe);
unsubscribe_def!(ProgramUnsubscribe);
unsubscribe_def!(SignatureUnsubscribe);
unsubscribe_def!(SlotUnsubscribe);
unsubscribe_def!(SlotsUpdatesUnsubscribe);
unsubscribe_def!(RootUnsubscribe);
unsubscribe_def!(VoteUnsubscribe);

impl From<serde_json::Error> for PyErrWrapper {
    fn from(e: serde_json::Error) -> Self {
        Self(SerdeJSONError::new_err(e.to_string()))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
pub enum V2 {
    #[default]
    #[serde(rename = "2.0")]
    TwoPointOh,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct RequestBase {
    jsonrpc: V2,
    id: u64,
}

impl RequestBase {
    fn new(id: Option<u64>) -> Self {
        Self {
            jsonrpc: V2::TwoPointOh,
            id: id.unwrap_or(0),
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAccountInfoParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(default)] Option<RpcAccountInfoConfig>,
);

/// A ``getAccountInfo`` request.
///
/// Args:
///     pubkey (Pubkey): Pubkey of account to query.
///     config (Optional[RpcAccountInfoConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetAccountInfo
///     >>> from solders.rpc.config import RpcAccountInfoConfig
///     >>> from solders.pubkey import Pubkey
///     >>> from solders.account_decoder import UiAccountEncoding
///     >>> config = RpcAccountInfoConfig(UiAccountEncoding.Base64)
///     >>> GetAccountInfo(Pubkey.default(), config).to_json()
///     '{"method":"getAccountInfo","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"encoding":"base64","dataSlice":null,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAccountInfo {
    #[serde(flatten)]
    base: RequestBase,
    params: GetAccountInfoParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetAccountInfo {
    #[new]
    fn new(pubkey: Pubkey, config: Option<RpcAccountInfoConfig>, id: Option<u64>) -> Self {
        let params = GetAccountInfoParams(pubkey, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: Pubkey of account to query.
    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcAccountInfoConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetAccountInfo);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBalanceParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(default)] Option<RpcContextConfig>,
);

/// A ``getBalance`` request.
///
/// Args:
///     pubkey (Pubkey): Pubkey of account to query.
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBalance
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> from solders.pubkey import Pubkey
///     >>> config = RpcContextConfig(min_context_slot=1)
///     >>> GetBalance(Pubkey.default(), config).to_json()
///     '{"method":"getBalance","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"minContextSlot":1}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBalance {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBalanceParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBalance {
    #[new]
    fn new(pubkey: Pubkey, config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = GetBalanceParams(pubkey, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: Pubkey of account to query.
    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetBalance);

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockParams(u64, #[serde(default)] Option<RpcBlockConfig>);

/// A ``getBlock`` request.
///
/// Args:
///     slot (int): The slot to query.
///     config (Optional[RpcBlockConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlock
///     >>> from solders.rpc.config import RpcBlockConfig
///     >>> from solders.transaction_status import TransactionDetails, UiTransactionEncoding
///     >>> config = RpcBlockConfig(encoding=UiTransactionEncoding.Base58, transaction_details=TransactionDetails.None_)
///     >>> GetBlock(123, config).to_json()
///     '{"method":"getBlock","jsonrpc":"2.0","id":0,"params":[123,{"encoding":"base58","transactionDetails":"none","rewards":null,"maxSupportedTransactionVersion":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlock {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlockParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlock {
    #[new]
    fn new(slot: u64, config: Option<RpcBlockConfig>, id: Option<u64>) -> Self {
        let params = GetBlockParams(slot, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The slot to query.
    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }

    /// Optional[RpcBlockConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcBlockConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetBlock);

/// A ``getBlockHeight`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockHeight
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> config = RpcContextConfig(min_context_slot=123)
///     >>> GetBlockHeight(config).to_json()
///     '{"method":"getBlockHeight","jsonrpc":"2.0","id":0,"params":[{"minContextSlot":123}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockHeight {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockHeight {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetBlockHeight);

/// A ``getBlockProduction`` request.
///
/// Args:
///     config (Optional[RpcBlockProductionConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockProduction
///     >>> from solders.rpc.config import RpcBlockProductionConfig, RpcBlockProductionConfigRange
///     >>> from solders.pubkey import Pubkey
///     >>> slot_range = RpcBlockProductionConfigRange(first_slot=10, last_slot=15)
///     >>> config = RpcBlockProductionConfig(identity=Pubkey.default(), range=slot_range)
///     >>> GetBlockProduction(config).to_json()
///     '{"method":"getBlockProduction","jsonrpc":"2.0","id":0,"params":[{"identity":"11111111111111111111111111111111","range":{"firstSlot":10,"lastSlot":15}}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockProduction {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcBlockProductionConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockProduction {
    #[new]
    fn new(config: Option<RpcBlockProductionConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcBlockProductionConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcBlockProductionConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetBlockProduction);

/// A ``getBlockCommitment`` request.
///
/// Args:
///     slot (int): The slot to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockCommitment
///     >>> GetBlockCommitment(123).to_json()
///     '{"method":"getBlockCommitment","jsonrpc":"2.0","id":0,"params":[123]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockCommitment {
    #[serde(flatten)]
    base: RequestBase,
    params: (u64,),
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockCommitment {
    #[new]
    fn new(slot: u64, id: Option<u64>) -> Self {
        let params = (slot,);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The slot to query.
    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }
}

request_boilerplate!(GetBlockCommitment);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlocksParams(
    u64,
    #[serde(default)] Option<u64>,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    Option<CommitmentLevel>,
);

/// A ``getBlocks`` request.
///
/// Args:
///     start (int): The start slot.
///     end (Optional[int]): The end slot.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlocks
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> GetBlocks(123, commitment=CommitmentLevel.Processed).to_json()
///     '{"method":"getBlocks","jsonrpc":"2.0","id":0,"params":[123,null,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlocks {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlocksParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlocks {
    #[new]
    fn new(
        start: u64,
        end: Option<u64>,
        commitment: Option<CommitmentLevel>,
        id: Option<u64>,
    ) -> Self {
        let params = GetBlocksParams(start, end, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The start slot.
    #[getter]
    pub fn start(&self) -> u64 {
        self.params.0
    }

    /// Optional[int]: The end slot.
    #[getter]
    pub fn end(&self) -> Option<u64> {
        self.params.1
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.2
    }
}

request_boilerplate!(GetBlocks);

/// A ``getBlocksWithLimit`` request.
///
/// Args:
///     start (int): The start slot.
///     limit (Optional[int]): Maximum number of blocks.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlocksWithLimit
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> GetBlocksWithLimit(123, 5, commitment=CommitmentLevel.Processed).to_json()
///     '{"method":"getBlocksWithLimit","jsonrpc":"2.0","id":0,"params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlocksWithLimit {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlocksParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlocksWithLimit {
    #[new]
    fn new(
        start: u64,
        limit: Option<u64>,
        commitment: Option<CommitmentLevel>,
        id: Option<u64>,
    ) -> Self {
        let params = GetBlocksParams(start, limit, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The start slot.
    #[getter]
    pub fn start(&self) -> u64 {
        self.params.0
    }

    /// Optional[int]: Maximum number of blocks.
    #[getter]
    pub fn limit(&self) -> Option<u64> {
        self.params.1
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.2
    }
}

request_boilerplate!(GetBlocksWithLimit);

/// A ``getBlockTime`` request.
///
/// Args:
///     slot (int): The slot to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockTime
///     >>> GetBlockTime(123).to_json()
///     '{"method":"getBlockTime","jsonrpc":"2.0","id":0,"params":[123]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockTime {
    #[serde(flatten)]
    base: RequestBase,
    params: (u64,),
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockTime {
    #[new]
    fn new(slot: u64, id: Option<u64>) -> Self {
        let params = (slot,);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The slot to query.
    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }
}

request_boilerplate!(GetBlockTime);

zero_param_req_def!(GetClusterNodes);

/// A ``getEpochInfo`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetEpochInfo
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> config = RpcContextConfig(commitment=CommitmentLevel.Processed)
///     >>> GetEpochInfo(config).to_json()
///     '{"method":"getEpochInfo","jsonrpc":"2.0","id":0,"params":[{"commitment":"processed","minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetEpochInfo {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetEpochInfo {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetEpochInfo);

zero_param_req_def!(GetEpochSchedule);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct MessageBase64(pub String);

impl From<Message> for MessageBase64 {
    fn from(m: Message) -> Self {
        Self(base64::encode(m.0.serialize()))
    }
}

impl From<MessageBase64> for Message {
    fn from(m: MessageBase64) -> Self {
        let bytes = base64::decode(&m.0).unwrap();
        bincode::deserialize::<MessageOriginal>(&bytes)
            .unwrap()
            .into()
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetFeeForMessageParams(
    #[serde_as(as = "FromInto<MessageBase64>")] Message,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    Option<CommitmentLevel>,
);

/// A ``getFeeForMessage`` request.
///
/// Args:
///     message (Message): The message for which to calculate the fee.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetFeeForMessage
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> from solders.message import Message
///     >>> GetFeeForMessage(Message.default(), commitment=CommitmentLevel.Processed).to_json()
///     '{"method":"getFeeForMessage","jsonrpc":"2.0","id":0,"params":["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==",{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetFeeForMessage {
    #[serde(flatten)]
    base: RequestBase,
    params: GetFeeForMessageParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetFeeForMessage {
    #[new]
    fn new(message: Message, commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = GetFeeForMessageParams(message, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Message: The message for which to calculate the fee.
    #[getter]
    pub fn message(&self) -> Message {
        self.params.0.clone()
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1
    }
}

request_boilerplate!(GetFeeForMessage);

zero_param_req_def!(GetFirstAvailableBlock);
zero_param_req_def!(GetGenesisHash);
zero_param_req_def!(GetHealth);
zero_param_req_def!(GetHighestSnapshotSlot);
zero_param_req_def!(GetIdentity);
zero_param_req_def!(ValidatorExit);

/// A ``getInflationGovernor`` request.
///
/// Args:
///     config (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetInflationGovernor
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> GetInflationGovernor(CommitmentLevel.Finalized).to_json()
///     '{"method":"getInflationGovernor","jsonrpc":"2.0","id":0,"params":[{"commitment":"finalized"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetInflationGovernor {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(CommitmentConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetInflationGovernor {
    #[new]
    fn new(commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = commitment.map(|c| (c.into(),));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.map(|p| p.0.into())
    }
}

request_boilerplate!(GetInflationGovernor);
zero_param_req_def!(GetInflationRate);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetInflationRewardParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] Vec<Pubkey>,
    #[serde(default)] Option<RpcEpochConfig>,
);

/// A ``getInflationReward`` request.
///
/// Args:
///     addresses (Optional[Sequence[Pubkey]]): Addresses to query.
///     config (Optional[RpcEpochConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetInflationReward
///     >>> from solders.rpc.config import RpcEpochConfig
///     >>> from solders.pubkey import Pubkey
///     >>> config = RpcEpochConfig(epoch=1234)
///     >>> addresses = [Pubkey.default(), Pubkey.default()]
///     >>> GetInflationReward(addresses, config).to_json()
///     '{"method":"getInflationReward","jsonrpc":"2.0","id":0,"params":[["11111111111111111111111111111111","11111111111111111111111111111111"],{"epoch":1234,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetInflationReward {
    #[serde(flatten)]
    base: RequestBase,
    params: GetInflationRewardParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetInflationReward {
    #[new]
    fn new(addresses: Vec<Pubkey>, config: Option<RpcEpochConfig>, id: Option<u64>) -> Self {
        let params = GetInflationRewardParams(addresses, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[Sequence[Pubkey]]: Addresses to query.
    #[getter]
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.params.0.clone()
    }

    /// Optional[RpcEpochConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcEpochConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetInflationReward);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLargestAccountsParams(
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    Option<CommitmentLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")] Option<RpcLargestAccountsFilter>,
);

/// A ``getLargestAccounts`` request.
///
/// Args:
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     filter_ (Optional[RpcLargestAccountsFilter]): Filter results by account type.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetLargestAccounts
///     >>> from solders.rpc.config import RpcLargestAccountsFilter
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> commitment = CommitmentLevel.Processed
///     >>> filter_ = RpcLargestAccountsFilter.Circulating
///     >>> GetLargestAccounts(commitment=commitment, filter_=filter_).to_json()
///     '{"method":"getLargestAccounts","jsonrpc":"2.0","id":0,"params":[{"commitment":"processed"},"circulating"]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLargestAccounts {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<GetLargestAccountsParams>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetLargestAccounts {
    #[new]
    fn new(
        commitment: Option<CommitmentLevel>,
        filter_: Option<RpcLargestAccountsFilter>,
        id: Option<u64>,
    ) -> Self {
        let params = if commitment.is_some() || filter_.is_some() {
            Some(GetLargestAccountsParams(commitment, filter_))
        } else {
            None
        };
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.clone().and_then(|p| p.0)
    }

    /// Optional[RpcLargestAccountsFilter]: Filter results by account type.
    #[getter]
    pub fn filter_(&self) -> Option<RpcLargestAccountsFilter> {
        self.params.clone().and_then(|p| p.1)
    }
}

request_boilerplate!(GetLargestAccounts);

/// A ``getLatestBlockhash`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetLatestBlockhash
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> config = RpcContextConfig(commitment=CommitmentLevel.Processed)
///     >>> GetLatestBlockhash(config).to_json()
///     '{"method":"getLatestBlockhash","jsonrpc":"2.0","id":0,"params":[{"commitment":"processed","minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLatestBlockhash {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetLatestBlockhash {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetLatestBlockhash);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLeaderScheduleParams(
    #[serde(default)] Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] Option<RpcLeaderScheduleConfig>,
);

/// A ``GetLeaderSchedule`` request.
///
/// Args:
///     slot (Optional[int]): The slot to query.
///     config (Optional[RpcLeaderScheduleConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetLeaderSchedule
///     >>> from solders.rpc.config import RpcLeaderScheduleConfig
///     >>> from solders.pubkey import Pubkey
///     >>> config = RpcLeaderScheduleConfig(identity=Pubkey.default())
///     >>> GetLeaderSchedule(123, config).to_json()
///     '{"method":"getLeaderSchedule","jsonrpc":"2.0","id":0,"params":[123,{"identity":"11111111111111111111111111111111"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLeaderSchedule {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<GetLeaderScheduleParams>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetLeaderSchedule {
    #[new]
    fn new(slot: Option<u64>, config: Option<RpcLeaderScheduleConfig>, id: Option<u64>) -> Self {
        let params = if slot.is_some() || config.is_some() {
            Some(GetLeaderScheduleParams(slot, config))
        } else {
            None
        };
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[int]: The slot to query.
    #[getter]
    pub fn slot(&self) -> Option<u64> {
        self.params.clone().and_then(|p| p.0)
    }

    /// Optional[RpcLeaderScheduleConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcLeaderScheduleConfig> {
        self.params.clone().and_then(|p| p.1)
    }
}

request_boilerplate!(GetLeaderSchedule);
zero_param_req_def!(GetMaxRetransmitSlot);
zero_param_req_def!(GetMaxShredInsertSlot);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMinimumBalanceForRentExemptionParams(
    usize,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    Option<CommitmentLevel>,
);

/// A ``getMinimumBalanceForRentExemption`` request.
///
/// Args:
///     length (int): Acccount data length
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetMinimumBalanceForRentExemption
///     >>> GetMinimumBalanceForRentExemption(50).to_json()
///     '{"method":"getMinimumBalanceForRentExemption","jsonrpc":"2.0","id":0,"params":[50]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMinimumBalanceForRentExemption {
    #[serde(flatten)]
    base: RequestBase,
    params: GetMinimumBalanceForRentExemptionParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetMinimumBalanceForRentExemption {
    #[new]
    fn new(length: usize, commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = GetMinimumBalanceForRentExemptionParams(length, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: Acccount data length
    #[getter]
    pub fn length(&self) -> usize {
        self.params.0
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1
    }
}

request_boilerplate!(GetMinimumBalanceForRentExemption);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMultipleAccountsParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] Vec<Pubkey>,
    #[serde(default)] Option<RpcAccountInfoConfig>,
);

/// A ``getMultipleAccounts`` request.
///
/// Args:
///     accounts (Sequence[Pubkey]): Accounts to query.
///     config (Optional[RpcAccountInfoConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetMultipleAccounts
///     >>> from solders.rpc.config import RpcAccountInfoConfig
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> from solders.pubkey import Pubkey
///     >>> from solders.account_decoder import UiAccountEncoding, UiDataSliceConfig
///     >>> encoding = UiAccountEncoding.Base64Zstd
///     >>> data_slice = UiDataSliceConfig(10, 8)
///     >>> config = RpcAccountInfoConfig(encoding=encoding, data_slice=data_slice)
///     >>> accounts = [Pubkey.default(), Pubkey.default()]
///     >>> GetMultipleAccounts(accounts, config).to_json()
///     '{"method":"getMultipleAccounts","jsonrpc":"2.0","id":0,"params":[["11111111111111111111111111111111","11111111111111111111111111111111"],{"encoding":"base64+zstd","dataSlice":{"offset":10,"length":8},"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMultipleAccounts {
    #[serde(flatten)]
    base: RequestBase,
    params: GetMultipleAccountsParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetMultipleAccounts {
    #[new]
    fn new(accounts: Vec<Pubkey>, config: Option<RpcAccountInfoConfig>, id: Option<u64>) -> Self {
        let params = GetMultipleAccountsParams(accounts, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Sequence[Pubkey]: Accounts to query.
    #[getter]
    pub fn accounts(&self) -> Vec<Pubkey> {
        self.params.0.clone()
    }

    /// Optional[RpcAccountInfoConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetMultipleAccounts);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetProgramAccountsParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(default)] Option<RpcProgramAccountsConfig>,
);

/// A ``getProgramAccounts`` request.
///
/// Args:
///     program (Pubkey): The program that owns the accounts
///     config (Optional[RpcProgramAccountsConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetProgramAccounts
///     >>> from solders.rpc.config import RpcProgramAccountsConfig, RpcAccountInfoConfig
///     >>> from solders.rpc.filter import Memcmp
///     >>> from solders.pubkey import Pubkey
///     >>> acc_info_config = RpcAccountInfoConfig.default()
///     >>> filters = [10, Memcmp(offset=10, bytes_=b"123")]
///     >>> config = RpcProgramAccountsConfig(acc_info_config, filters)
///     >>> GetProgramAccounts(Pubkey.default(), config).to_json()
///     '{"method":"getProgramAccounts","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"filters":[{"dataSize":10},{"memcmp":{"offset":10,"bytes":[49,50,51],"encoding":null}}],"encoding":null,"dataSlice":null,"minContextSlot":null,"withContext":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetProgramAccounts {
    #[serde(flatten)]
    base: RequestBase,
    params: GetProgramAccountsParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetProgramAccounts {
    #[new]
    fn new(program: Pubkey, config: Option<RpcProgramAccountsConfig>, id: Option<u64>) -> Self {
        let params = GetProgramAccountsParams(program, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The program that owns the accounts
    #[getter]
    pub fn program(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcProgramAccountsConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcProgramAccountsConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetProgramAccounts);

/// A ``getRecentPerformanceSamples`` request.
///
/// Args:
///     limit (int): Number of samples to return (maximum 720).
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetRecentPerformanceSamples
///     >>> GetRecentPerformanceSamples(5).to_json()
///     '{"method":"getRecentPerformanceSamples","jsonrpc":"2.0","id":0,"params":[5]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetRecentPerformanceSamples {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(usize,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetRecentPerformanceSamples {
    #[new]
    fn new(limit: Option<usize>, id: Option<u64>) -> Self {
        let params = limit.map(|x| (x,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: Number of samples to return.
    #[getter]
    pub fn limit(&self) -> Option<usize> {
        self.params.map(|x| x.0)
    }
}

request_boilerplate!(GetRecentPerformanceSamples);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignaturesForAddressParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(default)] Option<RpcSignaturesForAddressConfig>,
);

/// A ``getSignaturesForAddress`` request.
///
/// Args:
///     address (Pubkey): The address by which to filter transactions.
///     config (Optional[RpcSignaturesForAddressConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSignaturesForAddress
///     >>> from solders.rpc.config import RpcSignaturesForAddressConfig
///     >>> config = RpcSignaturesForAddressConfig(limit=10)
///     >>> GetSignaturesForAddress(Pubkey.default(), config).to_json()
///     '{"method":"getSignaturesForAddress","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"before":null,"until":null,"limit":10,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignaturesForAddress {
    #[serde(flatten)]
    base: RequestBase,
    params: GetSignaturesForAddressParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSignaturesForAddress {
    #[new]
    fn new(
        address: Pubkey,
        config: Option<RpcSignaturesForAddressConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = GetSignaturesForAddressParams(address, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The address by which to filter transactions.
    #[getter]
    pub fn address(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcSignaturesForAddressConfig]: Extra configuration
    #[getter]
    pub fn config(&self) -> Option<RpcSignaturesForAddressConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetSignaturesForAddress);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignatureStatusesParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] Vec<Signature>,
    #[serde(default)] Option<RpcSignatureStatusConfig>,
);

/// A ``getSignatureStatuses`` request.
///
/// Args:
///     signatures (Sequence[Signature]): The signatures to query.
///     config (Optional[RpcSignatureStatusConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSignatureStatuses
///     >>> from solders.signature import Signature
///     >>> from solders.rpc.config import RpcSignatureStatusConfig
///     >>> config = RpcSignatureStatusConfig(search_transaction_history=True)
///     >>> GetSignatureStatuses([Signature.default()], config).to_json()
///     '{"method":"getSignatureStatuses","jsonrpc":"2.0","id":0,"params":[["1111111111111111111111111111111111111111111111111111111111111111"],{"searchTransactionHistory":true}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignatureStatuses {
    #[serde(flatten)]
    base: RequestBase,
    params: GetSignatureStatusesParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSignatureStatuses {
    #[new]
    fn new(
        signatures: Vec<Signature>,
        config: Option<RpcSignatureStatusConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = GetSignatureStatusesParams(signatures, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Sequence[Signature]: The signatures to query.
    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        self.params.0.clone()
    }

    /// Optional[RpcSignatureStatusConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcSignatureStatusConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetSignatureStatuses);

/// A ``getSlot`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSlot
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> config = RpcContextConfig(min_context_slot=123)
///     >>> GetSlot(config).to_json()
///     '{"method":"getSlot","jsonrpc":"2.0","id":0,"params":[{"minContextSlot":123}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSlot {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSlot {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetSlot);

/// A ``getSlotLeader`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSlotLeader
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> config = RpcContextConfig(min_context_slot=123)
///     >>> GetSlotLeader(config).to_json()
///     '{"method":"getSlotLeader","jsonrpc":"2.0","id":0,"params":[{"minContextSlot":123}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSlotLeader {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSlotLeader {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetSlotLeader);

/// A ``getSlotLeaders`` request.
///
/// Args:
///     start (int): The start slot.
///     limit (int): The number of leaders to return.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSlotLeaders
///     >>> GetSlotLeaders(100, 10).to_json()
///     '{"method":"getSlotLeaders","jsonrpc":"2.0","id":0,"params":[100,10]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSlotLeaders {
    #[serde(flatten)]
    base: RequestBase,
    params: (u64, u64),
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSlotLeaders {
    #[new]
    fn new(start: u64, limit: u64, id: Option<u64>) -> Self {
        let params = (start, limit);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// int: The start slot.
    #[getter]
    pub fn start(&self) -> u64 {
        self.params.0
    }

    /// int: The number of leaders to return.
    #[getter]
    pub fn limit(&self) -> u64 {
        self.params.1
    }
}

request_boilerplate!(GetSlotLeaders);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetStakeActivationParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(default)] Option<RpcEpochConfig>,
);

/// A ``getStakeActivation`` request.
///
/// Args:
///     account (Pubkey): The stake account to query.
///     config (Optional[RpcEpochConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetStakeActivation
///     >>> from solders.rpc.config import RpcEpochConfig
///     >>> from solders.pubkey import Pubkey
///     >>> config = RpcEpochConfig(epoch=1234)
///     >>> GetStakeActivation(Pubkey.default(), config).to_json()
///     '{"method":"getStakeActivation","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"epoch":1234,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetStakeActivation {
    #[serde(flatten)]
    base: RequestBase,
    params: GetStakeActivationParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetStakeActivation {
    #[new]
    fn new(account: Pubkey, config: Option<RpcEpochConfig>, id: Option<u64>) -> Self {
        let params = GetStakeActivationParams(account, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The stake account to query.
    #[getter]
    pub fn account(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcEpochConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcEpochConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetStakeActivation);

/// A ``getSupply`` request.
///
/// Args:
///     config (Optional[RpcSupplyConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetSupply
///     >>> from solders.rpc.config import RpcSupplyConfig
///     >>> config = RpcSupplyConfig(exclude_non_circulating_accounts_list=True)
///     >>> GetSupply(config).to_json()
///     '{"method":"getSupply","jsonrpc":"2.0","id":0,"params":[{"excludeNonCirculatingAccountsList":true}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSupply {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcSupplyConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetSupply {
    #[new]
    fn new(config: Option<RpcSupplyConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcSupplyConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcSupplyConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetSupply);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PubkeyAndCommitmentParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    Option<CommitmentLevel>,
);

/// A ``getTokenAccountBalance`` request.
///
/// Args:
///     account (Pubkey): The token account to query.
///     commitment (Optional[CommitmentLevel]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTokenAccountBalance
///     >>> from solders.commitment_config import CommitmentLevel
///     >>> from solders.pubkey import Pubkey
///     >>> GetTokenAccountBalance(Pubkey.default(), CommitmentLevel.Processed).to_json()
///     '{"method":"getTokenAccountBalance","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenAccountBalance {
    #[serde(flatten)]
    base: RequestBase,
    params: PubkeyAndCommitmentParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTokenAccountBalance {
    #[new]
    fn new(account: Pubkey, commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = PubkeyAndCommitmentParams(account, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The token account to query.
    #[getter]
    pub fn account(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1
    }
}

request_boilerplate!(GetTokenAccountBalance);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenAccountsByDelegateParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde_as(as = "FromInto<RpcTokenAccountsFilter>")] RpcTokenAccountsFilterWrapper,
    #[serde(default)] Option<RpcAccountInfoConfig>,
);

/// A ``getTokenAccountsByDelegate`` request.
///
/// Args:
///     account (Pubkey): The account delegate to query.
///     filter_ (RpcTokenAccountsFilterMint | RpcTokenAccountsFilterProgramId): Filter by either token mint or token program.
///     config (Optional[RpcAccountInfoConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTokenAccountsByDelegate
///     >>> from solders.rpc.config import RpcTokenAccountsFilterProgramId, RpcAccountInfoConfig
///     >>> from solders.pubkey import Pubkey
///     >>> program_filter = RpcTokenAccountsFilterProgramId(Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
///     >>> config = RpcAccountInfoConfig(min_context_slot=1234)
///     >>> req = GetTokenAccountsByDelegate(Pubkey.default(), program_filter, config)
///     >>> req.to_json()
///     '{"method":"getTokenAccountsByDelegate","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"},{"encoding":null,"dataSlice":null,"minContextSlot":1234}]}'
///     >>> req.filter_
///     RpcTokenAccountsFilterProgramId(
///         Pubkey(
///             TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA,
///         ),
///     )
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenAccountsByDelegate {
    #[serde(flatten)]
    base: RequestBase,
    params: GetTokenAccountsByDelegateParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTokenAccountsByDelegate {
    #[new]
    fn new(
        account: Pubkey,
        filter_: RpcTokenAccountsFilterWrapper,
        config: Option<RpcAccountInfoConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = GetTokenAccountsByDelegateParams(account, filter_, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The account delegate to query.
    #[getter]
    pub fn account(&self) -> Pubkey {
        self.params.0
    }

    /// RpcTokenAccountsFilterWrapper: Filter by either token mint or token program.
    #[getter]
    pub fn filter_(&self) -> RpcTokenAccountsFilterWrapper {
        self.params.1.clone()
    }

    /// Optional[RpcAccountInfoConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.2.clone()
    }
}

request_boilerplate!(GetTokenAccountsByDelegate);

/// A ``getTokenAccountsByOwner`` request.
///
/// Args:
///     account (Pubkey): The account owner to query.
///     filter_ (RpcTokenAccountsFilterMint | RpcTokenAccountsFilterProgramId): Filter by either token mint or token program.
///     config (Optional[RpcAccountInfoConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTokenAccountsByOwner
///     >>> from solders.rpc.config import RpcTokenAccountsFilterMint, RpcAccountInfoConfig
///     >>> from solders.pubkey import Pubkey
///     >>> mint_filter = RpcTokenAccountsFilterMint(Pubkey.default())
///     >>> config = RpcAccountInfoConfig(min_context_slot=1234)
///     >>> req = GetTokenAccountsByOwner(Pubkey.default(), mint_filter, config)
///     >>> req.to_json()
///     '{"method":"getTokenAccountsByOwner","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"mint":"11111111111111111111111111111111"},{"encoding":null,"dataSlice":null,"minContextSlot":1234}]}'
///     >>> req.filter_
///     RpcTokenAccountsFilterMint(
///         Pubkey(
///             11111111111111111111111111111111,
///         ),
///     )
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenAccountsByOwner {
    #[serde(flatten)]
    base: RequestBase,
    params: GetTokenAccountsByDelegateParams, // not a mistake that we're reusing GetTokenAccountsByDelegateParams
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTokenAccountsByOwner {
    #[new]
    fn new(
        account: Pubkey,
        filter_: RpcTokenAccountsFilterWrapper,
        config: Option<RpcAccountInfoConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = GetTokenAccountsByDelegateParams(account, filter_, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The account owner to query.
    #[getter]
    pub fn account(&self) -> Pubkey {
        self.params.0
    }

    /// RpcTokenAccountsFilterWrapper: Filter by either token mint or token program.
    #[getter]
    pub fn filter_(&self) -> RpcTokenAccountsFilterWrapper {
        self.params.1.clone()
    }

    /// Optional[RpcAccountInfoConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.2.clone()
    }
}

request_boilerplate!(GetTokenAccountsByOwner);

/// A ``getTokenLargestAccounts`` request.
///
/// Args:
///     mint (Pubkey): The token mint to query.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTokenLargestAccounts
///     >>> from solders.pubkey import Pubkey
///     >>> GetTokenLargestAccounts(Pubkey.default()).to_json()
///     '{"method":"getTokenLargestAccounts","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111"]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenLargestAccounts {
    #[serde(flatten)]
    base: RequestBase,
    params: PubkeyAndCommitmentParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTokenLargestAccounts {
    #[new]
    fn new(mint: Pubkey, commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = PubkeyAndCommitmentParams(mint, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The token mint to query.
    #[getter]
    pub fn mint(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1
    }
}

request_boilerplate!(GetTokenLargestAccounts);

/// A ``getTokenSupply`` request.
///
/// Args:
///     mint (Pubkey): The token mint to query.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTokenSupply
///     >>> from solders.pubkey import Pubkey
///     >>> GetTokenSupply(Pubkey.default()).to_json()
///     '{"method":"getTokenSupply","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111"]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenSupply {
    #[serde(flatten)]
    base: RequestBase,
    params: PubkeyAndCommitmentParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTokenSupply {
    #[new]
    fn new(mint: Pubkey, commitment: Option<CommitmentLevel>, id: Option<u64>) -> Self {
        let params = PubkeyAndCommitmentParams(mint, commitment);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The token mint to query.
    #[getter]
    pub fn mint(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[CommitmentLevel]: Bank state to query.
    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1
    }
}

request_boilerplate!(GetTokenSupply);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTransactionParams(
    #[serde_as(as = "DisplayFromStr")] Signature,
    #[serde(default)] Option<RpcTransactionConfig>,
);

/// A ``getTransaction`` request.
///
/// Args:
///     signature (Signature): The transaction signature to query.
///     config (Optional[RpcTransactionConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTransaction
///     >>> from solders.rpc.config import RpcTransactionConfig
///     >>> from solders.signature import Signature
///     >>> config = RpcTransactionConfig(max_supported_transaction_version=1)
///     >>> GetTransaction(Signature.default(), config).to_json()
///     '{"method":"getTransaction","jsonrpc":"2.0","id":0,"params":["1111111111111111111111111111111111111111111111111111111111111111",{"encoding":null,"maxSupportedTransactionVersion":1}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTransaction {
    #[serde(flatten)]
    base: RequestBase,
    params: GetTransactionParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTransaction {
    #[new]
    fn new(signature: Signature, config: Option<RpcTransactionConfig>, id: Option<u64>) -> Self {
        let params = GetTransactionParams(signature, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The signature to query.
    #[getter]
    pub fn signature(&self) -> Signature {
        self.params.0
    }

    /// Optional[RpcTransactionConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcTransactionConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetTransaction);

/// A ``getTransactionCount`` request.
///
/// Args:
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetTransactionCount
///     >>> from solders.rpc.config import RpcContextConfig
///     >>> config = RpcContextConfig(min_context_slot=1234)
///     >>> GetTransactionCount(config).to_json()
///     '{"method":"getTransactionCount","jsonrpc":"2.0","id":0,"params":[{"minContextSlot":1234}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTransactionCount {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcContextConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetTransactionCount {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.clone().map(|c| c.0)
    }
}

request_boilerplate!(GetTransactionCount);
zero_param_req_def!(GetVersion);

/// A ``getVoteAccounts`` request.
///
/// Args:
///     config (Optional[RpcGetVoteAccountsConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetVoteAccounts
///     >>> from solders.rpc.config import RpcGetVoteAccountsConfig
///     >>> config = RpcGetVoteAccountsConfig(keep_unstaked_delinquents=False)
///     >>> GetVoteAccounts(config).to_json()
///     '{"method":"getVoteAccounts","jsonrpc":"2.0","id":0,"params":[{"votePubkey":null,"keepUnstakedDelinquents":false,"delinquentSlotDistance":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetVoteAccounts {
    #[serde(flatten)]
    base: RequestBase,
    #[serde(default)]
    params: Option<(RpcGetVoteAccountsConfig,)>,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetVoteAccounts {
    #[new]
    fn new(config: Option<RpcGetVoteAccountsConfig>, id: Option<u64>) -> Self {
        let params = config.map(|c| (c,));
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Optional[RpcGetVoteAccountsConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcGetVoteAccountsConfig> {
        self.params.clone().map(|p| p.0)
    }
}

request_boilerplate!(GetVoteAccounts);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IsBlockhashValidParams(
    #[serde_as(as = "DisplayFromStr")] SolderHash,
    #[serde(default)] Option<RpcContextConfig>,
);

/// An ``isBlockhashValid`` request.
///
/// Args:
///     blockhash (Hash): The blockhash to check.
///     config (Optional[RpcContextConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import IsBlockhashValid
///     >>> from solders.hash import Hash
///     >>> IsBlockhashValid(Hash.default()).to_json()
///     '{"method":"isBlockhashValid","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111"]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IsBlockhashValid {
    #[serde(flatten)]
    base: RequestBase,
    params: IsBlockhashValidParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl IsBlockhashValid {
    #[new]
    fn new(blockhash: SolderHash, config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = IsBlockhashValidParams(blockhash, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Hash: The blockhash to check.
    #[getter]
    pub fn blockhash(&self) -> SolderHash {
        self.params.0
    }

    /// Optional[RpcContextConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(IsBlockhashValid);
zero_param_req_def!(MinimumLedgerSlot);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct RequestAirdropParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    u64,
    #[serde(default)] Option<RpcRequestAirdropConfig>,
);

/// A ``requestAirdrop`` request.
///
/// Args:
///     pubkey (Pubkey): Pubkey of account to receive lamports.
///     lamports (int): How many lamports to airdrop.
///     config (Optional[RpcRequestAirdropConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from solders.rpc.requests import RequestAirdrop
///      >>> from solders.rpc.config import RpcRequestAirdropConfig
///      >>> from solders.pubkey import Pubkey
///      >>> from solders.commitment_config import CommitmentLevel
///      >>> config = RpcRequestAirdropConfig(commitment=CommitmentLevel.Confirmed)
///      >>> RequestAirdrop(Pubkey.default(), 1000, config).to_json()
///      '{"method":"requestAirdrop","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",1000,{"recentBlockhash":null,"commitment":"confirmed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestAirdrop {
    #[serde(flatten)]
    base: RequestBase,
    params: RequestAirdropParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl RequestAirdrop {
    #[new]
    fn new(
        pubkey: Pubkey,
        lamports: u64,
        config: Option<RpcRequestAirdropConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = RequestAirdropParams(pubkey, lamports, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: Pubkey of account to receive lamports.
    #[getter]
    fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    /// int: How many lamports to airdrop.
    #[getter]
    fn lamports(&self) -> u64 {
        self.params.1
    }

    /// Optional[RpcRequestAirdropConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcRequestAirdropConfig> {
        self.params.2.clone()
    }
}

request_boilerplate!(RequestAirdrop);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct TransactionBase64(pub String);

impl From<Transaction> for TransactionBase64 {
    fn from(tx: Transaction) -> Self {
        Self(base64::encode(bincode::serialize(&tx).unwrap()))
    }
}

impl From<TransactionBase64> for Transaction {
    fn from(tx: TransactionBase64) -> Self {
        let bytes = base64::decode(&tx.0).unwrap();
        bincode::deserialize::<TransactionOriginal>(&bytes)
            .unwrap()
            .into()
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SendTransactionParams(
    #[serde_as(as = "FromInto<TransactionBase64>")] Transaction,
    #[serde(default)] Option<RpcSendTransactionConfig>,
);

/// A ``sendTransaction`` request.
///
/// Args:
///     tx (Transaction): The signed transaction to send.
///     config (Optional[RpcSendTransactionConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from typing import List
///      >>> from solders.rpc.requests import SendTransaction
///      >>> from solders.rpc.config import RpcSendTransactionConfig
///      >>> from solders.transaction import Transaction
///      >>> from solders.message import Message
///      >>> from solders.keypair import Keypair
///      >>> from solders.instruction import Instruction, AccountMeta
///      >>> from solders.hash import Hash
///      >>> from solders.pubkey import Pubkey
///      >>> from solders.commitment_config import CommitmentLevel
///      >>> program_id = Pubkey.default()
///      >>> arbitrary_instruction_data = b"abc"
///      >>> accounts: List[AccountMeta] = []
///      >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
///      >>> seed = bytes([1] * 32)
///      >>> payer = Keypair.from_seed(seed)
///      >>> message = Message([instruction], payer.pubkey())
///      >>> blockhash = Hash.default()  # replace with a real blockhash
///      >>> tx = Transaction([payer], message, blockhash)
///      >>> commitment = CommitmentLevel.Confirmed
///      >>> config = RpcSendTransactionConfig(preflight_commitment=commitment)
///      >>> SendTransaction(tx, config).to_json()
///      '{"method":"sendTransaction","jsonrpc":"2.0","id":0,"params":["AaVkKDb3UlpidO/ucBnOcmS+1dY8ZAC4vHxTxiccV8zPBlupuozppRjwrILZJaoKggAcVSD1XlAKstDVEPFOVgwBAAECiojj3XQJ8ZX9UtstPLpdcspnCb8dlBIb83SIAbQPb1wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQEAA2FiYw==",{"skipPreflight":false,"preflightCommitment":"confirmed","encoding":"base64","maxRetries":null,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendTransaction {
    #[serde(flatten)]
    base: RequestBase,
    params: SendTransactionParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl SendTransaction {
    #[new]
    fn new(tx: Transaction, config: Option<RpcSendTransactionConfig>, id: Option<u64>) -> Self {
        let params = SendTransactionParams(tx, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Transaction: The signed transaction to send.
    #[getter]
    fn tx(&self) -> Transaction {
        self.params.0.clone()
    }

    /// Optional[RpcSendTransactionConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcSendTransactionConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(SendTransaction);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SimulateTransactionParams(
    #[serde_as(as = "FromInto<TransactionBase64>")] Transaction,
    #[serde(default)] Option<RpcSimulateTransactionConfig>,
);

/// A ``simulateTransaction`` request.
///
/// Args:
///     tx (Transaction): The (possibly unsigned) transaction to simulate.
///     config (Optional[RpcSimulateTransactionConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from solders.rpc.requests import SimulateTransaction
///      >>> from solders.rpc.config import RpcSimulateTransactionConfig, RpcSimulateTransactionAccountsConfig
///      >>> from solders.account_decoder import UiAccountEncoding
///      >>> from solders.transaction import Transaction
///      >>> from solders.message import Message
///      >>> from solders.keypair import Keypair
///      >>> from solders.instruction import Instruction
///      >>> from solders.hash import Hash
///      >>> from solders.pubkey import Pubkey
///      >>> from solders.commitment_config import CommitmentLevel
///      >>> program_id = Pubkey.default()
///      >>> arbitrary_instruction_data = b"abc"
///      >>> accounts = []
///      >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
///      >>> seed = bytes([1] * 32)
///      >>> payer = Keypair.from_seed(seed)
///      >>> message = Message([instruction], payer.pubkey())
///      >>> blockhash = Hash.default()  # replace with a real blockhash
///      >>> tx = Transaction([payer], message, blockhash)
///      >>> account_encoding = UiAccountEncoding.Base64Zstd
///      >>> accounts_config = RpcSimulateTransactionAccountsConfig([Pubkey.default()], account_encoding)
///      >>> commitment = CommitmentLevel.Confirmed
///      >>> config = RpcSimulateTransactionConfig(commitment=commitment, accounts=accounts_config)
///      >>> SimulateTransaction(tx, config).to_json()
///      '{"method":"simulateTransaction","jsonrpc":"2.0","id":0,"params":["AaVkKDb3UlpidO/ucBnOcmS+1dY8ZAC4vHxTxiccV8zPBlupuozppRjwrILZJaoKggAcVSD1XlAKstDVEPFOVgwBAAECiojj3XQJ8ZX9UtstPLpdcspnCb8dlBIb83SIAbQPb1wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQEAA2FiYw==",{"sigVerify":false,"replaceRecentBlockhash":false,"commitment":"confirmed","encoding":"base64","accounts":{"encoding":"base64+zstd","addresses":["11111111111111111111111111111111"]},"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SimulateTransaction {
    #[serde(flatten)]
    base: RequestBase,
    params: SimulateTransactionParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl SimulateTransaction {
    #[new]
    fn new(tx: Transaction, config: Option<RpcSimulateTransactionConfig>, id: Option<u64>) -> Self {
        let params = SimulateTransactionParams(tx, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Transaction: The signed transaction to send.
    #[getter]
    fn tx(&self) -> Transaction {
        self.params.0.clone()
    }

    /// Optional[RpcSimulateTransactionConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcSimulateTransactionConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(SimulateTransaction);

/// An ``accountSubscribe`` request.
///
/// Args:
///     account (Pubkey): Account to watch.
///     config (Optional[RpcAccountInfoConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import AccountSubscribe
///     >>> from solders.rpc.config import RpcAccountInfoConfig
///     >>> from solders.pubkey import Pubkey
///     >>> from solders.account_decoder import UiAccountEncoding
///     >>> config = RpcAccountInfoConfig(UiAccountEncoding.Base64)
///     >>> AccountSubscribe(Pubkey.default(), config).to_json()
///     '{"method":"accountSubscribe","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"encoding":"base64","dataSlice":null,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountSubscribe {
    #[serde(flatten)]
    base: RequestBase,
    params: GetAccountInfoParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl AccountSubscribe {
    #[new]
    fn new(account: Pubkey, config: Option<RpcAccountInfoConfig>, id: Option<u64>) -> Self {
        let params = GetAccountInfoParams(account, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: Account to watch.
    #[getter]
    fn account(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcAccountInfoConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(AccountSubscribe);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct BlockSubscribeParams(
    #[serde_as(as = "FromInto<RpcBlockSubscribeFilter>")] RpcBlockSubscribeFilterWrapper,
    #[serde(default)] Option<RpcBlockSubscribeConfig>,
);

/// A ``blockSubscribe`` request.
///
/// Args:
///     filter_ (RpcBlockSubscribeFilter | RpcBlockSubscribeFilterMentions): Filter criteria for the logs to receive results by account type.
///     config (Optional[RpcBlockSubscribeConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from solders.rpc.requests import BlockSubscribe
///      >>> from solders.rpc.config import RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcBlockSubscribeFilterMentions
///      >>> from solders.pubkey import Pubkey
///      >>> from solders.commitment_config import CommitmentLevel
///      >>> from solders.transaction_status import TransactionDetails
///      >>> config = RpcBlockSubscribeConfig(transaction_details=TransactionDetails.Signatures)
///      >>> BlockSubscribe(RpcBlockSubscribeFilter.All, config).to_json()
///      '{"method":"blockSubscribe","jsonrpc":"2.0","id":0,"params":["all",{"encoding":null,"transactionDetails":"signatures","showRewards":null,"maxSupportedTransactionVersion":null}]}'
///      >>> BlockSubscribe(RpcBlockSubscribeFilterMentions(Pubkey.default()), config).to_json()
///      '{"method":"blockSubscribe","jsonrpc":"2.0","id":0,"params":[{"mentionsAccountOrProgram":"11111111111111111111111111111111"},{"encoding":null,"transactionDetails":"signatures","showRewards":null,"maxSupportedTransactionVersion":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlockSubscribe {
    #[serde(flatten)]
    base: RequestBase,
    params: BlockSubscribeParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl BlockSubscribe {
    #[new]
    fn new(
        filter_: RpcBlockSubscribeFilterWrapper,
        config: Option<RpcBlockSubscribeConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = BlockSubscribeParams(filter_, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Union[RpcBlockSubscribeFilter | RpcBlockSubscribeFilterMentions]: The filter being applied.
    #[getter]
    fn filter_(&self) -> RpcBlockSubscribeFilterWrapper {
        self.params.0.clone()
    }

    /// Optional[RpcBlockSubscribeConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcBlockSubscribeConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(BlockSubscribe);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct LogsSubscribeParams(
    #[serde_as(as = "FromInto<RpcTransactionLogsFilter>")] TransactionLogsFilterWrapper,
    #[serde(default)] Option<RpcTransactionLogsConfig>,
);

/// A ``logsSubscribe`` request.
///
/// Args:
///     filter_ (RpcTransactionLogsFilter | RpcTransactionLogsFilterMentions): Filter criteria for the logs to receive results by account type.
///     config (Optional[RpcTransactionLogsConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from solders.rpc.requests import LogsSubscribe
///      >>> from solders.rpc.config import RpcTransactionLogsConfig, RpcTransactionLogsFilter, RpcTransactionLogsFilterMentions
///      >>> from solders.pubkey import Pubkey
///      >>> from solders.commitment_config import CommitmentLevel
///      >>> config = RpcTransactionLogsConfig(commitment=CommitmentLevel.Confirmed)
///      >>> LogsSubscribe(RpcTransactionLogsFilter.All, config).to_json()
///      '{"method":"logsSubscribe","jsonrpc":"2.0","id":0,"params":["all",{"commitment":"confirmed"}]}'
///      >>> LogsSubscribe(RpcTransactionLogsFilterMentions(Pubkey.default()), config).to_json()
///      '{"method":"logsSubscribe","jsonrpc":"2.0","id":0,"params":[{"mentions":["11111111111111111111111111111111"]},{"commitment":"confirmed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LogsSubscribe {
    #[serde(flatten)]
    base: RequestBase,
    params: LogsSubscribeParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl LogsSubscribe {
    #[new]
    fn new(
        filter_: TransactionLogsFilterWrapper,
        config: Option<RpcTransactionLogsConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = LogsSubscribeParams(filter_, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Union[RpcTransactionLogsFilter | RpcTransactionLogsFilterMentions]: The filter being applied.
    #[getter]
    fn filter_(&self) -> TransactionLogsFilterWrapper {
        self.params.0.clone()
    }

    /// Optional[RpcTransactionLogsConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcTransactionLogsConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(LogsSubscribe);

/// A ``programSubscribe`` request.
///
/// Args:
///     program (Pubkey): The program that owns the accounts
///     config (Optional[RpcProgramAccountsConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import ProgramSubscribe
///     >>> from solders.rpc.config import RpcProgramAccountsConfig, RpcAccountInfoConfig
///     >>> from solders.rpc.filter import Memcmp
///     >>> from solders.pubkey import Pubkey
///     >>> acc_info_config = RpcAccountInfoConfig.default()
///     >>> filters = [10, Memcmp(offset=10, bytes_=b"123")]
///     >>> config = RpcProgramAccountsConfig(acc_info_config, filters)
///     >>> ProgramSubscribe(Pubkey.default(), config).to_json()
///     '{"method":"programSubscribe","jsonrpc":"2.0","id":0,"params":["11111111111111111111111111111111",{"filters":[{"dataSize":10},{"memcmp":{"offset":10,"bytes":[49,50,51],"encoding":null}}],"encoding":null,"dataSlice":null,"minContextSlot":null,"withContext":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgramSubscribe {
    #[serde(flatten)]
    base: RequestBase,
    params: GetProgramAccountsParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl ProgramSubscribe {
    #[new]
    fn new(program: Pubkey, config: Option<RpcProgramAccountsConfig>, id: Option<u64>) -> Self {
        let params = GetProgramAccountsParams(program, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Pubkey: The program that owns the accounts
    #[getter]
    pub fn program(&self) -> Pubkey {
        self.params.0
    }

    /// Optional[RpcProgramAccountsConfig]: Extra configuration.
    #[getter]
    pub fn config(&self) -> Option<RpcProgramAccountsConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(ProgramSubscribe);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SignatureSubscribeParams(
    #[serde_as(as = "DisplayFromStr")] Signature,
    #[serde(default)] Option<RpcSignatureSubscribeConfig>,
);

/// A ``signatureSubscribe`` request.
///
/// Args:
///     signature (Signature): The transaction to watch.
///     config (Optional[RpcSignatureSubscribeConfig]): Extra configuration.
///     id (Optional[int]): Request ID.
///
/// Example:
///      >>> from solders.rpc.requests import SignatureSubscribe
///      >>> from solders.rpc.config import RpcSignatureSubscribeConfig
///      >>> from solders.signature import Signature
///      >>> config = RpcSignatureSubscribeConfig(enable_received_notification=False)
///      >>> SignatureSubscribe(Signature.default(), config).to_json()
///      '{"method":"signatureSubscribe","jsonrpc":"2.0","id":0,"params":["1111111111111111111111111111111111111111111111111111111111111111",{"enableReceivedNotification":false}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignatureSubscribe {
    #[serde(flatten)]
    base: RequestBase,
    params: SignatureSubscribeParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl SignatureSubscribe {
    #[new]
    fn new(
        signature: Signature,
        config: Option<RpcSignatureSubscribeConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = SignatureSubscribeParams(signature, config);
        let base = RequestBase::new(id);
        Self { base, params }
    }

    /// Signature: The signature being watched
    #[getter]
    fn signature(&self) -> Signature {
        self.params.0
    }

    /// Optional[RpcSignatureSubscribeConfig]: Extra configuration.
    #[getter]
    fn config(&self) -> Option<RpcSignatureSubscribeConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(SignatureSubscribe);
zero_param_req_def!(SlotSubscribe);
zero_param_req_def!(SlotsUpdatesSubscribe);
zero_param_req_def!(RootSubscribe);
zero_param_req_def!(VoteSubscribe);

macro_rules ! pyunion {
    ($name:ident, $($variant:ident),+) => {
        #[derive(FromPyObject, Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "method", rename_all = "camelCase")]
        pub enum $name {
            $($variant($variant),)+
        }

        impl IntoPy<PyObject> for $name {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    $(Self::$variant(x) => x.into_py(py),)+
                }
            }
        }
    }
}

pyunion!(
    Body,
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockHeight,
    GetBlockProduction,
    GetBlockCommitment,
    GetBlocks,
    GetBlocksWithLimit,
    GetBlockTime,
    GetClusterNodes,
    GetEpochInfo,
    GetEpochSchedule,
    GetFeeForMessage,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
    GetHighestSnapshotSlot,
    GetIdentity,
    GetInflationGovernor,
    GetInflationRate,
    GetInflationReward,
    GetLargestAccounts,
    GetLatestBlockhash,
    GetLeaderSchedule,
    GetMaxRetransmitSlot,
    GetMaxShredInsertSlot,
    GetMinimumBalanceForRentExemption,
    GetMultipleAccounts,
    GetProgramAccounts,
    GetRecentPerformanceSamples,
    GetSignaturesForAddress,
    GetSignatureStatuses,
    GetSlot,
    GetSlotLeader,
    GetSlotLeaders,
    GetStakeActivation,
    GetSupply,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenLargestAccounts,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    IsBlockhashValid,
    MinimumLedgerSlot,
    RequestAirdrop,
    SendTransaction,
    ValidatorExit,
    AccountSubscribe,
    BlockSubscribe,
    LogsSubscribe,
    ProgramSubscribe,
    SignatureSubscribe,
    SlotSubscribe,
    SlotsUpdatesSubscribe,
    RootSubscribe,
    VoteSubscribe,
    AccountUnsubscribe,
    BlockUnsubscribe,
    LogsUnsubscribe,
    ProgramUnsubscribe,
    SignatureUnsubscribe,
    SimulateTransaction,
    SlotUnsubscribe,
    SlotsUpdatesUnsubscribe,
    RootUnsubscribe,
    VoteUnsubscribe
);

/// Serialize a list of request objects into a single batch request JSON.
///
/// Args:
///     reqs: A list of request objects.
///
/// Returns:
///     str: The batch JSON string.
///
/// Example:
///     >>> from solders.rpc.requests import batch_to_json, GetClusterNodes, GetEpochSchedule
///     >>> batch_to_json([GetClusterNodes(0), GetEpochSchedule(1)])
///     '[{"method":"getClusterNodes","jsonrpc":"2.0","id":0},{"method":"getEpochSchedule","jsonrpc":"2.0","id":1}]'
///
#[pyfunction]
pub fn batch_to_json(reqs: Vec<Body>) -> String {
    serde_json::to_string(&reqs).unwrap()
}

/// Deserialize a batch request JSON string into a list of request objects.
///
/// Args:
///     raw (str): The batch JSON string.
///
/// Returns:
///     A list of request objects.
///
/// Example:
///     >>> from solders.rpc.requests import batch_from_json
///     >>> raw = '[{"jsonrpc":"2.0","id":0,"method":"getClusterNodes"},{"jsonrpc":"2.0","id":1,"method":"getEpochSchedule"}]'
///     >>> batch_from_json(raw)
///     [GetClusterNodes {
///         base: RequestBase {
///             jsonrpc: TwoPointOh,
///             id: 0,
///         },
///     }, GetEpochSchedule {
///         base: RequestBase {
///             jsonrpc: TwoPointOh,
///             id: 1,
///         },
///     }]
///
#[pyfunction]
pub fn batch_from_json(raw: &str) -> PyResult<Vec<PyObject>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let deser: Vec<Body> = serde_json::from_str(raw).unwrap();
    Ok(deser.into_iter().map(|x| x.into_py(py)).collect())
}

pub fn create_requests_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let alias_members = PyTuple::new(
        py,
        vec![
            GetAccountInfo::type_object(py),
            GetBalance::type_object(py),
            GetBlock::type_object(py),
            GetBlockHeight::type_object(py),
            GetBlockProduction::type_object(py),
            GetBlockCommitment::type_object(py),
            GetBlocks::type_object(py),
            GetBlocksWithLimit::type_object(py),
            GetBlockTime::type_object(py),
            GetClusterNodes::type_object(py),
            GetEpochInfo::type_object(py),
            GetEpochSchedule::type_object(py),
            GetFeeForMessage::type_object(py),
            GetFirstAvailableBlock::type_object(py),
            GetGenesisHash::type_object(py),
            GetHealth::type_object(py),
            GetHighestSnapshotSlot::type_object(py),
            GetIdentity::type_object(py),
            GetInflationGovernor::type_object(py),
            GetInflationRate::type_object(py),
            GetInflationReward::type_object(py),
            GetLargestAccounts::type_object(py),
            GetLatestBlockhash::type_object(py),
            GetLeaderSchedule::type_object(py),
            GetMaxRetransmitSlot::type_object(py),
            GetMaxShredInsertSlot::type_object(py),
            GetMinimumBalanceForRentExemption::type_object(py),
            GetMultipleAccounts::type_object(py),
            GetProgramAccounts::type_object(py),
            GetRecentPerformanceSamples::type_object(py),
            GetSignaturesForAddress::type_object(py),
            GetSignatureStatuses::type_object(py),
            GetSlot::type_object(py),
            GetSlotLeader::type_object(py),
            GetSlotLeaders::type_object(py),
            GetStakeActivation::type_object(py),
            GetSupply::type_object(py),
            GetTokenAccountBalance::type_object(py),
            GetTokenAccountsByDelegate::type_object(py),
            GetTokenAccountsByOwner::type_object(py),
            GetTokenLargestAccounts::type_object(py),
            GetTokenSupply::type_object(py),
            GetTransaction::type_object(py),
            GetTransactionCount::type_object(py),
            GetVersion::type_object(py),
            GetVoteAccounts::type_object(py),
            IsBlockhashValid::type_object(py),
            MinimumLedgerSlot::type_object(py),
            RequestAirdrop::type_object(py),
            SendTransaction::type_object(py),
            ValidatorExit::type_object(py),
            AccountSubscribe::type_object(py),
            BlockSubscribe::type_object(py),
            LogsSubscribe::type_object(py),
            ProgramSubscribe::type_object(py),
            SignatureSubscribe::type_object(py),
            SlotSubscribe::type_object(py),
            SlotsUpdatesSubscribe::type_object(py),
            RootSubscribe::type_object(py),
            VoteSubscribe::type_object(py),
            AccountUnsubscribe::type_object(py),
            BlockUnsubscribe::type_object(py),
            LogsUnsubscribe::type_object(py),
            ProgramUnsubscribe::type_object(py),
            SignatureUnsubscribe::type_object(py),
            SimulateTransaction::type_object(py),
            SlotUnsubscribe::type_object(py),
            SlotsUpdatesUnsubscribe::type_object(py),
            RootUnsubscribe::type_object(py),
            VoteUnsubscribe::type_object(py),
        ],
    );
    let body_alias = union.get_item(alias_members)?;
    let requests_mod = PyModule::new(py, "requests")?;
    requests_mod.add_class::<GetAccountInfo>()?;
    requests_mod.add_class::<GetBalance>()?;
    requests_mod.add_class::<GetBlock>()?;
    requests_mod.add_class::<GetBlockHeight>()?;
    requests_mod.add_class::<GetBlockProduction>()?;
    requests_mod.add_class::<GetBlockCommitment>()?;
    requests_mod.add_class::<GetBlocks>()?;
    requests_mod.add_class::<GetBlocksWithLimit>()?;
    requests_mod.add_class::<GetBlockTime>()?;
    requests_mod.add_class::<GetClusterNodes>()?;
    requests_mod.add_class::<GetEpochInfo>()?;
    requests_mod.add_class::<GetEpochSchedule>()?;
    requests_mod.add_class::<GetFeeForMessage>()?;
    requests_mod.add_class::<GetFirstAvailableBlock>()?;
    requests_mod.add_class::<GetGenesisHash>()?;
    requests_mod.add_class::<GetHealth>()?;
    requests_mod.add_class::<GetHighestSnapshotSlot>()?;
    requests_mod.add_class::<GetIdentity>()?;
    requests_mod.add_class::<GetInflationGovernor>()?;
    requests_mod.add_class::<GetInflationRate>()?;
    requests_mod.add_class::<GetInflationReward>()?;
    requests_mod.add_class::<GetLargestAccounts>()?;
    requests_mod.add_class::<GetLatestBlockhash>()?;
    requests_mod.add_class::<GetLeaderSchedule>()?;
    requests_mod.add_class::<GetMaxRetransmitSlot>()?;
    requests_mod.add_class::<GetMaxShredInsertSlot>()?;
    requests_mod.add_class::<GetMinimumBalanceForRentExemption>()?;
    requests_mod.add_class::<GetMultipleAccounts>()?;
    requests_mod.add_class::<GetProgramAccounts>()?;
    requests_mod.add_class::<GetRecentPerformanceSamples>()?;
    requests_mod.add_class::<GetSignaturesForAddress>()?;
    requests_mod.add_class::<GetSignatureStatuses>()?;
    requests_mod.add_class::<GetSlot>()?;
    requests_mod.add_class::<GetSlotLeader>()?;
    requests_mod.add_class::<GetSlotLeaders>()?;
    requests_mod.add_class::<GetStakeActivation>()?;
    requests_mod.add_class::<GetSupply>()?;
    requests_mod.add_class::<GetTokenAccountBalance>()?;
    requests_mod.add_class::<GetTokenAccountsByDelegate>()?;
    requests_mod.add_class::<GetTokenAccountsByOwner>()?;
    requests_mod.add_class::<GetTokenLargestAccounts>()?;
    requests_mod.add_class::<GetTokenSupply>()?;
    requests_mod.add_class::<GetTransaction>()?;
    requests_mod.add_class::<GetTransactionCount>()?;
    requests_mod.add_class::<GetVersion>()?;
    requests_mod.add_class::<GetVoteAccounts>()?;
    requests_mod.add_class::<IsBlockhashValid>()?;
    requests_mod.add_class::<MinimumLedgerSlot>()?;
    requests_mod.add_class::<RequestAirdrop>()?;
    requests_mod.add_class::<SendTransaction>()?;
    requests_mod.add_class::<ValidatorExit>()?;
    requests_mod.add_class::<AccountSubscribe>()?;
    requests_mod.add_class::<BlockSubscribe>()?;
    requests_mod.add_class::<LogsSubscribe>()?;
    requests_mod.add_class::<ProgramSubscribe>()?;
    requests_mod.add_class::<SignatureSubscribe>()?;
    requests_mod.add_class::<SlotSubscribe>()?;
    requests_mod.add_class::<SlotsUpdatesSubscribe>()?;
    requests_mod.add_class::<RootSubscribe>()?;
    requests_mod.add_class::<VoteSubscribe>()?;
    requests_mod.add_class::<AccountUnsubscribe>()?;
    requests_mod.add_class::<BlockUnsubscribe>()?;
    requests_mod.add_class::<LogsUnsubscribe>()?;
    requests_mod.add_class::<ProgramUnsubscribe>()?;
    requests_mod.add_class::<SignatureUnsubscribe>()?;
    requests_mod.add_class::<SimulateTransaction>()?;
    requests_mod.add_class::<SlotUnsubscribe>()?;
    requests_mod.add_class::<SlotsUpdatesUnsubscribe>()?;
    requests_mod.add_class::<RootUnsubscribe>()?;
    requests_mod.add_class::<VoteUnsubscribe>()?;
    requests_mod.add("Body", body_alias)?;
    let funcs = [
        wrap_pyfunction!(batch_to_json, requests_mod)?,
        wrap_pyfunction!(batch_from_json, requests_mod)?,
    ];
    for func in funcs {
        requests_mod.add_function(func)?;
    }
    Ok(requests_mod)
}
