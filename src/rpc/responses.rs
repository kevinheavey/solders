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
    transaction_context::TransactionReturnData as TransactionReturnDataOriginal,
};
use solders_macros::{common_methods, common_methods_rpc_resp, richcmp_eq_only};

use crate::epoch_schedule::EpochSchedule;
use crate::transaction_status::{TransactionErrorType, TransactionReturnData};
use crate::{
    account::{Account, AccountJSON},
    pubkey::Pubkey,
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    signature::Signature,
    tmp_account_decoder::UiAccount,
    to_py_err,
    transaction_status::{EncodedTransactionWithStatusMeta, Rewards},
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly, SolderHash,
};
use solana_client::rpc_response::{
    RpcBlockProduction as RpcBlockProductionOriginal,
    RpcBlockProductionRange as RpcBlockProductionRangeOriginal,
    RpcContactInfo as RpcContactInfoOriginal, RpcTransactionReturnData,
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

// note: this also covers get_blocks_with_limit
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
    Ok(m)
}
