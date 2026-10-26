from solders.account import Account
from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message
from solders.pubkey import Pubkey
from solders.system_program import transfer
from solders.transaction import VersionedTransaction
from solders.transaction_metadata import (
    FailedTransactionMetadata,
    SimulatedTransactionInfo,
    TransactionMetadata,
)


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


def test_loaded_accounts_data_size() -> None:
    client = LiteSVM()
    payer = Keypair()
    receiver = Pubkey.new_unique()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    client.set_account(
        receiver,
        Account(lamports=1_000_000, data=bytes(123), owner=Pubkey.new_unique()),
    )

    def transfer_tx(lamports: int) -> VersionedTransaction:
        ix = transfer(
            {
                "from_pubkey": payer.pubkey(),
                "to_pubkey": receiver,
                "lamports": lamports,
            }
        )
        msg = Message.new_with_blockhash(
            [ix], payer.pubkey(), client.latest_blockhash()
        )
        return VersionedTransaction(msg, [payer])

    tx = transfer_tx(1_000_000)
    # SIMD-0186 meters 64 bytes of overhead plus the data of each loaded account.
    expected_size = 0
    for address in tx.message.account_keys:
        account = client.get_account(address)
        assert account is not None
        expected_size += 64 + len(account.data)

    simulated = client.simulate_transaction(tx)
    assert isinstance(simulated, SimulatedTransactionInfo)
    succeeded = client.send_transaction(tx)
    assert isinstance(succeeded, TransactionMetadata)
    failed = client.send_transaction(transfer_tx(2_000_000_000))
    assert isinstance(failed, FailedTransactionMetadata)

    for meta in (simulated.meta(), succeeded, failed.meta()):
        assert meta.loaded_accounts_data_size() == expected_size
        assert TransactionMetadata.from_bytes(bytes(meta)) == meta
        assert TransactionMetadata.from_json(meta.to_json()) == meta
