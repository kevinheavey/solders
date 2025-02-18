"""These tests are mainly about getting mypy to check stuff, as it doesn't check doc examples."""

from typing import List, Union

from solders.pubkey import Pubkey
from solders.rpc.config import RpcSignatureStatusConfig
from solders.rpc.requests import (
    GetSignatureStatuses,
    RequestAirdrop,
    batch_to_json,
)
from solders.signature import Signature


def test_batch() -> None:
    reqs: List[Union[GetSignatureStatuses, RequestAirdrop]] = [
        GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True)),
        RequestAirdrop(Pubkey.default(), 1000),
    ]
    as_json = batch_to_json(reqs)
    assert as_json == (
        '[{"method":"getSignatureStatuses","jsonrpc":"2.0","id":0,"params"'
        ':[["1111111111111111111111111111111111111111111111111111111111111111"],'
        '{"searchTransactionHistory":true}]},{"method":"requestAirdrop","jsonrpc":"2.0","id":0,'
        '"params":["11111111111111111111111111111111",1000]}]'
    )
