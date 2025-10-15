from typing import cast, Tuple, Optional, List
from typing_extensions import Final, TypedDict

from .solders import SYSTEM_PROGRAM_ID as _ID
from .solders import advance_nonce_account as _advance_nonce_account
from .solders import allocate as _allocate
from .solders import allocate_with_seed as _allocate_with_seed
from .solders import assign as _assign
from .solders import assign_with_seed as _assign_with_seed
from .solders import authorize_nonce_account as _authorize_nonce_account
from .solders import create_account as _create_account
from .solders import (
    create_account_with_seed as _create_account_with_seed,
)
from .solders import (
    create_nonce_account,
    create_nonce_account_with_seed,
    transfer_many,
)
from .solders import (
    decode_advance_nonce_account as _decode_advance_nonce_account,
)
from .solders import decode_allocate as _decode_allocate
from .solders import (
    decode_allocate_with_seed as _decode_allocate_with_seed,
)
from .solders import decode_assign as _decode_assign
from .solders import decode_assign_with_seed as _decode_assign_with_seed
from .solders import (
    decode_authorize_nonce_account as _decode_authorize_nonce_account,
)
from .solders import decode_create_account as _decode_create_account
from .solders import (
    decode_create_account_with_seed as _decode_create_account_with_seed,
)
from .solders import (
    decode_initialize_nonce_account as _decode_initialize_nonce_account,
)
from .solders import decode_transfer as _decode_transfer
from .solders import (
    decode_transfer_with_seed as _decode_transfer_with_seed,
)
from .solders import (
    decode_withdraw_nonce_account as _decode_withdraw_nonce_account,
)
from .solders import (
    initialize_nonce_account as _initialize_nonce_account,
)
from .solders import (
    close_lookup_table as _close_lookup_table,
)
from .solders import (
    create_lookup_table as _create_lookup_table,
)
from .solders import (
    deactivate_lookup_table as _deactivate_lookup_table,
)
from .solders import (
    extend_lookup_table as _extend_lookup_table,
)
from .solders import (
    freeze_lookup_table as _freeze_lookup_table,
)
from .solders import transfer as _transfer
from .solders import transfer_with_seed as _transfer_with_seed
from .solders import withdraw_nonce_account as _withdraw_nonce_account
from solders.instruction import Instruction
from solders.pubkey import Pubkey

ID: Final[Pubkey] = _ID
"""Pubkey that identifies the System program."""


class CreateAccountParams(TypedDict):
    """Create account system transaction params."""

    from_pubkey: Pubkey
    """The account that will transfer lamports to the created account."""
    to_pubkey: Pubkey
    """Pubkey of the created account."""
    lamports: int
    """Amount of lamports to transfer to the created account."""
    space: int
    """Amount of space in bytes to allocate to the created account."""
    owner: Pubkey
    """Pubkey of the program to assign as the owner of the created account."""


def create_account(params: CreateAccountParams) -> Instruction:
    """Generate an instruction that creates a new account.

    Args:
        params: The CreateAccount params.

    Example:
        >>> from solders.pubkey import Pubkey
        >>> from solders.system_program import create_account, CreateAccountParams
        >>> from_account = Pubkey.new_unique()
        >>> new_account = Pubkey.new_unique()
        >>> program_id = Pubkey.new_unique()
        >>> instruction = create_account(
        ...     CreateAccountParams(
        ...         from_pubkey=from_account, to_pubkey=new_account,
        ...         lamports=1, space=1, owner=program_id)
        ... )
        >>> type(instruction)
        <class 'solders.instruction.Instruction'>
    Returns:
        Instruction: The instruction to create the account.
    """
    return _create_account(dict(params))


def decode_create_account(instruction: Instruction) -> CreateAccountParams:
    """Decode a create account instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The CreateAccount instruction.

    Returns:
        CreateAccountParams: The params used to create the instruction.
    """
    return cast(CreateAccountParams, _decode_create_account(instruction))


class CreateAccountWithSeedParams(TypedDict):
    """Create account with seed system transaction params."""

    from_pubkey: Pubkey
    """The account that will transfer lamports to the created account."""
    to_pubkey: Pubkey
    """Pubkey of the created account.
    Must be pre-calculated with :meth:`~solders.pubkey.Pubkey.create_with_seed`."""
    base: Pubkey
    """Base public key to use to derive the address of the created account.
    Must be the same as the base key used to create ``to_pubkey``."""
    seed: str
    """Seed to use to derive the address of the created account.
    Must be the same as the seed used to create ``to_pubkey``."""
    lamports: int
    """Amount of lamports to transfer to the created account."""
    space: int
    """Amount of space in bytes to allocate to the created account."""
    owner: Pubkey
    """Pubkey of the program to assign as the owner of the created account."""


def create_account_with_seed(params: CreateAccountWithSeedParams) -> Instruction:
    """Generate an instruction that creates a new account at an address generated with ``from``, a seed, and program_id.

    Args:
        params (CreateAccountWithSeedParams): The CreateAccountWithSeed params.

    Returns:
        Instruction: The instruction to create the account.
    """  # noqa: E501
    return _create_account_with_seed(dict(params))


def decode_create_account_with_seed(
    instruction: Instruction,
) -> CreateAccountWithSeedParams:
    """Decode create account with seed instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The CreateAccountWithSeed instruction.

    Returns:
        CreateAccountWithSeedParams: The params used to create the instruction.
    """
    return cast(
        CreateAccountWithSeedParams, _decode_create_account_with_seed(instruction)
    )


class AssignParams(TypedDict):
    """Assign system transaction params."""

    pubkey: Pubkey
    """Pubkey of the account which will be assigned a new owner."""
    owner: Pubkey
    """Pubkey of the program to assign as the owner."""


def assign(params: AssignParams) -> Instruction:
    """Generate an instruction that assigns an account to a program.

    Args:
        params (AssignParams): The assign params.

    Returns:
        Instruction: The generated instruction.

    Example:
        >>> from solders.pubkey import Pubkey
        >>> from solders.system_program import assign, AssignParams
        >>> account, program_id = Pubkey.default(), Pubkey.default()
        >>> instruction = assign(
        ...     AssignParams(pubkey=account, owner=program_id)
        ... )
        >>> type(instruction)
        <class 'solders.instruction.Instruction'>
    """
    return _assign(dict(params))


def decode_assign(instruction: Instruction) -> AssignParams:
    """Decode an assign instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The Assign instruction.

    Returns:
        AssignParams: The params used to create the instruction.
    """
    return cast(AssignParams, _decode_assign(instruction))


class AssignWithSeedParams(TypedDict):
    """Assign account with seed system transaction params."""

    address: Pubkey
    """Pubkey of the account which will be assigned a new owner."""
    base: Pubkey
    """Base public key to use to derive the address of the assigned account."""
    seed: str
    """Seed to use to derive the address of the assigned account."""
    owner: Pubkey
    """Pubkey of the program to assign as the owner."""


def assign_with_seed(params: AssignWithSeedParams) -> Instruction:
    """Generate an instruction that assigns an account to a program.

    Args:
        params (AssignWithSeedParams): The AssignWithSeed params.

    Returns:
        Instruction: The generated instruction.
    """
    return _assign_with_seed(dict(params))


def decode_assign_with_seed(instruction: Instruction) -> AssignWithSeedParams:
    """Decode an assign with seed instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The AssignWithSeed instruction.

    Returns:
        AssignWithSeedParams: The params used to create the instruction.
    """
    return cast(AssignWithSeedParams, _decode_assign_with_seed(instruction))


class TransferParams(TypedDict):
    """Transfer system transaction params."""

    from_pubkey: Pubkey
    """Account that will transfer lamports."""
    to_pubkey: Pubkey
    """Account that will receive transferred lamports."""
    lamports: int
    """Amount of lamports to transfer."""


def transfer(params: TransferParams) -> Instruction:
    """Generate an instruction that transfers lamports from one account to another.

    Args:
        params: The transfer params.

    Example:
        >>> from solders.pubkey import Pubkey
        >>> from solders.system_program import transfer, TransferParams
        >>> sender, receiver = Pubkey.default(), Pubkey.default()
        >>> instruction = transfer(
        ...     TransferParams(from_pubkey=sender, to_pubkey=receiver, lamports=1000)
        ... )
        >>> type(instruction)
        <class 'solders.instruction.Instruction'>

    Returns:
        Instruction: The transfer instruction.
    """
    return _transfer(dict(params))


def decode_transfer(instruction: Instruction) -> TransferParams:
    """Decode a transfer instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The Transfer instruction.

    Returns:
        TransferParams: The params used to create the instruction.
    """
    return cast(TransferParams, _decode_transfer(instruction))


class TransferWithSeedParams(TypedDict):
    """Transfer with seed system transaction params."""

    from_pubkey: Pubkey
    """Account that will transfer lamports."""
    from_base: Pubkey
    """Base public key to use to derive the funding account address."""
    from_seed: str
    """Seed to use to derive the funding account address."""
    from_owner: Pubkey
    """Program id to use to derive the funding account address."""
    to_pubkey: Pubkey
    """Account that will receive transferred lamports."""
    lamports: int
    """Amount of lamports to transfer."""


def transfer_with_seed(params: TransferWithSeedParams) -> Instruction:
    """Generate an instruction that transfers lamports from one account to another.

    Args:
        params (TransferWithSeedParams): The TransferWithSeed params.

    Returns:
        Instruction: The TransferWithSeed instruction.
    """
    return _transfer_with_seed(dict(params))


def decode_transfer_with_seed(instruction: Instruction) -> TransferWithSeedParams:
    """Decode a transfer with seed instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The TransferWithSeed instruction.

    Returns:
        TransferWithSeedParams: The params used to create the instruction.
    """
    return cast(TransferWithSeedParams, _decode_transfer_with_seed(instruction))


class AllocateParams(TypedDict):
    """Allocate account system transaction params."""

    pubkey: Pubkey
    """Account to allocate."""
    space: int
    """Amount of space in bytes to allocate."""


def allocate(params: AllocateParams) -> Instruction:
    """Generate an instruction that allocates space in an account without funding.

    Args:
        params (AllocateParams): The allocate params.

    Returns:
        Instruction: The allocate instruction.

    Example:
        >>> from solders.pubkey import Pubkey
        >>> from solders.system_program import allocate, AllocateParams
        >>> allocator = Pubkey.default()
        >>> instruction = allocate(
        ...     AllocateParams(pubkey=allocator, space=65537)
        ... )
        >>> type(instruction)
        <class 'solders.instruction.Instruction'>

    """
    return _allocate(dict(params))


def decode_allocate(instruction: Instruction) -> AllocateParams:
    """Decode an allocate instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The Allocate instruction.

    Returns:
        AllocateParams: The params used to create the instruction.
    """
    return cast(AllocateParams, _decode_allocate(instruction))


class AllocateWithSeedParams(TypedDict):
    """Allocate account with seed system transaction params."""

    address: Pubkey
    """Account to allocate."""
    base: Pubkey
    """Base public key to use to derive the address of the allocated account."""
    seed: str
    """Seed to use to derive the address of the allocated account."""
    space: int
    """Amount of space in bytes to allocate."""
    owner: Pubkey
    """Pubkey of the program to assign as the owner of the allocated account."""


def allocate_with_seed(params: AllocateWithSeedParams) -> Instruction:
    """Generate an instruction that allocates space in an account without funding.

    Args:
        params (AllocateWithSeedParams): The AllocateWithSeed params.

    Returns:
        Instruction: The AllocateWithSeed instruction.
    """
    return _allocate_with_seed(dict(params))


def decode_allocate_with_seed(instruction: Instruction) -> AllocateWithSeedParams:
    """Decode an allocate with seed instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The AllocateWithSeed instruction.

    Returns:
        AllocateWithSeedParams: The params used to create the instruction.
    """
    return cast(AllocateWithSeedParams, _decode_allocate_with_seed(instruction))


class InitializeNonceAccountParams(TypedDict):
    """Initialize nonce account system instruction params."""

    nonce_pubkey: Pubkey
    """Nonce account which will be initialized."""
    authority: Pubkey
    """Pubkey to set as authority of the initialized nonce account."""


def initialize_nonce_account(params: InitializeNonceAccountParams) -> Instruction:
    """Generate an instruction to initialize a Nonce account.

    Args:
        params (InitializeNonceAccountParams): The InitializeNonceAccount params.

    Returns:
        Instruction: The InitializeNonceAccount instruction.
    """
    return _initialize_nonce_account(dict(params))


def decode_initialize_nonce_account(
    instruction: Instruction,
) -> InitializeNonceAccountParams:
    """Decode initialize nonce account instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The InitializeNonceAccount instruction.

    Returns:
        InitializeNonceAccountParams: The params used to create the instruction.
    """
    return cast(
        InitializeNonceAccountParams, _decode_initialize_nonce_account(instruction)
    )


class AdvanceNonceAccountParams(TypedDict):
    """Advance nonce account system instruction params."""

    nonce_pubkey: Pubkey
    """Nonce account."""
    authorized_pubkey: Pubkey
    """Pubkey of the nonce authority."""


def advance_nonce_account(params: AdvanceNonceAccountParams) -> Instruction:
    """Generate an instruction to advance the nonce in a Nonce account.

    Args:
        params (AdvanceNonceAccountParams): The AdvanceNonceAccount params.

    Returns:
        Instruction: The AdvanceNonceAccount instruction.
    """
    return _advance_nonce_account(dict(params))


def decode_advance_nonce_account(instruction: Instruction) -> AdvanceNonceAccountParams:
    """Decode an advance nonce account instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The AdvanceNonceAccount instruction.

    Returns:
        AdvanceNonceAccountParams: The params used to create the instruction.
    """
    return cast(AdvanceNonceAccountParams, _decode_advance_nonce_account(instruction))


class WithdrawNonceAccountParams(TypedDict):
    """Withdraw nonce account system transaction params."""

    nonce_pubkey: Pubkey
    """Nonce account."""
    authorized_pubkey: Pubkey
    """Pubkey of the nonce authority."""
    to_pubkey: Pubkey
    """Pubkey of the account which will receive the withdrawn nonce account balance."""
    lamports: int
    """Amount of lamports to withdraw from the nonce account."""


def withdraw_nonce_account(params: WithdrawNonceAccountParams) -> Instruction:
    """Generate an instruction that withdraws lamports from a Nonce account.

    Args:
        params (WithdrawNonceAccountParams): The WithdrawNonceAccount params.

    Returns:
        Instruction: The WithdrawNonceAccount instruction.
    """
    return _withdraw_nonce_account(dict(params))


def decode_withdraw_nonce_account(
    instruction: Instruction,
) -> WithdrawNonceAccountParams:
    """Decode a withdraw nonce account instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The WithdrawNonceAccount instruction.

    Returns:
        WithdrawNonceAccountParams: The params used to create the instruction.
    """
    return cast(WithdrawNonceAccountParams, _decode_withdraw_nonce_account(instruction))


class AuthorizeNonceAccountParams(TypedDict):
    """Authorize nonce account system transaction params."""

    nonce_pubkey: Pubkey
    """Nonce account."""
    authorized_pubkey: Pubkey
    """Pubkey of the current nonce authority."""
    new_authority: Pubkey
    """Pubkey of the new nonce authority."""


def authorize_nonce_account(params: AuthorizeNonceAccountParams) -> Instruction:
    """Generate an instruction that authorizes a new Pubkey as the nonce authority.

    Args:
        params (AuthorizeNonceAccountParams): The AuthorizeNonceAccount params.

    Returns:
        Instruction: The AuthorizeNonceAccount instruction.
    """
    return _authorize_nonce_account(dict(params))


def decode_authorize_nonce_account(
    instruction: Instruction,
) -> AuthorizeNonceAccountParams:
    """Decode authorize nonce account instruction and retrieve the instruction params.

    Args:
        instruction (Instruction): The AuthorizeNonceAccount instruction.

    Returns:
        AuthorizeNonceAccountParams: The params used to create the instruction.
    """
    return cast(
        AuthorizeNonceAccountParams, _decode_authorize_nonce_account(instruction)
    )


class CloseLookupTableParams(TypedDict):
    """Close lookup table system transaction params."""

    lookup_table_address: Pubkey
    authority_address: Pubkey
    recipient_address: Pubkey


def close_lookup_table(params: CloseLookupTableParams) -> Instruction:
    """Returns an instruction that closes an address lookup table account.

    The account will be deallocated and the lamports will be drained
    to the recipient address.

    Args:
        params (CloseLookupTableParams): The CloseLookupTable params.

    Returns:
        Instruction: The CloseLookupTable instruction.
    """
    return _close_lookup_table(dict(params))


class CreateLookupTableParams(TypedDict):
    """Create lookup table system transaction params."""

    authority_address: Pubkey
    payer_address: Pubkey
    recent_slot: int


def create_lookup_table(params: CreateLookupTableParams) -> Tuple[Instruction, Pubkey]:
    """Constructs an instruction to create a table account.

    Returns the instruction and the table account`s derived address.

    Args:
        params (CreateLookupTableParams): The CreateLookupTable params.

    Returns:
        Tuple[Instruction, PubKey]: The CreateLookupTable instruction
        and the table account`s derived address
    """
    return _create_lookup_table(dict(params))


class CreateLookupTableSignedParams(TypedDict):
    """Create lookup table signed system transaction params."""

    authority_address: Pubkey
    payer_address: Pubkey
    recent_slot: int


class DeactivateLookupTableParams(TypedDict):
    """Deactivate lookup table system transaction params."""

    lookup_table_address: Pubkey
    authority_address: Pubkey


def deactivate_lookup_table(params: DeactivateLookupTableParams) -> Instruction:
    """Constructs an instruction that deactivates an address lookup table.

    So that it cannot be extended again and will be unusable and eligible
    for closure after a short amount of time.

    Args:
        params (DeactivateLookupTableParams): The DeactivateLookupTable params.

    Returns:
        Instruction: The DeactivateLookupTable instruction.

    """
    return _deactivate_lookup_table(dict(params))


class ExtendLookupTableParams(TypedDict):
    """Extend lookup table system transaction params."""

    payer_address: Optional[Pubkey]
    lookup_table_address: Pubkey
    authority_address: Pubkey
    new_addresses: List[Pubkey]


def extend_lookup_table(params: ExtendLookupTableParams) -> Instruction:
    """Constructs an instruction which extends an ATL account with new addresses.

    Args:
        params (ExtendLookupTableParams): the ExtendLookupTable params.

    Returns:
        Instruction: The ExtendLookupTable instruction.
    """
    return _extend_lookup_table(dict(params))


class FreezeLookupTableParams(TypedDict):
    """Freeze lookup table system transaction params."""

    lookup_table_address: Pubkey
    authority_address: Pubkey


def freeze_lookup_table(params: FreezeLookupTableParams) -> Instruction:
    """Constructs an instruction that freezes an address lookup table.

    So that it can never be closed or extended again.
    Empty lookup tables cannot be frozen.

    Args:
        params (FreezeLookupTableParams): The FreezeLookupTable params.

    Returns:
        Instruction: The FreezeLookupTable instruction.
    """
    return _freeze_lookup_table(dict(params))


__all__ = [
    "ID",
    "transfer_many",
    "create_nonce_account",
    "create_nonce_account_with_seed",
    "CreateAccountParams",
    "create_account",
    "decode_create_account",
    "CreateAccountWithSeedParams",
    "create_account_with_seed",
    "decode_create_account_with_seed",
    "AssignParams",
    "assign",
    "decode_assign",
    "AssignWithSeedParams",
    "assign_with_seed",
    "decode_assign_with_seed",
    "TransferParams",
    "transfer",
    "decode_transfer",
    "TransferWithSeedParams",
    "transfer_with_seed",
    "decode_transfer_with_seed",
    "AllocateParams",
    "allocate",
    "decode_allocate",
    "AllocateWithSeedParams",
    "allocate_with_seed",
    "decode_allocate_with_seed",
    "InitializeNonceAccountParams",
    "initialize_nonce_account",
    "decode_initialize_nonce_account",
    "AdvanceNonceAccountParams",
    "advance_nonce_account",
    "decode_advance_nonce_account",
    "WithdrawNonceAccountParams",
    "withdraw_nonce_account",
    "decode_withdraw_nonce_account",
    "AuthorizeNonceAccountParams",
    "authorize_nonce_account",
    "decode_authorize_nonce_account",
    "CloseLookupTableParams",
    "close_lookup_table",
    "CreateLookupTableParams",
    "create_lookup_table",
    "CreateLookupTableSignedParams",
    "DeactivateLookupTableParams",
    "deactivate_lookup_table",
    "ExtendLookupTableParams",
    "extend_lookup_table",
    "FreezeLookupTableParams",
    "freeze_lookup_table",
]
