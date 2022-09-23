#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
use std::fmt::Display;
use std::{collections::HashMap, str::FromStr};

use derive_more::{From, Into};
use pyo3::exceptions::PyValueError;
use pyo3::types::PyType;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyTuple},
    PyClass, PyTypeInfo,
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
use crate::rpc::tmp_response::{
    RpcAccountBalance as RpcAccountBalanceOriginal,
    RpcBlockProduction as RpcBlockProductionOriginal,
    RpcBlockProductionRange as RpcBlockProductionRangeOriginal,
    RpcContactInfo as RpcContactInfoOriginal, RpcInflationGovernor as RpcInflationGovernorOriginal,
    RpcInflationRate as RpcInflationRateOriginal, RpcInflationReward as RpcInflationRewardOriginal,
    RpcLogsResponse as RpcLogsResponseOriginal, RpcPerfSample as RpcPerfSampleOriginal,
    RpcSnapshotSlotInfo as RpcSnapshotSlotInfoOriginal,
    RpcStakeActivation as RpcStakeActivationOriginal, RpcSupply as RpcSupplyOriginal,
    StakeActivationState as StakeActivationStateOriginal,
};
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
        TransactionStatus as TransactionStatusOriginal, UiTransactionReturnData,
    },
    to_py_err,
    transaction_status::UiConfirmedBlock,
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly, SolderHash,
};

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
        let cloned = self.clone();
        Python::with_gil(|py| {
            let constructor = cloned.into_py(py).getattr(py, "from_bytes")?;
            Ok((constructor, (self.pybytes(py).to_object(py),).to_object(py)))
        })
    }

    fn py_to_json(&self) -> String {
        let to_serialize = Resp::Result {
            jsonrpc: crate::rpc::requests::V2::default(),
            result: self.clone(),
            id: 0,
        };
        serde_json::to_string(&to_serialize).unwrap()
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

macro_rules! contextless_struct_def_no_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name($inner);
        resp_traits!($name);
    };
}

macro_rules! contextless_struct_def_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name($inner);
        resp_traits!($name);
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        #[serde_as]
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name(#[serde_as(as = $serde_as)] $inner);
        resp_traits!($name);
    };
}

macro_rules! contextless_resp_methods_no_clone {
    ($name:ident, $inner:ty) => {
        #[common_methods_rpc_resp]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            #[getter]
            pub fn value(&self) -> $inner {
                self.0
            }
        }
    };
}

macro_rules! contextless_resp_methods_clone {
    ($name:ident, $inner:ty) => {
        #[common_methods_rpc_resp]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            #[getter]
            pub fn value(&self) -> $inner {
                self.0.clone()
            }
        }
    };
}

macro_rules! contextless_resp_eq {
    ($name:ident, $inner:ty) => {
        contextless_struct_def_eq!($name, $inner);
        contextless_resp_methods_no_clone!($name, $inner);
    };
    ($name:ident, $inner:ty, clone) => {
        contextless_struct_def_eq!($name, $inner);
        contextless_resp_methods_clone!($name, $inner);
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        contextless_struct_def_eq!($name, $inner, $serde_as);
        contextless_resp_methods_no_clone!($name, $inner);
    };
    ($name:ident, $inner:ty, clone, $serde_as:expr) => {
        contextless_struct_def_eq!($name, $inner, $serde_as);
        contextless_resp_methods_clone!($name, $inner);
    };
}

macro_rules! contextless_resp_no_eq {
    ($name:ident, $inner:ty, clone) => {
        contextless_struct_def_no_eq!($name, $inner);
        contextless_resp_methods_clone!($name, $inner);
    };
    ($name:ident, $inner:ty) => {
        contextless_struct_def_no_eq!($name, $inner);
        contextless_resp_methods_no_clone!($name, $inner);
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Resp<T: PyClass + IntoPy<PyObject>> {
    Result {
        #[serde(skip_deserializing)]
        jsonrpc: crate::rpc::requests::V2,
        result: T,
        #[serde(skip_deserializing)]
        id: u64,
    },
    Error {
        #[serde(skip_deserializing)]
        jsonrpc: crate::rpc::requests::V2,
        error: RpcError,
        #[serde(skip_deserializing)]
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

macro_rules! contextful_struct_def_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            value: $inner,
        }
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        #[serde_as]
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            #[serde_as(as = $serde_as)]
            value: $inner,
        }
    };
}

macro_rules! contextful_struct_def_no_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            value: $inner,
        }
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        #[serde_as]
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            #[serde_as(as = $serde_as)]
            value: $inner,
        }
    };
}

macro_rules! contextful_resp_boilerplate {
    ($name:ident, $inner:ty) => {
        resp_traits!($name);
        #[common_methods_rpc_resp]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: $inner, context: RpcResponseContext) -> Self {
                Self { value, context }
            }
        }
    };
}

macro_rules! contextful_resp_eq {
    ($name:ident, $inner:ty) => {
        contextful_struct_def_eq!($name, $inner);
        contextful_resp_boilerplate!($name, $inner);
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        contextful_struct_def_eq!($name, $inner, $serde_as);
        contextful_resp_boilerplate!($name, $inner);
    };
}

macro_rules! contextful_resp_no_eq {
    ($name:ident, $inner:ty) => {
        contextful_struct_def_no_eq!($name, $inner);
        contextful_resp_boilerplate!($name, $inner);
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        contextful_struct_def_no_eq!($name, $inner, $serde_as);
        contextful_resp_boilerplate!($name, $inner);
    };
}

contextful_resp_eq!(
    GetAccountInfoResp,
    Option<Account>,
    "Option<FromInto<UiAccount>>"
);

contextful_resp_eq!(
    GetAccountInfoJsonParsedResp,
    Option<AccountJSON>,
    "Option<FromInto<UiAccount>>"
);

contextful_resp_eq!(GetBalanceResp, u64);

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

contextless_resp_eq!(GetBlockHeightResp, u64);

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

contextful_resp_eq!(GetBlockProductionResp, RpcBlockProduction);

contextless_resp_no_eq!(GetBlockResp, Option<UiConfirmedBlock>, clone);

contextless_resp_eq!(GetBlocksResp, Vec<u64>, clone);
contextless_resp_eq!(GetBlocksWithLimitResp, Vec<u64>, clone);
contextless_resp_eq!(GetBlockTimeResp, Option<u64>);

// the one in solana_client doesn't derive Eq or PartialEq
// TODO: it does derive these things in latest unreleased version
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

contextless_resp_eq!(GetClusterNodesResp, Vec<RpcContactInfo>, clone);
contextless_resp_eq!(GetEpochInfoResp, EpochInfo, clone);
contextless_resp_eq!(GetEpochScheduleResp, EpochSchedule, clone);
contextful_resp_eq!(GetFeeForMessageResp, Option<u64>);

contextless_resp_eq!(GetFirstAvailableBlockResp, u64);
contextless_resp_eq!(GetGenesisHashResp, SolderHash, "DisplayFromStr");
contextless_resp_eq!(GetHealthResp, String, clone);

impl From<TransactionReturnData> for UiTransactionReturnData {
    fn from(t: TransactionReturnData) -> Self {
        TransactionReturnDataOriginal::from(t).into()
    }
}

impl From<UiTransactionReturnData> for TransactionReturnData {
    fn from(r: UiTransactionReturnData) -> Self {
        Self::new(
            r.program_id.parse().unwrap(),
            base64::decode(r.data.0).unwrap(),
        )
    }
}

// the one in solana_client doesn't derive Eq
// TODO: latest does
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
    #[serde_as(as = "Option<FromInto<UiTransactionReturnData>>")]
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

contextless_resp_eq!(GetHighestSnapshotSlotResp, RpcSnapshotSlotInfo, clone);

// the one in solana_client doesn't derive Eq
// TODO: latest does
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

contextless_resp_eq!(GetIdentityResp, RpcIdentity, clone);

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

contextless_resp_no_eq!(GetInflationGovernorResp, RpcInflationGovernor, clone);

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

contextless_resp_no_eq!(GetInflationRateResp, RpcInflationRate, clone);

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

contextless_resp_eq!(
    GetInflationRewardResp,
    Vec<Option<RpcInflationReward>>,
    clone
);

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
contextful_resp_eq!(GetLargestAccountsResp, Vec<RpcAccountBalance>);

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

contextful_resp_eq!(GetLatestBlockhashResp, RpcBlockhash);

type RpcLeaderSchedule = Option<HashMap<Pubkey, Vec<usize>>>;

contextless_resp_eq!(
    GetLeaderScheduleResp,
    RpcLeaderSchedule,
    clone,
    "Option<HashMap<DisplayFromStr, _>>"
);

contextless_resp_eq!(GetMaxRetransmitSlotResp, u64);
contextless_resp_eq!(GetMaxShredInsertSlotResp, u64);
contextless_resp_eq!(GetMinimumBalanceForRentExemptionResp, u64);
contextful_resp_eq!(
    GetMultipleAccountsResp,
    Vec<Option<Account>>,
    "Vec<Option<FromInto<UiAccount>>>"
);
contextful_resp_eq!(
    GetMultipleAccountsJsonParsedResp,
    Vec<Option<AccountJSON>>,
    "Vec<Option<FromInto<UiAccount>>>"
);

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

contextful_resp_eq!(GetProgramAccountsWithContextResp, Vec<RpcKeyedAccount>);
contextful_resp_eq!(
    GetProgramAccountsWithContextJsonParsedResp,
    Vec<RpcKeyedAccountJsonParsed>
);

contextless_resp_eq!(
    GetProgramAccountsWithoutContextResp,
    Vec<RpcKeyedAccount>,
    clone
);
contextless_resp_eq!(
    GetProgramAccountsWithoutContextJsonParsedResp,
    Vec<RpcKeyedAccountJsonParsed>,
    clone
);

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

contextless_resp_eq!(GetRecentPerformanceSamplesResp, Vec<RpcPerfSample>, clone);

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

contextless_resp_eq!(
    GetSignaturesForAddressResp,
    Vec<RpcConfirmedTransactionStatusWithSignature>,
    clone
);

contextful_resp_eq!(
    GetSignatureStatusesResp,
    Vec<Option<TransactionStatus>>,
    "Vec<Option<FromInto<TransactionStatusOriginal>>>"
);

contextless_resp_eq!(GetSlotResp, Slot);
contextless_resp_eq!(GetSlotLeaderResp, Pubkey, "DisplayFromStr");
contextless_resp_eq!(
    GetSlotLeadersResp,
    Vec<Pubkey>,
    clone,
    "Vec<DisplayFromStr>"
);

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

contextless_resp_eq!(GetStakeActivationResp, RpcStakeActivation, clone);

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

contextful_resp_eq!(GetSupplyResp, RpcSupply);
contextful_resp_no_eq!(GetTokenAccountBalanceResp, UiTokenAmount);
contextful_resp_eq!(GetTokenAccountsByDelegateResp, Vec<RpcKeyedAccount>);
contextful_resp_eq!(
    GetTokenAccountsByDelegateJsonParsedResp,
    Vec<RpcKeyedAccountJsonParsed>
);
contextful_resp_eq!(GetTokenAccountsByOwnerResp, Vec<RpcKeyedAccount>);
contextful_resp_eq!(
    GetTokenAccountsByOwnerJsonParsedResp,
    Vec<RpcKeyedAccountJsonParsed>
);

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

contextful_resp_no_eq!(GetTokenLargestAccountsResp, Vec<RpcTokenAccountBalance>);
contextful_resp_no_eq!(GetTokenSupplyResp, UiTokenAmount);
contextless_resp_no_eq!(
    GetTransactionResp,
    Option<EncodedConfirmedTransactionWithStatusMeta>,
    clone
);
contextless_resp_eq!(GetTransactionCountResp, u64);
contextless_resp_eq!(GetVersionResp, RpcVersionInfo, clone);

// the one in solana_client doesn't implement PartialEq
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct RpcVersionInfoOriginal {
    /// The current version of solana-core
    pub solana_core: String,
    /// first 4 bytes of the FeatureSet identifier
    pub feature_set: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVersionInfo(RpcVersionInfoOriginal);

response_data_boilerplate!(RpcVersionInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVersionInfo {
    #[new]
    pub fn new(solana_core: String, feature_set: Option<u32>) -> Self {
        RpcVersionInfoOriginal {
            solana_core,
            feature_set,
        }
        .into()
    }

    #[getter]
    pub fn solana_core(&self) -> String {
        self.0.solana_core.clone()
    }

    #[getter]
    pub fn feature_set(&self) -> Option<u32> {
        self.0.feature_set
    }
}

// the one in solana_client doesn't implement PartialEq
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountInfoOriginal {
    /// Vote account address, as base-58 encoded string
    pub vote_pubkey: String,

    /// The validator identity, as base-58 encoded string
    pub node_pubkey: String,

    /// The current stake, in lamports, delegated to this vote account
    pub activated_stake: u64,

    /// An 8-bit integer used as a fraction (commission/MAX_U8) for rewards payout
    pub commission: u8,

    /// Whether this account is staked for the current epoch
    pub epoch_vote_account: bool,

    /// History of how many credits earned by the end of each epoch
    ///   each tuple is (Epoch, credits, prev_credits)
    pub epoch_credits: Vec<(Epoch, u64, u64)>,

    /// Most recent slot voted on by this vote account (0 if no votes exist)
    pub last_vote: u64,

    /// Current root slot for this vote account (0 if not root slot exists)
    pub root_slot: Slot,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVoteAccountInfo(RpcVoteAccountInfoOriginal);

response_data_boilerplate!(RpcVoteAccountInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVoteAccountInfo {
    #[new]
    pub fn new(
        vote_pubkey: Pubkey,
        node_pubkey: Pubkey,
        activated_stake: u64,
        commission: u8,
        epoch_vote_account: bool,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_vote: u64,
        root_slot: Slot,
    ) -> Self {
        RpcVoteAccountInfoOriginal {
            vote_pubkey: vote_pubkey.to_string(),
            node_pubkey: node_pubkey.to_string(),
            activated_stake,
            commission,
            epoch_vote_account,
            epoch_credits,
            last_vote,
            root_slot,
        }
        .into()
    }
    #[getter]
    pub fn vote_pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.vote_pubkey).unwrap()
    }
    #[getter]
    pub fn node_pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.node_pubkey).unwrap()
    }
    #[getter]
    pub fn activated_stake(&self) -> u64 {
        self.0.activated_stake
    }
    #[getter]
    pub fn commission(&self) -> u8 {
        self.0.commission
    }
    #[getter]
    pub fn epoch_vote_account(&self) -> bool {
        self.0.epoch_vote_account
    }
    #[getter]
    pub fn epoch_credits(&self) -> Vec<(Epoch, u64, u64)> {
        self.0.epoch_credits.clone()
    }
    #[getter]
    pub fn last_vote(&self) -> u64 {
        self.0.last_vote
    }
    #[getter]
    pub fn root_slot(&self) -> Slot {
        self.0.root_slot
    }
}

// the one in solana_client doesn't derive PartialEq
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountStatusOriginal {
    pub current: Vec<RpcVoteAccountInfoOriginal>,
    pub delinquent: Vec<RpcVoteAccountInfoOriginal>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVoteAccountStatus(RpcVoteAccountStatusOriginal);

response_data_boilerplate!(RpcVoteAccountStatus);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVoteAccountStatus {
    #[new]
    pub fn new(current: Vec<RpcVoteAccountInfo>, delinquent: Vec<RpcVoteAccountInfo>) -> Self {
        RpcVoteAccountStatusOriginal {
            current: current.into_iter().map(|x| x.into()).collect(),
            delinquent: delinquent.into_iter().map(|x| x.into()).collect(),
        }
        .into()
    }
    #[getter]
    pub fn current(&self) -> Vec<RpcVoteAccountInfo> {
        self.0
            .current
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }

    #[getter]
    pub fn delinquent(&self) -> Vec<RpcVoteAccountInfo> {
        self.0
            .delinquent
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcLogsResponse(RpcLogsResponseOriginal);

response_data_boilerplate!(RpcLogsResponse);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcLogsResponse {
    #[new]
    pub fn new(signature: Signature, err: Option<TransactionErrorType>, logs: Vec<String>) -> Self {
        RpcLogsResponseOriginal {
            signature: signature.to_string(),
            err: err.map(|e| e.into()),
            logs,
        }
        .into()
    }

    #[getter]
    pub fn signature(&self) -> Signature {
        self.0.signature.parse().unwrap()
    }
    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.0.err.clone().map(|e| e.into())
    }
    #[getter]
    pub fn logs(&self) -> Vec<String> {
        self.0.logs.clone()
    }
}

contextless_resp_eq!(GetVoteAccountsResp, RpcVoteAccountStatus, clone);
contextful_resp_eq!(IsBlockhashValidResp, bool);
contextless_resp_eq!(MinimumLedgerSlotResp, u64);
contextless_resp_eq!(RequestAirdropResp, Signature, "DisplayFromStr");
contextless_resp_eq!(SendTransactionResp, Signature, "DisplayFromStr");
contextful_resp_eq!(SimulateTransactionResp, RpcSimulateTransactionResult);

macro_rules ! pyunion_resp {
    ($name:ident, $($variant:ident),+) => {
        #[derive(FromPyObject, Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(untagged, rename_all = "camelCase")]
        pub enum $name {
            $($variant($variant),)+
        }

        impl $name {
            fn to_json(&self) -> String {
                match self {
                    $(Self::$variant(x) => x.py_to_json(),)+
                }
            }
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

pyunion_resp!(
    RPCResult,
    RpcError,
    GetAccountInfoResp,
    GetAccountInfoJsonParsedResp,
    GetBalanceResp,
    GetBlockProductionResp,
    GetBlockResp,
    GetBlockCommitmentResp,
    GetBlockHeightResp,
    GetBlocksResp,
    GetBlocksWithLimitResp,
    GetBlockTimeResp,
    GetClusterNodesResp,
    GetEpochInfoResp,
    GetEpochScheduleResp,
    GetFeeForMessageResp,
    GetFirstAvailableBlockResp,
    GetGenesisHashResp,
    GetHealthResp,
    GetHighestSnapshotSlotResp,
    GetIdentityResp,
    GetInflationGovernorResp,
    GetInflationRateResp,
    GetInflationRewardResp,
    GetLargestAccountsResp,
    GetLatestBlockhashResp,
    GetLeaderScheduleResp,
    GetMaxRetransmitSlotResp,
    GetMaxShredInsertSlotResp,
    GetMinimumBalanceForRentExemptionResp,
    GetMultipleAccountsResp,
    GetMultipleAccountsJsonParsedResp,
    GetProgramAccountsWithContextResp,
    GetProgramAccountsWithoutContextResp,
    GetProgramAccountsWithContextJsonParsedResp,
    GetProgramAccountsWithoutContextJsonParsedResp,
    GetRecentPerformanceSamplesResp,
    GetSignaturesForAddressResp,
    GetSignatureStatusesResp,
    GetSlotResp,
    GetSlotLeaderResp,
    GetSlotLeadersResp,
    GetStakeActivationResp,
    GetSupplyResp,
    GetTokenAccountBalanceResp,
    GetTokenAccountsByDelegateResp,
    GetTokenAccountsByDelegateJsonParsedResp,
    GetTokenAccountsByOwnerResp,
    GetTokenAccountsByOwnerJsonParsedResp,
    GetTokenLargestAccountsResp,
    GetTokenSupplyResp,
    GetTransactionResp,
    GetTransactionCountResp,
    GetVersionResp,
    RpcVersionInfo,
    GetVoteAccountsResp,
    IsBlockhashValidResp,
    MinimumLedgerSlotResp,
    RequestAirdropResp,
    SendTransactionResp,
    SimulateTransactionResp
);

/// Serialize a list of response objects into a single batch response JSON.
///
/// Args:
///     resps: A list of response objects.
///
/// Returns:
///     str: The batch JSON string.
///
/// Example:
///     >>> from solders.rpc.responses import batch_to_json, GetBlockHeightResp, GetFirstAvailableBlockResp
///     >>> batch_to_json([GetBlockHeightResp(1233), GetFirstAvailableBlockResp(1)])
///     '[{"id":0,"jsonrpc":"2.0","result":1233},{"id":0,"jsonrpc":"2.0","result":1}]'
///
#[pyfunction]
pub fn batch_to_json(resps: Vec<RPCResult>) -> String {
    let objects: Vec<serde_json::Map<String, serde_json::Value>> = resps
        .iter()
        .map(|r| serde_json::from_str(&r.to_json()).unwrap())
        .collect();
    serde_json::to_string(&objects).unwrap()
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
///     >>> from solders.rpc.responses import batch_from_json, GetBlockHeightResp, GetFirstAvailableBlockResp
///     >>> raw = '[{ "jsonrpc": "2.0", "result": 1233, "id": 1 },{ "jsonrpc": "2.0", "result": 111, "id": 1 }]'
///     >>> batch_from_json(raw, [GetBlockHeightResp, GetFirstAvailableBlockResp])
///     [GetBlockHeightResp(
///         1233,
///     ), GetFirstAvailableBlockResp(
///         111,
///     )]
///
#[pyfunction]
pub fn batch_from_json(raw: &str, parsers: Vec<&PyType>) -> PyResult<Vec<PyObject>> {
    let raw_objects: Vec<serde_json::Map<String, serde_json::Value>> =
        serde_json::from_str(raw).map_err(to_py_err)?;
    let raw_objects_len = raw_objects.len();
    let parsers_len = parsers.len();
    if raw_objects_len != parsers_len {
        let msg = format!("Number of parsers does not match number of response objects. Num parsers: {}. Num responses: {}", parsers_len, raw_objects_len);
        Err(PyValueError::new_err(msg))
    } else {
        Python::with_gil(|py| {
            Ok(raw_objects
                .iter()
                .zip(parsers.iter())
                .map(|(res, parser)| {
                    parser
                        .call_method1("from_json", (serde_json::to_string(res).unwrap(),))
                        .unwrap()
                        .into_py(py)
                })
                .collect())
        })
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
    let rpc_result_members = PyTuple::new(
        py,
        vec![
            RpcError::type_object(py),
            GetAccountInfoResp::type_object(py),
            GetAccountInfoJsonParsedResp::type_object(py),
            GetBalanceResp::type_object(py),
            GetBlockProductionResp::type_object(py),
            GetBlockResp::type_object(py),
            GetBlockCommitmentResp::type_object(py),
            GetBlockHeightResp::type_object(py),
            GetBlocksResp::type_object(py),
            GetBlocksWithLimitResp::type_object(py),
            GetBlockTimeResp::type_object(py),
            GetClusterNodesResp::type_object(py),
            GetEpochInfoResp::type_object(py),
            GetEpochScheduleResp::type_object(py),
            GetFeeForMessageResp::type_object(py),
            GetFirstAvailableBlockResp::type_object(py),
            GetGenesisHashResp::type_object(py),
            GetHealthResp::type_object(py),
            GetHighestSnapshotSlotResp::type_object(py),
            GetIdentityResp::type_object(py),
            GetInflationGovernorResp::type_object(py),
            GetInflationRateResp::type_object(py),
            GetInflationRewardResp::type_object(py),
            GetLargestAccountsResp::type_object(py),
            GetLatestBlockhashResp::type_object(py),
            GetLeaderScheduleResp::type_object(py),
            GetMaxRetransmitSlotResp::type_object(py),
            GetMaxShredInsertSlotResp::type_object(py),
            GetMinimumBalanceForRentExemptionResp::type_object(py),
            GetMultipleAccountsResp::type_object(py),
            GetMultipleAccountsJsonParsedResp::type_object(py),
            GetProgramAccountsWithContextResp::type_object(py),
            GetProgramAccountsWithoutContextResp::type_object(py),
            GetProgramAccountsWithContextJsonParsedResp::type_object(py),
            GetProgramAccountsWithoutContextJsonParsedResp::type_object(py),
            GetRecentPerformanceSamplesResp::type_object(py),
            GetSignaturesForAddressResp::type_object(py),
            GetSignatureStatusesResp::type_object(py),
            GetSlotResp::type_object(py),
            GetSlotLeaderResp::type_object(py),
            GetSlotLeadersResp::type_object(py),
            GetStakeActivationResp::type_object(py),
            GetSupplyResp::type_object(py),
            GetTokenAccountBalanceResp::type_object(py),
            GetTokenAccountsByDelegateResp::type_object(py),
            GetTokenAccountsByDelegateJsonParsedResp::type_object(py),
            GetTokenAccountsByOwnerResp::type_object(py),
            GetTokenAccountsByOwnerJsonParsedResp::type_object(py),
            GetTokenLargestAccountsResp::type_object(py),
            GetTokenSupplyResp::type_object(py),
            GetTransactionResp::type_object(py),
            GetTransactionCountResp::type_object(py),
            GetVersionResp::type_object(py),
            RpcVersionInfo::type_object(py),
            GetVoteAccountsResp::type_object(py),
            IsBlockhashValidResp::type_object(py),
            MinimumLedgerSlotResp::type_object(py),
            RequestAirdropResp::type_object(py),
            SendTransactionResp::type_object(py),
            SimulateTransactionResp::type_object(py),
        ],
    );
    let rpc_result_alias = union.get_item(rpc_result_members)?;
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
    m.add_class::<GetMinimumBalanceForRentExemptionResp>()?;
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
    m.add_class::<GetTransactionCountResp>()?;
    m.add_class::<GetVersionResp>()?;
    m.add_class::<RpcVersionInfo>()?;
    m.add_class::<RpcVoteAccountInfo>()?;
    m.add_class::<RpcVoteAccountStatus>()?;
    m.add_class::<GetVoteAccountsResp>()?;
    m.add_class::<IsBlockhashValidResp>()?;
    m.add_class::<MinimumLedgerSlotResp>()?;
    m.add_class::<RequestAirdropResp>()?;
    m.add_class::<SendTransactionResp>()?;
    m.add_class::<SimulateTransactionResp>()?;
    m.add_class::<RpcLogsResponse>()?;
    m.add("RPCResult", rpc_result_alias)?;
    let funcs = [
        wrap_pyfunction!(batch_to_json, m)?,
        wrap_pyfunction!(batch_from_json, m)?,
    ];
    for func in funcs {
        m.add_function(func)?;
    }
    Ok(m)
}
