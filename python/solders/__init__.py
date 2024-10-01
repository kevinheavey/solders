import contextlib as __ctxlib

from solders.solders import __version__ as _version_untyped  # type: ignore
from solders.solders import (
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
    signature,
    token,
    transaction,
)

from . import system_program, sysvar

__has_bankrun = False
with __ctxlib.suppress(ImportError):
    from solders.solders import bankrun

    __has_bankrun = True

__has_ring = False
with __ctxlib.suppress(ImportError):
    from solders.solders import account, account_decoder, rpc, transaction_status

    __has_ring = True


__ring_modules = ["account", "account_decoder", "rpc", "transaction_status"]

__all_core = [
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
    "signature",
    "token",
    "transaction",
    "system_program",
    "sysvar",
]

__with_ring_modules = [*__all_core, *__ring_modules]

if __has_ring:
    if __has_bankrun:
        __all__ = [*__with_ring_modules, "bankrun"]  # noqa: PLE0604
    else:
        __all__ = __with_ring_modules  # noqa: PLE0605
else:
    __all__ = __all_core  # noqa: PLE0605


__version__: str = _version_untyped
