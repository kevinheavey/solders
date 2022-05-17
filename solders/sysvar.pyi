from typing import ClassVar
from solders.pubkey import Pubkey

class Sysvar:
    CLOCK: ClassVar[Pubkey]
    RECENT_BLOCKHASHES: ClassVar[Pubkey]
    RENT: ClassVar[Pubkey]
    REWARDS: ClassVar[Pubkey]
    STAKE_HISTORY: ClassVar[Pubkey]
    EPOCH_SCHEDULE: ClassVar[Pubkey]
    INSTRUCTIONS: ClassVar[Pubkey]
    SLOT_HASHES: ClassVar[Pubkey]