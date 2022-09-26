from typing import Optional, Union
from solders.rpc.responses import RpcSimulateTransactionResult
from solders.transaction_status import TransactionErrorType

class BlockCleanedUp:
    slot: int
    first_available_block: int
    def __init__(self, slot: int, first_available_block: int) -> None: ...

class SendTransactionPreflightFailure:
    message: str
    result: RpcSimulateTransactionResult
    def __init__(self, message: str, result: RpcSimulateTransactionResult) -> None: ...

class RpcCustomErrorFieldless:
    TransactionSignatureVerificationFailure: "RpcCustomErrorFieldless"
    NoSnapshot: "RpcCustomErrorFieldless"
    TransactionHistoryNotAvailable: "RpcCustomErrorFieldless"
    TransactionSignatureLenMismatch: "RpcCustomErrorFieldless"
    Base64Zstd: "RpcCustomErrorFieldless"
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...

class BlockNotAvailable:
    slot: int
    def __init__(self, slot: int) -> None: ...

class NodeUnhealthy:
    num_slots_behind: Optional[int]
    def __init__(self, num_slots_behind: Optional[int] = None) -> None: ...

class TransactionPrecompileVerificationFailure:
    def __init__(self, error: TransactionErrorType) -> None: ...
    def error(self) -> TransactionErrorType: ...

class SlotSkipped:
    slot: int
    def __init__(self, slot: int) -> None: ...

class LongTermStorageSlotSkipped:
    slot: int
    def __init__(self, slot: int) -> None: ...

class KeyExcludedFromSecondaryIndex:
    index_key: str
    def __init__(self, index_key: str) -> None: ...

class ScanError:
    message: str
    def __init__(self, message: str) -> None: ...

class BlockStatusNotAvailableYet:
    slot: int
    def __init__(self, slot: int) -> None: ...

class MinContextSlotNotReached:
    context_slot: int
    def __init__(self, context_slot: int) -> None: ...

class UnsupportedTransactionVersion:
    def __init__(self, value: int) -> None: ...
    def value(self) -> int: ...

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
