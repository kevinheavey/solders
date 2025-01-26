from .solders import Message, MessageAddressTableLookup, MessageHeader, MessageV0, to_bytes_versioned, from_bytes_versioned
from typing import Union

VersionedMessage = Union[Message, MessageV0]

__all__ = ["Message", "MessageAddressTableLookup", "MessageHeader", "MessageV0", "to_bytes_versioned", "from_bytes_versioned", "VersionedMessage"]
