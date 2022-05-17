from typing import List
from solders.system_program import SystemProgram
from solders.pubkey import Pubkey
from solders.instruction import Instruction


def test_id() -> None:
    assert SystemProgram.ID == Pubkey.from_string("11111111111111111111111111111111")


def get_keys(instruction: Instruction) -> List[Pubkey]:
    return [x.pubkey for x in instruction.accounts]


def test_move_many() -> None:
    alice_pubkey = Pubkey.new_unique()
    bob_pubkey = Pubkey.new_unique()
    carol_pubkey = Pubkey.new_unique()
    to_lamports = [(bob_pubkey, 1), (carol_pubkey, 2)]

    instructions = SystemProgram.transfer_many(alice_pubkey, to_lamports)
    assert len(instructions) == 2
    assert get_keys(instructions[0]) == [alice_pubkey, bob_pubkey]
    assert get_keys(instructions[1]) == [alice_pubkey, carol_pubkey]


def test_create_nonce_account() -> None:
    from_pubkey = Pubkey.new_unique()
    nonce_pubkey = Pubkey.new_unique()
    authorized = nonce_pubkey
    ixs = SystemProgram.create_nonce_account(from_pubkey, nonce_pubkey, authorized, 42)
    assert len(ixs) == 2
    ix = ixs[0]
    assert ix.program_id == SystemProgram.ID
    pubkeys = [am.pubkey for am in ix.accounts]
    assert from_pubkey in pubkeys
    assert nonce_pubkey in pubkeys
