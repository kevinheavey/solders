from pytest import mark
from solders.account import Account
from solders.instruction import AccountMeta, Instruction
from solders.message import Message
from solders.program_test import start
from solders.pubkey import Pubkey
from solders.transaction import VersionedTransaction


@mark.asyncio
async def test_logging() -> None:
    program_id = Pubkey.from_string("Logging111111111111111111111111111111111111")
    ix = Instruction(
        program_id,
        bytes([5, 10, 11, 12, 13, 14]),
        [AccountMeta(Pubkey.new_unique(), is_signer=False, is_writable=True)],
    )
    client, payer, blockhash = await start(
        programs=[("spl_example_logging", program_id)]
    )
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    await client.process_transaction(tx)


@mark.asyncio
async def test_helloworld() -> None:
    program_id = Pubkey.new_unique()
    greeted_pubkey = Pubkey.new_unique()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    client, payer, blockhash = await start(
        programs=[("helloworld", program_id)],
        accounts=[
            (
                greeted_pubkey,
                Account(lamports=5, data=bytes([0, 0, 0, 0]), owner=program_id),
            )
        ],
    )
    greeted_account_before = await client.get_account(greeted_pubkey)
    assert greeted_account_before is not None
    assert greeted_account_before.data == bytes([0, 0, 0, 0])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    await client.process_transaction(tx)
    greeted_account_after = await client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == bytes([1, 0, 0, 0])
