#![allow(deprecated)]
use crate::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode, CommonMethods, Message, Pubkey,
    PyBytesBincode, PyErrWrapper, PyFromBytesBincode, RichcmpEqualityOnly,
};
use pyo3::{create_exception, exceptions::PyException, prelude::*};
extern crate base64;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};
use solana_sdk::message::Message as MessageOriginal;
use solders_macros::{common_methods, richcmp_eq_only, rpc_id_getter};

use crate::Signature;

use super::config::{
    RpcAccountInfoConfig, RpcBlockConfig, RpcBlockProductionConfig, RpcContextConfig,
    RpcRequestAirdropConfig, RpcSignatureStatusConfig,
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
        impl CommonMethods<'_> for $name {}
        impl RichcmpEqualityOnly for $name {}
        pybytes_general_via_bincode!($name);
        py_from_bytes_general_via_bincode!($name);
    };
}

impl From<serde_json::Error> for PyErrWrapper {
    fn from(e: serde_json::Error) -> Self {
        Self(SerdeJSONError::new_err(e.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub enum RpcRequest {
    DeregisterNode,
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockCommitment,
    GetBlockHeight,
    GetBlockProduction,
    GetBlocks,
    GetBlocksWithLimit,
    GetBlockTime,
    GetClusterNodes,
    #[deprecated(since = "1.7.0", note = "Please use RpcRequest::GetBlock instead")]
    GetConfirmedBlock,
    #[deprecated(since = "1.7.0", note = "Please use RpcRequest::GetBlocks instead")]
    GetConfirmedBlocks,
    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetBlocksWithLimit instead"
    )]
    GetConfirmedBlocksWithLimit,
    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetSignaturesForAddress instead"
    )]
    GetConfirmedSignaturesForAddress2,
    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetTransaction instead"
    )]
    GetConfirmedTransaction,
    GetEpochInfo,
    GetEpochSchedule,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetFeeForMessage instead"
    )]
    GetFeeCalculatorForBlockhash,
    GetFeeForMessage,
    #[deprecated(
        since = "1.9.0",
        note = "Please do not use, will no longer be available in the future"
    )]
    GetFeeRateGovernor,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetFeeForMessage instead"
    )]
    GetFees,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
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
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetLatestBlockhash instead"
    )]
    GetRecentBlockhash,
    GetRecentPerformanceSamples,
    GetHighestSnapshotSlot,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetHighestSnapshotSlot instead"
    )]
    GetSnapshotSlot,
    GetSignaturesForAddress,
    GetSignatureStatuses,
    GetSlot,
    GetSlotLeader,
    GetSlotLeaders,
    GetStorageTurn,
    GetStorageTurnRate,
    GetSlotsPerSegment,
    GetStakeActivation,
    GetStoragePubkeysForSlot,
    GetSupply,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    IsBlockhashValid,
    MinimumLedgerSlot,
    RegisterNode,
    RequestAirdrop,
    SendTransaction,
    SimulateTransaction,
    SignVote,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct RequestBase {
    jsonrpc: String,
    id: u64,
    method: RpcRequest,
}

impl RequestBase {
    fn new(method: RpcRequest, id: Option<u64>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id: id.unwrap_or(0),
            method,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetAccountInfoParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcAccountInfoConfig>,
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
///     '{"jsonrpc":"2.0","id":0,"method":"getAccountInfo","params":["11111111111111111111111111111111",{"encoding":"base64","dataSlice":null,"minContextSlot":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let base = RequestBase::new(RpcRequest::GetAccountInfo, id);
        Self { base, params }
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    #[getter]
    pub fn config(&self) -> Option<RpcAccountInfoConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetAccountInfo);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalanceParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcContextConfig>,
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
///     '{"jsonrpc":"2.0","id":0,"method":"getBalance","params":["11111111111111111111111111111111",{"minContextSlot":1}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let base = RequestBase::new(RpcRequest::GetBalance, id);
        Self { base, params }
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetBalance);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockParams(
    u64,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcBlockConfig>,
);

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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlock","params":[123,{"encoding":"base58","transactionDetails":"none","rewards":null,"maxSupportedTransactionVersion":null}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let base = RequestBase::new(RpcRequest::GetBlock, id);
        Self { base, params }
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }

    #[getter]
    pub fn config(&self) -> Option<RpcBlockConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetBlock);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeightParams(
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcContextConfig>,
);

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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlockHeight","params":{"minContextSlot":123}}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeight {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlockHeightParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockHeight {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = GetBlockHeightParams(config);
        let base = RequestBase::new(RpcRequest::GetBlockHeight, id);
        Self { base, params }
    }

    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.0.clone()
    }
}

request_boilerplate!(GetBlockHeight);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockProductionParams(
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcBlockProductionConfig>,
);

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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlockProduction","params":{"identity":"11111111111111111111111111111111","range":{"firstSlot":10,"lastSlot":15}}}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockProduction {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlockProductionParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockProduction {
    #[new]
    fn new(config: Option<RpcBlockProductionConfig>, id: Option<u64>) -> Self {
        let params = GetBlockProductionParams(config);
        let base = RequestBase::new(RpcRequest::GetBlockProduction, id);
        Self { base, params }
    }

    #[getter]
    pub fn config(&self) -> Option<RpcBlockProductionConfig> {
        self.params.0.clone()
    }
}

request_boilerplate!(GetBlockProduction);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockCommitmentParams(u64);

/// A ``getBlockCommitment`` request.
///
/// Args:
///     slot (int): The slot to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockCommitment
///     >>> GetBlockCommitment(123).to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlockCommitment","params":123}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockCommitment {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlockCommitmentParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockCommitment {
    #[new]
    fn new(slot: u64, id: Option<u64>) -> Self {
        let params = GetBlockCommitmentParams(slot);
        let base = RequestBase::new(RpcRequest::GetBlockCommitment, id);
        Self { base, params }
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }
}

request_boilerplate!(GetBlockCommitment);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlocksParams(
    u64,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<CommitmentConfig>,
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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let params = GetBlocksParams(start, end, commitment.map(|c| c.into()));
        let base = RequestBase::new(RpcRequest::GetBlocks, id);
        Self { base, params }
    }

    #[getter]
    pub fn start(&self) -> u64 {
        self.params.0
    }

    #[getter]
    pub fn end(&self) -> Option<u64> {
        self.params.1
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.2.map(|c| c.into())
    }
}

request_boilerplate!(GetBlocks);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlocksWithLimitParams(
    u64,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<CommitmentConfig>,
);

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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlocksWithLimit {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlocksWithLimitParams,
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
        let params = GetBlocksWithLimitParams(start, limit, commitment.map(|c| c.into()));
        let base = RequestBase::new(RpcRequest::GetBlocks, id);
        Self { base, params }
    }

    #[getter]
    pub fn start(&self) -> u64 {
        self.params.0
    }

    #[getter]
    pub fn limit(&self) -> Option<u64> {
        self.params.1
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.2.map(|c| c.into())
    }
}

request_boilerplate!(GetBlocksWithLimit);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTimeParams(u64);

/// A ``getBlockTime`` request.
///
/// Args:
///     slot (int): The slot to query.
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetBlockTime
///     >>> GetBlockTime(123).to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTime {
    #[serde(flatten)]
    base: RequestBase,
    params: GetBlockTimeParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetBlockTime {
    #[new]
    fn new(slot: u64, id: Option<u64>) -> Self {
        let params = GetBlockTimeParams(slot);
        let base = RequestBase::new(RpcRequest::GetBlockTime, id);
        Self { base, params }
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.params.0
    }
}

request_boilerplate!(GetBlockTime);

/// A ``getClusterNodes`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetClusterNodes
///     >>> GetClusterNodes().to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetClusterNodes {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetClusterNodes {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetClusterNodes, id);
        Self { base }
    }
}

request_boilerplate!(GetClusterNodes);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetEpochInfoParams(
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcContextConfig>,
);

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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetEpochInfo {
    #[serde(flatten)]
    base: RequestBase,
    params: GetEpochInfoParams,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetEpochInfo {
    #[new]
    fn new(config: Option<RpcContextConfig>, id: Option<u64>) -> Self {
        let params = GetEpochInfoParams(config);
        let base = RequestBase::new(RpcRequest::GetEpochInfo, id);
        Self { base, params }
    }

    #[getter]
    pub fn config(&self) -> Option<RpcContextConfig> {
        self.params.0.clone()
    }
}

request_boilerplate!(GetEpochInfo);

/// A ``getEpochSchedule`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetEpochSchedule
///     >>> GetEpochSchedule(3).to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetEpochSchedule {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetEpochSchedule {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetEpochSchedule, id);
        Self { base }
    }
}

request_boilerplate!(GetEpochSchedule);

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
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetFeeForMessageParams(
    #[serde_as(as = "FromInto<MessageBase64>")] Message,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<CommitmentConfig>,
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
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let params = GetFeeForMessageParams(message, commitment.map(|c| c.into()));
        let base = RequestBase::new(RpcRequest::GetFeeForMessage, id);
        Self { base, params }
    }

    #[getter]
    pub fn message(&self) -> Message {
        self.params.0.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.params.1.map(|c| c.into())
    }
}

request_boilerplate!(GetFeeForMessage);

/// A ``getFirstAvailableBlock`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetFirstAvailableBlock
///     >>> GetFirstAvailableBlock(id=123).to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetFirstAvailableBlock {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetFirstAvailableBlock {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetFirstAvailableBlock, id);
        Self { base }
    }
}

request_boilerplate!(GetFirstAvailableBlock);

/// A ``getGenesisHash`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetGenesisHash
///     >>> GetGenesisHash().to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetGenesisHash {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetGenesisHash {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetGenesisHash, id);
        Self { base }
    }
}

request_boilerplate!(GetGenesisHash);

/// A ``getHealth`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetHealth
///     >>> GetHealth().to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetHealth {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetHealth {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetHealth, id);
        Self { base }
    }
}

request_boilerplate!(GetHealth);

/// A ``getHighestSnapshotSlot`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetHighestSnapshotSlot
///     >>> getHighestSnapshotSlot().to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetHighestSnapshotSlot {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetHighestSnapshotSlot {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetHighestSnapshotSlot, id);
        Self { base }
    }
}

request_boilerplate!(GetHighestSnapshotSlot);

/// A ``getIdentity`` request.
///
/// Args:
///     id (Optional[int]): Request ID.
///
/// Example:
///     >>> from solders.rpc.requests import GetIdentity
///     >>> GetIdentity().to_json()
///     '{"jsonrpc":"2.0","id":0,"method":"getBlocks","params":[123,5,{"commitment":"processed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetIdentity {
    #[serde(flatten)]
    base: RequestBase,
}

#[richcmp_eq_only]
#[common_methods]
#[rpc_id_getter]
#[pymethods]
impl GetIdentity {
    #[new]
    fn new(id: Option<u64>) -> Self {
        let base = RequestBase::new(RpcRequest::GetIdentity, id);
        Self { base }
    }
}

request_boilerplate!(GetIdentity);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetSignatureStatusesParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] Vec<Signature>,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcSignatureStatusConfig>,
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
///     '{"jsonrpc":"2.0","id":0,"method":"getSignatureStatuses","params":[["1111111111111111111111111111111111111111111111111111111111111111"],{"searchTransactionHistory":true}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let base = RequestBase::new(RpcRequest::GetSignatureStatuses, id);
        Self { base, params }
    }

    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        self.params.0.clone()
    }

    #[getter]
    pub fn config(&self) -> Option<RpcSignatureStatusConfig> {
        self.params.1.clone()
    }
}

request_boilerplate!(GetSignatureStatuses);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct RequestAirdropParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    u64,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcRequestAirdropConfig>,
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
///      '{"jsonrpc":"2.0","id":0,"method":"requestAirdrop","params":["11111111111111111111111111111111",1000,{"recentBlockhash":null,"commitment":"confirmed"}]}'
///
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        let base = RequestBase::new(RpcRequest::RequestAirdrop, id);
        Self { base, params }
    }

    #[getter]
    fn pubkey(&self) -> Pubkey {
        self.params.0
    }

    #[getter]
    fn lamports(&self) -> u64 {
        self.params.1
    }

    #[getter]
    fn config(&self) -> Option<RpcRequestAirdropConfig> {
        self.params.2.clone()
    }
}

request_boilerplate!(RequestAirdrop);

#[derive(FromPyObject, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    GetSignatureStatuses(GetSignatureStatuses),
    RequestAirdrop(RequestAirdrop),
}

impl IntoPy<PyObject> for Body {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Body::GetSignatureStatuses(x) => x.into_py(py),
            Body::RequestAirdrop(x) => x.into_py(py),
        }
    }
}

#[pyfunction]
pub fn batch_to_json(reqs: Vec<Body>) -> String {
    serde_json::to_string(&reqs).unwrap()
}

#[pyfunction]
pub fn batch_from_json(raw: &str) -> PyResult<Vec<PyObject>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let deser: Vec<Body> = serde_json::from_str(raw).unwrap();
    Ok(deser.into_iter().map(|x| x.into_py(py)).collect())
}

pub fn create_requests_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let requests_mod = PyModule::new(py, "requests")?;
    requests_mod.add_class::<GetAccountInfo>()?;
    requests_mod.add_class::<GetBalance>()?;
    requests_mod.add_class::<GetBlock>()?;
    requests_mod.add_class::<GetBlockHeight>()?;
    requests_mod.add_class::<GetBlockProduction>()?;
    requests_mod.add_class::<GetBlockCommitment>()?;
    requests_mod.add_class::<GetBlocks>()?;
    requests_mod.add_class::<GetBlocksWithLimit>()?;
    requests_mod.add_class::<GetSignatureStatuses>()?;
    requests_mod.add_class::<RequestAirdrop>()?;
    let funcs = [
        wrap_pyfunction!(batch_to_json, requests_mod)?,
        wrap_pyfunction!(batch_from_json, requests_mod)?,
    ];
    for func in funcs {
        requests_mod.add_function(func)?;
    }
    Ok(requests_mod)
}
