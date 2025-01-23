from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message
from solders.pubkey import Pubkey
from solders.system_program import transfer
from solders.transaction import VersionedTransaction


def test_transfer() -> None:
    receiver = Pubkey.new_unique()
    client = LiteSVM()
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
    client.send_transaction(tx)
    balance_after = client.get_balance(receiver)
    assert balance_after == transfer_lamports
