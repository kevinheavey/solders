import copy
import pickle
from typing import Any, List

import pytest
from solders.account import Account
from solders.hash import Hash
from solders.instruction import Instruction
from solders.keypair import Keypair
from solders.message import Message
from solders.pubkey import Pubkey
from solders.rent import Rent
from solders.rpc.requests import GetBalance
from solders.rpc.responses import GetBalanceResp, GetBlockHeightResp, RpcResponseContext
from solders.signature import Signature

# A spread across the macro families: byte wrappers and complex types
# (common_methods), a primitive, and a request (common_methods_ser_only).
objects: List[Any] = [
    Pubkey.new_unique(),
    Hash.default(),
    Signature.default(),
    Keypair(),
    Account.default(),
    Message.default(),
    Instruction(Pubkey.default(), b"data", []),
    Rent.default(),
    GetBalance(Pubkey.default()),
]


@pytest.mark.parametrize("original", objects)
def test_pickle(original: Any) -> None:
    assert pickle.loads(pickle.dumps(original)) == original


@pytest.mark.parametrize("original", objects)
def test_deepcopy(original: Any) -> None:
    copied = copy.deepcopy(original)
    assert copied == original
    assert copied is not original


# RPC response types support deepcopy (via clone) but not pickle, since their
# bincode round-trip is broken by skip_serializing_if.
response_objects: List[Any] = [
    GetBlockHeightResp(1234),
    GetBalanceResp(5, RpcResponseContext(slot=1)),
]


@pytest.mark.parametrize("original", response_objects)
def test_deepcopy_responses(original: Any) -> None:
    copied = copy.deepcopy(original)
    assert copied == original
    assert copied is not original
