from pathlib import Path

from solders.instruction import AccountMeta, Instruction
from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message
from solders.pubkey import Pubkey
from solders.transaction import VersionedTransaction
from solders.transaction_metadata import TransactionMetadata


def test_logging() -> None:
    program_id = Pubkey.from_string("Logging111111111111111111111111111111111111")
    ix = Instruction(
        program_id,
        bytes([5, 10, 11, 12, 13, 14]),
        [AccountMeta(Pubkey.new_unique(), is_signer=False, is_writable=True)],
    )
    client = LiteSVM()
    payer = Keypair()
    client.add_program_from_file(
        program_id, Path("tests/fixtures/spl_example_logging.so")
    )
    client.airdrop(payer.pubkey(), 1_000_000_000)
    blockhash = client.latest_blockhash()
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    # let's sim it first
    sim_res = client.simulate_transaction(tx)
    meta = client.send_transaction(tx)
    assert isinstance(meta, TransactionMetadata)
    assert sim_res.meta() == meta
    assert meta.logs()[1] == "Program log: static string"
    assert (
        meta.compute_units_consumed() < 10_000
    )  # not being precise here in case it changes
