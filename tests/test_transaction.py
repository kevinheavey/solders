from typing import Tuple, Optional, List, Union
from base64 import b64decode, b64encode
from based58 import b58encode
from pytest import raises
from solders.transaction import Transaction, SanitizeError
from solders.pubkey import Pubkey
from solders.keypair import Keypair
from solders.instruction import CompiledInstruction, Instruction, AccountMeta
from solders.hash import Hash
from solders.message import Message
from solders import system_program
from solders.signature import Signature
from solders.sysvar import RECENT_BLOCKHASHES
from solders.presigner import Presigner
from solders.errors import SignerError
from .utils import ZERO_BYTES

BLOCKHASH = Hash.from_string("EETubP5AKHgjPAhzPAFcb8BAY1hMH639CWCFTqi3hq1k")
SENDER = Keypair.from_seed(bytes([8] * Pubkey.LENGTH))
RECIPIENT = Pubkey.from_string("J3dxNj7nDRRqRRXuEMynDG57DkZK4jYRuv3Garmb1i99")


def get_program_id(tx: Transaction, instruction_index: int) -> Pubkey:
    message = tx.message
    instruction = message.instructions[instruction_index]
    return instruction.program_id(message.account_keys)


def test_refs() -> None:
    key = Keypair()
    key1 = Pubkey.new_unique()
    key2 = Pubkey.new_unique()
    prog1 = Pubkey.new_unique()
    prog2 = Pubkey.new_unique()
    instructions = [
        CompiledInstruction(3, b"", bytes([0, 1])),
        CompiledInstruction(4, b"", bytes([0, 2])),
    ]
    tx = Transaction.new_with_compiled_instructions(
        [key],
        [key1, key2],
        Hash.default(),
        [prog1, prog2],
        instructions,
    )
    tx.sanitize()

    assert tx.key(0, 0) == key.pubkey()
    assert tx.signer_key(0, 0) == key.pubkey()

    assert tx.key(1, 0) == key.pubkey()
    assert tx.signer_key(1, 0) == key.pubkey()

    assert tx.key(0, 1) == key1
    assert tx.signer_key(0, 1) is None

    assert tx.key(1, 1) == key2
    assert tx.signer_key(1, 1) is None

    assert tx.key(2, 0) is None
    assert tx.signer_key(2, 0) is None

    assert tx.key(0, 2) is None
    assert tx.signer_key(0, 2) is None

    assert get_program_id(tx, 0) == prog1
    assert get_program_id(tx, 1) == prog2


def test_refs_invalid_program_id() -> None:
    key = Keypair()
    instructions = [CompiledInstruction(1, b"", bytes([]))]
    tx = Transaction.new_with_compiled_instructions(
        [key],
        [],
        Hash.default(),
        [],
        instructions,
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"


def test_refs_invalid_account() -> None:
    key = Keypair()
    instructions = [CompiledInstruction(1, b"", bytes([2]))]
    tx = Transaction.new_with_compiled_instructions(
        [key],
        [],
        Hash.default(),
        [Pubkey.default()],
        instructions,
    )
    assert get_program_id(tx, 0) == Pubkey.default()
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"


def make_tx(ix: Instruction, key: Keypair) -> Transaction:
    message = Message([ix], key.pubkey())
    return Transaction.new_unsigned(message)


def with_changed_header(
    tx: Transaction,
    num_required_signatures: Optional[int] = None,
    num_readonly_signed_accounts: Optional[int] = None,
    num_readonly_unsigned_accounts: Optional[int] = None,
) -> Transaction:
    original_message = tx.message
    original_header = original_message.header
    message = Message.new_with_compiled_instructions(
        original_header.num_required_signatures
        if num_required_signatures is None
        else num_required_signatures,
        original_header.num_readonly_signed_accounts
        if num_readonly_signed_accounts is None
        else num_readonly_signed_accounts,
        original_header.num_readonly_unsigned_accounts
        if num_readonly_unsigned_accounts is None
        else num_readonly_unsigned_accounts,
        original_message.account_keys,
        original_message.recent_blockhash,
        original_message.instructions,
    )
    return Transaction.new_unsigned(message)


def with_changed_fields(
    tx: Transaction,
    program_id_index: Optional[int] = None,
    accounts_first_byte: Optional[int] = None,
    account_keys: Optional[List[Pubkey]] = None,
) -> Transaction:
    original_message = tx.message
    original_instruction = original_message.instructions[0]
    original_accounts = original_instruction.accounts
    header = original_message.header
    program_id_index_to_use = (
        original_instruction.program_id_index
        if program_id_index is None
        else program_id_index
    )
    accounts_to_use = (
        original_accounts
        if accounts_first_byte is None
        else bytes([accounts_first_byte, *list(original_accounts)[1:]])
    )
    new_instruction = CompiledInstruction(
        program_id_index_to_use,
        original_instruction.data,
        accounts_to_use,
    )
    account_keys_to_use = (
        original_message.account_keys if account_keys is None else account_keys
    )
    new_message = Message.new_with_compiled_instructions(
        header.num_required_signatures,
        header.num_readonly_signed_accounts,
        header.num_readonly_unsigned_accounts,
        account_keys_to_use,
        original_message.recent_blockhash,
        [new_instruction],
    )
    return Transaction.new_unsigned(new_message)


def test_sanitize_txs() -> None:
    key = Keypair()
    id0 = Pubkey.default()
    program_id = Pubkey.new_unique()
    ix = Instruction(
        program_id,
        ZERO_BYTES,
        [
            AccountMeta(key.pubkey(), True, True),
            AccountMeta(id0, True, True),
        ],
    )
    original_tx = Transaction.new_with_payer([ix], key.pubkey())
    tx = with_changed_header(original_tx)
    tx.sanitize()
    assert len(tx.message.account_keys) == 3
    tx = with_changed_header(original_tx, num_required_signatures=3)
    assert tx.message.header.num_required_signatures == 3
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_header(
        original_tx, num_readonly_signed_accounts=4, num_readonly_unsigned_accounts=0
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_header(
        original_tx, num_readonly_signed_accounts=2, num_readonly_unsigned_accounts=2
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_header(
        original_tx, num_readonly_signed_accounts=0, num_readonly_unsigned_accounts=4
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_fields(original_tx, program_id_index=3)
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_fields(original_tx, accounts_first_byte=3)
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_fields(tx, program_id_index=0)
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx_tmp = with_changed_header(
        original_tx, num_readonly_signed_accounts=2, num_readonly_unsigned_accounts=3
    )
    tx = with_changed_fields(
        tx_tmp, account_keys=tx_tmp.message.account_keys + [Pubkey.default()]
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"

    tx = with_changed_header(
        original_tx, num_readonly_signed_accounts=2, num_required_signatures=1
    )
    with raises(SanitizeError) as excinfo:
        tx.sanitize()
    assert excinfo.value.args[0] == "index out of bounds"


def create_sample_transaction() -> Transaction:
    keypair = Keypair.from_bytes(
        [
            48,
            83,
            2,
            1,
            1,
            48,
            5,
            6,
            3,
            43,
            101,
            112,
            4,
            34,
            4,
            32,
            255,
            101,
            36,
            24,
            124,
            23,
            167,
            21,
            132,
            204,
            155,
            5,
            185,
            58,
            121,
            75,
            156,
            227,
            116,
            193,
            215,
            38,
            142,
            22,
            8,
            14,
            229,
            239,
            119,
            93,
            5,
            218,
            161,
            35,
            3,
            33,
            0,
            36,
            100,
            158,
            252,
            33,
            161,
            97,
            185,
            62,
            89,
            99,
        ]
    )

    to = Pubkey(
        [
            1,
            1,
            1,
            4,
            5,
            6,
            7,
            8,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            8,
            7,
            6,
            5,
            4,
            1,
            1,
            1,
        ]
    )

    program_id = Pubkey(
        [
            2,
            2,
            2,
            4,
            5,
            6,
            7,
            8,
            9,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            9,
            8,
            7,
            6,
            5,
            4,
            2,
            2,
            2,
        ]
    )
    account_metas = [
        AccountMeta(keypair.pubkey(), True, True),
        AccountMeta(to, False, True),
    ]
    instruction = Instruction(program_id, bytes([1, 2, 3]), account_metas)
    message = Message([instruction], keypair.pubkey())
    return Transaction([keypair], message, Hash.default())


def test_transaction_serialize() -> None:
    tx = create_sample_transaction()
    ser = bytes(tx)
    deser = Transaction.from_bytes(ser)
    assert tx == deser


# / Detect changes to the serialized size of payment transactions, which affects TPS.


def test_transaction_minimum_serialized_size() -> None:
    alice_keypair = Keypair()
    alice_pubkey = alice_keypair.pubkey()
    bob_pubkey = Pubkey.new_unique()
    params = system_program.TransferParams(
        from_pubkey=alice_pubkey, to_pubkey=bob_pubkey, lamports=42
    )
    ix = system_program.transfer(params)
    u32_size = 4
    u64_size = 8
    expected_data_size = u32_size + u64_size
    assert expected_data_size == 12
    assert len(ix.data) == expected_data_size, "unexpected system instruction size"

    expected_instruction_size = 1 + 1 + len(ix.accounts) + 1 + expected_data_size
    assert expected_instruction_size == 17

    message = Message([ix], alice_pubkey)
    assert (
        len(bytes(message.instructions[0])) == expected_instruction_size
    ), "unexpected Instruction.serialized_size"

    tx = Transaction([alice_keypair], message, Hash.default())

    len_size = 1
    num_required_sigs_size = 1
    num_readonly_accounts_size = 2
    blockhash_size = 32
    signature_size = Signature.LENGTH
    pubkey_size = 32
    tx_sigs_len = len(tx.signatures)
    assert tx_sigs_len == 1
    account_keys_len = len(tx.message.account_keys)
    assert account_keys_len == 3
    expected_transaction_size = (
        len_size
        + (tx_sigs_len * signature_size)
        + num_required_sigs_size
        + num_readonly_accounts_size
        + len_size
        + (account_keys_len * pubkey_size)
        + blockhash_size
        + len_size
        + expected_instruction_size
    )
    assert expected_transaction_size == 215
    assert (
        len(bytes(tx)) == expected_transaction_size
    ), "unexpected serialized transaction size"


# / Detect binary changes in the serialized transaction data, which could have a downstream
# / affect on SDKs and applications


def test_sdk_serialize() -> None:
    assert bytes(create_sample_transaction()) == bytes(
        [
            1,
            71,
            59,
            9,
            187,
            190,
            129,
            150,
            165,
            21,
            33,
            158,
            72,
            87,
            110,
            144,
            120,
            79,
            238,
            132,
            134,
            105,
            39,
            102,
            116,
            209,
            29,
            229,
            154,
            36,
            105,
            44,
            172,
            118,
            131,
            22,
            124,
            131,
            179,
            142,
            176,
            27,
            117,
            160,
            89,
            102,
            224,
            204,
            1,
            252,
            141,
            2,
            136,
            0,
            37,
            218,
            225,
            129,
            92,
            154,
            250,
            59,
            97,
            178,
            10,
            1,
            0,
            1,
            3,
            156,
            227,
            116,
            193,
            215,
            38,
            142,
            22,
            8,
            14,
            229,
            239,
            119,
            93,
            5,
            218,
            161,
            35,
            3,
            33,
            0,
            36,
            100,
            158,
            252,
            33,
            161,
            97,
            185,
            62,
            89,
            99,
            1,
            1,
            1,
            4,
            5,
            6,
            7,
            8,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            9,
            8,
            7,
            6,
            5,
            4,
            1,
            1,
            1,
            2,
            2,
            2,
            4,
            5,
            6,
            7,
            8,
            9,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            9,
            8,
            7,
            6,
            5,
            4,
            2,
            2,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            2,
            2,
            0,
            1,
            3,
            1,
            2,
            3,
        ]
    )


def test_transaction_missing_key() -> None:
    keypair = Keypair()
    message = Message([], None)
    with raises(SignerError) as excinfo:
        Transaction.new_unsigned(message).sign([keypair], Hash.default())
    assert excinfo.value.args[0] == "keypair-pubkey mismatch"


def test_partial_sign_mismatched_key() -> None:
    keypair = Keypair()
    fee_payer = Pubkey.new_unique()
    ix = Instruction(
        Pubkey.default(),
        ZERO_BYTES,
        [AccountMeta(fee_payer, True, True)],
    )
    message = Message([ix], fee_payer)
    with raises(SignerError) as excinfo:
        Transaction.new_unsigned(message).partial_sign([keypair], Hash.default())
    assert excinfo.value.args[0] == "keypair-pubkey mismatch"


def test_partial_sign() -> None:
    keypair0 = Keypair()
    keypair1 = Keypair()
    keypair2 = Keypair()
    ix = Instruction(
        Pubkey.default(),
        ZERO_BYTES,
        [
            AccountMeta(keypair0.pubkey(), True, True),
            AccountMeta(keypair1.pubkey(), True, True),
            AccountMeta(keypair2.pubkey(), True, True),
        ],
    )
    message = Message([ix], keypair0.pubkey())
    tx = Transaction.new_unsigned(message)

    tx.partial_sign([keypair0, keypair2], Hash.default())
    assert not tx.is_signed()
    tx.partial_sign([keypair1], Hash.default())
    assert tx.is_signed()

    hash_ = Hash.hash(bytes([1]))
    tx.partial_sign([keypair1], hash_)
    assert not tx.is_signed()
    tx.partial_sign([keypair0, keypair2], hash_)
    assert tx.is_signed()


def test_transaction_missing_keypair() -> None:
    program_id = Pubkey.default()
    keypair0 = Keypair()
    id0 = keypair0.pubkey()
    ix = Instruction(program_id, ZERO_BYTES, [AccountMeta(id0, True, True)])
    message = Message([ix], id0)
    with raises(SignerError) as excinfo:
        Transaction.new_unsigned(message).sign([], Hash.default())
    assert excinfo.value.args[0] == "not enough signers"


def test_transaction_wrong_key() -> None:
    program_id = Pubkey.default()
    keypair0 = Keypair()
    wrong_id = Pubkey.default()
    ix = Instruction(program_id, ZERO_BYTES, [AccountMeta(wrong_id, True, True)])
    message = Message([ix], wrong_id)
    with raises(SignerError) as excinfo:
        Transaction.new_unsigned(message).sign([keypair0], Hash.default())
    assert excinfo.value.args[0] == "keypair-pubkey mismatch"


def test_transaction_correct_key() -> None:
    program_id = Pubkey.default()
    keypair0 = Keypair()
    id0 = keypair0.pubkey()
    ix = Instruction(program_id, ZERO_BYTES, [AccountMeta(id0, True, True)])
    message = Message([ix], id0)
    tx = Transaction.new_unsigned(message)
    tx.sign([keypair0], Hash.default())
    assert tx.message.instructions[0] == CompiledInstruction(1, ZERO_BYTES, bytes([0]))
    assert tx.is_signed()


def test_transaction_instruction_with_duplicate_keys() -> None:
    program_id = Pubkey.default()
    keypair0 = Keypair()
    id0 = keypair0.pubkey()
    id1 = Pubkey.new_unique()
    ix = Instruction(
        program_id,
        ZERO_BYTES,
        [
            AccountMeta(id0, True, True),
            AccountMeta(id1, False, True),
            AccountMeta(id0, False, True),
            AccountMeta(id1, False, True),
        ],
    )
    message = Message([ix], id0)
    tx = Transaction.new_unsigned(message)
    tx.sign([keypair0], Hash.default())
    assert tx.message.instructions[0] == CompiledInstruction(
        2, ZERO_BYTES, bytes([0, 1, 0, 1])
    )
    assert tx.is_signed()


def test_sign_dyn_keypairs() -> None:
    program_id = Pubkey.default()
    keypair = Keypair()
    pubkey = keypair.pubkey()
    presigner_keypair = Keypair()
    presigner_pubkey = presigner_keypair.pubkey()

    ix = Instruction(
        program_id,
        ZERO_BYTES,
        [
            AccountMeta(pubkey, True, True),
            AccountMeta(presigner_pubkey, True, True),
        ],
    )
    message = Message([ix], pubkey)
    tx = Transaction.new_unsigned(message)

    presigner_sig = presigner_keypair.sign_message(tx.message_data())
    presigner = Presigner(presigner_pubkey, presigner_sig)

    signers: List[Union[Keypair, Presigner]] = [keypair, presigner]

    tx.sign(signers, Hash.default())
    assert tx.signatures[0] == keypair.sign_message(tx.message_data())
    assert tx.signatures[1] == presigner_sig

    # Wrong key should error
    another_pubkey = Pubkey.new_unique()
    ix = Instruction(
        program_id,
        ZERO_BYTES,
        [
            AccountMeta(another_pubkey, True, True),
            AccountMeta(presigner_pubkey, True, True),
        ],
    )
    message = Message([ix], another_pubkey)
    tx = Transaction.new_unsigned(message)
    with raises(SignerError) as excinfo:
        tx.sign(signers, Hash.default())
    assert excinfo.value.args[0] == "keypair-pubkey mismatch"
    assert tx.signatures == [Signature.default(), Signature.default()]


def test_offline_multisig() -> None:
    """Test signing a transaction, serializing it, then having someone else deserialize and sign it."""
    program_id = Pubkey.default()
    alice_keypair = Keypair()
    bob_keypair = Keypair()
    alice_pubkey = alice_keypair.pubkey()
    bob_pubkey = bob_keypair.pubkey()

    ix = Instruction(
        program_id,
        ZERO_BYTES,
        [
            AccountMeta(alice_pubkey, True, True),
            AccountMeta(bob_pubkey, True, True),
        ],
    )
    message = Message([ix], bob_pubkey)
    tx = Transaction.new_unsigned(message)
    tx.partial_sign([bob_keypair], BLOCKHASH)
    assert tx.signatures[0] == bob_keypair.sign_message(tx.message_data())
    serialized = bytes(tx)
    deserialized = Transaction.from_bytes(serialized)

    deserialized.partial_sign([alice_keypair], BLOCKHASH)
    assert deserialized.signatures[0] == bob_keypair.sign_message(tx.message_data())
    assert deserialized.signatures[1] == alice_keypair.sign_message(tx.message_data())
    assert deserialized.message == tx.message


def nonced_transfer_tx() -> Tuple[Pubkey, Pubkey, Transaction]:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        system_program.advance_nonce_account(
            system_program.AdvanceNonceAccountParams(
                nonce_pubkey=nonce_pubkey, authorized_pubkey=nonce_pubkey
            )
        ),
        system_program.transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
    ]
    message = Message(instructions, nonce_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    return (from_pubkey, nonce_pubkey, tx)


def test_tx_uses_nonce_ok() -> None:
    (_, _, tx) = nonced_transfer_tx()
    assert tx.uses_durable_nonce() is not None


def test_tx_uses_nonce_empty_ix_fail() -> None:
    assert Transaction.default().uses_durable_nonce() is None


def test_tx_uses_nonce_bad_prog_id_idx_fail() -> None:
    (_, _, tx) = nonced_transfer_tx()
    with_changed_pid_index = with_changed_fields(tx, program_id_index=255)
    assert with_changed_pid_index.uses_durable_nonce() is None


def test_tx_uses_nonce_first_prog_id_not_nonce_fail() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        system_program.transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
        system_program.advance_nonce_account(
            {"nonce_pubkey": nonce_pubkey, "authorized_pubkey": nonce_pubkey}
        ),
    ]
    message = Message(instructions, from_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    assert tx.uses_durable_nonce() is None


def test_tx_uses_ro_nonce_account() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    account_metas = [
        AccountMeta(nonce_pubkey, False, False),
        AccountMeta(RECENT_BLOCKHASHES, False, False),
        AccountMeta(nonce_pubkey, True, False),
    ]
    advance_nonce_account_idx = b"\x04\x00\x00\x00"
    nonce_instruction = Instruction(
        system_program.ID,
        advance_nonce_account_idx,
        account_metas,
    )
    tx = Transaction.new_signed_with_payer(
        [nonce_instruction],
        from_pubkey,
        [from_keypair, nonce_keypair],
        Hash.default(),
    )
    assert tx.uses_durable_nonce() is None


def test_tx_uses_nonce_wrong_first_nonce_ix_fail() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    nonce_keypair = Keypair()
    nonce_pubkey = nonce_keypair.pubkey()
    instructions = [
        system_program.withdraw_nonce_account(
            {
                "nonce_pubkey": nonce_pubkey,
                "authorized_pubkey": nonce_pubkey,
                "to_pubkey": from_pubkey,
                "lamports": 42,
            },
        ),
        system_program.transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": nonce_pubkey, "lamports": 42}
        ),
    ]
    message = Message(instructions, nonce_pubkey)
    tx = Transaction([from_keypair, nonce_keypair], message, Hash.default())
    assert tx.uses_durable_nonce() is None


def test_get_nonce_pub_from_ix_ok() -> None:
    (_, nonce_pubkey, tx) = nonced_transfer_tx()
    nonce_ix = tx.uses_durable_nonce()
    assert nonce_ix is not None
    assert tx.get_nonce_pubkey_from_instruction(nonce_ix) == nonce_pubkey


def test_get_nonce_pub_from_ix_no_accounts_fail() -> None:
    (_, _, tx) = nonced_transfer_tx()
    nonce_ix = tx.uses_durable_nonce()
    assert nonce_ix is not None
    nonce_ix = tx.uses_durable_nonce()
    assert nonce_ix is not None
    nonce_ix.accounts = b""
    assert tx.get_nonce_pubkey_from_instruction(nonce_ix) is None


def test_get_nonce_pub_from_ix_bad_acc_idx_fail() -> None:
    (_, _, tx) = nonced_transfer_tx()
    nonce_ix = tx.uses_durable_nonce()
    assert nonce_ix is not None
    nonce_ix.accounts = bytes([255, *list(nonce_ix.accounts[1:])])
    assert tx.get_nonce_pubkey_from_instruction(nonce_ix) is None


def test_tx_keypair_pubkey_mismatch() -> None:
    from_keypair = Keypair()
    from_pubkey = from_keypair.pubkey()
    to_pubkey = Pubkey.new_unique()
    instructions = [
        system_program.transfer(
            {"from_pubkey": from_pubkey, "to_pubkey": to_pubkey, "lamports": 42}
        )
    ]
    tx = Transaction.new_with_payer(instructions, from_pubkey)
    unused_keypair = Keypair()
    with raises(SignerError) as excinfo:
        tx.partial_sign([from_keypair, unused_keypair], Hash.default())
    assert excinfo.value.args[0] == "keypair-pubkey mismatch"


# The below tests are ported from solana-py tests.


def test_dedup_signatures() -> None:
    """Test signature deduplication."""
    kp1, kp2 = Keypair(), Keypair()
    transfer1 = system_program.transfer(
        {"from_pubkey": kp1.pubkey(), "to_pubkey": kp2.pubkey(), "lamports": 123}
    )
    transfer2 = system_program.transfer(
        {"from_pubkey": kp1.pubkey(), "to_pubkey": kp2.pubkey(), "lamports": 123}
    )
    instructions = [transfer1, transfer2]
    message = Message(instructions)
    txn = Transaction.new_unsigned(message)
    txn.sign([kp1], BLOCKHASH)


def test_wire_format_and_deserialize() -> None:
    """Test serialize/derialize transaction to/from wire format."""
    transfer = system_program.transfer(
        {
            "from_pubkey": SENDER.pubkey(),
            "to_pubkey": RECIPIENT,
            "lamports": 49,
        }
    )
    message = Message([transfer], SENDER.pubkey())
    expected_txn = Transaction.new_unsigned(message)
    expected_txn.sign([SENDER], BLOCKHASH)
    wire_txn = b64decode(
        b"AVuErQHaXv0SG0/PchunfxHKt8wMRfMZzqV0tkC5qO6owYxWU2v871AoWywGoFQr4z+q/7mE8lIufNl/kxj+nQ0BAAEDE5j2"
        b"LG0aRXxRumpLXz29L2n8qTIWIY3ImX5Ba9F9k8r9Q5/Mtmcn8onFxt47xKj+XdXXd3C8j/FcPu7csUrz/AAAAAAAAAAAAAAA"
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAxJrndgN4IFTxep3s6kO0ROug7bEsbx0xxuDkqEvwUusBAgIAAQwCAAAAMQAAAAAAAAA="
    )
    txn = Transaction.from_bytes(wire_txn)
    assert txn == expected_txn
    assert wire_txn == bytes(expected_txn)


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
    transfer = system_program.transfer(
        {
            "from_pubkey": SENDER.pubkey(),
            "to_pubkey": RECIPIENT,
            "lamports": 49,
        }
    )
    message = Message([transfer])
    txn = Transaction.new_unsigned(message)
    assert (
        txn.signatures == [Signature.default()] * message.header.num_required_signatures
    )
    assert Transaction.from_bytes(bytes(txn)) == txn

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
    assert bytes(txn_with_payer) == expected_serialization
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
        system_program.transfer(
            {
                "from_pubkey": signer_one.pubkey(),
                "to_pubkey": receiver_one.pubkey(),
                "lamports": 2_000_000,
            }
        ),
        system_program.transfer(
            {
                "from_pubkey": signer_two.pubkey(),
                "to_pubkey": receiver_two.pubkey(),
                "lamports": 2_000_000,
            }
        ),
        system_program.transfer(
            {
                "from_pubkey": signer_three.pubkey(),
                "to_pubkey": receiver_three.pubkey(),
                "lamports": 2_000_000,
            }
        ),
    ]
    fee_payer = signer_one
    message = Message.new_with_blockhash(instructions, fee_payer.pubkey(), BLOCKHASH)
    signers = [signer_one, signer_two, signer_three]
    sorted_signers = sorted(
        [x.pubkey() for x in signers],
        key=str,
    )
    sorted_signers_excluding_fee_payer = [
        x for x in sorted_signers if str(x) != str(fee_payer.pubkey())
    ]
    receivers = [receiver_one, receiver_two, receiver_three]
    sorted_receivers = sorted(
        [x.pubkey() for x in receivers],
        key=str,
    )
    txn = Transaction.new_unsigned(message)
    tx_msg = txn.message

    js_msg_b64_check = b"AwABBwZtbiRMvgQjcE2kVx9yon8XqPSO5hwc2ApflnOZMu0Qo9G5/xbhB0sp8/03Rv9x4MKSkQ+k4LB6lNLvCgKZ/ju/aw+EyQpTObVa3Xm+NA1gSTzutgFCTfkDto/0KtuIHHAMpKRb92NImxKeWQJ2/291j6nTzFj1D6nW25p7TofHmVsGt8uFnTv7+8vsWZ0uN7azdxa+jCIIm4WzKK+4uKfX39t5UA7S1soBQaJkTGOQkSbBo39gIjDkbW0TrevslgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAxJrndgN4IFTxep3s6kO0ROug7bEsbx0xxuDkqEvwUusDBgIABAwCAAAAgIQeAAAAAAAGAgIFDAIAAACAhB4AAAAAAAYCAQMMAgAAAICEHgAAAAAA"  # noqa: E501 pylint: disable=line-too-long

    assert b64encode(bytes(tx_msg)) == js_msg_b64_check

    # Transaction should organize AccountMetas by PublicKey
    assert tx_msg.account_keys[0] == fee_payer.pubkey()
    assert tx_msg.account_keys[1] == sorted_signers_excluding_fee_payer[0]
    assert tx_msg.account_keys[2] == sorted_signers_excluding_fee_payer[1]
    assert tx_msg.account_keys[3] == sorted_receivers[0]
    assert tx_msg.account_keys[4] == sorted_receivers[1]
    assert tx_msg.account_keys[5] == sorted_receivers[2]
