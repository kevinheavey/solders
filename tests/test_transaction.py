from solders import Transaction, Pubkey, Keypair, CompiledInstruction, Hash

def get_program_id(tx: Transaction, instruction_index: int) -> Pubkey:
    message = tx.message
    instruction = message.instructions[instruction_index]
    return instruction.program_id(message.account_keys)

def test_refs() -> None:
    key = Keypair()
    key1 = Pubkey.new_unique()
    key2 = Pubkey.new_unique()
    prog1 = Pubkey.new_unique()
    prog2 = Pubkey.new_unique()
    instructions = [
        CompiledInstruction(3, b"", bytes([0, 1])),
        CompiledInstruction(4, b"", bytes([0, 2])),
    ]
    tx = Transaction.new_with_compiled_instructions(
        [key],
        [key1, key2],
        Hash.default(),
        [prog1, prog2],
        instructions,
    )
    tx.sanitize()

    assert tx.key(0, 0) == key.pubkey()
    assert tx.signer_key(0, 0) == key.pubkey()

    assert tx.key(1, 0) == key.pubkey()
    assert tx.signer_key(1, 0) == key.pubkey()

    assert tx.key(0, 1) == key1
    assert tx.signer_key(0, 1) is None

    assert tx.key(1, 1) == key2
    assert tx.signer_key(1, 1) is None

    assert tx.key(2, 0) is None
    assert tx.signer_key(2, 0) is None

    assert tx.key(0, 2) is None
    assert tx.signer_key(0, 2) is None

    assert get_program_id(tx, 0) == prog1
    assert get_program_id(tx, 1) == prog2
