from pytest import mark
from solders.account import Account
from solders.instruction import AccountMeta, Instruction
from solders.message import Message
from solders.program_test import start
from solders.pubkey import Pubkey
from solders.transaction import VersionedTransaction


@mark.asyncio
async def test_logging() -> None:
    # https://github.com/solana-labs/solana-program-library/blob/bd216c8103cd8eb9f5f32e742973e7afb52f3b81/examples/rust/logging/tests/functional.rs
    program_id = Pubkey.from_string("Logging111111111111111111111111111111111111")
    ix = Instruction(
        program_id,
        bytes([5, 10, 11, 12, 13, 14]),
        [AccountMeta(Pubkey.new_unique(), is_signer=False, is_writable=True)],
    )
    context = await start(
        programs=[("spl_example_logging", program_id)]
    )
    payer = context.payer
    blockhash = context.last_blockhash
    client = context.banks_client
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    meta = (await client.process_transaction_with_metadata(tx)).meta
    assert meta is not None
    assert meta.log_messages[1] == 'Program log: static string'


@mark.asyncio
async def test_helloworld() -> None:
    # https://github.com/solana-labs/example-helloworld/blob/36eb41d1290732786e13bd097668d8676254a139/src/program-rust/tests/lib.rs
    program_id = Pubkey.new_unique()
    greeted_pubkey = Pubkey.new_unique()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    context = await start(
        programs=[("helloworld", program_id)],
        accounts=[
            (
                greeted_pubkey,
                Account(lamports=5, data=bytes([0, 0, 0, 0]), owner=program_id),
            )
        ],
    )
    client = context.banks_client
    payer = context.payer
    blockhash = context.last_blockhash
    greeted_account_before = await client.get_account(greeted_pubkey)
    assert greeted_account_before is not None
    assert greeted_account_before.data == bytes([0, 0, 0, 0])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    await client.process_transaction(tx)
    greeted_account_after = await client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == bytes([1, 0, 0, 0])
