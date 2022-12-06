use crate::transaction_status::{transaction_status_boilerplate, TransactionErrorType};
use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyTuple, PyTypeInfo};
use serde::{Deserialize, Serialize};
use solana_sdk::slot_history::Slot;
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_traits::RichcmpEqualityOnly;
use std::fmt::Display;

use super::responses::RpcSimulateTransactionResult;

macro_rules! error_message {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
        #[pyclass(module = "solders.rpc.errors", subclass)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[pyo3(get)]
            message: String,
        }

        transaction_status_boilerplate!($name);

        #[richcmp_eq_only]
        #[common_methods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(message: String) -> Self {
                message.into()
            }
        }
    };
    ($name:ident, $data_type:ty) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
        #[pyclass(module = "solders.rpc.errors", subclass)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[pyo3(get)]
            message: String,
            #[pyo3(get)]
            data: $data_type,
        }

        transaction_status_boilerplate!($name);

        #[richcmp_eq_only]
        #[common_methods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(message: String, data: $data_type) -> Self {
                (message, data).into()
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
#[serde(rename_all = "camelCase")]
pub struct BlockCleanedUp {
    #[pyo3(get)]
    slot: Slot,
    #[pyo3(get)]
    first_available_block: Slot,
}

transaction_status_boilerplate!(BlockCleanedUp);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BlockCleanedUp {
    #[new]
    pub fn new(slot: Slot, first_available_block: Slot) -> Self {
        (slot, first_available_block).into()
    }
}

error_message!(BlockCleanedUpMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct SendTransactionPreflightFailure {
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    result: RpcSimulateTransactionResult,
}

transaction_status_boilerplate!(SendTransactionPreflightFailure);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl SendTransactionPreflightFailure {
    #[new]
    pub fn new(message: String, result: RpcSimulateTransactionResult) -> Self {
        (message, result).into()
    }
}

error_message!(
    SendTransactionPreflightFailureMessage,
    RpcSimulateTransactionResult
);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[pyclass(module = "solders.transaction_status")]
pub enum RpcCustomErrorFieldless {
    TransactionSignatureVerificationFailure,
    NoSnapshot,
    TransactionHistoryNotAvailable,
    TransactionSignatureLenMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct BlockNotAvailable {
    #[pyo3(get)]
    slot: Slot,
}

transaction_status_boilerplate!(BlockNotAvailable);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BlockNotAvailable {
    #[new]
    pub fn new(slot: Slot) -> Self {
        slot.into()
    }
}

error_message!(BlockNotAvailableMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
#[serde(rename_all = "camelCase")]
pub struct NodeUnhealthy {
    #[pyo3(get)]
    num_slots_behind: Option<Slot>,
}

transaction_status_boilerplate!(NodeUnhealthy);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl NodeUnhealthy {
    #[new]
    pub fn new(num_slots_behind: Option<Slot>) -> Self {
        num_slots_behind.into()
    }
}

error_message!(NodeUnhealthyMessage, NodeUnhealthy);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct TransactionPrecompileVerificationFailure(TransactionErrorType);

transaction_status_boilerplate!(TransactionPrecompileVerificationFailure);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionPrecompileVerificationFailure {
    #[new]
    pub fn new(error: TransactionErrorType) -> Self {
        error.into()
    }

    #[getter]
    pub fn error(&self) -> TransactionErrorType {
        self.0.clone()
    }
}

error_message!(TransactionPrecompileVerificationFailureMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct SlotSkipped {
    #[pyo3(get)]
    slot: Slot,
}

transaction_status_boilerplate!(SlotSkipped);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl SlotSkipped {
    #[new]
    pub fn new(slot: Slot) -> Self {
        slot.into()
    }
}

error_message!(SlotSkippedMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct LongTermStorageSlotSkipped {
    #[pyo3(get)]
    slot: Slot,
}

transaction_status_boilerplate!(LongTermStorageSlotSkipped);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl LongTermStorageSlotSkipped {
    #[new]
    pub fn new(slot: Slot) -> Self {
        slot.into()
    }
}

error_message!(LongTermStorageSlotSkippedMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
#[serde(rename_all = "camelCase")]
pub struct KeyExcludedFromSecondaryIndex {
    #[pyo3(get)]
    index_key: String,
}

transaction_status_boilerplate!(KeyExcludedFromSecondaryIndex);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl KeyExcludedFromSecondaryIndex {
    #[new]
    pub fn new(index_key: String) -> Self {
        index_key.into()
    }
}

error_message!(KeyExcludedFromSecondaryIndexMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct ScanError {
    #[pyo3(get)]
    message: String,
}

transaction_status_boilerplate!(ScanError);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ScanError {
    #[new]
    pub fn new(message: String) -> Self {
        message.into()
    }
}

error_message!(ScanErrorMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct BlockStatusNotAvailableYet {
    #[pyo3(get)]
    slot: Slot,
}

transaction_status_boilerplate!(BlockStatusNotAvailableYet);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BlockStatusNotAvailableYet {
    #[new]
    pub fn new(slot: Slot) -> Self {
        slot.into()
    }
}

error_message!(BlockStatusNotAvailableYetMessage);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
#[serde(rename_all = "camelCase")]
pub struct MinContextSlotNotReached {
    #[pyo3(get)]
    context_slot: Slot,
}

transaction_status_boilerplate!(MinContextSlotNotReached);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl MinContextSlotNotReached {
    #[new]
    pub fn new(context_slot: Slot) -> Self {
        context_slot.into()
    }
}

error_message!(MinContextSlotNotReachedMessage, MinContextSlotNotReached);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct UnsupportedTransactionVersion(pub u8);

transaction_status_boilerplate!(UnsupportedTransactionVersion);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UnsupportedTransactionVersion {
    #[new]
    pub fn new(value: u8) -> Self {
        value.into()
    }

    #[getter]
    pub fn value(&self) -> u8 {
        self.0
    }
}

error_message!(UnsupportedTransactionVersionMessage);

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
#[serde(untagged)]
pub enum RpcCustomError {
    Fieldless(RpcCustomErrorFieldless),
    BlockCleanedUp(BlockCleanedUp),
    SendTransactionPreflightFailure(SendTransactionPreflightFailure),
    BlockNotAvailable(BlockNotAvailable),
    NodeUnhealthy(NodeUnhealthy),
    TransactionPrecompileVerificationFailure(TransactionPrecompileVerificationFailure),
    SlotSkipped(SlotSkipped),
    LongTermStorageSlotSkipped(LongTermStorageSlotSkipped),
    KeyExcludedFromSecondaryIndex(KeyExcludedFromSecondaryIndex),
    ScanError(ScanError),
    BlockStatusNotAvailableYet(BlockStatusNotAvailableYet),
    MinContextSlotNotReached(MinContextSlotNotReached),
    UnsupportedTransactionVersion(UnsupportedTransactionVersion),
}

error_message!(ParseErrorMessage);
error_message!(InvalidRequestMessage);
error_message!(MethodNotFoundMessage);
error_message!(InvalidParamsMessage);
error_message!(InternalErrorMessage);

pub(crate) fn create_errors_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "errors")?;
    m.add_class::<RpcCustomErrorFieldless>()?;
    m.add_class::<BlockCleanedUp>()?;
    m.add_class::<SendTransactionPreflightFailure>()?;
    m.add_class::<BlockNotAvailable>()?;
    m.add_class::<BlockCleanedUpMessage>()?;
    m.add_class::<SendTransactionPreflightFailureMessage>()?;
    m.add_class::<BlockNotAvailableMessage>()?;
    m.add_class::<NodeUnhealthy>()?;
    m.add_class::<NodeUnhealthyMessage>()?;
    m.add_class::<TransactionPrecompileVerificationFailure>()?;
    m.add_class::<SlotSkipped>()?;
    m.add_class::<LongTermStorageSlotSkipped>()?;
    m.add_class::<BlockCleanedUp>()?;
    m.add_class::<KeyExcludedFromSecondaryIndex>()?;
    m.add_class::<TransactionPrecompileVerificationFailureMessage>()?;
    m.add_class::<SlotSkippedMessage>()?;
    m.add_class::<LongTermStorageSlotSkippedMessage>()?;
    m.add_class::<BlockCleanedUpMessage>()?;
    m.add_class::<KeyExcludedFromSecondaryIndexMessage>()?;
    m.add_class::<ScanError>()?;
    m.add_class::<BlockStatusNotAvailableYet>()?;
    m.add_class::<ScanErrorMessage>()?;
    m.add_class::<BlockStatusNotAvailableYetMessage>()?;
    m.add_class::<MinContextSlotNotReached>()?;
    m.add_class::<MinContextSlotNotReachedMessage>()?;
    m.add_class::<UnsupportedTransactionVersion>()?;
    m.add_class::<UnsupportedTransactionVersionMessage>()?;
    m.add_class::<ParseErrorMessage>()?;
    m.add_class::<InvalidRequestMessage>()?;
    m.add_class::<MethodNotFoundMessage>()?;
    m.add_class::<InvalidParamsMessage>()?;
    m.add_class::<InternalErrorMessage>()?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let union_members = vec![
        RpcCustomErrorFieldless::type_object(py),
        BlockCleanedUp::type_object(py),
        SendTransactionPreflightFailure::type_object(py),
        BlockNotAvailable::type_object(py),
        NodeUnhealthy::type_object(py),
        TransactionPrecompileVerificationFailure::type_object(py),
        SlotSkipped::type_object(py),
        LongTermStorageSlotSkipped::type_object(py),
        BlockCleanedUp::type_object(py),
        KeyExcludedFromSecondaryIndex::type_object(py),
        ScanError::type_object(py),
        BlockStatusNotAvailableYet::type_object(py),
        MinContextSlotNotReached::type_object(py),
        UnsupportedTransactionVersion::type_object(py),
    ];
    m.add(
        "RpcCustomError",
        union.get_item(PyTuple::new(py, union_members))?,
    )?;
    Ok(m)
}
