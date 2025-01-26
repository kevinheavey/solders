from typing import Union

from .solders import (
    Message,
    MessageAddressTableLookup,
    MessageHeader,
    MessageV0,
    from_bytes_versioned,
    to_bytes_versioned,
)

VersionedMessage = Union[Message, MessageV0]

__all__ = [
    "Message",
    "MessageAddressTableLookup",
    "MessageHeader",
    "MessageV0",
    "to_bytes_versioned",
    "from_bytes_versioned",
    "VersionedMessage",
]
