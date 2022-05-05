from solders import Message, Pubkey, Instruction, AccountMeta, CompiledInstruction


def test_message_signed_keys_len() -> None:
    program_id = Pubkey.default()
    id0 = Pubkey.default()
    ix = Instruction(program_id, b"\x00", [AccountMeta(id0, False, True)])
    message = Message([ix], None)
    assert message.header.num_required_signatures == 0

    ix = Instruction(program_id, b"\x00", [AccountMeta(id0, True, True)])
    message = Message([ix], id0)
    assert message.header.num_required_signatures == 1


def test_message_kitchen_sink() -> None:
    program_id0 = Pubkey.new_unique()
    program_id1 = Pubkey.new_unique()
    id0 = Pubkey.default()
    id1 = Pubkey.new_unique()
    zero_byte = b"\x00"
    message = Message(
        [
            Instruction(program_id0, zero_byte, [AccountMeta(id0, False, True)]),
            Instruction(program_id1, zero_byte, [AccountMeta(id1, True, True)]),
            Instruction(program_id0, zero_byte, [AccountMeta(id1, False, True)]),
        ],
        id1,
    )
    assert message.instructions[0] == CompiledInstruction(2, zero_byte, bytes([1]))
    assert message.instructions[1] == CompiledInstruction(3, zero_byte, bytes([0]))
    assert message.instructions[2] == CompiledInstruction(2, zero_byte, bytes([0]))
