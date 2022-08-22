from pathlib import Path
from pytest import mark
from solders.rpc.responses import (
    GetAccountInfoResp,
    GetAccountInfoJsonParsedResp,
    GetBalanceResp,
    GetBlockResp,
    GetBlockCommitmentResp,
    RpcBlockProduction,
    RpcBlockProductionRange,
    GetBlockProductionResp,
    GetBlockHeightResp,
    GetBlocksResp,
    GetBlockTimeResp,
    GetClusterNodesResp,
    GetEpochInfoResp,
    GetEpochScheduleResp,
    GetFeeForMessageResp,
    GetFirstAvailableBlockResp,
    GetGenesisHashResp,
    GetHealthResp,
    GetHighestSnapshotSlotResp,
    GetIdentityResp,
    GetInflationGovernorResp,
    RpcSnapshotSlotInfo,
    RpcResponseContext,
    RpcContactInfo,
    RpcIdentity,
    RpcInflationGovernor,
    EpochInfo,
    RpcError,
)
from solders.rpc.errors import NodeUnhealthy
from solders.hash import Hash
from solders.account import Account, AccountJSON
from solders.epoch_schedule import EpochSchedule
from solders.pubkey import Pubkey
from solders.account_decoder import ParsedAccount
from solders.signature import Signature
from solders.transaction_status import (
    Reward,
    RewardType,
    UiTransactionStatusMeta,
    UiLoadedAddresses,
    UiTransaction,
    UiRawMessage,
    UiParsedMessage,
    UiCompiledInstruction,
    ParsedAccount as ParsedAccountTxStatus,
    ParsedInstruction,
)
from solders.message import MessageHeader, Message
from solders.transaction import VersionedTransaction
from based58 import b58decode


def test_get_account_info() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1
    },
    "value": {
      "data": [
        "11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf",
        "base58"
      ],
      "executable": false,
      "lamports": 1000000000,
      "owner": "11111111111111111111111111111111",
      "rentEpoch": 2
    }
  },
  "id": 1
}"""
    parsed = GetAccountInfoResp.from_json(raw)
    assert isinstance(parsed, GetAccountInfoResp)
    context = RpcResponseContext(slot=1)
    value = Account(
        data=b58decode(
            b"11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf"
        ),
        executable=False,
        lamports=1000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        rent_epoch=2,
    )
    assert parsed == GetAccountInfoResp(context=context, value=value)
    # It's good to test some properties in case we forget to add them
    assert parsed.value is not None
    assert parsed.value.rent_epoch == 2


def test_get_account_info_null() -> None:
    raw = '{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.10.26","slot":146423291},"value":null},"id":1}'
    parsed = GetAccountInfoResp.from_json(raw)
    assert isinstance(parsed, GetAccountInfoResp)
    assert parsed.value is None
    context = RpcResponseContext(slot=146423291, api_version="1.10.26")
    value = None
    assert parsed == GetAccountInfoResp(context=context, value=value)


def test_get_account_info_error() -> None:
    raw = '{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid param: WrongSize"},"id":1}'
    parsed = GetAccountInfoResp.from_json(raw)
    assert isinstance(parsed, RpcError)
    error = RpcError(code=-32602, message="Invalid param: WrongSize")
    assert parsed == error


def test_get_account_info_json_parsed() -> None:
    raw = '{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.10.25","slot":140702417},"value":{"data":{"parsed":{"info":{"isNative":false,"mint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","owner":"vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg","state":"initialized","tokenAmount":{"amount":"36010000000","decimals":6,"uiAmount":36010.0,"uiAmountString":"36010"}},"type":"account"},"program":"spl-token","space":165},"executable":false,"lamports":2039280,"owner":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","rentEpoch":325}},"id":1}'
    parsed = GetAccountInfoJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetAccountInfoJsonParsedResp)
    parsed_account = ParsedAccount(
        program="spl-token",
        space=165,
        parsed='{"info":{"isNative":false,"mint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","owner":"vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg","state":"initialized","tokenAmount":{"amount":"36010000000","decimals":6,"uiAmount":36010.0,"uiAmountString":"36010"}},"type":"account"}',
    )
    assert parsed.value is not None
    assert parsed.value.data == parsed_account
    account_json = AccountJSON(
        lamports=2039280,
        data=parsed_account,
        owner=Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        executable=False,
        rent_epoch=325,
    )
    context = RpcResponseContext(slot=140702417, api_version="1.10.25")
    assert parsed == GetAccountInfoJsonParsedResp(context=context, value=account_json)
    assert parsed.context.slot == 140702417
    assert parsed.value.data.program == "spl-token"


def test_get_balance_resp() -> None:
    raw = """{
"jsonrpc": "2.0",
"result": { "context": { "slot": 1 }, "value": 0 },
"id": 1
}"""
    parsed = GetBalanceResp.from_json(raw)
    assert isinstance(parsed, GetBalanceResp)
    assert parsed == GetBalanceResp(value=0, context=RpcResponseContext(slot=1))


def test_get_block_production_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 9887
    },
    "value": {
      "byIdentity": {
        "85iYT5RuzRTDgjyRa3cP8SYhM2j21fj7NhfJ3peu1DPr": [9888, 9886]
      },
      "range": {
        "firstSlot": 0,
        "lastSlot": 9887
      }
    }
  },
  "id": 1
}"""
    parsed = GetBlockProductionResp.from_json(raw)
    expected = GetBlockProductionResp(
        RpcBlockProduction(
            {
                Pubkey.from_string("85iYT5RuzRTDgjyRa3cP8SYhM2j21fj7NhfJ3peu1DPr"): (
                    9888,
                    9886,
                )
            },
            RpcBlockProductionRange(0, 9887),
        ),
        RpcResponseContext(9887),
    )


def test_get_block_height_resp() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1233, "id": 1 }'
    parsed = GetBlockHeightResp.from_json(raw)
    assert isinstance(parsed, GetBlockHeightResp)
    assert parsed.height == 1233


def test_get_block_commitment_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "commitment": [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 10, 32
    ],
    "totalStake": 42
  },
  "id": 1
}"""
    parsed = GetBlockCommitmentResp.from_json(raw)
    expected = GetBlockCommitmentResp(
        [
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
            10,
            32,
        ],
        42,
    )
    assert parsed == expected


@mark.parametrize(
    "path",
    [
        "get_block_json_encoding.json",
        "get_block_base64_encoding.json",
        "get_block_json_parsed_encoding.json",
    ],
)
def test_get_block_resp(path: str) -> None:
    raw = (Path(__file__).parent / "data" / path).read_text()
    parsed = GetBlockResp.from_json(raw)
    # pub transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
    assert isinstance(parsed, GetBlockResp)
    assert isinstance(parsed.previous_blockhash, Hash)
    assert parsed.rewards is not None
    parsed.rewards[0] == Reward(
        pubkey=Pubkey.from_string("8vio2CKbM54Pfo7LZrRVZdopDxBYMtoBx2YXgfh2rBo6"),
        commission=None,
        lamports=-125,
        post_balance=2020030,
        reward_type=RewardType.Rent,
    )
    transactions = parsed.transactions
    assert transactions is not None
    tx = transactions[0]
    meta = tx.meta
    expected_meta = UiTransactionStatusMeta(
        err=None,
        fee=5000,
        pre_balances=[
            1002554666275,
            100000000000,
            1169280,
            143487360,
            1,
        ],
        post_balances=[
            1002554661275,
            100000000000,
            1169280,
            143487360,
            1,
        ],
        inner_instructions=[],
        log_messages=[
            "Program Vote111111111111111111111111111111111111111 invoke [1]",
            "Program Vote111111111111111111111111111111111111111 success",
        ],
        pre_token_balances=[],
        post_token_balances=[],
        rewards=[],
        loaded_addresses=UiLoadedAddresses([], []),
        return_data=None,
    )
    version = tx.version
    assert meta == expected_meta
    assert version is None  # always None in the test data
    assert parsed.signatures is None  # always None in the test data
    expected_signature = Signature.from_string(
        "2DtNjd9uPve3HHHNroiKoNByaGZ1jgRKqsQGzh4JPcE2NjmVvbYuxJMNfAUgecQnLYqfxSgdKjvj3LNigLZeAx2N"
    )
    assert tx.transaction.signatures[0] == expected_signature
    expected_account_keys = [
        Pubkey.from_string(
            "dv1ZAGvdsz5hHLwWXsVnM94hWf1pjbKVau1QVkaMJ92",
        ),
        Pubkey.from_string(
            "5ZWgXcyqrrNpQHCme5SdC5hCeYb2o3fEJhF7Gok3bTVN",
        ),
        Pubkey.from_string(
            "SysvarC1ock11111111111111111111111111111111",
        ),
        Pubkey.from_string(
            "SysvarS1otHashes111111111111111111111111111",
        ),
        Pubkey.from_string(
            "Vote111111111111111111111111111111111111111",
        ),
    ]
    encoded_tx = tx.transaction
    msg = encoded_tx.message
    assert msg.recent_blockhash == Hash.from_string(
        "BeSgJqfSEkmtQ6S42d2Y7qUXdfLSaXeS9DHQYqw1MLxe"
    )
    if "_json_" in path:
        assert isinstance(msg, (UiParsedMessage, UiRawMessage))
        assert msg.address_table_lookups is None
        assert isinstance(encoded_tx, UiTransaction)
        if "parsed" in path:
            assert isinstance(msg, UiParsedMessage)
            writable_vals = [True, True, False, False, False]
            signer_vals = [True, False, False, False, False]
            expected_parsed_accounts = [
                ParsedAccountTxStatus(pubkey, writable, signer)
                for pubkey, writable, signer in zip(
                    expected_account_keys, writable_vals, signer_vals
                )
            ]
            assert msg.account_keys == expected_parsed_accounts
            json_data = '{"info":{"clockSysvar":"SysvarC1ock11111111111111111111111111111111","slotHashesSysvar":"SysvarS1otHashes111111111111111111111111111","vote":{"hash":"RgNYwznrTfJZXsPkoWFPzvx4iRPV2GBvjA1LFkyzv9L","slots":[147078734],"timestamp":1657486664},"voteAccount":"5ZWgXcyqrrNpQHCme5SdC5hCeYb2o3fEJhF7Gok3bTVN","voteAuthority":"dv1ZAGvdsz5hHLwWXsVnM94hWf1pjbKVau1QVkaMJ92"},"type":"vote"}'
            assert msg.instructions == [
                ParsedInstruction(
                    program="vote",
                    program_id=expected_account_keys[-1],
                    parsed=json_data,
                )
            ]
            assert msg.instructions
        else:
            assert isinstance(msg, UiRawMessage)
            assert msg.account_keys == expected_account_keys
            assert msg.address_table_lookups is None
            assert msg.header == MessageHeader(
                num_required_signatures=1,
                num_readonly_signed_accounts=0,
                num_readonly_unsigned_accounts=3,
            )
            assert msg.instructions == [
                UiCompiledInstruction(
                    program_id_index=4,
                    accounts=bytes(
                        [
                            1,
                            3,
                            2,
                            0,
                        ]
                    ),
                    data="2ZjTR1vUs2pHXyTM33S8Pve4vipY8gLSFTQspocWzMPFfownBTgtTPgwAqQh2vZ7jok9PFA5FG8WmdP8yWP",
                )
            ]
    else:
        assert isinstance(encoded_tx, VersionedTransaction)
        assert isinstance(msg, Message)
        # don't need so many assertions here since we already have tests for Message
    assert parsed.block_height == 139015678
    assert parsed.block_time == 1657486664
    assert isinstance(parsed.blockhash, Hash)
    assert parsed.parent_slot == 147078734
    assert isinstance(parsed.previous_blockhash, Hash)


def test_get_blocks_resp() -> None:
    raw = '{ "jsonrpc": "2.0", "result": [5, 6, 7, 8, 9, 10], "id": 1 }'
    parsed = GetBlocksResp.from_json(raw)
    assert isinstance(parsed, GetBlocksResp)
    assert parsed.blocks == [5, 6, 7, 8, 9, 10]


def test_get_block_time_resp() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1574721591, "id": 1 }'
    parsed = GetBlockTimeResp.from_json(raw)
    assert isinstance(parsed, GetBlockTimeResp)
    assert parsed.time == 1574721591


def test_get_cluster_nodes_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    {
      "gossip": "10.239.6.48:8001",
      "pubkey": "9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ",
      "rpc": "10.239.6.48:8899",
      "tpu": "10.239.6.48:8856",
      "version": "1.0.0 c375ce1f"
    }
  ],
  "id": 1
}"""
    parsed = GetClusterNodesResp.from_json(raw)
    assert isinstance(parsed, GetClusterNodesResp)
    assert parsed.nodes == [
        RpcContactInfo(
            pubkey=Pubkey.from_string("9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ"),
            gossip="10.239.6.48:8001",
            tpu="10.239.6.48:8856",
            rpc="10.239.6.48:8899",
            version="1.0.0 c375ce1f",
        )
    ]


def test_get_epoch_info_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "absoluteSlot": 166598,
    "blockHeight": 166500,
    "epoch": 27,
    "slotIndex": 2790,
    "slotsInEpoch": 8192,
    "transactionCount": 22661093
  },
  "id": 1
}"""
    parsed = GetEpochInfoResp.from_json(raw)
    assert isinstance(parsed, GetEpochInfoResp)
    assert parsed.info == EpochInfo(
        absolute_slot=166598,
        block_height=166500,
        epoch=27,
        slot_index=2790,
        slots_in_epoch=8192,
        transaction_count=22661093,
    )


def test_get_epoch_schedule_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "firstNormalEpoch": 8,
    "firstNormalSlot": 8160,
    "leaderScheduleSlotOffset": 8192,
    "slotsPerEpoch": 8192,
    "warmup": true
  },
  "id": 1
}"""
    parsed = GetEpochScheduleResp.from_json(raw)
    assert isinstance(parsed, GetEpochScheduleResp)
    schedule = parsed.schedule
    assert schedule == EpochSchedule(
        slots_per_epoch=8192,
    )
    assert schedule.first_normal_epoch == 8
    assert schedule.first_normal_slot == 8160
    assert schedule.leader_schedule_slot_offset == 8192
    assert schedule.warmup is True


def test_get_fee_for_message_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": { "context": { "slot": 5068 }, "value": 5000 },
  "id": 1
}"""
    parsed = GetFeeForMessageResp.from_json(raw)
    assert isinstance(parsed, GetFeeForMessageResp)
    assert parsed == GetFeeForMessageResp(
        value=5000, context=RpcResponseContext(slot=5068)
    )


def test_get_first_available_block_resp() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 250000, "id": 1 }'
    parsed = GetFirstAvailableBlockResp.from_json(raw)
    assert isinstance(parsed, GetFirstAvailableBlockResp)
    assert parsed.slot == 250000


def test_get_genesis_hash_resp() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": "GH7ome3EiwEr7tu9JuTh2dpYWBJK3z69Xm1ZE3MEE6JC",
  "id": 1
}"""
    parsed = GetGenesisHashResp.from_json(raw)
    assert isinstance(parsed, GetGenesisHashResp)
    assert parsed.value == Hash.from_string(
        "GH7ome3EiwEr7tu9JuTh2dpYWBJK3z69Xm1ZE3MEE6JC"
    )


def test_get_health_healthy() -> None:
    raw = '{ "jsonrpc": "2.0", "result": "ok", "id": 1 }'
    parsed = GetHealthResp.from_json(raw)
    assert isinstance(parsed, GetHealthResp)
    assert parsed.health == "ok"


def test_get_health_resp_unhealthy_generic() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "error": {
    "code": -32005,
    "message": "Node is unhealthy",
    "data": {}
  },
  "id": 1
}"""
    parsed = GetHealthResp.from_json(raw)
    assert isinstance(parsed, RpcError)
    # This is the only custom rpc error that can be empty.
    # Thus, if the JSON error has `"data": {}`,
    # it's implicitly a NodeUnhealthy error.
    assert parsed.data == NodeUnhealthy()


def test_get_health_additional_info() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "error": {
    "code": -32005,
    "message": "Node is behind by 42 slots",
    "data": {
      "numSlotsBehind": 42
    }
  },
  "id": 1
}"""
    parsed = GetHealthResp.from_json(raw)
    assert isinstance(parsed, RpcError)
    assert parsed.data == NodeUnhealthy(42)


def test_get_highest_snapshot_slot() -> None:
    raw = '{ "jsonrpc": "2.0", "result": { "full": 100, "incremental": 110 }, "id": 1 }'
    parsed = GetHighestSnapshotSlotResp.from_json(raw)
    assert isinstance(parsed, GetHighestSnapshotSlotResp)
    assert parsed.info == RpcSnapshotSlotInfo(full=100, incremental=110)


def test_get_identity() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": { "identity": "2r1F4iWqVcb8M1DbAjQuFpebkQHY9hcVU4WuW2DJBppN" },
  "id": 1
}"""
    parsed = GetIdentityResp.from_json(raw)
    assert isinstance(parsed, GetIdentityResp)
    assert parsed.value == RpcIdentity(
        Pubkey.from_string("2r1F4iWqVcb8M1DbAjQuFpebkQHY9hcVU4WuW2DJBppN")
    )


def test_get_inflation_governor() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "foundation": 0.05,
    "foundationTerm": 7,
    "initial": 0.15,
    "taper": 0.15,
    "terminal": 0.015
  },
  "id": 1
}"""
    parsed = GetInflationGovernorResp.from_json(raw)
    assert isinstance(parsed, GetInflationGovernorResp)
    assert parsed.governor == RpcInflationGovernor(
        initial=0.15, terminal=0.015, taper=0.15, foundation=0.05, foundation_term=7
    )
