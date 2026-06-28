from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message
from solders.pubkey import Pubkey
from solders.system_program import transfer
from solders.transaction import VersionedTransaction
from solders.transaction_metadata import TransactionMetadata


def test_transfer() -> None:
    receiver = Pubkey.new_unique()
    client = LiteSVM()
    assert isinstance(client.airdrop_pubkey(), Pubkey)
    payer = Keypair()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    blockhash = client.latest_blockhash()
    transfer_lamports = 1_000_000
    ixs = [
        transfer(
            {
                "from_pubkey": payer.pubkey(),
                "to_pubkey": receiver,
                "lamports": transfer_lamports,
            }
        )
    ]
    msg = Message.new_with_blockhash(ixs, payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    meta = client.send_transaction(tx)
    balance_after = client.get_balance(receiver)
    assert balance_after == transfer_lamports
    # TransactionMetadata.fee / pretty_logs (added for litesvm 0.13)
    assert isinstance(meta, TransactionMetadata)
    assert isinstance(meta.fee(), int)
    assert isinstance(meta.pretty_logs(), str)
