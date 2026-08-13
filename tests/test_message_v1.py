from pytest import fixture, raises
from solders.hash import Hash
from solders.instruction import AccountMeta, CompiledInstruction, Instruction
from solders.keypair import Keypair
from solders.message import (
    Message,
    MessageAddressTableLookup,
    MessageHeader,
    MessageV0,
    MessageV1,
    TransactionConfig,
    from_bytes_versioned,
    to_bytes_versioned,
    v0,
    v1,
)
from solders.pubkey import Pubkey
from solders.transaction import SanitizeError, Transaction, VersionedTransaction


@fixture
def default_header_with_one_req_signature() -> MessageHeader:
    default = MessageHeader.default()
    return MessageHeader(
        num_required_signatures=1,
        num_readonly_signed_accounts=default.num_readonly_signed_accounts,
        num_readonly_unsigned_accounts=default.num_readonly_unsigned_accounts,
    )


def test_transaction_config_defaults() -> None:
    config = TransactionConfig.default()
    assert config.priority_fee is None
    assert config.compute_unit_limit is None
    assert config.loaded_accounts_data_size_limit is None
    assert config.heap_size is None
    assert config.size() == 0
    assert TransactionConfig() == config


def test_transaction_config_fields() -> None:
    config = TransactionConfig(
        priority_fee=1000,
        compute_unit_limit=200_000,
        loaded_accounts_data_size_limit=64 * 1024,
        heap_size=64 * 1024,
    )
    assert config.priority_fee == 1000
    assert config.compute_unit_limit == 200_000
    assert config.loaded_accounts_data_size_limit == 64 * 1024
    assert config.heap_size == 64 * 1024
    # u64 + three u32s
    assert config.size() == 20
    assert TransactionConfig.from_bytes(bytes(config)) == config
    assert TransactionConfig.from_json(config.to_json()) == config


def test_flat_names_alias_the_submodules() -> None:
    assert v1.Message is MessageV1
    assert v0.Message is MessageV0
    assert v1.TransactionConfig is TransactionConfig
    assert v0.MessageAddressTableLookup is MessageAddressTableLookup


def test_constants() -> None:
    assert v1.V1_PREFIX == 0x81
    assert v1.MAX_TRANSACTION_SIZE == 4096
    assert v1.MAX_ADDRESSES == 64
    assert v1.MAX_INSTRUCTIONS == 64
    assert v1.MAX_SIGNATURES == 12
    assert v1.MIN_HEAP_SIZE == 32768
    assert v1.MAX_HEAP_SIZE == 262144
    assert v1.DEFAULT_HEAP_SIZE == v1.MIN_HEAP_SIZE


def test_getters(default_header_with_one_req_signature: MessageHeader) -> None:
    blockhash = Hash.new_unique()
    keys = [Pubkey.new_unique(), Pubkey.new_unique()]
    config = TransactionConfig(compute_unit_limit=200_000)
    instructions = [
        CompiledInstruction(program_id_index=1, accounts=bytes([0]), data=bytes([1, 2]))
    ]
    msg = MessageV1(
        header=default_header_with_one_req_signature,
        config=config,
        lifetime_specifier=blockhash,
        account_keys=keys,
        instructions=instructions,
    )
    assert msg.header == default_header_with_one_req_signature
    assert msg.config == config
    assert msg.lifetime_specifier == blockhash
    assert msg.recent_blockhash == blockhash  # alias
    assert msg.account_keys == keys
    assert msg.instructions == instructions
    assert msg.fee_payer() == keys[0]
    assert msg.is_signer(0)
    assert not msg.is_signer(1)


def test_default() -> None:
    msg = MessageV1.default()
    assert msg.account_keys == []
    assert msg.instructions == []
    assert msg.config == TransactionConfig.default()
    assert msg.lifetime_specifier == Hash.default()


def test_try_compile() -> None:
    keys = [Pubkey.new_unique() for _ in range(4)]
    payer = keys[0]
    program_id = keys[3]
    instructions = [
        Instruction(
            program_id,
            accounts=[
                AccountMeta(keys[1], True, True),
                AccountMeta(keys[2], False, False),
            ],
            data=bytes([]),
        )
    ]
    blockhash = Hash.new_unique()
    config = TransactionConfig(compute_unit_limit=200_000)
    msg = MessageV1.try_compile(payer, instructions, blockhash, config)
    assert msg == MessageV1(
        header=MessageHeader(
            num_required_signatures=2,
            num_readonly_signed_accounts=0,
            num_readonly_unsigned_accounts=2,
        ),
        config=config,
        lifetime_specifier=blockhash,
        account_keys=[keys[0], keys[1], keys[2], program_id],
        instructions=[
            CompiledInstruction(
                program_id_index=3, accounts=bytes([1, 2]), data=bytes([])
            )
        ],
    )
    assert msg.fee_payer() == payer


def test_try_compile_without_config() -> None:
    payer = Pubkey.new_unique()
    program_id = Pubkey.new_unique()
    instructions = [Instruction(program_id, bytes([]), [])]
    blockhash = Hash.new_unique()
    msg = MessageV1.try_compile(payer, instructions, blockhash)
    assert msg.config == TransactionConfig.default()
    assert msg == MessageV1.try_compile(
        payer, instructions, blockhash, TransactionConfig()
    )


def test_bytes_roundtrip() -> None:
    payer = Pubkey.new_unique()
    program_id = Pubkey.new_unique()
    instructions = [Instruction(program_id, bytes([1, 2, 3]), [])]
    msg = MessageV1.try_compile(
        payer, instructions, Hash.new_unique(), TransactionConfig(priority_fee=5)
    )
    serialized = bytes(msg)
    assert MessageV1.from_bytes(serialized) == msg
    assert len(serialized) == msg.size()  # size() excludes the version prefix


def test_json_roundtrip() -> None:
    msg = MessageV1.try_compile(
        Pubkey.new_unique(),
        [Instruction(Pubkey.new_unique(), bytes([1]), [])],
        Hash.new_unique(),
        TransactionConfig(heap_size=64 * 1024),
    )
    assert MessageV1.from_json(msg.to_json()) == msg


def test_versioned_roundtrip() -> None:
    msg = MessageV1.try_compile(
        Pubkey.new_unique(),
        [Instruction(Pubkey.new_unique(), bytes([1]), [])],
        Hash.new_unique(),
        TransactionConfig(compute_unit_limit=1234),
    )
    serialized = to_bytes_versioned(msg)
    assert serialized[0] == v1.V1_PREFIX
    assert serialized[1:] == bytes(msg)
    deserialized = from_bytes_versioned(serialized)
    assert isinstance(deserialized, MessageV1)
    assert deserialized == msg


def test_versioned_roundtrip_still_works_for_legacy_and_v0() -> None:
    payer = Pubkey.new_unique()
    instructions = [Instruction(Pubkey.new_unique(), bytes([1]), [])]
    legacy = Message(instructions, payer)
    assert from_bytes_versioned(to_bytes_versioned(legacy)) == legacy
    # legacy messages carry no version prefix
    assert to_bytes_versioned(legacy) == bytes(legacy)
    v0 = MessageV0.try_compile(payer, instructions, [], Hash.new_unique())
    assert from_bytes_versioned(to_bytes_versioned(v0)) == v0
    assert to_bytes_versioned(v0) == bytes([0x80]) + bytes(v0)


def test_versioned_encoding_matches_bincode_for_legacy() -> None:
    # `Transaction` still goes through bincode, so it pins the encoding.
    payer = Keypair()
    tx = Transaction(
        [payer],
        Message([Instruction(Pubkey.new_unique(), bytes([1]), [])], payer.pubkey()),
        Hash.new_unique(),
    )
    assert bytes(VersionedTransaction.from_legacy(tx)) == bytes(tx)


def test_sanitize(default_header_with_one_req_signature: MessageHeader) -> None:
    MessageV1(
        header=default_header_with_one_req_signature,
        config=TransactionConfig.default(),
        lifetime_specifier=Hash.default(),
        account_keys=[Pubkey.new_unique(), Pubkey.new_unique()],
        instructions=[
            CompiledInstruction(program_id_index=1, accounts=bytes([0]), data=bytes([]))
        ],
    ).sanitize()


def test_sanitize_without_signer() -> None:
    msg = MessageV1(
        header=MessageHeader.default(),
        config=TransactionConfig.default(),
        lifetime_specifier=Hash.default(),
        account_keys=[Pubkey.new_unique()],
        instructions=[],
    )
    with raises(SanitizeError):
        msg.sanitize()


def test_sanitize_with_invalid_ix_program_id(
    default_header_with_one_req_signature: MessageHeader,
) -> None:
    msg = MessageV1(
        header=default_header_with_one_req_signature,
        config=TransactionConfig.default(),
        lifetime_specifier=Hash.default(),
        account_keys=[Pubkey.new_unique()],
        instructions=[
            CompiledInstruction(program_id_index=2, accounts=bytes([]), data=bytes([]))
        ],
    )
    with raises(SanitizeError):
        msg.sanitize()


def test_validate_too_many_signatures(
    default_header_with_one_req_signature: MessageHeader,
) -> None:
    too_many = v1.MAX_SIGNATURES + 1
    msg = MessageV1(
        header=MessageHeader(
            num_required_signatures=too_many,
            num_readonly_signed_accounts=0,
            num_readonly_unsigned_accounts=0,
        ),
        config=TransactionConfig.default(),
        lifetime_specifier=Hash.default(),
        account_keys=[Pubkey.new_unique() for _ in range(too_many)],
        instructions=[],
    )
    with raises(v1.MessageError):
        msg.validate()


def test_validate_ok() -> None:
    msg = MessageV1.try_compile(
        Pubkey.new_unique(),
        [Instruction(Pubkey.new_unique(), bytes([1]), [])],
        Hash.new_unique(),
    )
    msg.validate()


def test_hash_is_stable() -> None:
    msg = MessageV1.try_compile(
        Pubkey.new_unique(),
        [Instruction(Pubkey.new_unique(), bytes([1]), [])],
        Hash.new_unique(),
    )
    assert msg.hash() == MessageV1.hash_raw_message(to_bytes_versioned(msg))


def test_versioned_transaction() -> None:
    payer = Keypair()
    program_id = Pubkey.new_unique()
    instructions = [Instruction(program_id, bytes([1]), [])]
    msg = MessageV1.try_compile(
        payer.pubkey(),
        instructions,
        Hash.new_unique(),
        TransactionConfig(compute_unit_limit=200_000),
    )
    tx = VersionedTransaction(msg, [payer])
    assert tx.verify_with_results() == [True]
    assert tx.message == msg
    assert VersionedTransaction.from_bytes(bytes(tx)) == tx
    assert tx.version() == 1
