from typing import cast
from solders import Instruction, CompiledInstruction, AccountMeta, Pubkey


def test_accounts_setter() -> None:
    ix = Instruction(
        Pubkey.default(), b"1", [AccountMeta(Pubkey.new_unique(), True, True)]
    )
    new_pubkey = Pubkey.new_unique()
    new_accounts = [AccountMeta(Pubkey.new_unique(), True, True)]
    ix.accounts = new_accounts
    assert ix.accounts == new_accounts


def test_accounts_setter_compiled_ix() -> None:
    ix = CompiledInstruction(0, b"1", b"123")
    new_accounts = b"456"
    ix.accounts = new_accounts
    assert ix.accounts == new_accounts
    new_accounts_as_list = list(b"foo")
    ix.accounts = cast(bytes, new_accounts_as_list)
    assert ix.accounts == bytes(new_accounts_as_list)
