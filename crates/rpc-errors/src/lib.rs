use pyo3::{prelude::*, types::PyTuple, PyTypeInfo};
use serde::{Deserialize, Serialize};
use solders_macros::EnumIntoPy;
use solders_rpc_errors_no_tx_status::{
    BlockCleanedUp, BlockCleanedUpMessage, BlockNotAvailable, BlockNotAvailableMessage,
    BlockStatusNotAvailableYet, BlockStatusNotAvailableYetMessage, InternalErrorMessage,
    InvalidParamsMessage, InvalidRequestMessage, KeyExcludedFromSecondaryIndex,
    KeyExcludedFromSecondaryIndexMessage, LongTermStorageSlotSkipped,
    LongTermStorageSlotSkippedMessage, MethodNotFoundMessage, MinContextSlotNotReached,
    MinContextSlotNotReachedMessage, NodeUnhealthy, NodeUnhealthyMessage, ParseErrorMessage,
    RpcCustomErrorFieldless, ScanError, ScanErrorMessage, SlotSkipped, SlotSkippedMessage,
    TransactionPrecompileVerificationFailure, TransactionPrecompileVerificationFailureMessage,
    UnsupportedTransactionVersion, UnsupportedTransactionVersionMessage,
};
use solders_rpc_errors_tx_status::{
    SendTransactionPreflightFailure, SendTransactionPreflightFailureMessage,
};

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

pub fn create_errors_mod(py: Python<'_>) -> PyResult<&PyModule> {
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
