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
from solders.rpc.responses import (
    GetBalanceResp,
    GetBlockHeightResp,
    GetTransactionResp,
    RpcResponseContext,
)
from solders.signature import Signature
from solders.transaction_status import EncodedConfirmedTransactionWithStatusMeta

_TX_RESP = GetTransactionResp.from_json(
    '{"jsonrpc":"2.0","result":{"slot":430,"transaction":{"message":{"accountKeys":'
    '["3UVYmECPPMZSCqWKfENfuoTv51fTDTWicX9xmBD2euKe"],"header":'
    '{"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":0,'
    '"numRequiredSignatures":1},"instructions":[],"recentBlockhash":'
    '"11111111111111111111111111111111"},"signatures":["5Ve"]},"meta":null,'
    '"blockTime":null},"id":1}'
)
assert isinstance(_TX_RESP, GetTransactionResp)
_ENCODED_TX_RESP = _TX_RESP.value
assert isinstance(_ENCODED_TX_RESP, EncodedConfirmedTransactionWithStatusMeta)

# A spread across the macro families: byte wrappers and complex types
# (common_methods), a primitive, a request (common_methods_ser_only), and a
# type with a #[serde(flatten)] field (serialized via CBOR, not bincode).
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
    _ENCODED_TX_RESP,
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
