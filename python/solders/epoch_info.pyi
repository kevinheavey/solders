from __future__ import annotations

from typing import Optional

class EpochInfo:
    epoch: int
    slot_index: int
    slots_in_epoch: int
    absolute_slot: int
    block_height: int
    transaction_count: Optional[int]
    def __init__(
        self,
        epoch: int,
        slot_index: int,
        slots_in_epoch: int,
        absolute_slot: int,
        block_height: int,
        transaction_count: Optional[int] = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> EpochInfo: ...
    @staticmethod
    def from_bytes(data: bytes) -> EpochInfo: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...
