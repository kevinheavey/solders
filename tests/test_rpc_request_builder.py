from solders.rpc.requests import GetSignatureStatuses3, RequestAirdrop
from solders.rpc.config import RpcSignatureStatusConfig, RpcRequestAirdropConfig
from solders.signature import Signature
from solders.pubkey import Pubkey


def test_get_signature_statuses() -> None:
    req = GetSignatureStatuses3([Signature.default()], RpcSignatureStatusConfig(True))
    as_json = req.to_json()
    assert GetSignatureStatuses3.from_json(as_json) == req


def test_request_airdrop() -> None:
    req = RequestAirdrop(Pubkey.default(), 1000)
    as_json = req.to_json()
    assert RequestAirdrop.from_json(as_json) == req
