import copy
from typing import Any
import pytest
from solders.account import Account
from solders.pubkey import Pubkey
from solders.hash import Hash
from solders.signature import Signature
from solders.message import Message


objects_to_copy = [
    Account.default(),
    Pubkey.default(),
    Hash.default(),
    Signature.default(),
    Message.default(),
]


@pytest.mark.parametrize("original", objects_to_copy)
def test_deepcopy(original: Any) -> None:
    copied = copy.deepcopy(original)

    assert copied == original
    assert id(copied) != id(original)
