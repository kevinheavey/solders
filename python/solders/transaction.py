from .solders import (
    Keypair,
    Presigner,
    NullSigner,
    Legacy,
    Transaction,
    VersionedTransaction,
    SanitizeError,
    TransactionError,
)
from typing import Union

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
