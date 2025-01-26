use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_errors_common::error_message;
use solders_traits_core::transaction_status_boilerplate;
use solders_transaction_error::TransactionErrorType;
type Slot = u64;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[pyclass(module = "solders.transaction_status", eq, eq_int)]
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
    #[pyo3(signature = (num_slots_behind=None))]
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

error_message!(ParseErrorMessage);
error_message!(InvalidRequestMessage);
error_message!(MethodNotFoundMessage);
error_message!(InvalidParamsMessage);
error_message!(InternalErrorMessage);
