#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
use std::fmt::Display;
use std::{collections::HashMap, str::FromStr};

use derive_more::{From, Into};
use pyo3::{
    prelude::*,
    type_object::PyTypeObject,
    types::{PyBytes, PyTuple},
    PyClass,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};
use solana_sdk::{
    clock::{Epoch, Slot, UnixTimestamp},
    epoch_info::EpochInfo as EpochInfoOriginal,
    transaction::TransactionError as TransactionErrorOriginal,
    transaction_context::TransactionReturnData as TransactionReturnDataOriginal,
};
use solders_macros::{
    common_methods, common_methods_rpc_resp, enum_original_mapping, richcmp_eq_only,
};

use crate::account_decoder::UiTokenAmount;
use crate::epoch_schedule::EpochSchedule;
use crate::transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, TransactionConfirmationStatus, TransactionErrorType,
    TransactionReturnData, TransactionStatus,
};
use crate::{
    account::{Account, AccountJSON},
    pubkey::Pubkey,
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    signature::Signature,
    tmp_account_decoder::{UiAccount, UiTokenAmount as UiTokenAmountOriginal},
    tmp_transaction_status::{
        TransactionConfirmationStatus as TransactionConfirmationStatusOriginal,
        TransactionStatus as TransactionStatusOriginal,
    },
    to_py_err,
    transaction_status::{EncodedTransactionWithStatusMeta, Rewards},
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly, SolderHash,
};
use solana_client::rpc_response::{
    RpcAccountBalance as RpcAccountBalanceOriginal,
    RpcBlockProduction as RpcBlockProductionOriginal,
    RpcBlockProductionRange as RpcBlockProductionRangeOriginal,
    RpcContactInfo as RpcContactInfoOriginal, RpcInflationGovernor as RpcInflationGovernorOriginal,
    RpcInflationRate as RpcInflationRateOriginal, RpcInflationReward as RpcInflationRewardOriginal,
    RpcPerfSample as RpcPerfSampleOriginal, RpcSnapshotSlotInfo as RpcSnapshotSlotInfoOriginal,
    RpcStakeActivation as RpcStakeActivationOriginal, RpcSupply as RpcSupplyOriginal,
    RpcTransactionReturnData, StakeActivationState as StakeActivationStateOriginal,
};
use solana_rpc::rpc;

use super::errors::RpcCustomError;

// note: the `data` field of the error struct is always None

pub trait CommonMethodsRpcResp<'a>:
    std::fmt::Display
    + std::fmt::Debug
    + PyBytesBincode
    + PyFromBytesBincode<'a>
    + IntoPy<PyObject>
    + Clone
    + Serialize
    + Deserialize<'a>
    + PyClass
{
    fn pybytes<'b>(&self, py: Python<'b>) -> &'b PyBytes {
        self.pybytes_bincode(py)
    }

    fn pystr(&self) -> String {
        self.to_string()
    }
    fn pyrepr(&self) -> String {
        format!("{:#?}", self)
    }

    fn py_from_bytes(raw: &'a [u8]) -> PyResult<Self> {
        Self::py_from_bytes_bincode(raw)
    }

    fn pyreduce(&self) -> PyResult<(PyObject, PyObject)> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let cloned = self.clone();
        let constructor = cloned.into_py(py).getattr(py, "from_bytes")?;
        Ok((constructor, (self.pybytes(py).to_object(py),).to_object(py)))
    }

    fn py_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn py_from_json(raw: &'a str) -> PyResult<Resp<Self>> {
        serde_json::from_str(raw).map_err(to_py_err)
    }
}

macro_rules! resp_traits {
    ($name:ident) => {
        impl PyBytesBincode for $name {}
        impl PyFromBytesBincode<'_> for $name {}
        impl RichcmpEqualityOnly for $name {}
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        impl<'a> CommonMethodsRpcResp<'a> for $name {}
    };
}

macro_rules! response_data_boilerplate {
    ($name:ident) => {
        impl RichcmpEqualityOnly for $name {}
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        pybytes_general_via_bincode!($name);
        py_from_bytes_general_via_bincode!($name);
        impl<'a> CommonMethods<'a> for $name {}
    };
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcError {
    /// Code
    #[pyo3(get)]
    pub code: i64,
    /// Message
    #[pyo3(get)]
    pub message: String,
    /// Data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[pyo3(get)]
    pub data: Option<RpcCustomError>,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcError {
    #[new]
    pub fn new(code: i64, message: &str, data: Option<RpcCustomError>) -> Self {
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }
}

response_data_boilerplate!(RpcError);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcResponseContext {
    #[pyo3(get)]
    pub slot: Slot,
    #[pyo3(get)]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub api_version: Option<String>,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcResponseContext {
    #[new]
    pub fn new(slot: Slot, api_version: Option<String>) -> Self {
        Self { slot, api_version }
    }
}

response_data_boilerplate!(RpcResponseContext);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Resp<T: PyClass + IntoPy<PyObject>> {
    Result {
        #[serde(skip)]
        jsonrpc: crate::rpc::requests::V2,
        result: T,
        #[serde(skip)]
        id: u64,
    },
    Error {
        #[serde(skip)]
        jsonrpc: crate::rpc::requests::V2,
        error: RpcError,
        #[serde(skip)]
        id: u64,
    },
}

impl<T: PyClass + IntoPy<PyObject>> IntoPy<PyObject> for Resp<T> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Error { error: e, .. } => e.into_py(py),
            Self::Result { result: r, .. } => r.into_py(py),
        }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetAccountInfoResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    #[serde_as(as = "Option<FromInto<UiAccount>>")]
    value: Option<Account>,
}

resp_traits!(GetAccountInfoResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetAccountInfoResp {
    #[new]
    pub fn new(value: Option<Account>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetAccountInfoJsonParsedResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    #[serde_as(as = "Option<FromInto<UiAccount>>")]
    value: Option<AccountJSON>,
}

resp_traits!(GetAccountInfoJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetAccountInfoJsonParsedResp {
    #[new]
    pub fn new(value: Option<AccountJSON>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBalanceResp {
    #[pyo3(get)]
    pub context: RpcResponseContext,
    #[pyo3(get)]
    pub value: u64,
}

resp_traits!(GetBalanceResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBalanceResp {
    #[new]
    pub fn new(value: u64, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

// The one in solana_client isn't clonable
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlockCommitmentResp {
    #[pyo3(get)]
    pub commitment: Option<[u64; 32]>,
    #[pyo3(get)]
    pub total_stake: u64,
}

resp_traits!(GetBlockCommitmentResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlockCommitmentResp {
    #[new]
    pub fn new(commitment: Option<[u64; 32]>, total_stake: u64) -> Self {
        Self {
            commitment,
            total_stake,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlockHeightResp(u64);

resp_traits!(GetBlockHeightResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlockHeightResp {
    #[new]
    pub fn new(height: u64) -> Self {
        Self(height)
    }

    #[getter]
    pub fn height(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcBlockProductionRange(RpcBlockProductionRangeOriginal);

response_data_boilerplate!(RpcBlockProductionRange);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockProductionRange {
    #[new]
    pub fn new(first_slot: u64, last_slot: u64) -> Self {
        RpcBlockProductionRangeOriginal {
            first_slot,
            last_slot,
        }
        .into()
    }

    #[getter]
    pub fn first_slot(&self) -> u64 {
        self.0.first_slot
    }

    #[getter]
    pub fn last_slot(&self) -> u64 {
        self.0.last_slot
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcBlockProduction(RpcBlockProductionOriginal);

response_data_boilerplate!(RpcBlockProduction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockProduction {
    #[new]
    pub fn new(
        by_identity: HashMap<Pubkey, (usize, usize)>,
        range: RpcBlockProductionRange,
    ) -> Self {
        RpcBlockProductionOriginal {
            by_identity: by_identity
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            range: range.into(),
        }
        .into()
    }

    #[getter]
    pub fn by_identity(&self) -> HashMap<Pubkey, (usize, usize)> {
        self.0
            .by_identity
            .clone()
            .into_iter()
            .map(|(k, v)| (Pubkey::from_str(&k).unwrap(), v))
            .collect()
    }

    #[getter]
    pub fn range(&self) -> RpcBlockProductionRange {
        self.0.range.clone().into()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlockProductionResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: RpcBlockProduction,
}

resp_traits!(GetBlockProductionResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlockProductionResp {
    #[new]
    pub fn new(value: RpcBlockProduction, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlockResp {
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub previous_blockhash: SolderHash,
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub blockhash: SolderHash,
    #[pyo3(get)]
    pub parent_slot: Slot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[pyo3(get)]
    pub transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[pyo3(get)]
    pub signatures: Option<Vec<Signature>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[pyo3(get)]
    pub rewards: Option<Rewards>,
    #[pyo3(get)]
    pub block_time: Option<UnixTimestamp>,
    #[pyo3(get)]
    pub block_height: Option<u64>,
}

resp_traits!(GetBlockResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlockResp {
    #[new]
    pub fn new(
        previous_blockhash: SolderHash,
        blockhash: SolderHash,
        parent_slot: Slot,
        transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
        signatures: Option<Vec<Signature>>,
        rewards: Option<Rewards>,
        block_time: Option<UnixTimestamp>,
        block_height: Option<u64>,
    ) -> Self {
        Self {
            previous_blockhash,
            blockhash,
            parent_slot,
            transactions,
            signatures,
            rewards,
            block_time,
            block_height,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlocksResp(Vec<u64>);

resp_traits!(GetBlocksResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlocksResp {
    #[new]
    pub fn new(blocks: Vec<u64>) -> Self {
        Self(blocks)
    }

    #[getter]
    pub fn blocks(&self) -> Vec<u64> {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlocksWithLimitResp(Vec<u64>);

resp_traits!(GetBlocksWithLimitResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlocksWithLimitResp {
    #[new]
    pub fn new(blocks: Vec<u64>) -> Self {
        Self(blocks)
    }

    #[getter]
    pub fn blocks(&self) -> Vec<u64> {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetBlockTimeResp(Option<u64>);

resp_traits!(GetBlockTimeResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetBlockTimeResp {
    #[new]
    pub fn new(time: Option<u64>) -> Self {
        Self(time)
    }

    #[getter]
    pub fn time(&self) -> Option<u64> {
        self.0
    }
}

// the one in solana_client doesn't derive Eq or PartialEq
#[serde_as]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcContactInfo {
    #[pyo3(get)]
    #[serde_as(as = "DisplayFromStr")]
    pub pubkey: Pubkey,
    #[pyo3(get)]
    pub gossip: Option<String>,
    #[pyo3(get)]
    pub tpu: Option<String>,
    #[pyo3(get)]
    pub rpc: Option<String>,
    #[pyo3(get)]
    pub version: Option<String>,
    #[pyo3(get)]
    pub feature_set: Option<u32>,
    #[pyo3(get)]
    pub shred_version: Option<u16>,
}

response_data_boilerplate!(RpcContactInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcContactInfo {
    #[new]
    pub fn new(
        pubkey: Pubkey,
        gossip: Option<String>,
        tpu: Option<String>,
        rpc: Option<String>,
        version: Option<String>,
        feature_set: Option<u32>,
        shred_version: Option<u16>,
    ) -> Self {
        Self {
            pubkey,
            gossip,
            tpu,
            rpc,
            version,
            feature_set,
            shred_version,
        }
    }
}

impl From<RpcContactInfo> for RpcContactInfoOriginal {
    fn from(r: RpcContactInfo) -> Self {
        let RpcContactInfo {
            version,
            feature_set,
            shred_version,
            ..
        } = r;
        Self {
            pubkey: r.pubkey.to_string(),
            gossip: r.gossip.map(|x| x.parse().unwrap()),
            tpu: r.tpu.map(|x| x.parse().unwrap()),
            rpc: r.rpc.map(|x| x.parse().unwrap()),
            version,
            feature_set,
            shred_version,
        }
    }
}

impl From<RpcContactInfoOriginal> for RpcContactInfo {
    fn from(r: RpcContactInfoOriginal) -> Self {
        let RpcContactInfoOriginal {
            version,
            feature_set,
            shred_version,
            ..
        } = r;
        Self {
            pubkey: r.pubkey.parse().unwrap(),
            gossip: r.gossip.map(|x| x.to_string()),
            tpu: r.tpu.map(|x| x.to_string()),
            rpc: r.tpu.map(|x| x.to_string()),
            version,
            feature_set,
            shred_version,
        }
    }
}

// the one in solana_client doesn't derive Eq or PartialEq
#[serde_as]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct EpochInfo {
    #[pyo3(get)]
    pub epoch: Epoch,
    #[pyo3(get)]
    pub slot_index: u64,
    #[pyo3(get)]
    pub slots_in_epoch: u64,
    #[pyo3(get)]
    pub absolute_slot: Slot,
    #[pyo3(get)]
    pub block_height: u64,
    #[pyo3(get)]
    pub transaction_count: Option<u64>,
}
response_data_boilerplate!(EpochInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EpochInfo {
    #[new]
    pub fn new(
        epoch: Epoch,
        slot_index: u64,
        slots_in_epoch: u64,
        absolute_slot: Slot,
        block_height: u64,
        transaction_count: Option<u64>,
    ) -> Self {
        Self {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        }
    }
}

impl From<EpochInfo> for EpochInfoOriginal {
    fn from(e: EpochInfo) -> Self {
        let EpochInfo {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        } = e;
        Self {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        }
    }
}

impl From<EpochInfoOriginal> for EpochInfo {
    fn from(e: EpochInfoOriginal) -> Self {
        let EpochInfoOriginal {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        } = e;
        Self {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetClusterNodesResp(Vec<RpcContactInfo>);
resp_traits!(GetClusterNodesResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetClusterNodesResp {
    #[new]
    pub fn new(nodes: Vec<RpcContactInfo>) -> Self {
        Self(nodes)
    }

    #[getter]
    pub fn nodes(&self) -> Vec<RpcContactInfo> {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetEpochInfoResp(EpochInfo);
resp_traits!(GetEpochInfoResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetEpochInfoResp {
    #[new]
    pub fn new(info: EpochInfo) -> Self {
        Self(info)
    }

    #[getter]
    pub fn info(&self) -> EpochInfo {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetEpochScheduleResp(EpochSchedule);
resp_traits!(GetEpochScheduleResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetEpochScheduleResp {
    #[new]
    pub fn new(schedule: EpochSchedule) -> Self {
        Self(schedule)
    }

    #[getter]
    pub fn schedule(&self) -> EpochSchedule {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetFeeForMessageResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Option<u64>,
}

resp_traits!(GetFeeForMessageResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetFeeForMessageResp {
    #[new]
    pub fn new(value: Option<u64>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetFirstAvailableBlockResp(u64);

resp_traits!(GetFirstAvailableBlockResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetFirstAvailableBlockResp {
    #[new]
    pub fn new(slot: u64) -> Self {
        Self(slot)
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.0
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetGenesisHashResp(#[serde_as(as = "DisplayFromStr")] SolderHash);

resp_traits!(GetGenesisHashResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetGenesisHashResp {
    #[new]
    pub fn new(value: SolderHash) -> Self {
        Self(value)
    }

    #[getter]
    pub fn value(&self) -> SolderHash {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetHealthResp(String);

resp_traits!(GetHealthResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetHealthResp {
    #[new]
    pub fn new(health: String) -> Self {
        Self(health)
    }

    #[getter]
    pub fn health(&self) -> String {
        self.0.clone()
    }
}

impl From<TransactionReturnData> for RpcTransactionReturnData {
    fn from(t: TransactionReturnData) -> Self {
        TransactionReturnDataOriginal::from(t).into()
    }
}

impl From<RpcTransactionReturnData> for TransactionReturnData {
    fn from(r: RpcTransactionReturnData) -> Self {
        Self::new(
            r.program_id.parse().unwrap(),
            base64::decode(r.data.0).unwrap(),
        )
    }
}

// the one in solana_client doesn't derive Eq
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSimulateTransactionResult {
    #[pyo3(get)]
    pub err: Option<TransactionErrorType>,
    #[pyo3(get)]
    pub logs: Option<Vec<String>>,
    #[serde_as(as = "Option<Vec<Option<FromInto<UiAccount>>>>")]
    #[pyo3(get)]
    pub accounts: Option<Vec<Option<Account>>>,
    #[pyo3(get)]
    pub units_consumed: Option<u64>,
    #[serde_as(as = "Option<FromInto<RpcTransactionReturnData>>")]
    #[pyo3(get)]
    pub return_data: Option<TransactionReturnData>,
}

response_data_boilerplate!(RpcSimulateTransactionResult);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionResult {
    #[new]
    pub fn new(
        err: Option<TransactionErrorType>,
        logs: Option<Vec<String>>,
        accounts: Option<Vec<Option<Account>>>,
        units_consumed: Option<u64>,
        return_data: Option<TransactionReturnData>,
    ) -> Self {
        Self {
            err,
            logs,
            accounts,
            units_consumed,
            return_data,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSnapshotSlotInfo(RpcSnapshotSlotInfoOriginal);

response_data_boilerplate!(RpcSnapshotSlotInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSnapshotSlotInfo {
    #[new]
    pub fn new(full: Slot, incremental: Option<Slot>) -> Self {
        RpcSnapshotSlotInfoOriginal { full, incremental }.into()
    }

    #[getter]
    pub fn full(&self) -> Slot {
        self.0.full
    }

    #[getter]
    pub fn incremental(&self) -> Option<Slot> {
        self.0.incremental
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetHighestSnapshotSlotResp(RpcSnapshotSlotInfo);
resp_traits!(GetHighestSnapshotSlotResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetHighestSnapshotSlotResp {
    #[new]
    pub fn new(info: RpcSnapshotSlotInfo) -> Self {
        Self(info)
    }

    #[getter]
    pub fn info(&self) -> RpcSnapshotSlotInfo {
        self.0.clone()
    }
}

// the one in solana_client doesn't derive Eq
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcIdentity {
    /// The current node identity pubkey
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub identity: Pubkey,
}

response_data_boilerplate!(RpcIdentity);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcIdentity {
    #[new]
    pub fn new(identity: Pubkey) -> Self {
        RpcIdentity { identity }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetIdentityResp(RpcIdentity);
resp_traits!(GetIdentityResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetIdentityResp {
    #[new]
    pub fn new(value: RpcIdentity) -> Self {
        Self(value)
    }

    #[getter]
    pub fn value(&self) -> RpcIdentity {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcInflationGovernor(RpcInflationGovernorOriginal);

response_data_boilerplate!(RpcInflationGovernor);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcInflationGovernor {
    #[new]
    pub fn new(
        initial: f64,
        terminal: f64,
        taper: f64,
        foundation: f64,
        foundation_term: f64,
    ) -> Self {
        RpcInflationGovernorOriginal {
            initial,
            terminal,
            taper,
            foundation,
            foundation_term,
        }
        .into()
    }

    #[getter]
    pub fn initial(&self) -> f64 {
        self.0.initial
    }
    #[getter]
    pub fn terminal(&self) -> f64 {
        self.0.terminal
    }
    #[getter]
    pub fn taper(&self) -> f64 {
        self.0.taper
    }
    #[getter]
    pub fn foundation(&self) -> f64 {
        self.0.foundation
    }
    #[getter]
    pub fn foundation_term(&self) -> f64 {
        self.0.foundation_term
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetInflationGovernorResp(RpcInflationGovernor);
resp_traits!(GetInflationGovernorResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetInflationGovernorResp {
    #[new]
    pub fn new(governor: RpcInflationGovernor) -> Self {
        Self(governor)
    }

    #[getter]
    pub fn governor(&self) -> RpcInflationGovernor {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcInflationRate(RpcInflationRateOriginal);

response_data_boilerplate!(RpcInflationRate);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcInflationRate {
    #[new]
    pub fn new(total: f64, validator: f64, foundation: f64, epoch: Epoch) -> Self {
        RpcInflationRateOriginal {
            total,
            validator,
            foundation,
            epoch,
        }
        .into()
    }

    #[getter]
    pub fn total(&self) -> f64 {
        self.0.total
    }
    #[getter]
    pub fn validator(&self) -> f64 {
        self.0.validator
    }
    #[getter]
    pub fn foundation(&self) -> f64 {
        self.0.foundation
    }
    #[getter]
    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetInflationRateResp(RpcInflationRate);
resp_traits!(GetInflationRateResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetInflationRateResp {
    #[new]
    pub fn new(rate: RpcInflationRate) -> Self {
        Self(rate)
    }

    #[getter]
    pub fn rate(&self) -> RpcInflationRate {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcInflationReward(RpcInflationRewardOriginal);

response_data_boilerplate!(RpcInflationReward);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcInflationReward {
    #[new]
    pub fn new(
        epoch: Epoch,
        effective_slot: Slot,
        amount: u64,
        post_balance: u64,
        commission: Option<u8>,
    ) -> Self {
        RpcInflationRewardOriginal {
            epoch,
            effective_slot,
            amount,
            post_balance,
            commission,
        }
        .into()
    }
    #[getter]
    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }
    #[getter]
    pub fn effective_slot(&self) -> Slot {
        self.0.effective_slot
    }
    #[getter]
    pub fn amount(&self) -> u64 {
        self.0.amount
    }
    #[getter]
    pub fn post_balance(&self) -> u64 {
        self.0.post_balance
    }
    #[getter]
    pub fn commission(&self) -> Option<u8> {
        self.0.commission
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetInflationRewardResp(Vec<Option<RpcInflationReward>>);
resp_traits!(GetInflationRewardResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetInflationRewardResp {
    #[new]
    pub fn new(rewards: Vec<Option<RpcInflationReward>>) -> Self {
        Self(rewards)
    }

    #[getter]
    pub fn rewards(&self) -> Vec<Option<RpcInflationReward>> {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcAccountBalance(RpcAccountBalanceOriginal);

response_data_boilerplate!(RpcAccountBalance);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcAccountBalance {
    #[new]
    pub fn new(address: Pubkey, lamports: u64) -> Self {
        RpcAccountBalanceOriginal {
            address: address.to_string(),
            lamports,
        }
        .into()
    }
    #[getter]
    pub fn address(&self) -> Pubkey {
        Pubkey::from_str(&self.0.address).unwrap()
    }

    #[getter]
    pub fn lamports(&self) -> u64 {
        self.0.lamports
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetLargestAccountsResp {
    #[pyo3(get)]
    pub context: RpcResponseContext,
    #[pyo3(get)]
    pub value: Vec<RpcAccountBalance>,
}

resp_traits!(GetLargestAccountsResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetLargestAccountsResp {
    #[new]
    pub fn new(value: Vec<RpcAccountBalance>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

// the one in solana_client doesn't derive Eq
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcBlockhash {
    #[serde_as(as = "DisplayFromStr")]
    pub blockhash: SolderHash,
    pub last_valid_block_height: u64,
}

response_data_boilerplate!(RpcBlockhash);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockhash {
    #[new]
    pub fn new(blockhash: SolderHash, last_valid_block_height: u64) -> Self {
        RpcBlockhash {
            blockhash,
            last_valid_block_height,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetLatestBlockhashResp {
    #[pyo3(get)]
    pub context: RpcResponseContext,
    #[pyo3(get)]
    pub value: RpcBlockhash,
}

resp_traits!(GetLatestBlockhashResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetLatestBlockhashResp {
    #[new]
    pub fn new(value: RpcBlockhash, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

type RpcLeaderSchedule = Option<HashMap<Pubkey, Vec<usize>>>;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetLeaderScheduleResp(
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")] RpcLeaderSchedule,
);
resp_traits!(GetLeaderScheduleResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetLeaderScheduleResp {
    #[new]
    pub fn new(schedule: RpcLeaderSchedule) -> Self {
        Self(schedule)
    }

    #[getter]
    pub fn schedule(&self) -> RpcLeaderSchedule {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetMaxRetransmitSlotResp(u64);

resp_traits!(GetMaxRetransmitSlotResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetMaxRetransmitSlotResp {
    #[new]
    pub fn new(slot: u64) -> Self {
        Self(slot)
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetMaxShredInsertSlotResp(u64);

resp_traits!(GetMaxShredInsertSlotResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetMaxShredInsertSlotResp {
    #[new]
    pub fn new(slot: u64) -> Self {
        Self(slot)
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetMinimumBalanceForRentExemption(u64);

resp_traits!(GetMinimumBalanceForRentExemption);

#[common_methods_rpc_resp]
#[pymethods]
impl GetMinimumBalanceForRentExemption {
    #[new]
    pub fn new(slot: u64) -> Self {
        Self(slot)
    }

    #[getter]
    pub fn slot(&self) -> u64 {
        self.0
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetMultipleAccountsResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    #[serde_as(as = "Vec<Option<FromInto<UiAccount>>>")]
    value: Vec<Option<Account>>,
}

resp_traits!(GetMultipleAccountsResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetMultipleAccountsResp {
    #[new]
    pub fn new(value: Vec<Option<Account>>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetMultipleAccountsJsonParsedResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    #[serde_as(as = "Vec<Option<FromInto<UiAccount>>>")]
    value: Vec<Option<AccountJSON>>,
}

resp_traits!(GetMultipleAccountsJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetMultipleAccountsJsonParsedResp {
    #[new]
    pub fn new(value: Vec<Option<AccountJSON>>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

// the one in solana_client uses UiAccount from account_decoder which currently isn't portable
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccount {
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub pubkey: Pubkey,
    #[serde_as(as = "FromInto<UiAccount>")]
    #[pyo3(get)]
    pub account: Account,
}

response_data_boilerplate!(RpcKeyedAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcKeyedAccount {
    #[new]
    pub fn new(pubkey: Pubkey, account: Account) -> Self {
        Self { pubkey, account }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccountJsonParsed {
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub pubkey: Pubkey,
    #[serde_as(as = "FromInto<UiAccount>")]
    #[pyo3(get)]
    pub account: AccountJSON,
}

response_data_boilerplate!(RpcKeyedAccountJsonParsed);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcKeyedAccountJsonParsed {
    #[new]
    pub fn new(pubkey: Pubkey, account: AccountJSON) -> Self {
        Self { pubkey, account }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetProgramAccountsWithContextResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccount>,
}

resp_traits!(GetProgramAccountsWithContextResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetProgramAccountsWithContextResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccount>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetProgramAccountsWithContextJsonParsedResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccountJsonParsed>,
}

resp_traits!(GetProgramAccountsWithContextJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetProgramAccountsWithContextJsonParsedResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccountJsonParsed>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetProgramAccountsWithoutContextResp(Vec<RpcKeyedAccount>);

resp_traits!(GetProgramAccountsWithoutContextResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetProgramAccountsWithoutContextResp {
    #[new]
    pub fn new(accounts: Vec<RpcKeyedAccount>) -> Self {
        Self(accounts)
    }

    #[getter]
    pub fn accounts(&self) -> Vec<RpcKeyedAccount> {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetProgramAccountsWithoutContextJsonParsedResp(Vec<RpcKeyedAccountJsonParsed>);

resp_traits!(GetProgramAccountsWithoutContextJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetProgramAccountsWithoutContextJsonParsedResp {
    #[new]
    pub fn new(accounts: Vec<RpcKeyedAccountJsonParsed>) -> Self {
        Self(accounts)
    }

    #[getter]
    pub fn accounts(&self) -> Vec<RpcKeyedAccountJsonParsed> {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcPerfSample(RpcPerfSampleOriginal);

response_data_boilerplate!(RpcPerfSample);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcPerfSample {
    #[new]
    pub fn new(slot: Slot, num_transactions: u64, num_slots: u64, sample_period_secs: u16) -> Self {
        RpcPerfSampleOriginal {
            slot,
            num_transactions,
            num_slots,
            sample_period_secs,
        }
        .into()
    }

    #[getter]
    pub fn slot(&self) -> Slot {
        self.0.slot
    }
    #[getter]
    pub fn num_transactions(&self) -> u64 {
        self.0.num_transactions
    }
    #[getter]
    pub fn num_slots(&self) -> u64 {
        self.0.num_slots
    }
    #[getter]
    pub fn sample_period_secs(&self) -> u16 {
        self.0.sample_period_secs
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetRecentPerformanceSamplesResp(Vec<RpcPerfSample>);

resp_traits!(GetRecentPerformanceSamplesResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetRecentPerformanceSamplesResp {
    #[new]
    pub fn new(samples: Vec<RpcPerfSample>) -> Self {
        Self(samples)
    }

    #[getter]
    pub fn samples(&self) -> Vec<RpcPerfSample> {
        self.0.clone()
    }
}

// the one in solana_client uses transaction_status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConfirmedTransactionStatusWithSignatureOriginal {
    pub signature: String,
    pub slot: Slot,
    pub err: Option<TransactionErrorOriginal>,
    pub memo: Option<String>,
    pub block_time: Option<UnixTimestamp>,
    pub confirmation_status: Option<TransactionConfirmationStatusOriginal>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcConfirmedTransactionStatusWithSignature(
    RpcConfirmedTransactionStatusWithSignatureOriginal,
);

response_data_boilerplate!(RpcConfirmedTransactionStatusWithSignature);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcConfirmedTransactionStatusWithSignature {
    #[new]
    pub fn new(
        signature: Signature,
        slot: Slot,
        err: Option<TransactionErrorType>,
        memo: Option<String>,
        block_time: Option<UnixTimestamp>,
        confirmation_status: Option<TransactionConfirmationStatus>,
    ) -> Self {
        RpcConfirmedTransactionStatusWithSignatureOriginal {
            signature: signature.to_string(),
            slot,
            err: err.map(|e| e.into()),
            memo,
            block_time,
            confirmation_status: confirmation_status.map(|c| c.into()),
        }
        .into()
    }

    #[getter]
    pub fn signature(&self) -> Signature {
        Signature::from_str(&self.0.signature).unwrap()
    }
    #[getter]
    pub fn slot(&self) -> Slot {
        self.0.slot
    }
    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.0.err.clone().map(|e| e.into())
    }
    #[getter]
    pub fn memo(&self) -> Option<String> {
        self.0.memo.clone()
    }
    #[getter]
    pub fn block_time(&self) -> Option<UnixTimestamp> {
        self.0.block_time
    }
    #[getter]
    pub fn confirmation_status(&self) -> Option<TransactionConfirmationStatus> {
        self.0.confirmation_status.clone().map(|s| s.into())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSignaturesForAddressResp(Vec<RpcConfirmedTransactionStatusWithSignature>);

resp_traits!(GetSignaturesForAddressResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSignaturesForAddressResp {
    #[new]
    pub fn new(signatures: Vec<RpcConfirmedTransactionStatusWithSignature>) -> Self {
        Self(signatures)
    }

    #[getter]
    pub fn signatures(&self) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
        self.0.clone()
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSignatureStatusesResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    #[serde_as(as = "Vec<Option<FromInto<TransactionStatusOriginal>>>")]
    value: Vec<Option<TransactionStatus>>,
}

resp_traits!(GetSignatureStatusesResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSignatureStatusesResp {
    #[new]
    pub fn new(value: Vec<Option<TransactionStatus>>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSlotResp(Slot);

resp_traits!(GetSlotResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSlotResp {
    #[new]
    pub fn new(slot: Slot) -> Self {
        Self(slot)
    }

    #[getter]
    pub fn slot(&self) -> Slot {
        self.0
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSlotLeaderResp(#[serde_as(as = "DisplayFromStr")] Pubkey);

resp_traits!(GetSlotLeaderResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSlotLeaderResp {
    #[new]
    pub fn new(leader: Pubkey) -> Self {
        Self(leader)
    }

    #[getter]
    pub fn leader(&self) -> Pubkey {
        self.0
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSlotLeadersResp(#[serde_as(as = "Vec<DisplayFromStr>")] Vec<Pubkey>);

resp_traits!(GetSlotLeadersResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSlotLeadersResp {
    #[new]
    pub fn new(leaders: Vec<Pubkey>) -> Self {
        Self(leaders)
    }

    #[getter]
    pub fn leaders(&self) -> Vec<Pubkey> {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses")]
#[enum_original_mapping(StakeActivationStateOriginal)]
pub enum StakeActivationState {
    Activating,
    Active,
    Deactivating,
    Inactive,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcStakeActivation(RpcStakeActivationOriginal);

response_data_boilerplate!(RpcStakeActivation);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcStakeActivation {
    #[new]
    pub fn new(state: StakeActivationState, active: u64, inactive: u64) -> Self {
        RpcStakeActivationOriginal {
            state: state.into(),
            active,
            inactive,
        }
        .into()
    }

    #[getter]
    pub fn state(&self) -> StakeActivationState {
        self.0.state.clone().into()
    }
    #[getter]
    pub fn active(&self) -> u64 {
        self.0.active
    }
    #[getter]
    pub fn inactive(&self) -> u64 {
        self.0.inactive
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetStakeActivationResp(RpcStakeActivation);

resp_traits!(GetStakeActivationResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetStakeActivationResp {
    #[new]
    pub fn new(activation: RpcStakeActivation) -> Self {
        Self(activation)
    }

    #[getter]
    pub fn activation(&self) -> RpcStakeActivation {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSupply(RpcSupplyOriginal);

response_data_boilerplate!(RpcSupply);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSupply {
    #[new]
    pub fn new(
        total: u64,
        circulating: u64,
        non_circulating: u64,
        non_circulating_accounts: Vec<Pubkey>,
    ) -> Self {
        RpcSupplyOriginal {
            total,
            circulating,
            non_circulating,
            non_circulating_accounts: non_circulating_accounts
                .into_iter()
                .map(|p| p.to_string())
                .collect(),
        }
        .into()
    }

    #[getter]
    pub fn total(&self) -> u64 {
        self.0.total
    }
    #[getter]
    pub fn circulating(&self) -> u64 {
        self.0.circulating
    }
    #[getter]
    pub fn non_circulating(&self) -> u64 {
        self.0.non_circulating
    }
    #[getter]
    pub fn non_circulating_accounts(&self) -> Vec<Pubkey> {
        self.0
            .non_circulating_accounts
            .iter()
            .map(|s| Pubkey::from_str(s).unwrap())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetSupplyResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: RpcSupply,
}

resp_traits!(GetSupplyResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetSupplyResp {
    #[new]
    pub fn new(value: RpcSupply, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenAccountBalanceResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: UiTokenAmount,
}

resp_traits!(GetTokenAccountBalanceResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenAccountBalanceResp {
    #[new]
    pub fn new(value: UiTokenAmount, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenAccountsByDelegateResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccount>,
}

resp_traits!(GetTokenAccountsByDelegateResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenAccountsByDelegateResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccount>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenAccountsByDelegateJsonParsedResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccountJsonParsed>,
}

resp_traits!(GetTokenAccountsByDelegateJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenAccountsByDelegateJsonParsedResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccountJsonParsed>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenAccountsByOwnerResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccount>,
}

resp_traits!(GetTokenAccountsByOwnerResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenAccountsByOwnerResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccount>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenAccountsByOwnerJsonParsedResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcKeyedAccountJsonParsed>,
}

resp_traits!(GetTokenAccountsByOwnerJsonParsedResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenAccountsByOwnerJsonParsedResp {
    #[new]
    pub fn new(value: Vec<RpcKeyedAccountJsonParsed>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

// the one in solana_client uses account_decoder
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RpcTokenAccountBalanceOriginal {
    pub address: String,
    #[serde(flatten)]
    pub amount: UiTokenAmountOriginal,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcTokenAccountBalance(RpcTokenAccountBalanceOriginal);

response_data_boilerplate!(RpcTokenAccountBalance);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTokenAccountBalance {
    #[new]
    pub fn new(address: Pubkey, amount: UiTokenAmount) -> Self {
        RpcTokenAccountBalanceOriginal {
            address: address.to_string(),
            amount: amount.into(),
        }
        .into()
    }

    #[getter]
    pub fn address(&self) -> Pubkey {
        Pubkey::from_str(&self.0.address).unwrap()
    }

    #[getter]
    pub fn amount(&self) -> UiTokenAmount {
        self.0.amount.clone().into()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenLargestAccountsResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: Vec<RpcTokenAccountBalance>,
}

resp_traits!(GetTokenLargestAccountsResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenLargestAccountsResp {
    #[new]
    pub fn new(value: Vec<RpcTokenAccountBalance>, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTokenSupplyResp {
    #[pyo3(get)]
    context: RpcResponseContext,
    #[pyo3(get)]
    value: UiTokenAmount,
}

resp_traits!(GetTokenSupplyResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTokenSupplyResp {
    #[new]
    pub fn new(value: UiTokenAmount, context: RpcResponseContext) -> Self {
        Self { value, context }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct GetTransactionResp(Option<EncodedConfirmedTransactionWithStatusMeta>);

resp_traits!(GetTransactionResp);

#[common_methods_rpc_resp]
#[pymethods]
impl GetTransactionResp {
    #[new]
    pub fn new(transaction: Option<EncodedConfirmedTransactionWithStatusMeta>) -> Self {
        Self(transaction)
    }

    #[getter]
    pub fn transaction(&self) -> Option<EncodedConfirmedTransactionWithStatusMeta> {
        self.0.clone()
    }
}

pub(crate) fn create_responses_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "responses")?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let typevar = typing.getattr("TypeVar")?;
    let t = typevar.call1(("T",))?;
    m.add("T", t)?;
    m.add(
        "Resp",
        union.get_item(PyTuple::new(
            py,
            vec![RpcError::type_object(py).as_ref(), t],
        ))?,
    )?;
    m.add_class::<RpcResponseContext>()?;
    m.add_class::<RpcError>()?;
    m.add_class::<GetAccountInfoResp>()?;
    m.add_class::<GetAccountInfoJsonParsedResp>()?;
    m.add_class::<GetBalanceResp>()?;
    m.add_class::<RpcBlockProduction>()?;
    m.add_class::<RpcBlockProductionRange>()?;
    m.add_class::<GetBlockProductionResp>()?;
    m.add_class::<GetBlockResp>()?;
    m.add_class::<GetBlockCommitmentResp>()?;
    m.add_class::<GetBlockHeightResp>()?;
    m.add_class::<GetBlocksResp>()?;
    m.add_class::<GetBlocksWithLimitResp>()?;
    m.add_class::<GetBlockTimeResp>()?;
    m.add_class::<RpcContactInfo>()?;
    m.add_class::<GetClusterNodesResp>()?;
    m.add_class::<EpochInfo>()?;
    m.add_class::<GetEpochInfoResp>()?;
    m.add_class::<GetEpochScheduleResp>()?;
    m.add_class::<GetFeeForMessageResp>()?;
    m.add_class::<GetFirstAvailableBlockResp>()?;
    m.add_class::<GetGenesisHashResp>()?;
    m.add_class::<GetHealthResp>()?;
    m.add_class::<RpcSimulateTransactionResult>()?;
    m.add_class::<RpcSnapshotSlotInfo>()?;
    m.add_class::<GetHighestSnapshotSlotResp>()?;
    m.add_class::<RpcIdentity>()?;
    m.add_class::<GetIdentityResp>()?;
    m.add_class::<RpcInflationGovernor>()?;
    m.add_class::<GetInflationGovernorResp>()?;
    m.add_class::<RpcInflationRate>()?;
    m.add_class::<GetInflationRateResp>()?;
    m.add_class::<RpcInflationReward>()?;
    m.add_class::<GetInflationRewardResp>()?;
    m.add_class::<RpcAccountBalance>()?;
    m.add_class::<GetLargestAccountsResp>()?;
    m.add_class::<RpcBlockhash>()?;
    m.add_class::<GetLatestBlockhashResp>()?;
    m.add_class::<GetLeaderScheduleResp>()?;
    m.add_class::<GetMaxRetransmitSlotResp>()?;
    m.add_class::<GetMaxShredInsertSlotResp>()?;
    m.add_class::<GetMinimumBalanceForRentExemption>()?;
    m.add_class::<GetMultipleAccountsResp>()?;
    m.add_class::<GetMultipleAccountsJsonParsedResp>()?;
    m.add_class::<RpcKeyedAccount>()?;
    m.add_class::<RpcKeyedAccountJsonParsed>()?;
    m.add_class::<GetProgramAccountsWithContextResp>()?;
    m.add_class::<GetProgramAccountsWithoutContextResp>()?;
    m.add_class::<GetProgramAccountsWithContextJsonParsedResp>()?;
    m.add_class::<GetProgramAccountsWithoutContextJsonParsedResp>()?;
    m.add_class::<RpcPerfSample>()?;
    m.add_class::<GetRecentPerformanceSamplesResp>()?;
    m.add_class::<RpcConfirmedTransactionStatusWithSignature>()?;
    m.add_class::<GetSignaturesForAddressResp>()?;
    m.add_class::<GetSignatureStatusesResp>()?;
    m.add_class::<GetSlotResp>()?;
    m.add_class::<GetSlotLeaderResp>()?;
    m.add_class::<GetSlotLeadersResp>()?;
    m.add_class::<StakeActivationState>()?;
    m.add_class::<RpcStakeActivation>()?;
    m.add_class::<GetStakeActivationResp>()?;
    m.add_class::<RpcSupply>()?;
    m.add_class::<GetSupplyResp>()?;
    m.add_class::<GetTokenAccountBalanceResp>()?;
    m.add_class::<GetTokenAccountsByDelegateResp>()?;
    m.add_class::<GetTokenAccountsByDelegateJsonParsedResp>()?;
    m.add_class::<GetTokenAccountsByOwnerResp>()?;
    m.add_class::<GetTokenAccountsByOwnerJsonParsedResp>()?;
    m.add_class::<RpcTokenAccountBalance>()?;
    m.add_class::<GetTokenLargestAccountsResp>()?;
    m.add_class::<GetTokenSupplyResp>()?;
    m.add_class::<GetTransactionResp>()?;
    Ok(m)
}
