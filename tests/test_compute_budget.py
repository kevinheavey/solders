from solders.compute_budget import (
    ComputeBudget,
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


def test_compute_budget_v4_cost_fields() -> None:
    """The alt_bn128 g2 and bls12_381 cost fields added in compute-budget v4."""
    cb = ComputeBudget(False)
    fields = [
        "alt_bn128_g2_addition_cost",
        "alt_bn128_g2_multiplication_cost",
        "bls12_381_g1_add_cost",
        "bls12_381_g2_add_cost",
        "bls12_381_g1_subtract_cost",
        "bls12_381_g2_subtract_cost",
        "bls12_381_g1_multiply_cost",
        "bls12_381_g2_multiply_cost",
        "bls12_381_g1_decompress_cost",
        "bls12_381_g2_decompress_cost",
        "bls12_381_g1_validate_cost",
        "bls12_381_g2_validate_cost",
        "bls12_381_one_pair_cost",
        "bls12_381_additional_pair_cost",
    ]
    for i, name in enumerate(fields):
        setattr(cb, name, i)
        assert getattr(cb, name) == i
