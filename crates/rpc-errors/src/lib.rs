use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, IntoPyObject)]
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

pub fn include_errors(m: &Bound<'_, PyModule>) -> PyResult<()> {
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
    Ok(())
}
