from solders.rpc.requests import (
    GetSignatureStatuses,
    RequestAirdrop,
    batch,
    batch_from_json,
)
from solders.rpc.config import RpcSignatureStatusConfig, RpcRequestAirdropConfig
from solders.signature import Signature
from solders.pubkey import Pubkey


def test_get_signature_statuses() -> None:
    req = GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True))
    as_json = req.to_json()
    assert GetSignatureStatuses.from_json(as_json) == req


def test_request_airdrop() -> None:
    req = RequestAirdrop(Pubkey.default(), 1000)
    as_json = req.to_json()
    assert RequestAirdrop.from_json(as_json) == req


def test_batch() -> None:
    reqs = [
        GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True)),
        RequestAirdrop(Pubkey.default(), 1000),
    ]
    as_json = batch(reqs)
    assert as_json == (
        '[{"jsonrpc":"2.0","id":0,"method":"getSignatureStatuses","params"'
        ':[["1111111111111111111111111111111111111111111111111111111111111111"],'
        '{"searchTransactionHistory":true}]},{"jsonrpc":"2.0","id":0,'
        '"method":"requestAirdrop","params":["11111111111111111111111111111111",1000]}]'
    )
    assert batch_from_json(as_json) == reqs
