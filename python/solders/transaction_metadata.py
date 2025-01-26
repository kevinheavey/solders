from typing import Union

from .solders import (
    FailedTransactionMetadata,
    InnerInstruction,
    SimulatedTransactionInfo,
    TransactionMetadata,
)

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
