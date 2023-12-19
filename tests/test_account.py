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
    assert pickle.loads(pickle.dumps(account)) == account


def test_json(account: Account) -> None:
    assert Account.from_json(account.to_json()) == account
