from typing import TypedDict, cast
from solders.pubkey import Pubkey
from solders.instruction import Instruction
from solders._system_program import (
    create_account as _create_account,
    decode_create_account as _decode_create_account,
    create_account_with_seed as _create_account_with_seed,
    decode_create_account_with_seed as _decode_create_account_with_seed,
    assign as _assign,
    decode_assign as _decode_assign,
    assign_with_seed as _assign_with_seed,
    decode_assign_with_seed as _decode_assign_with_seed,
    transfer as _transfer,
    decode_transfer as _decode_transfer,
    transfer_with_seed as _transfer_with_seed,
    decode_transfer_with_seed as _decode_transfer_with_seed,
    allocate as _allocate,
    decode_allocate as _decode_allocate,
    allocate_with_seed as _allocate_with_seed,
    decode_allocate_with_seed as _decode_allocate_with_seed,
    transfer_many as transfer_many,
    create_nonce_account as create_nonce_account,
    create_nonce_account_with_seed as create_nonce_account_with_seed,
    initialize_nonce_account as _initialize_nonce_account,
    decode_initialize_nonce_account as _decode_initialize_nonce_account,
    advance_nonce_account as _advance_nonce_account,
    decode_advance_nonce_account as _decode_advance_nonce_account,
    withdraw_nonce_account as _withdraw_nonce_account,
    decode_withdraw_nonce_account as _decode_withdraw_nonce_account,
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


class CreateAccountWithSeedParams(TypedDict):
    from_pubkey: Pubkey
    to_pubkey: Pubkey
    base: Pubkey
    seed: str
    lamports: int
    space: int
    owner: Pubkey


def create_account_with_seed(params: CreateAccountWithSeedParams) -> Instruction:
    return _create_account_with_seed(dict(params))


def decode_create_account_with_seed(
    instruction: Instruction,
) -> CreateAccountWithSeedParams:
    return cast(
        CreateAccountWithSeedParams, _decode_create_account_with_seed(instruction)
    )


class AssignParams(TypedDict):
    pubkey: Pubkey
    owner: Pubkey


def assign(params: AssignParams) -> Instruction:
    return _assign(dict(params))


def decode_assign(instruction: Instruction) -> AssignParams:
    return cast(AssignParams, _decode_assign(instruction))


class AssignWithSeedParams(TypedDict):
    address: Pubkey
    base: Pubkey
    seed: str
    owner: Pubkey


def assign_with_seed(params: AssignWithSeedParams) -> Instruction:
    return _assign_with_seed(dict(params))


def decode_assign_with_seed(instruction: Instruction) -> AssignWithSeedParams:
    return cast(AssignWithSeedParams, _decode_create_account(instruction))


class TransferParams(TypedDict):
    from_pubkey: Pubkey
    to_pubkey: Pubkey
    lamports: int


def transfer(params: TransferParams) -> Instruction:
    return _transfer(dict(params))


def decode_transfer(instruction: Instruction) -> TransferParams:
    return cast(TransferParams, _decode_transfer(instruction))


class TransferWithSeedParams(TypedDict):
    from_pubkey: Pubkey
    from_base: Pubkey
    from_seed: str
    from_owner: Pubkey
    to_pubkey: Pubkey
    lamports: int


def transfer_with_seed(params: TransferWithSeedParams) -> Instruction:
    return _transfer_with_seed(dict(params))


def decode_transfer_with_seed(instruction: Instruction) -> TransferWithSeedParams:
    return cast(TransferWithSeedParams, _decode_transfer_with_seed(instruction))


class AllocateParams(TypedDict):
    pubkey: Pubkey
    space: int


def allocate(params: AllocateParams) -> Instruction:
    return _allocate(dict(params))


def decode_allocate(instruction: Instruction) -> AllocateParams:
    return cast(AllocateParams, _decode_allocate(instruction))


class AllocateWithSeedParams(TypedDict):
    address: Pubkey
    base: Pubkey
    seed: str
    space: int
    owner: Pubkey


def allocate_with_seed(params: AllocateWithSeedParams) -> Instruction:
    return _allocate_with_seed(dict(params))


def decode_allocate_with_seed(instruction: Instruction) -> AllocateWithSeedParams:
    return cast(AllocateWithSeedParams, _decode_allocate_with_seed(instruction))


class InitializeNonceAccountParams(TypedDict):
    nonce_pubkey: Pubkey
    authority: Pubkey


def initialize_nonce_account(params: InitializeNonceAccountParams) -> Instruction:
    return _initialize_nonce_account(dict(params))


def decode_initialize_nonce_account(
    instruction: Instruction,
) -> InitializeNonceAccountParams:
    return cast(
        InitializeNonceAccountParams, _decode_initialize_nonce_account(instruction)
    )


class AdvanceNonceAccountParams(TypedDict):
    nonce_pubkey: Pubkey
    authorized_pubkey: Pubkey


def advance_nonce_account(params: AdvanceNonceAccountParams) -> Instruction:
    return _advance_nonce_account(dict(params))


def decode_advance_nonce_account(instruction: Instruction) -> AdvanceNonceAccountParams:
    return cast(AdvanceNonceAccountParams, _decode_advance_nonce_account(instruction))


class WithdrawNonceAccountParams(TypedDict):
    nonce_pubkey: Pubkey
    authorized_pubkey: Pubkey
    to_pubkey: Pubkey
    lamports: int


def withdraw_nonce_account(params: WithdrawNonceAccountParams) -> Instruction:
    return _withdraw_nonce_account(dict(params))


def decode_withdraw_nonce_account(
    instruction: Instruction,
) -> WithdrawNonceAccountParams:
    return cast(WithdrawNonceAccountParams, _decode_withdraw_nonce_account(instruction))
