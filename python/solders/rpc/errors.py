from typing import Union
from solders.internal import (
    RpcCustomErrorFieldless,
BlockCleanedUp,
SendTransactionPreflightFailure,
BlockNotAvailable,
NodeUnhealthy,
TransactionPrecompileVerificationFailure,
SlotSkipped,
LongTermStorageSlotSkipped,
BlockCleanedUp,
KeyExcludedFromSecondaryIndex,
ScanError,
BlockStatusNotAvailableYet,
MinContextSlotNotReached,
UnsupportedTransactionVersion,
)

RpcCustomError = Union[
    RpcCustomErrorFieldless,
    BlockCleanedUp,
    SendTransactionPreflightFailure,
    BlockNotAvailable,
    NodeUnhealthy,
    TransactionPrecompileVerificationFailure,
    SlotSkipped,
    LongTermStorageSlotSkipped,
    BlockCleanedUp,
    KeyExcludedFromSecondaryIndex,
    ScanError,
    BlockStatusNotAvailableYet,
    MinContextSlotNotReached,
    UnsupportedTransactionVersion,
]
