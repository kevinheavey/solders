from solders.internal import Keypair, Presigner, NullSigner, Legacy
from typing import Union

Signer = Union[Keypair, Presigner, NullSigner]
TransactionVersion = Union[Legacy, int]
