from solders.internal import InnerInstruction, TransactionMetadata, FailedTransactionMetadata, SimulatedTransactionInfo
from typing import Union

SimulateResult = Union[SimulatedTransactionInfo, FailedTransactionMetadata]
TransactionResult = Union[TransactionMetadata, FailedTransactionMetadata]
