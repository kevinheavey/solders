"""Synthetic accounts that allow programs to access certain network states."""
from typing_extensions import Final
from solders.pubkey import Pubkey
from solders import _sysvar as sv

CLOCK: Final[Pubkey] = sv.CLOCK
"""Pubkey of the synthetic account that serves the current network time."""

RECENT_BLOCKHASHES: Final[Pubkey] = sv.RECENT_BLOCKHASHES
"""Pubkey of the synthetic account that serves recent blockhashes."""

RENT: Final[Pubkey] = sv.RENT
"""Pubkey of the synthetic account that serves the network fee resource consumption."""

REWARDS: Final[Pubkey] = sv.REWARDS
"""Pubkey of the synthetic account that serves the network rewards."""

STAKE_HISTORY: Final[Pubkey] = sv.STAKE_HISTORY
"""Pubkey of the synthetic account that serves the stake history."""

EPOCH_SCHEDULE: Final[Pubkey] = sv.EPOCH_SCHEDULE
"""The EpochSchedule sysvar contains epoch scheduling constants that are set in genesis,
and enables calculating the number of slots in a given epoch,
the epoch for a given slot, etc.
(Note: the epoch schedule is distinct from the
`leader schedule <https://docs.solana.com/terminology#leader-schedule>`_).
"""

INSTRUCTIONS: Final[Pubkey] = sv.INSTRUCTIONS
"""
The Instructions sysvar contains the serialized instructions in a
Message while that Message is being processed.
This allows program instructions to reference other instructions
in the same transaction.
Read more information on `instruction introspection
<https://docs.solana.com/implemented-proposals/instruction_introspection>`_.
"""

SLOT_HASHES: Final[Pubkey] = sv.SLOT_HASHES
"""The SlotHashes sysvar contains the most recent hashes of the slot's parent banks.
It is updated every slot."""
