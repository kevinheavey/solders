import contextlib as __ctxlib

from solders.solders import __version__ as _version_untyped  # type: ignore
from solders.solders import (
    account,
    account_decoder,
    address_lookup_table_account,
    clock,
    commitment_config,
    compute_budget,
    epoch_info,
    epoch_schedule,
    errors,
    hash,
    instruction,
    keypair,
    message,
    null_signer,
    presigner,
    pubkey,
    rent,
    rpc,
    signature,
    token,
    transaction,
    transaction_status,
)

from . import system_program, sysvar

__has_bankrun = False
with __ctxlib.suppress(ImportError):
    from solders.solders import bankrun

    __has_bankrun = True


__all_core = [
    "account_decoder",
    "address_lookup_table_account",
    "commitment_config",
    "errors",
    "hash",
    "instruction",
    "keypair",
    "message",
    "null_signer",
    "presigner",
    "pubkey",
    "rpc",
    "signature",
    "token",
    "transaction",
    "transaction_status",
    "sysvar",
    "system_program",
]

__all__ = [*__all_core, "bankrun"] if __has_bankrun else __all_core

__version__: str = _version_untyped
