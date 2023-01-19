import pickle

from pytest import fixture
from solders.account import Account
from solders.pubkey import Pubkey


@fixture
def account() -> Account:
    return Account(1, b"123", Pubkey.default(), True, 1)


def test_bytes(account: Account) -> None:
    assert Account.from_bytes(bytes(account))


def test_pickle(account: Account) -> None:
    obj = Account.default()
    assert pickle.loads(pickle.dumps(obj)) == obj


def test_json() -> None:
    obj = Account.default()
    assert Account.from_json(obj.to_json()) == obj
