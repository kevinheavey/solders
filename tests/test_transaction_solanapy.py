"""These tests are ported from test_transaction.py in solana-py"""
from base64 import b64decode, b64encode

import pytest
from based58 import b58encode

from solders import (
    SystemProgram,
    Transaction,
    CompiledInstruction,
    Message,
    MessageHeader,
    Keypair,
    Pubkey,
    Hash,
    Signature,
)

BLOCKHASH = Hash.from_string("EETubP5AKHgjPAhzPAFcb8BAY1hMH639CWCFTqi3hq1k")
SENDER = Keypair.from_seed(bytes([8] * Pubkey.LENGTH))
RECIPIENT = Pubkey.from_string("J3dxNj7nDRRqRRXuEMynDG57DkZK4jYRuv3Garmb1i99")


def test_dedup_signatures():
    """Test signature deduplication."""
    kp1, kp2 = Keypair(), Keypair()
    transfer1 = SystemProgram.transfer(
        from_pubkey=kp1.pubkey(), to_pubkey=kp2.pubkey(), lamports=123
    )
    transfer2 = SystemProgram.transfer(
        from_pubkey=kp1.pubkey(), to_pubkey=kp2.pubkey(), lamports=123
    )
    instructions = [transfer1, transfer2]
    message = Message(instructions)
    txn = Transaction.new_unsigned(message)
    txn.sign([kp1], BLOCKHASH)


def test_wire_format_and_deserialize() -> None:
    """Test serialize/derialize transaction to/from wire format."""
    transfer = SystemProgram.transfer(
        from_pubkey=SENDER.pubkey(),
        to_pubkey=RECIPIENT,
        lamports=49,
    )
    message = Message([transfer], SENDER.pubkey())
    expected_txn = Transaction.new_unsigned(message)
    expected_txn.sign([SENDER], BLOCKHASH)
    wire_txn = b64decode(
        b"AVuErQHaXv0SG0/PchunfxHKt8wMRfMZzqV0tkC5qO6owYxWU2v871AoWywGoFQr4z+q/7mE8lIufNl/kxj+nQ0BAAEDE5j2"
        b"LG0aRXxRumpLXz29L2n8qTIWIY3ImX5Ba9F9k8r9Q5/Mtmcn8onFxt47xKj+XdXXd3C8j/FcPu7csUrz/AAAAAAAAAAAAAAA"
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAxJrndgN4IFTxep3s6kO0ROug7bEsbx0xxuDkqEvwUusBAgIAAQwCAAAAMQAAAAAAAAA="
    )
    txn = Transaction.deserialize(wire_txn)
    assert txn == expected_txn
    assert wire_txn == expected_txn.serialize()


def test_populate() -> None:
    """Test populating transaction with a message and two signatures."""
    account_keys = [
        Pubkey(bytes([i + 1]).rjust(Pubkey.LENGTH, b"\0")) for i in range(5)
    ]
    msg = Message.new_with_compiled_instructions(
        num_required_signatures=2,
        num_readonly_signed_accounts=3,
        num_readonly_unsigned_accounts=0,
        account_keys=account_keys,
        recent_blockhash=BLOCKHASH,
        instructions=[
            CompiledInstruction(
                accounts=bytes([1, 2, 3]),
                data=b58encode(bytes([9] * 5)),
                program_id_index=4,
            )
        ],
    )
    signatures = [
        Signature(bytes([1] * Signature.LENGTH)),
        Signature(bytes([2] * Signature.LENGTH)),
    ]
    transaction = Transaction.populate(msg, signatures)
    assert len(transaction.message.instructions) == len(msg.instructions)
    assert len(transaction.signatures) == len(signatures)
    assert transaction.message.recent_blockhash == msg.recent_blockhash


def test_serialize_unsigned_transaction() -> None:
    """Test to serialize an unsigned transaction."""
    transfer = SystemProgram.transfer(
        from_pubkey=SENDER.pubkey(),
        to_pubkey=RECIPIENT,
        lamports=49,
    )
    message = Message([transfer])
    txn = Transaction.new_unsigned(message)
    assert (
        txn.signatures == [Signature.default()] * message.header.num_required_signatures
    )
    assert Transaction.deserialize(txn.serialize()) == txn

    message_with_payer = Message([transfer], SENDER.pubkey())
    txn_with_payer = Transaction.new_signed_with_payer(
        [transfer], SENDER.pubkey(), [SENDER], BLOCKHASH
    )
    # Properly signed transaction succeeds
    assert len(txn_with_payer.message.instructions) == 1
    expected_serialization = b64decode(
        b"AVuErQHaXv0SG0/PchunfxHKt8wMRfMZzqV0tkC5qO6owYxWU2v871AoWywGoFQr4z+q/7mE8lIufNl/kxj+nQ0BAAEDE5j2"
        b"LG0aRXxRumpLXz29L2n8qTIWIY3ImX5Ba9F9k8r9Q5/Mtmcn8onFxt47xKj+XdXXd3C8j/FcPu7csUrz/AAAAAAAAAAAAAAA"
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAxJrndgN4IFTxep3s6kO0ROug7bEsbx0xxuDkqEvwUusBAgIAAQwCAAAAMQAAAAAAAAA="
    )
    assert txn_with_payer.serialize() == expected_serialization
    assert len(txn_with_payer.signatures) == 1


def test_sort_account_metas() -> None:
    """Test AccountMeta sorting."""

    # S6EA7XsNyxg4yx4DJRMm7fP21jgZb1fuzBAUGhgVtkP
    signer_one = Keypair.from_seed(
        bytes(
            [
                216,
                214,
                184,
                213,
                199,
                75,
                129,
                160,
                237,
                96,
                96,
                228,
                46,
                251,
                146,
                3,
                71,
                162,
                37,
                117,
                121,
                70,
                143,
                16,
                128,
                78,
                53,
                189,
                222,
                230,
                165,
                249,
            ]
        )
    )

    # BKdt9U6V922P17ui81dzLoqgSY2B5ds1UD13rpwFB2zi
    receiver_one = Keypair.from_seed(
        bytes(
            [
                3,
                140,
                94,
                243,
                0,
                38,
                92,
                138,
                52,
                79,
                153,
                83,
                42,
                236,
                220,
                82,
                227,
                187,
                101,
                104,
                126,
                159,
                103,
                100,
                29,
                183,
                242,
                68,
                144,
                184,
                114,
                211,
            ]
        )
    )

    # DtDZCnXEN69n5W6rN5SdJFgedrWdK8NV9bsMiJekNRyu
    signer_two = Keypair.from_seed(
        bytes(
            [
                177,
                182,
                154,
                154,
                5,
                145,
                253,
                138,
                211,
                126,
                222,
                195,
                21,
                64,
                117,
                211,
                225,
                47,
                115,
                31,
                247,
                242,
                80,
                195,
                38,
                8,
                236,
                155,
                255,
                27,
                20,
                142,
            ]
        )
    )

    # FXgds3n6SNCoVVV4oELSumv8nKzAfqSgmeu7cNPikKFT
    receiver_two = Keypair.from_seed(
        bytes(
            [
                180,
                204,
                139,
                131,
                244,
                6,
                180,
                121,
                191,
                193,
                45,
                109,
                198,
                50,
                163,
                140,
                34,
                4,
                172,
                76,
                129,
                45,
                194,
                83,
                192,
                112,
                76,
                58,
                32,
                174,
                49,
                248,
            ]
        )
    )

    # C2UwQHqJ3BmEJHSMVmrtZDQGS2fGv8fZrWYGi18nHF5k
    signer_three = Keypair.from_seed(
        bytes(
            [
                29,
                79,
                73,
                16,
                137,
                117,
                183,
                2,
                131,
                0,
                209,
                142,
                134,
                100,
                190,
                35,
                95,
                220,
                200,
                163,
                247,
                237,
                161,
                70,
                226,
                223,
                100,
                148,
                49,
                202,
                154,
                180,
            ]
        )
    )

    # 8YPqwYXZtWPd31puVLEUPamS4wTv6F89n8nXDA5Ce2Bg
    receiver_three = Keypair.from_seed(
        bytes(
            [
                167,
                102,
                49,
                166,
                202,
                0,
                132,
                182,
                239,
                182,
                252,
                59,
                25,
                103,
                76,
                217,
                65,
                215,
                210,
                159,
                168,
                50,
                10,
                229,
                144,
                231,
                221,
                74,
                182,
                161,
                52,
                193,
            ]
        )
    )
    instructions = [
        SystemProgram.transfer(
            from_pubkey=signer_one.pubkey(),
            to_pubkey=receiver_one.pubkey(),
            lamports=2_000_000,
        ),
        SystemProgram.transfer(
            from_pubkey=signer_two.pubkey(),
            to_pubkey=receiver_two.pubkey(),
            lamports=2_000_000,
        ),
        SystemProgram.transfer(
            from_pubkey=signer_three.pubkey(),
            to_pubkey=receiver_three.pubkey(),
            lamports=2_000_000,
        ),
    ]
    fee_payer = signer_one
    message = Message.new_with_blockhash(instructions, fee_payer.pubkey(), BLOCKHASH)
    sorted_signers = sorted(
        [x.pubkey() for x in [signer_one, signer_two, signer_three]],
        key=lambda x: str(x),
    )
    sorted_signers_excluding_fee_payer = [
        x for x in sorted_signers if str(x) != str(fee_payer.pubkey())
    ]
    sorted_receivers = sorted(
        [x.pubkey() for x in [receiver_one, receiver_two, receiver_three]],
        key=lambda x: str(x),
    )
    txn = Transaction.new_unsigned(message)
    tx_msg = txn.message

    js_msg_b64_check = b"AwABBwZtbiRMvgQjcE2kVx9yon8XqPSO5hwc2ApflnOZMu0Qo9G5/xbhB0sp8/03Rv9x4MKSkQ+k4LB6lNLvCgKZ/ju/aw+EyQpTObVa3Xm+NA1gSTzutgFCTfkDto/0KtuIHHAMpKRb92NImxKeWQJ2/291j6nTzFj1D6nW25p7TofHmVsGt8uFnTv7+8vsWZ0uN7azdxa+jCIIm4WzKK+4uKfX39t5UA7S1soBQaJkTGOQkSbBo39gIjDkbW0TrevslgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAxJrndgN4IFTxep3s6kO0ROug7bEsbx0xxuDkqEvwUusDBgIABAwCAAAAgIQeAAAAAAAGAgIFDAIAAACAhB4AAAAAAAYCAQMMAgAAAICEHgAAAAAA"  # noqa: E501 pylint: disable=line-too-long

    assert b64encode(tx_msg.serialize()) == js_msg_b64_check

    # Transaction should organize AccountMetas by PublicKey
    assert tx_msg.account_keys[0] == fee_payer.pubkey()
    assert tx_msg.account_keys[1] == sorted_signers_excluding_fee_payer[0]
    assert tx_msg.account_keys[2] == sorted_signers_excluding_fee_payer[1]
    assert tx_msg.account_keys[3] == sorted_receivers[0]
    assert tx_msg.account_keys[4] == sorted_receivers[1]
    assert tx_msg.account_keys[5] == sorted_receivers[2]
