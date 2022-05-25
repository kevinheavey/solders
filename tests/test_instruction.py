from typing import cast, Union
from pytest import mark, raises
from solders.instruction import Instruction, CompiledInstruction, AccountMeta
from solders.pubkey import Pubkey
from solders.errors import BincodeError


def test_accounts_setter() -> None:
    ix = Instruction(
        Pubkey.default(), b"1", [AccountMeta(Pubkey.new_unique(), True, True)]
    )
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


@mark.parametrize("to_deserialize", [Instruction, CompiledInstruction])
def test_bincode_error(to_deserialize: Union[Instruction, CompiledInstruction]) -> None:
    with raises(BincodeError) as excinfo:
        Instruction.from_bytes(b"foo")
    assert excinfo.value.args[0] == "io error: unexpected end of file"
