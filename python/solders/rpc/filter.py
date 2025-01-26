from typing import Union

from ..solders import Memcmp, RpcFilterTypeFieldless

RpcFilterType = Union[int, Memcmp, RpcFilterTypeFieldless]

__all__ = [
    "Memcmp",
    "RpcFilterTypeFieldless",
    "RpcFilterType",
]
