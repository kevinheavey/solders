from typing import Tuple

from pytest import raises

from solders.keypair import Keypair
from solders.message import Message
from solders.instruction import Instruction, AccountMeta
from solders.pubkey import Pubkey
from solders.hash import Hash
from solders.transaction import Transaction, VersionedTransaction
from solders.system_program import (
    advance_nonce_account,
    transfer,
    withdraw_nonce_account,
)
from solders.errors import SignerError
from solders.null_signer import NullSigner


def test_try_new() -> None:
    keypair0 = Keypair()
    keypair1 = Keypair()
    keypair2 = Keypair()

    message = Message(
        [
            Instruction(
                Pubkey.new_unique(),
                b"",
                [
                    AccountMeta(keypair1.pubkey(), True, False),
                    AccountMeta(keypair2.pubkey(), False, False),
                ],
            )
        ],
        keypair0.pubkey(),
    )

    with raises(SignerError) as excinfo:
        VersionedTransaction(message, [keypair0])
    assert "not enough signers" in str(excinfo)
    with raises(SignerError) as excinfo:
        VersionedTransaction(message, [keypair0, keypair0])
    assert "keypair-pubkey mismatch" in str(excinfo)
    with raises(SignerError) as excinfo:
        VersionedTransaction(message, [keypair1, keypair2])
    assert "keypair-pubkey mismatch" in str(excinfo)

    tx = VersionedTransaction(message, [keypair0, keypair1])
    assert tx.verify_with_results() == [True, True]

    tx = VersionedTransaction(message, [keypair1, keypair0])
    assert tx.verify_with_results() == [True, True]


def nonced_transfer_tx() -> Tuple[Pubkey, Pubkey, VersionedTransaction]:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        advance_nonce_account(
            {"nonce_pubkey": nonce_pubkey, "authorized_pubkey": nonce_pubkey}
        ),
        transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
    ]
    message = Message(instructions, nonce_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    return (from_pubkey, nonce_pubkey, VersionedTransaction.from_legacy(tx))


def test_tx_uses_nonce_ok() -> None:
    _, _, tx = nonced_transfer_tx()
    assert tx.uses_durable_nonce()


def test_tx_uses_nonce_empty_ix_fail() -> None:
    assert not VersionedTransaction.default().uses_durable_nonce()


def test_tx_uses_nonce_first_prog_id_not_nonce_fail() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
        advance_nonce_account(
            {"nonce_pubkey": nonce_pubkey, "authorized_pubkey": nonce_pubkey}
        ),
    ]
    message = Message(instructions, from_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    versioned = VersionedTransaction.from_legacy(tx)
    assert not versioned.uses_durable_nonce()


def test_tx_uses_nonce_wrong_first_nonce_ix_fail() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        withdraw_nonce_account(
            {
                "nonce_pubkey": nonce_pubkey,
                "authorized_pubkey": nonce_pubkey,
                "to_pubkey": from_pubkey,
                "lamports": 42,
            }
        ),
        transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
    ]
    message = Message(instructions, nonce_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    versioned = VersionedTransaction.from_legacy(tx)
    assert not versioned.uses_durable_nonce()


def test_partial_signing() -> None:
    keypair0 = Keypair()
    keypair1 = Keypair()

    message = Message(
        [
            Instruction(
                Pubkey.new_unique(), b"", [AccountMeta(keypair1.pubkey(), True, False)]
            )
        ],
        keypair0.pubkey(),
    )
    signers = [keypair0, NullSigner(keypair1.pubkey())]
    partially_signed = VersionedTransaction(message, signers)
    serialized = bytes(partially_signed)
    deserialized = VersionedTransaction.from_bytes(serialized)
    assert deserialized == partially_signed
    deserialized_message = deserialized.message
    keypair1_sig_index = next(
        i
        for i, key in enumerate(deserialized_message.account_keys)
        if key == keypair1.pubkey()
    )
    sigs = deserialized.signatures
    sigs[keypair1_sig_index] = keypair1.sign_message(bytes(deserialized_message))
    deserialized.signatures = sigs
    fully_signed = VersionedTransaction(message, [keypair0, keypair1])
    assert deserialized.signatures == fully_signed.signatures
