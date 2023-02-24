from typing import Optional, Tuple

from pytest import mark, raises
from solders.account import Account
from solders.clock import Clock
from solders.instruction import AccountMeta, Instruction
from solders.message import Message
from solders.program_test import BanksClientError, ProgramTestContext, start
from solders.pubkey import Pubkey
from solders.rent import Rent
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


async def helloworld_program(compute_max_units: Optional[int] = None) -> Tuple[ProgramTestContext, Pubkey, Pubkey]:
    program_id = Pubkey.new_unique()
    greeted_pubkey = Pubkey.new_unique()
    context = await start(
        programs=[("helloworld", program_id)],
        accounts=[
            (
                greeted_pubkey,
                Account(lamports=5, data=bytes([0, 0, 0, 0]), owner=program_id),
            )
        ],
        compute_max_units=compute_max_units
    )
    return context, program_id, greeted_pubkey


@mark.asyncio
async def test_helloworld() -> None:
    # https://github.com/solana-labs/example-helloworld/blob/36eb41d1290732786e13bd097668d8676254a139/src/program-rust/tests/lib.rs
    context, program_id, greeted_pubkey = await helloworld_program()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
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

@mark.asyncio
async def test_compute_limit() -> None:
    context, program_id, greeted_pubkey = await helloworld_program(compute_max_units=10)
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    client = context.banks_client
    payer = context.payer
    blockhash = context.last_blockhash
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    with raises(BanksClientError):
        await client.process_transaction(tx)

@mark.asyncio()
async def test_sysvar() -> None:
    context = await start()
    client = context.banks_client
    rent_before = await client.get_rent()
    assert rent_before.burn_percent == 50
    new_rent = Rent(burn_percent=0, exemption_threshold=rent_before.exemption_threshold, lamports_per_byte_year=rent_before.lamports_per_byte_year)
    context.set_rent(new_rent)
    rent_after = await client.get_rent()
    assert rent_after == new_rent
    clock_before = await client.get_clock()
    assert clock_before.epoch == 0
    new_clock = Clock(slot=1000, epoch_start_timestamp=1, epoch=100, leader_schedule_epoch=3, unix_timestamp=4)
    context.set_clock(new_clock)
    clock_after = await client.get_clock()
    assert clock_after == new_clock
    # see that setting the clock sysvar doesn't change the result of get_slot
    slot = await client.get_slot()
    assert slot == 1

@mark.asyncio()
async def test_warp() -> None:
    context = await start()
    client = context.banks_client
    slot0 = await client.get_slot()
    assert slot0 == 1
    new_slot = 1000
    context.warp_to_slot(new_slot)
    slot1 = await client.get_slot()
    assert slot1 == new_slot
