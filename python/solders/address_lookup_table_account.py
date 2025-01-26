from typing import Final, Union

from .solders import (
    ADDRESS_LOOKUP_TABLE_ID,
    AddressLookupTable,
    AddressLookupTableAccount,
    LookupTableMeta,
    LookupTableStatusDeactivating,
    LookupTableStatusFieldless,
    Pubkey,
    SlotHashes,
    derive_lookup_table_address,
)
from .solders import (
    LOOKUP_TABLE_MAX_ADDRESSES as _LOOKUP_TABLE_MAX_ADDRESSES,
)
from .solders import (
    LOOKUP_TABLE_META_SIZE as _LOOKUP_TABLE_META_SIZE,
)

ID: Final[Pubkey] = ADDRESS_LOOKUP_TABLE_ID
"""Address lookup table program ID."""

LOOKUP_TABLE_MAX_ADDRESSES: Final[int] = _LOOKUP_TABLE_MAX_ADDRESSES
"""The maximum number of addresses that a lookup table can hold"""

LOOKUP_TABLE_META_SIZE: Final[int] = _LOOKUP_TABLE_META_SIZE
"""The serialized size of lookup table metadata"""

LookupTableStatusType = Union[LookupTableStatusFieldless, LookupTableStatusDeactivating]

__all__ = [
    "LookupTableStatusFieldless",
    "LookupTableStatusDeactivating",
    "AddressLookupTableAccount",
    "AddressLookupTable",
    "LookupTableMeta",
    "ID",
    "LOOKUP_TABLE_MAX_ADDRESSES",
    "LOOKUP_TABLE_META_SIZE",
    "LookupTableStatusType",
    "SlotHashes",
    "derive_lookup_table_address",
]
