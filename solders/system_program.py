from typing import TypedDict, cast
from solders.pubkey import Pubkey
from solders.instruction import Instruction
from solders._system_program import (
    create_account as _create_account,
    decode_create_account as _decode_create_account,
    create_account_with_seed,
    assign,
    assign_with_seed,
    transfer,
    transfer_with_seed,
    allocate,
    allocate_with_seed,
    transfer_many,
    create_nonce_account,
    create_nonce_account_with_seed,
    advance_nonce_account,
    withdraw_nonce_account,
    ID as ID,
)


class CreateAccountParams(TypedDict):
    from_pubkey: Pubkey
    to_pubkey: Pubkey
    lamports: int
    space: int
    owner: Pubkey


def create_account(params: CreateAccountParams) -> Instruction:
    return _create_account(dict(params))


def decode_create_account(instruction: Instruction) -> CreateAccountParams:
    return cast(CreateAccountParams, _decode_create_account(instruction))
