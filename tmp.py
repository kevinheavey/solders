print("running tmp.py")
from solders.system_program import (
    decode_create_account,
    create_account,
    CreateAccountParams,
)

print("imported decode_create_account")
from solders.instruction import Instruction
from solders.pubkey import Pubkey
from solders.keypair import Keypair

params = CreateAccountParams(
    from_pubkey=Keypair().pubkey(),
    to_pubkey=Keypair().pubkey(),
    lamports=123,
    space=1,
    owner=Pubkey.default(),
)
ix = create_account(params)
print("running decode_create_account")
res = decode_create_account(ix)
print("ran decode_create_account")
print(f"res: {res}")
