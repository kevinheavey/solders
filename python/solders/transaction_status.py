from typing import Union
from solders.internal import (
    ParsedInstruction,
    UiPartiallyDecodedInstruction,
    UiCompiledInstruction,
    UiParsedMessage,
    UiRawMessage,
    VersionedTransaction, 
    UiTransaction,
    UiAccountsList,
    InstructionErrorFieldless,
    InstructionErrorCustom,
    InstructionErrorBorshIO,
    TransactionErrorFieldless,
    TransactionErrorInstructionError,
    TransactionErrorDuplicateInstruction,
    TransactionErrorInsufficientFundsForRent,
    TransactionErrorProgramExecutionTemporarilyRestricted,
)

UiParsedInstruction = Union[ParsedInstruction, UiPartiallyDecodedInstruction]
UiInstruction = Union[UiParsedInstruction, UiCompiledInstruction]
UiMessage = Union[UiParsedMessage, UiRawMessage]
EncodedVersionedTransaction = Union[VersionedTransaction, UiTransaction, UiAccountsList]
InstructionErrorType = Union[
    InstructionErrorFieldless,
    InstructionErrorCustom,
    InstructionErrorBorshIO,
]
TransactionErrorType = Union[
    TransactionErrorFieldless,
    TransactionErrorInstructionError,
    TransactionErrorDuplicateInstruction,
    TransactionErrorInsufficientFundsForRent,
    TransactionErrorProgramExecutionTemporarilyRestricted,
]
