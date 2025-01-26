from .solders import (COMPUTE_BUDGET_ID as _ID, Pubkey, ComputeBudget, request_heap_frame,
set_compute_unit_limit,
set_compute_unit_price)
from typing import Final

ID: Final[Pubkey] = _ID
"""Compute buddget program ID"""

__all__ = ["ComputeBudget", "request_heap_frame", "set_compute_unit_limit", "set_compute_unit_price", "ID"]
