from .solders import (
    InnerInstruction,
    TransactionMetadata,
    FailedTransactionMetadata,
    SimulatedTransactionInfo,
)
from typing import Union

SimulateResult = Union[SimulatedTransactionInfo, FailedTransactionMetadata]
TransactionResult = Union[TransactionMetadata, FailedTransactionMetadata]

__all__ = [
    "InnerInstruction",
    "TransactionMetadata",
    "FailedTransactionMetadata",
    "SimulatedTransactionInfo",
    "SimulateResult",
    "TransactionResult",
]
