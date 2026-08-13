from typing import Union

from ..solders import (
    CompileError,
    Message,
    MessageAddressTableLookup,
    MessageHeader,
    MessageV0,
    MessageV1,
    TransactionConfig,
    from_bytes_versioned,
    to_bytes_versioned,
)
from . import v0, v1

VersionedMessage = Union[Message, MessageV0, MessageV1]

__all__ = [
    "CompileError",
    "Message",
    "MessageAddressTableLookup",
    "MessageHeader",
    "MessageV0",
    "MessageV1",
    "TransactionConfig",
    "to_bytes_versioned",
    "from_bytes_versioned",
    "VersionedMessage",
    "v0",
    "v1",
]
