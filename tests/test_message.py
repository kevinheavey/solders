from solders import Message, Pubkey, Instruction, AccountMeta


def test_message_signed_keys_len() -> None:
    program_id = Pubkey.default()
    id0 = Pubkey.default()
    ix = Instruction(program_id, b"\x00", [AccountMeta(id0, False, True)])
    message = Message([ix], None)
    assert message.header.num_required_signatures == 0

    ix = Instruction(program_id, b"\x00", [AccountMeta(id0, True, True)])
    message = Message([ix], id0)
    assert message.header.num_required_signatures == 1
