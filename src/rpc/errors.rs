use crate::transaction_status_boilerplate;
use derive_more::{From, Into};
use pyo3::prelude::*;
use solders_macros::{common_methods, richcmp_eq_only};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[pyclass(module = "solders.transaction_status")]
pub enum RpcCustomErrorFieldless {
    TransactionSignatureVerificationFailure,
    NoSnapshot,
    TransactionHistoryNotAvailable,
    TransactionSignatureLenMismatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
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
        slot.into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct TransactionPrecompileVerificationFailure(TransactionErrorType);

transaction_status_boilerplate!(NodeUnhealthy);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionPrecompileVerificationFailure {
    #[new]
    pub fn new(error: TransactionErrorType) -> Self {
        slot.into()
    }

    #[getter]
    pub fn error(&self) -> TransactionErrorType {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.errors", subclass)]
pub struct UnsupportedTransactionVersion(u8);

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
        self.into()
    }
}

#[derive(FromPyObject, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum RpcCustomError {
    Fieldless(RpcCustomErrorFieldless),
    BlockCleanedUp(BlockCleanedUp),
    SendTransactionPreflightFailure(SendTransactionPreflightFailure),
    BlockNotAvailable(BlockNotAvailable),
    NodeUnhealthy(NodeUnhealthy),
    TransactionPrecompileVerificationFailure(TransactionPrecompileVerificationFailure),
    SlotSkipped(SlotSkipped),
    LongTermStorageSlotSkipped(LongTermStorageSlotSkipped),
    BlockCleanedUp(BlockCleanedUp),
    KeyExcludedFromSecondaryIndex(KeyExcludedFromSecondaryIndex),
    ScanError(ScanError),
    BlockStatusNotAvailableYet(BlockStatusNotAvailableYet),
    MinContextSlotNotReached(MinContextSlotNotReached),
    UnsupportedTransactionVersion(UnsupportedTransactionVersion),
}
