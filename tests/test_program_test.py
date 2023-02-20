from pytest import mark
from solders.program_test import start
from solders.transaction import VersionedTransaction
from solders.message import Message
from solders.instruction import Instruction, AccountMeta
from solders.pubkey import Pubkey

@mark.asyncio
async def test_basic() -> None:
    program_id = Pubkey.from_string("Logging111111111111111111111111111111111111")
    ix = Instruction(program_id, bytes([5, 10, 11, 12, 13, 14]), [AccountMeta(Pubkey.new_unique(), is_signer=False, is_writable=True)])
    client, payer, blockhash = await start(programs=[("spl_example_logging", program_id)])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    await client.process_transaction(tx)
