from pathlib import Path

from solders.instruction import Instruction
from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message
from solders.pubkey import Pubkey
from solders.transaction import VersionedTransaction
from solders.transaction_metadata import FailedTransactionMetadata, TransactionMetadata


def test_set_clock() -> None:
    program_id = Pubkey.new_unique()
    client = LiteSVM()
    client.add_program_from_file(
        program_id, Path("tests/fixtures/solders_clock_example.so")
    )
    payer = Keypair()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    blockhash = client.latest_blockhash()
    ixs = [Instruction(program_id=program_id, data=b"", accounts=[])]
    msg = Message.new_with_blockhash(ixs, payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    # set the time to January 1st 2000
    initial_clock = client.get_clock()
    initial_clock.unix_timestamp = 1735689600
    client.set_clock(initial_clock)
    # this will fail because it's not January 1970 anymore
    bad_res = client.send_transaction(tx)
    assert isinstance(bad_res, FailedTransactionMetadata)
    # so let's turn back time
    clock = client.get_clock()
    clock.unix_timestamp = 50
    client.set_clock(clock)
    ixs2 = [
        Instruction(
            program_id=program_id,
            data=b"foobar",  # unused, this is just to dedup the transaction
            accounts=[],
        )
    ]
    msg2 = Message.new_with_blockhash(ixs2, payer.pubkey(), blockhash)
    tx2 = VersionedTransaction(msg2, [payer])
    # now the transaction goes through
    good_res = client.send_transaction(tx2)
    assert isinstance(good_res, TransactionMetadata)
