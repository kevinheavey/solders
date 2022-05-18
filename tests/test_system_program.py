from typing import List
from solders import system_program as sp
from solders.pubkey import Pubkey
from solders.instruction import Instruction
from solders.keypair import Keypair


def test_id() -> None:
    assert sp.ID == Pubkey.from_string("11111111111111111111111111111111")


def get_keys(instruction: Instruction) -> List[Pubkey]:
    return [x.pubkey for x in instruction.accounts]


def test_move_many() -> None:
    alice_pubkey = Pubkey.new_unique()
    bob_pubkey = Pubkey.new_unique()
    carol_pubkey = Pubkey.new_unique()
    to_lamports = [(bob_pubkey, 1), (carol_pubkey, 2)]

    instructions = sp.transfer_many(alice_pubkey, to_lamports)
    assert len(instructions) == 2
    assert get_keys(instructions[0]) == [alice_pubkey, bob_pubkey]
    assert get_keys(instructions[1]) == [alice_pubkey, carol_pubkey]


def test_create_nonce_account() -> None:
    from_pubkey = Pubkey.new_unique()
    nonce_pubkey = Pubkey.new_unique()
    authorized = nonce_pubkey
    ixs = sp.create_nonce_account(from_pubkey, nonce_pubkey, authorized, 42)
    assert len(ixs) == 2
    ix = ixs[0]
    assert ix.program_id == sp.ID
    pubkeys = [am.pubkey for am in ix.accounts]
    assert from_pubkey in pubkeys
    assert nonce_pubkey in pubkeys


def test_create_account():
    """Test creating a transaction for create account."""
    params = sp.CreateAccountParams(
        from_pubkey=Keypair().pubkey(),
        to_pubkey=Keypair().pubkey(),
        lamports=123,
        space=1,
        owner=Pubkey.default(),
    )
    assert sp.decode_create_account(sp.create_account(params)) == params
