import contextlib as __ctxlib

from . import (
    account,
    account_decoder,
    address_lookup_table_account,
    clock,
    commitment_config,
    compute_budget,
    epoch_info,
    epoch_rewards,
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
    slot_history,
    stake_history,
    system_program,
    sysvar,
    token,
    transaction,
    transaction_status,
)

__has_litesvm = False
with __ctxlib.suppress(ImportError):
    from . import litesvm, transaction_metadata

    __has_litesvm = True

__has_ring = False
with __ctxlib.suppress(ImportError):
    from . import rpc

    __has_ring = True


__ring_modules = ["rpc"]

__all_core = [
    "account",
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
    "signature",
    "token",
    "transaction",
    "transaction_status",
    "system_program",
    "sysvar",
]

__with_ring_modules = [*__all_core, *__ring_modules]

if __has_ring:
    if __has_litesvm:
        __all__ = [*__with_ring_modules, "litesvm", "transaction_metadata"]
    else:
        __all__ = __with_ring_modules
else:
    __all__ = __all_core


__version__ = "0.26.0"
