from solders.rpc.responses import GetAccountInfoResp, RpcResponseContext, RpcError
from solders.account import Account
from solders.pubkey import Pubkey
from based58 import b58decode


def test_get_account_info() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1
    },
    "value": {
      "data": [
        "11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf",
        "base58"
      ],
      "executable": false,
      "lamports": 1000000000,
      "owner": "11111111111111111111111111111111",
      "rentEpoch": 2
    }
  },
  "id": 1
}"""
    parsed = GetAccountInfoResp.from_json(raw)
    context = RpcResponseContext(slot=1)
    value = Account(
        data=b58decode(
            b"11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf"
        ),
        executable=False,
        lamports=1000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        rent_epoch=2,
    )
    assert parsed == GetAccountInfoResp(context=context, value=value)


def test_get_account_info_null() -> None:
    raw = '{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.10.26","slot":146423291},"value":null},"id":1}'
    parsed = GetAccountInfoResp.from_json(raw)
    assert parsed.value is None
    context = RpcResponseContext(slot=146423291, api_version="1.10.26")
    value = None
    assert parsed == GetAccountInfoResp(context=context, value=value)


def test_get_account_info_error() -> None:
    raw = '{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid param: WrongSize"},"id":1}'
    parsed = GetAccountInfoResp.from_json(raw)
    error = RpcError(code=-32602, message="Invalid param: WrongSize")
    assert parsed == error
