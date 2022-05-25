from solders.pubkey import Pubkey

key = 
to_pubkey = Pubkey.new_unique()
program_id = Pubkey.new_unique()
instruction_data = bytes([1])
instruction = Instruction(
    program_id,
    instruction_data,
    [
        AccountMeta(from_pubkey, is_signer=True, is_writable=True),
        AccountMeta(to_pubkey, is_signer=True, is_writable=True),
    ],
)
message = Message([instruction])
serialized = bytes(message)
assert Message.from_bytes(serialized) == message
