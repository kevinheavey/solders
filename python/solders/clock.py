from typing import Final

from . import solders as sd
from .solders import Clock

DEFAULT_DEV_SLOTS_PER_EPOCH: Final[int] = sd.DEFAULT_DEV_SLOTS_PER_EPOCH
""""""
DEFAULT_HASHES_PER_SECOND: Final[int] = sd.DEFAULT_HASHES_PER_SECOND
""""""
DEFAULT_HASHES_PER_TICK: Final[int] = sd.DEFAULT_HASHES_PER_TICK
""""""
DEFAULT_MS_PER_SLOT: Final[int] = sd.DEFAULT_MS_PER_SLOT
"""The expected duration of a slot in milliseconds."""
DEFAULT_SLOTS_PER_EPOCH: Final[int] = sd.DEFAULT_SLOTS_PER_EPOCH
"""The number of slots per epoch after initial network warmup."""
DEFAULT_S_PER_SLOT: Final[float] = sd.DEFAULT_S_PER_SLOT
""""""
DEFAULT_TICKS_PER_SECOND: Final[int] = sd.DEFAULT_TICKS_PER_SECOND
"""The default tick rate that the cluster attempts to achieve."""
DEFAULT_TICKS_PER_SLOT: Final[int] = sd.DEFAULT_TICKS_PER_SLOT
""""""
FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET: Final[
    int
] = sd.FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET
"""Transaction forwarding, which leader to forward to"""
GENESIS_EPOCH: Final[int] = sd.GENESIS_EPOCH
""""""
HOLD_TRANSACTIONS_SLOT_OFFSET: Final[int] = sd.HOLD_TRANSACTIONS_SLOT_OFFSET
"""Transaction forwarding, how long to hold"""
INITIAL_RENT_EPOCH: Final[int] = sd.INITIAL_RENT_EPOCH
""""""
MAX_HASH_AGE_IN_SECONDS: Final[int] = sd.MAX_HASH_AGE_IN_SECONDS
"""The window of recent blockhashes over which the bank will track signatures."""
MAX_PROCESSING_AGE: Final[int] = sd.MAX_PROCESSING_AGE
"""The maximum age of a blockhash that will be accepted by the leader"""
MAX_RECENT_BLOCKHASHES: Final[int] = sd.MAX_RECENT_BLOCKHASHES
"""Max number of recent blockhashes (one blockhash per non-skipped slot)"""
MAX_TRANSACTION_FORWARDING_DELAY: Final[int] = sd.MAX_TRANSACTION_FORWARDING_DELAY
""""""
MAX_TRANSACTION_FORWARDING_DELAY_GPU: Final[
    int
] = sd.MAX_TRANSACTION_FORWARDING_DELAY_GPU
""""""
MS_PER_TICK: Final[int] = sd.MS_PER_TICK
"""The number of milliseconds per tick."""
NUM_CONSECUTIVE_LEADER_SLOTS: Final[int] = sd.NUM_CONSECUTIVE_LEADER_SLOTS
""""""
SECONDS_PER_DAY: Final[int] = sd.SECONDS_PER_DAY
""""""
TICKS_PER_DAY: Final[int] = sd.TICKS_PER_DAY
""""""

__all__ = [
    "Clock",
    "DEFAULT_DEV_SLOTS_PER_EPOCH",
    "DEFAULT_HASHES_PER_SECOND",
    "DEFAULT_HASHES_PER_TICK",
    "DEFAULT_MS_PER_SLOT",
    "DEFAULT_SLOTS_PER_EPOCH",
    "DEFAULT_S_PER_SLOT",
    "DEFAULT_TICKS_PER_SECOND",
    "DEFAULT_TICKS_PER_SLOT",
    "FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET",
    "GENESIS_EPOCH",
    "HOLD_TRANSACTIONS_SLOT_OFFSET",
    "INITIAL_RENT_EPOCH",
    "MAX_HASH_AGE_IN_SECONDS",
    "MAX_PROCESSING_AGE",
    "MAX_RECENT_BLOCKHASHES",
    "MAX_TRANSACTION_FORWARDING_DELAY",
    "MAX_TRANSACTION_FORWARDING_DELAY_GPU",
    "MS_PER_TICK",
    "NUM_CONSECUTIVE_LEADER_SLOTS",
    "SECONDS_PER_DAY",
    "TICKS_PER_DAY",
]
