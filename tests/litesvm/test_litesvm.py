from dataclasses import dataclass
from pathlib import Path
from typing import Optional, Tuple

from solders.account import Account
from solders.clock import Clock
from solders.compute_budget import ComputeBudget
from solders.instruction import AccountMeta, Instruction
from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.message import Message, MessageV1, TransactionConfig
from solders.pubkey import Pubkey
from solders.rent import Rent
from solders.system_program import transfer
from solders.transaction import Transaction, VersionedTransaction
from solders.transaction_metadata import FailedTransactionMetadata, TransactionMetadata


def helloworld_program(
    compute_max_units: Optional[int] = None,
) -> Tuple[LiteSVM, Pubkey, Pubkey]:
    program_id = Pubkey.new_unique()
    greeted_pubkey = Pubkey.new_unique()
    svm = LiteSVM()
    if compute_max_units is not None:
        compute_budget = ComputeBudget(False)
        compute_budget.compute_unit_limit = compute_max_units
        svm = svm.with_compute_budget(compute_budget)
    svm.set_account(
        greeted_pubkey, Account(lamports=5, data=bytes([0, 0, 0, 0]), owner=program_id)
    )
    svm.add_program_from_file(program_id, Path("tests/fixtures/helloworld.so"))
    return svm, program_id, greeted_pubkey


def helloworld_program_via_set_account(
    compute_max_units: Optional[int] = None,
) -> Tuple[LiteSVM, Pubkey, Pubkey]:
    program_id = Pubkey.from_string("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt")
    greeted_pubkey = Pubkey.from_string("4ivLcnNLhe4cKdpV9b4jyEmxgbYWFgktHcTyyBvYavsD")
    svm = LiteSVM()
    if compute_max_units is not None:
        compute_budget = ComputeBudget(False)
        compute_budget.compute_unit_limit = compute_max_units
        svm = svm.with_compute_budget(compute_budget)
    svm.set_account(
        greeted_pubkey,
        Account(lamports=1_000_000_000, data=bytes([0, 0, 0, 0]), owner=program_id),
    )
    program_bytes = Path("tests/fixtures/helloworld.so").read_bytes()
    executable_account = Account(
        lamports=1_000_000_000_000,
        data=program_bytes,
        owner=Pubkey.from_string("BPFLoader2111111111111111111111111111111111"),
        executable=True,
    )
    svm.set_account(program_id, executable_account)
    return svm, program_id, greeted_pubkey


@dataclass
class HelloworldSetup:  # noqa: D101
    client: LiteSVM
    payer: Keypair
    msg: Message
    greeted_pubkey: Pubkey


def helloworld_setup() -> HelloworldSetup:
    # https://github.com/solana-labs/example-helloworld/blob/36eb41d1290732786e13bd097668d8676254a139/src/program-rust/tests/lib.rs
    client, program_id, greeted_pubkey = helloworld_program()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    payer = Keypair()
    airdrop_res = client.airdrop(payer.pubkey(), 1_000_000_000)
    assert isinstance(airdrop_res, TransactionMetadata)
    blockhash = client.latest_blockhash()
    greeted_account_before = client.get_account(greeted_pubkey)
    assert greeted_account_before is not None
    assert greeted_account_before.data == bytes([0, 0, 0, 0])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    return HelloworldSetup(client, payer, msg, greeted_pubkey)


def test_helloworld() -> None:
    setup = helloworld_setup()
    msg = setup.msg
    payer = setup.payer
    client = setup.client
    greeted_pubkey = setup.greeted_pubkey
    tx = Transaction([payer], msg, msg.recent_blockhash)
    res = client.send_transaction(tx)
    assert isinstance(res, TransactionMetadata)
    greeted_account_after = client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == bytes([1, 0, 0, 0])


def test_helloworld_legacy_tx() -> None:
    setup = helloworld_setup()
    msg = setup.msg
    payer = setup.payer
    client = setup.client
    greeted_pubkey = setup.greeted_pubkey
    tx = Transaction([payer], msg, msg.recent_blockhash)
    res = client.send_transaction(tx)
    assert isinstance(res, TransactionMetadata)
    greeted_account_after = client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == bytes([1, 0, 0, 0])


def test_compute_limit() -> None:
    client, program_id, greeted_pubkey = helloworld_program(compute_max_units=10)
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    payer = Keypair()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    blockhash = client.latest_blockhash()
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    res = client.send_transaction(tx)
    assert isinstance(res, FailedTransactionMetadata)


def test_sysvar() -> None:
    client = LiteSVM()
    rent_before = client.get_rent()
    assert rent_before.burn_percent == 50
    assert rent_before.minimum_balance(123) == 1746960
    new_rent = Rent(
        burn_percent=0,
        exemption_threshold=rent_before.exemption_threshold,
        lamports_per_byte=rent_before.lamports_per_byte,
    )
    client.set_rent(new_rent)
    rent_after = client.get_rent()
    assert rent_after == new_rent
    clock_before = client.get_clock()
    assert clock_before.epoch == 0
    new_clock = Clock(
        slot=1000,
        epoch_start_timestamp=1,
        epoch=100,
        leader_schedule_epoch=3,
        unix_timestamp=4,
    )
    client.set_clock(new_clock)
    clock_after = client.get_clock()
    assert clock_after == new_clock


def test_nonexistent_account() -> None:
    client = LiteSVM()
    acc = client.get_account(Pubkey.new_unique())
    assert acc is None


def test_warp() -> None:
    client = LiteSVM()
    # litesvm seeds the clock with a mainnet-like slot rather than 0.
    slot0 = client.get_clock().slot
    new_slot = slot0 + 1000
    client.warp_to_slot(new_slot)
    slot1 = client.get_clock().slot
    assert slot1 == new_slot


def test_many_instructions() -> None:
    # https://github.com/solana-labs/example-helloworld/blob/36eb41d1290732786e13bd097668d8676254a139/src/program-rust/tests/lib.rs
    client, program_id, greeted_pubkey = helloworld_program()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    payer = Keypair()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    blockhash = client.latest_blockhash()
    greeted_account_before = client.get_account(greeted_pubkey)
    assert greeted_account_before is not None
    assert greeted_account_before.data == bytes([0, 0, 0, 0])
    num_ixs = 64
    msg = Message.new_with_blockhash(
        [ix for _ in range(num_ixs)], payer.pubkey(), blockhash
    )
    tx = VersionedTransaction(msg, [payer])
    client.send_transaction(tx)
    greeted_account_after = client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == (num_ixs).to_bytes(4, "little")


def test_transfer_v1_message() -> None:
    # `enable_tx_v1` is not active on mainnet yet; this relies on LiteSVM
    # enabling all features by default.
    client = LiteSVM()
    receiver = Pubkey.new_unique()
    payer = Keypair()
    client.airdrop(payer.pubkey(), 1_000_000_000)
    transfer_lamports = 1_000_000
    msg = MessageV1.try_compile(
        payer.pubkey(),
        [
            transfer(
                {
                    "from_pubkey": payer.pubkey(),
                    "to_pubkey": receiver,
                    "lamports": transfer_lamports,
                }
            )
        ],
        client.latest_blockhash(),
        TransactionConfig(compute_unit_limit=200_000),
    )
    tx = VersionedTransaction(msg, [payer])
    assert tx.version() == 1
    res = client.send_transaction(tx)
    assert isinstance(res, TransactionMetadata)
    assert client.get_balance(receiver) == transfer_lamports


def test_get_program_accounts() -> None:
    client = LiteSVM()
    program_id = Pubkey.new_unique()
    other_program = Pubkey.new_unique()
    owned = [Pubkey.new_unique() for _ in range(3)]
    for pk in owned:
        client.set_account(pk, Account(lamports=5, data=bytes([1]), owner=program_id))
    client.set_account(
        Pubkey.new_unique(), Account(lamports=5, data=bytes([1]), owner=other_program)
    )
    found = client.get_program_accounts(program_id)
    assert {addr for addr, _ in found} == set(owned)
    assert all(acc.owner == program_id for _, acc in found)
    assert client.get_program_accounts(Pubkey.new_unique()) == []


def test_epoch_stake() -> None:
    client = LiteSVM()
    vote_a = Pubkey.new_unique()
    vote_b = Pubkey.new_unique()
    assert client.epoch_total_stake() == 0
    assert client.epoch_stake(vote_a) == 0

    client.set_epoch_stake(vote_a, 100)
    client.set_epoch_stake(vote_b, 50)
    assert client.epoch_stake(vote_a) == 100
    assert client.epoch_total_stake() == 150

    # overwriting adjusts the total rather than accumulating
    client.set_epoch_stake(vote_a, 25)
    assert client.epoch_total_stake() == 75

    # zero removes the vote account
    client.set_epoch_stake(vote_a, 0)
    assert client.epoch_stake(vote_a) == 0
    assert client.epoch_total_stake() == 50

    client.set_epoch_stakes({vote_a: 7, vote_b: 3})
    assert client.epoch_stake(vote_a) == 7
    assert client.epoch_total_stake() == 10


def test_transfer() -> None:
    client = LiteSVM()
    receiver = Pubkey.new_unique()
    num_txs = 2
    payer = Keypair()
    airdrop_res = client.airdrop(payer.pubkey(), 1_000_000_000)
    assert isinstance(airdrop_res, TransactionMetadata)
    blockhash = client.latest_blockhash()
    num_ixs = 1
    transfer_lamports_base = 1_000_000
    for i in range(num_txs):
        ixs = [
            transfer(
                {
                    "from_pubkey": payer.pubkey(),
                    "to_pubkey": receiver,
                    "lamports": transfer_lamports_base + i,
                }
            )
            for _ in range(num_ixs)
        ]
        msg = Message.new_with_blockhash(ixs, payer.pubkey(), blockhash)
        tx = VersionedTransaction(msg, [payer])
        client.send_transaction(tx)
    total_ix_count = num_ixs * num_txs
    balance_after = client.get_balance(receiver)
    assert (
        balance_after
        == total_ix_count * transfer_lamports_base
        + num_ixs * ((num_txs - 1) * num_txs) / 2
    )


def test_missing_program() -> None:
    program_id = Pubkey.new_unique()
    client = LiteSVM()
    payer = Keypair()
    airdrop_res = client.airdrop(payer.pubkey(), 1_000_000_000)
    assert isinstance(airdrop_res, TransactionMetadata)
    blockhash = client.latest_blockhash()
    ix = Instruction(program_id, b"", [])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    res = client.send_transaction(tx)
    assert isinstance(res, FailedTransactionMetadata)


def test_add_program_via_set_account() -> None:
    # https://github.com/solana-labs/example-helloworld/blob/36eb41d1290732786e13bd097668d8676254a139/src/program-rust/tests/lib.rs
    client, program_id, greeted_pubkey = helloworld_program_via_set_account()
    ix = Instruction(
        program_id,
        bytes([0]),
        [AccountMeta(greeted_pubkey, is_signer=False, is_writable=True)],
    )
    payer = Keypair()
    airdrop_res = client.airdrop(payer.pubkey(), 1_000_000_000)
    assert isinstance(airdrop_res, TransactionMetadata)
    blockhash = client.latest_blockhash()
    greeted_account_before = client.get_account(greeted_pubkey)
    assert greeted_account_before is not None
    assert greeted_account_before.data == bytes([0, 0, 0, 0])
    msg = Message.new_with_blockhash([ix], payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    client.send_transaction(tx)
    greeted_account_after = client.get_account(greeted_pubkey)
    assert greeted_account_after is not None
    assert greeted_account_after.data == bytes([1, 0, 0, 0])
