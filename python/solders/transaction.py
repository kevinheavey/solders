from typing import Union

from .solders import (
    Keypair,
    Legacy,
    NullSigner,
    Presigner,
    SanitizeError,
    Transaction,
    TransactionError,
    VersionedTransaction,
)

Signer = Union[Keypair, Presigner, NullSigner]
TransactionVersion = Union[Legacy, int]

__all__ = [
    "Legacy",
    "Transaction",
    "VersionedTransaction",
    "SanitizeError",
    "TransactionError",
    "Signer",
    "TransactionVersion",
]
