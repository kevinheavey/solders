from typing import cast, Union
from pytest import mark, raises, fixture
from solders.instruction import Instruction, CompiledInstruction, AccountMeta
from solders.pubkey import Pubkey
from solders.errors import BincodeError


@fixture
def ix() -> Instruction:
    return Instruction(
        Pubkey.default(), b"1", [AccountMeta(Pubkey.new_unique(), True, True)]
    )


@fixture
def compiled_ix() -> CompiledInstruction:
    return CompiledInstruction(0, b"1", b"123")


@fixture
def am() -> AccountMeta:
    return AccountMeta(Pubkey.new_unique(), True, True)


def test_account_meta_hashable(am: AccountMeta) -> None:
    assert isinstance(hash(am), int)


def test_accounts_setter(ix: Instruction, am: AccountMeta) -> None:
    new_accounts = [am]
    ix.accounts = new_accounts
    assert ix.accounts == new_accounts


def test_ix_from_bytes(ix: Instruction) -> None:
    assert Instruction.from_bytes(bytes(ix)) == ix


def test_am_from_bytes(am: AccountMeta) -> None:
    assert AccountMeta.from_bytes(bytes(am)) == am


def test_accounts_setter_compiled_ix(compiled_ix: CompiledInstruction) -> None:
    ix = compiled_ix
    new_accounts = b"456"
    ix.accounts = new_accounts
    assert ix.accounts == new_accounts
    new_accounts_as_list = list(b"foo")
    ix.accounts = cast(bytes, new_accounts_as_list)
    assert ix.accounts == bytes(new_accounts_as_list)


def test_compiled_accounts_eq(compiled_ix: CompiledInstruction) -> None:
    assert compiled_ix == compiled_ix


@mark.parametrize("to_deserialize", [Instruction, CompiledInstruction])
def test_bincode_error(to_deserialize: Union[Instruction, CompiledInstruction]) -> None:
    with raises(BincodeError) as excinfo:
        Instruction.from_bytes(b"foo")
    assert excinfo.value.args[0] == "io error: unexpected end of file"
