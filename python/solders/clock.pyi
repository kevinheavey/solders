DEFAULT_DEV_SLOTS_PER_EPOCH: int
DEFAULT_HASHES_PER_SECOND: int
DEFAULT_HASHES_PER_TICK: int
DEFAULT_MS_PER_SLOT: int
DEFAULT_SLOTS_PER_EPOCH: int
DEFAULT_S_PER_SLOT: float
DEFAULT_TICKS_PER_SECOND: int
DEFAULT_TICKS_PER_SLOT: int
FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET: int
GENESIS_EPOCH: int
HOLD_TRANSACTIONS_SLOT_OFFSET: int
INITIAL_RENT_EPOCH: int
MAX_HASH_AGE_IN_SECONDS: int
MAX_PROCESSING_AGE: int
MAX_RECENT_BLOCKHASHES: int
MAX_TRANSACTION_FORWARDING_DELAY: int
MAX_TRANSACTION_FORWARDING_DELAY_GPU: int
MS_PER_TICK: int
NUM_CONSECUTIVE_LEADER_SLOTS: int
SECONDS_PER_DAY: int
TICKS_PER_DAY: int

class Clock:
    def __init__(
        self,
        slot: int,
        epoch_start_timestamp: int,
        epoch: int,
        leader_schedule_epoch: int,
        unix_timestamp: int,
    ) -> None: ...
    @property
    def epoch(self) -> int: ...
    @property
    def epoch_start_timestamp(self) -> int: ...
    @property
    def slot(self) -> int: ...
    @property
    def leader_schedule_epoch(self) -> int: ...
    @property
    def unix_timestamp(self) -> int: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "Clock": ...
    @staticmethod
    def from_json(raw: str) -> "Clock": ...
    def to_json(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "Clock", op: int) -> bool: ...
