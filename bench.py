from pytest import fixture
from typing import Tuple, Any
from solders.pubkey import Pubkey
from solana.publickey import PublicKey
from solana.keypair import Keypair as SolpyKeypair
from solders.keypair import Keypair
from solders.system_program import (
    create_account,
    CreateAccountParams,
    decode_create_account,
)
from solana.system_program import (
    create_account as solpy_create_account,
    decode_create_account as solpy_decode_create_account,
    CreateAccountParams as SolpyCreateAccountParams,
)
from solders.transaction import Transaction
from solders.hash import Hash
from solders.instruction import Instruction, AccountMeta
from solders.message import Message
from solana.transaction import Transaction as SolpyTransaction


def test_solders_create_program_address(benchmark: Any) -> None:
    program_id = Pubkey.from_string("BPFLoader1111111111111111111111111111111111")
    benchmark(Pubkey.create_program_address, [b"", bytes([1])], program_id)


def test_solpy_create_program_address(benchmark: Any) -> None:
    program_id = PublicKey("BPFLoader1111111111111111111111111111111111")
    benchmark(PublicKey.create_program_address, [b"", bytes([1])], program_id)


def test_solders_sign_message(benchmark: Any) -> None:
    kp = Keypair()
    benchmark(kp.sign_message, b"macaroni")


def test_solpy_sign_message(benchmark: Any) -> None:
    kp = SolpyKeypair()
    benchmark(kp.sign, b"macaroni")


def test_solders_create_account_ix(benchmark: Any) -> None:
    from_account = Pubkey.new_unique()
    new_account = Pubkey.new_unique()
    program_id = Pubkey.new_unique()
    params = CreateAccountParams(
        from_pubkey=from_account,
        to_pubkey=new_account,
        lamports=1,
        space=1,
        owner=program_id,
    )
    benchmark(create_account, params)


def test_solpy_create_account_ix(benchmark: Any) -> None:
    from_account = SolpyKeypair().public_key
    new_account = SolpyKeypair().public_key
    program_id = SolpyKeypair().public_key
    params = SolpyCreateAccountParams(
        from_pubkey=from_account,
        new_account_pubkey=new_account,
        lamports=1,
        space=1,
        program_id=program_id,
    )
    benchmark(solpy_create_account, params)


def test_solders_decode_create_account(benchmark: Any) -> None:
    from_account = Pubkey.new_unique()
    new_account = Pubkey.new_unique()
    program_id = Pubkey.new_unique()
    params = CreateAccountParams(
        from_pubkey=from_account,
        to_pubkey=new_account,
        lamports=1,
        space=1,
        owner=program_id,
    )
    ix = create_account(params)
    benchmark(decode_create_account, ix)


def test_solpy_decode_create_account(benchmark: Any) -> None:
    from_account = SolpyKeypair().public_key
    new_account = SolpyKeypair().public_key
    program_id = SolpyKeypair().public_key
    params = SolpyCreateAccountParams(
        from_pubkey=from_account,
        new_account_pubkey=new_account,
        lamports=1,
        space=1,
        program_id=program_id,
    )
    ix = solpy_create_account(params)
    benchmark(solpy_decode_create_account, ix)


@fixture(scope="function")
def example_signed_tx() -> Transaction:
    program_id = Pubkey.default()
    arbitrary_instruction_data = bytes([1])
    instruction = Instruction(program_id, arbitrary_instruction_data, [])
    payer = Keypair()
    message = Message([instruction], payer.pubkey())
    blockhash = Hash.default()
    return Transaction([payer], message, blockhash)


@fixture(scope="function")
def example_unsigned_tx() -> Tuple[Transaction, Keypair]:
    program_id = Pubkey.default()
    blockhash = Hash.default()
    arbitrary_instruction_data = bytes([1])
    accounts: list[AccountMeta] = []
    instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    payer = Keypair()
    message = Message.new_with_blockhash([instruction], payer.pubkey(), blockhash)
    return Transaction.new_unsigned(message), payer


def test_solders_tx_to_bytes(example_signed_tx: Transaction, benchmark: Any) -> None:
    benchmark(bytes, example_signed_tx)


def test_solpy_tx_to_bytes(example_signed_tx: Transaction, benchmark: Any) -> None:
    solpy_tx = SolpyTransaction.deserialize(bytes(example_signed_tx))
    benchmark(solpy_tx.serialize)


def test_solders_tx_from_bytes(example_signed_tx: Transaction, benchmark: Any) -> None:
    raw = bytes(example_signed_tx)
    benchmark(Transaction.from_bytes, raw)


def test_solpy_tx_from_bytes(example_signed_tx: Transaction, benchmark: Any) -> None:
    raw = bytes(example_signed_tx)
    benchmark(SolpyTransaction.deserialize, raw)


def test_solders_sign_tx(example_unsigned_tx: Tuple[Transaction, Keypair], benchmark: Any) -> None:
    tx, payer = example_unsigned_tx
    blockhash = tx.message.recent_blockhash
    benchmark(tx.sign, [payer], blockhash)


def test_solpy_sign_tx(example_unsigned_tx: Tuple[Transaction, Keypair], benchmark: Any) -> None:
    tx, payer = example_unsigned_tx
    tx_solpy = SolpyTransaction.deserialize(bytes(tx))
    payer_solpy = SolpyKeypair.from_secret_key(bytes(payer))
    benchmark(tx_solpy.sign, payer_solpy)
