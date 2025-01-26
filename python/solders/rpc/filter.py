from ..solders import Memcmp, RpcFilterTypeFieldless
from typing import Union

RpcFilterType = Union[int, Memcmp, RpcFilterTypeFieldless]

__all__ = [
    "Memcmp",
    "RpcFilterTypeFieldless",
    "RpcFilterType",
]
