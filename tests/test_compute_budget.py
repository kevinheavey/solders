from solders.compute_budget import (
    request_heap_frame,
    set_compute_unit_limit,
    set_compute_unit_price,
    set_loaded_accounts_data_size_limit,
)
from solders.instruction import Instruction


def test_compute_budget() -> None:
    assert isinstance(request_heap_frame(2048), Instruction)
    assert isinstance(set_compute_unit_limit(1_000_000), Instruction)
    assert isinstance(set_compute_unit_price(1000), Instruction)
    assert isinstance((set_loaded_accounts_data_size_limit(2**26)), Instruction)
