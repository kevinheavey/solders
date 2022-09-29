from pathlib import Path
from typing import List, Union
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
    GetBlocksWithLimitResp,
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
    GetInflationRateResp,
    GetInflationRewardResp,
    GetLargestAccountsResp,
    GetLatestBlockhashResp,
    GetLeaderScheduleResp,
    GetMaxRetransmitSlotResp,
    GetMaxShredInsertSlotResp,
    GetMinimumBalanceForRentExemptionResp,
    GetMultipleAccountsResp,
    GetMultipleAccountsJsonParsedResp,
    GetProgramAccountsWithContextResp,
    GetProgramAccountsWithoutContextResp,
    GetProgramAccountsWithContextJsonParsedResp,
    GetProgramAccountsWithoutContextJsonParsedResp,
    GetRecentPerformanceSamplesResp,
    GetSignaturesForAddressResp,
    GetSignatureStatusesResp,
    GetSlotResp,
    GetSlotLeaderResp,
    GetSlotLeadersResp,
    GetStakeActivationResp,
    GetSupplyResp,
    GetTokenAccountBalanceResp,
    GetTokenAccountsByDelegateResp,
    GetTokenAccountsByDelegateJsonParsedResp,
    GetTokenAccountsByOwnerResp,
    GetTokenAccountsByOwnerJsonParsedResp,
    GetTokenLargestAccountsResp,
    GetTokenSupplyResp,
    GetTransactionResp,
    GetTransactionCountResp,
    GetVersionResp,
    GetVoteAccountsResp,
    IsBlockhashValidResp,
    MinimumLedgerSlotResp,
    RequestAirdropResp,
    ValidatorExitResp,
    SendTransactionResp,
    SimulateTransactionResp,
    StakeActivationState,
    RpcSnapshotSlotInfo,
    RpcResponseContext,
    RpcContactInfo,
    RpcIdentity,
    RpcInflationGovernor,
    RpcInflationRate,
    RpcInflationReward,
    RpcAccountBalance,
    RpcBlockhash,
    RpcKeyedAccount,
    RpcKeyedAccountJsonParsed,
    RpcPerfSample,
    RpcConfirmedTransactionStatusWithSignature,
    RpcSimulateTransactionResult,
    RpcStakeActivation,
    RpcSupply,
    RpcTokenAccountBalance,
    RpcVersionInfo,
    RpcVoteAccountInfo,
    RpcVoteAccountStatus,
    RpcSignatureResponse,
    EpochInfo,
    RpcError,
    RpcBlockUpdate,
    RpcLogsResponse,
    RpcVote,
    AccountNotification,
    AccountNotificationResult,
    AccountNotificationJsonParsed,
    BlockNotification,
    BlockNotificationResult,
    LogsNotification,
    LogsNotificationResult,
    ProgramNotification,
    ProgramNotificationResult,
    SignatureNotification,
    SignatureNotificationResult,
    SlotNotification,
    SlotInfo,
    SlotUpdateOptimisticConfirmation,
    SlotUpdateNotification,
    VoteNotification,
    RootNotification,
    SubscriptionError,
    SubscriptionResult,
    batch_from_json,
    batch_to_json,
    parse_notification,
    parse_websocket_message,
)
from solders.rpc.errors import NodeUnhealthy
from solders.hash import Hash
from solders.account import Account, AccountJSON
from solders.epoch_schedule import EpochSchedule
from solders.pubkey import Pubkey
from solders.account_decoder import ParsedAccount, UiTokenAmount
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
    TransactionStatus,
    TransactionConfirmationStatus,
    TransactionErrorInstructionError,
    InstructionErrorCustom,
)
from solders.message import MessageHeader, Message
from solders.transaction import VersionedTransaction
from based58 import b58decode
from base64 import b64decode


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


def test_get_balance() -> None:
    raw = """{
"jsonrpc": "2.0",
"result": { "context": { "slot": 1 }, "value": 0 },
"id": 1
}"""
    parsed = GetBalanceResp.from_json(raw)
    assert isinstance(parsed, GetBalanceResp)
    assert parsed == GetBalanceResp(value=0, context=RpcResponseContext(slot=1))


def test_get_block_production() -> None:
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
    assert parsed == expected


def test_get_block_height() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1233, "id": 1 }'
    parsed = GetBlockHeightResp.from_json(raw)
    assert isinstance(parsed, GetBlockHeightResp)
    assert parsed.value == 1233


def test_get_block_commitment() -> None:
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
def test_get_block(path: str) -> None:
    raw = (Path(__file__).parent / "data" / path).read_text()
    parsed = GetBlockResp.from_json(raw)
    # pub transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
    assert isinstance(parsed, GetBlockResp)
    val = parsed.value
    assert isinstance(val.previous_blockhash, Hash)
    assert val.rewards is not None
    assert val.rewards[0] == Reward(
        pubkey=Pubkey.from_string("8vio2CKbM54Pfo7LZrRVZdopDxBYMtoBx2YXgfh2rBo6"),
        commission=None,
        lamports=-125,
        post_balance=2020030,
        reward_type=RewardType.Rent,
    )
    transactions = val.transactions
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
    assert val.signatures is None  # always None in the test data
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
    assert val.block_height == 139015678
    assert val.block_time == 1657486664
    assert isinstance(val.blockhash, Hash)
    assert val.parent_slot == 147078734


def test_get_blocks() -> None:
    raw = '{ "jsonrpc": "2.0", "result": [5, 6, 7, 8, 9, 10], "id": 1 }'
    parsed = GetBlocksResp.from_json(raw)
    assert isinstance(parsed, GetBlocksResp)
    assert parsed.value == [5, 6, 7, 8, 9, 10]


def test_get_blocks_with_limit() -> None:
    raw = '{ "jsonrpc": "2.0", "result": [5, 6, 7, 8, 9, 10], "id": 1 }'
    parsed = GetBlocksWithLimitResp.from_json(raw)
    assert isinstance(parsed, GetBlocksWithLimitResp)
    assert parsed.value == [5, 6, 7, 8, 9, 10]


def test_get_block_time() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1574721591, "id": 1 }'
    parsed = GetBlockTimeResp.from_json(raw)
    assert isinstance(parsed, GetBlockTimeResp)
    assert parsed.value == 1574721591


def test_get_cluster_nodes() -> None:
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
    assert parsed.value == [
        RpcContactInfo(
            pubkey=Pubkey.from_string("9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ"),
            gossip="10.239.6.48:8001",
            tpu="10.239.6.48:8856",
            rpc="10.239.6.48:8899",
            version="1.0.0 c375ce1f",
        )
    ]


def test_get_epoch_info() -> None:
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
    assert parsed.value == EpochInfo(
        absolute_slot=166598,
        block_height=166500,
        epoch=27,
        slot_index=2790,
        slots_in_epoch=8192,
        transaction_count=22661093,
    )


def test_get_epoch_schedule() -> None:
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
    schedule = parsed.value
    assert schedule == EpochSchedule(
        slots_per_epoch=8192,
    )
    assert schedule.first_normal_epoch == 8
    assert schedule.first_normal_slot == 8160
    assert schedule.leader_schedule_slot_offset == 8192
    assert schedule.warmup is True


def test_get_fee_for_message() -> None:
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


def test_get_first_available_block() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 250000, "id": 1 }'
    parsed = GetFirstAvailableBlockResp.from_json(raw)
    assert isinstance(parsed, GetFirstAvailableBlockResp)
    assert parsed.value == 250000


def test_get_genesis_hash() -> None:
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
    assert parsed.value == "ok"


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
    assert parsed.value == RpcSnapshotSlotInfo(full=100, incremental=110)


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
    assert parsed.value == RpcInflationGovernor(
        initial=0.15, terminal=0.015, taper=0.15, foundation=0.05, foundation_term=7
    )


def test_get_inflation_rate() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "epoch": 100,
    "foundation": 0.001,
    "total": 0.149,
    "validator": 0.148
  },
  "id": 1
}"""
    parsed = GetInflationRateResp.from_json(raw)
    assert isinstance(parsed, GetInflationRateResp)
    assert parsed.value == RpcInflationRate(
        total=0.149, validator=0.148, foundation=0.001, epoch=100
    )


def test_get_inflation_reward() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    {
      "amount": 2500,
      "effectiveSlot": 224,
      "epoch": 2,
      "postBalance": 499999442500
    },
    null
  ],
  "id": 1
}"""
    parsed = GetInflationRewardResp.from_json(raw)
    assert isinstance(parsed, GetInflationRewardResp)
    assert parsed.value == [
        RpcInflationReward(
            amount=2500, effective_slot=224, epoch=2, post_balance=499999442500
        ),
        None,
    ]


def test_get_largest_accounts() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 54
    },
    "value": [
      {
        "lamports": 999974,
        "address": "99P8ZgtJYe1buSK8JXkvpLh8xPsCFuLYhz9hQFNw93WJ"
      },
      {
        "lamports": 42,
        "address": "uPwWLo16MVehpyWqsLkK3Ka8nLowWvAHbBChqv2FZeL"
      },
      {
        "lamports": 42,
        "address": "aYJCgU7REfu3XF8b3QhkqgqQvLizx8zxuLBHA25PzDS"
      },
      {
        "lamports": 42,
        "address": "CTvHVtQ4gd4gUcw3bdVgZJJqApXE9nCbbbP4VTS5wE1D"
      },
      {
        "lamports": 20,
        "address": "4fq3xJ6kfrh9RkJQsmVd5gNMvJbuSHfErywvEjNQDPxu"
      },
      {
        "lamports": 4,
        "address": "AXJADheGVp9cruP8WYu46oNkRbeASngN5fPCMVGQqNHa"
      },
      {
        "lamports": 2,
        "address": "8NT8yS6LiwNprgW4yM1jPPow7CwRUotddBVkrkWgYp24"
      },
      {
        "lamports": 1,
        "address": "SysvarEpochSchedu1e111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "11111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "Stake11111111111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "SysvarC1ock11111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "StakeConfig11111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "SysvarRent111111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "Config1111111111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "SysvarStakeHistory1111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "SysvarRecentB1ockHashes11111111111111111111"
      },
      {
        "lamports": 1,
        "address": "SysvarFees111111111111111111111111111111111"
      },
      {
        "lamports": 1,
        "address": "Vote111111111111111111111111111111111111111"
      }
    ]
  },
  "id": 1
}"""
    parsed = GetLargestAccountsResp.from_json(raw)
    assert isinstance(parsed, GetLargestAccountsResp)
    assert parsed.value[0] == RpcAccountBalance(
        Pubkey.from_string("99P8ZgtJYe1buSK8JXkvpLh8xPsCFuLYhz9hQFNw93WJ"), 999974
    )


def test_get_latest_blockhash() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 2792
    },
    "value": {
      "blockhash": "EkSnNWid2cvwEVnVx9aBqawnmiCNiDgp3gUdkDPTKN1N",
      "lastValidBlockHeight": 3090
    }
  },
  "id": 1
}"""
    parsed = GetLatestBlockhashResp.from_json(raw)
    assert isinstance(parsed, GetLatestBlockhashResp)
    assert parsed.value == RpcBlockhash(
        blockhash=Hash.from_string("EkSnNWid2cvwEVnVx9aBqawnmiCNiDgp3gUdkDPTKN1N"),
        last_valid_block_height=3090,
    )


def test_get_leader_schedule() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F": [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
      39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
      57, 58, 59, 60, 61, 62, 63
    ]
  },
  "id": 1
}"""
    parsed = GetLeaderScheduleResp.from_json(raw)
    assert isinstance(parsed, GetLeaderScheduleResp)
    assert parsed.value == {
        Pubkey.from_string("4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F"): [
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            16,
            17,
            18,
            19,
            20,
            21,
            22,
            23,
            24,
            25,
            26,
            27,
            28,
            29,
            30,
            31,
            32,
            33,
            34,
            35,
            36,
            37,
            38,
            39,
            40,
            41,
            42,
            43,
            44,
            45,
            46,
            47,
            48,
            49,
            50,
            51,
            52,
            53,
            54,
            55,
            56,
            57,
            58,
            59,
            60,
            61,
            62,
            63,
        ]
    }


def test_get_max_retransmit_slot() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1234, "id": 1 }'
    parsed = GetMaxRetransmitSlotResp.from_json(raw)
    assert isinstance(parsed, GetMaxRetransmitSlotResp)
    assert parsed.value == 1234


def test_get_max_shred_insert_slot() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1234, "id": 1 }'
    parsed = GetMaxShredInsertSlotResp.from_json(raw)
    assert isinstance(parsed, GetMaxShredInsertSlotResp)
    assert parsed.value == 1234


def test_get_minimum_balance_for_tent_exemption() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 500, "id": 1 }'
    parsed = GetMinimumBalanceForRentExemptionResp.from_json(raw)
    assert isinstance(parsed, GetMinimumBalanceForRentExemptionResp)
    assert parsed.value == 500


def test_get_multiple_accounts_base64() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1
    },
    "value": [
      {
        "data": ["AAAAAAEAAAACtzNsyJrW0g==", "base64"],
        "executable": false,
        "lamports": 1000000000,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 2
      },
      {
        "data": ["", "base64"],
        "executable": false,
        "lamports": 5000000000,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 2
      }
    ]
  },
  "id": 1
}"""
    parsed = GetMultipleAccountsResp.from_json(raw)
    assert isinstance(parsed, GetMultipleAccountsResp)
    val = parsed.value
    assert val[0] == Account(
        lamports=1000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        executable=False,
        rent_epoch=2,
        data=b64decode("AAAAAAEAAAACtzNsyJrW0g=="),
    )
    assert val[1] == Account(
        lamports=5000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        executable=False,
        rent_epoch=2,
        data=b"",
    )


def test_get_multiple_accounts_base58() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1
    },
    "value": [
      {
        "data": [
          "11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf",
          "base58"
        ],
        "executable": false,
        "lamports": 1000000000,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 2
      },
      {
        "data": ["", "base58"],
        "executable": false,
        "lamports": 5000000000,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 2
      }
    ]
  },
  "id": 1
}"""
    parsed = GetMultipleAccountsResp.from_json(raw)
    assert isinstance(parsed, GetMultipleAccountsResp)
    val = parsed.value
    assert val[0] == Account(
        lamports=1000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        executable=False,
        rent_epoch=2,
        data=b58decode(
            b"11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf"
        ),
    )
    assert val[1] == Account(
        lamports=5000000000,
        owner=Pubkey.from_string("11111111111111111111111111111111"),
        executable=False,
        rent_epoch=2,
        data=b"",
    )


def test_get_multiple_accounts_json_parsed() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "apiVersion": "1.10.25",
            "slot": 140702417
        },
        "value": [
            {
                "data": {
                    "parsed": {
                        "info": {
                            "isNative": false,
                            "mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                            "owner": "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
                            "state": "initialized",
                            "tokenAmount": {
                                "amount": "36010000000",
                                "decimals": 6,
                                "uiAmount": 36010.0,
                                "uiAmountString": "36010"
                            }
                        },
                        "type": "account"
                    },
                    "program": "spl-token",
                    "space": 165
                },
                "executable": false,
                "lamports": 2039280,
                "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "rentEpoch": 325
            }
        ]
    },
    "id": 1
}"""
    parsed = GetMultipleAccountsJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetMultipleAccountsJsonParsedResp)
    val = parsed.value
    acc = val[0]
    assert isinstance(acc, AccountJSON)
    assert acc.executable is False
    assert acc.lamports == 2039280
    assert acc.owner == Pubkey.from_string(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    )
    assert acc.rent_epoch == 325
    data = acc.data
    assert isinstance(data, ParsedAccount)
    assert data.program == "spl-token"
    assert data.space == 165
    assert isinstance(data.parsed, str)


def test_get_program_accounts_without_context() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    {
      "account": {
        "data": "2R9jLfiAQ9bgdcw6h8s44439",
        "executable": false,
        "lamports": 15298080,
        "owner": "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
        "rentEpoch": 28
      },
      "pubkey": "CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"
    }
  ],
  "id": 1
}"""
    parsed = GetProgramAccountsWithoutContextResp.from_json(raw)
    assert isinstance(parsed, GetProgramAccountsWithoutContextResp)
    assert parsed.value[0] == RpcKeyedAccount(
        account=Account(
            data=b58decode(b"2R9jLfiAQ9bgdcw6h8s44439"),
            executable=False,
            lamports=15298080,
            owner=Pubkey.from_string("4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T"),
            rent_epoch=28,
        ),
        pubkey=Pubkey.from_string("CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"),
    )


def test_get_program_accounts_without_context_json_parsed() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": [
        {
            "account": {
                "data": {
                    "parsed": {
                        "info": {
                            "extensions": [
                                {
                                    "extension": "immutableOwner"
                                },
                                {
                                    "extension": "transferFeeAmount",
                                    "state": {
                                        "withheldAmount": 0
                                    }
                                }
                            ],
                            "isNative": false,
                            "mint": "CYRKaU7PaCnSAnDWT1UgSk3uF2gXrkauJYJcHuDPYwLr",
                            "owner": "QT1tXf1kz2fMyRPmGdDCCTY6aFcxXaXK3CuwvyCprb1",
                            "state": "initialized",
                            "tokenAmount": {
                                "amount": "0",
                                "decimals": 6,
                                "uiAmount": 0.0,
                                "uiAmountString": "0"
                            }
                        },
                        "type": "account"
                    },
                    "program": "spl-token-2022",
                    "space": 182
                },
                "executable": false,
                "lamports": 2157600,
                "owner": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                "rentEpoch": 364
            },
            "pubkey": "2je6rgKzzBX6SiYUpEhrshmpmEJvEewDNCkLq1Rbreh7"
        }
    ]
}"""
    parsed = GetProgramAccountsWithoutContextJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetProgramAccountsWithoutContextJsonParsedResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccountJsonParsed)
    acc = val.account
    data = acc.data
    assert isinstance(acc, AccountJSON)
    assert acc.owner == Pubkey.from_string(
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
    )
    assert acc.lamports == 2157600
    assert acc.executable is False
    assert acc.rent_epoch == 364
    assert isinstance(data, ParsedAccount)
    assert data.program == "spl-token-2022"
    assert data.space == 182
    assert isinstance(data.parsed, str)


def test_get_program_accounts_with_context() -> None:
    raw = '{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.10.34","slot":156892898},"value":[]},"id":1}'
    parsed = GetProgramAccountsWithContextResp.from_json(raw)
    assert isinstance(parsed, GetProgramAccountsWithContextResp)
    assert not parsed.value


def test_get_program_accounts_with_context_json_parsed() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "apiVersion": "1.10.34",
            "slot": 157310305
        },
        "value": [
            {
                "account": {
                    "data": {
                        "parsed": {
                            "info": {
                                "extensions": [
                                    {
                                        "extension": "immutableOwner"
                                    },
                                    {
                                        "extension": "transferFeeAmount",
                                        "state": {
                                            "withheldAmount": 0
                                        }
                                    }
                                ],
                                "isNative": false,
                                "mint": "CYRKaU7PaCnSAnDWT1UgSk3uF2gXrkauJYJcHuDPYwLr",
                                "owner": "QT1tXf1kz2fMyRPmGdDCCTY6aFcxXaXK3CuwvyCprb1",
                                "state": "initialized",
                                "tokenAmount": {
                                    "amount": "0",
                                    "decimals": 6,
                                    "uiAmount": 0.0,
                                    "uiAmountString": "0"
                                }
                            },
                            "type": "account"
                        },
                        "program": "spl-token-2022",
                        "space": 182
                    },
                    "executable": false,
                    "lamports": 2157600,
                    "owner": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                    "rentEpoch": 364
                },
                "pubkey": "2je6rgKzzBX6SiYUpEhrshmpmEJvEewDNCkLq1Rbreh7"
            }
        ]
    }
}"""
    parsed = GetProgramAccountsWithContextJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetProgramAccountsWithContextJsonParsedResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccountJsonParsed)
    acc = val.account
    data = acc.data
    assert isinstance(acc, AccountJSON)
    assert acc.owner == Pubkey.from_string(
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
    )
    assert acc.lamports == 2157600
    assert acc.executable is False
    assert acc.rent_epoch == 364
    assert isinstance(data, ParsedAccount)
    assert data.program == "spl-token-2022"
    assert data.space == 182
    assert isinstance(data.parsed, str)


def test_get_recent_performance_samples() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    {
      "numSlots": 126,
      "numTransactions": 126,
      "samplePeriodSecs": 60,
      "slot": 348125
    },
    {
      "numSlots": 126,
      "numTransactions": 126,
      "samplePeriodSecs": 60,
      "slot": 347999
    },
    {
      "numSlots": 125,
      "numTransactions": 125,
      "samplePeriodSecs": 60,
      "slot": 347873
    },
    {
      "numSlots": 125,
      "numTransactions": 125,
      "samplePeriodSecs": 60,
      "slot": 347748
    }
  ],
  "id": 1
}"""
    parsed = GetRecentPerformanceSamplesResp.from_json(raw)
    assert isinstance(parsed, GetRecentPerformanceSamplesResp)
    assert parsed.value[0] == RpcPerfSample(
        num_slots=126, num_transactions=126, sample_period_secs=60, slot=348125
    )


def test_get_signatures_for_address() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    {
      "err": null,
      "memo": null,
      "signature": "5h6xBEauJ3PK6SWCZ1PGjBvj8vDdWG3KpwATGy1ARAXFSDwt8GFXM7W5Ncn16wmqokgpiKRLuS83KUxyZyv2sUYv",
      "slot": 114,
      "blockTime": null
    }
  ],
  "id": 1
}"""
    parsed = GetSignaturesForAddressResp.from_json(raw)
    assert isinstance(parsed, GetSignaturesForAddressResp)
    assert parsed.value[0] == RpcConfirmedTransactionStatusWithSignature(
        err=None,
        memo=None,
        signature=Signature.from_string(
            "5h6xBEauJ3PK6SWCZ1PGjBvj8vDdWG3KpwATGy1ARAXFSDwt8GFXM7W5Ncn16wmqokgpiKRLuS83KUxyZyv2sUYv"
        ),
        slot=114,
        block_time=None,
    )


def test_get_signature_statuses() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 82
    },
    "value": [
      {
        "slot": 72,
        "confirmations": 10,
        "err": null,
        "status": {
          "Ok": null
        },
        "confirmationStatus": "confirmed"
      },
      null
    ]
  },
  "id": 1
}"""
    parsed = GetSignatureStatusesResp.from_json(raw)
    assert isinstance(parsed, GetSignatureStatusesResp)
    assert parsed.value[0] == TransactionStatus(
        slot=72,
        confirmations=10,
        err=None,
        status=None,
        confirmation_status=TransactionConfirmationStatus.Confirmed,
    )


def test_get_slot() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1234, "id": 1 }'
    parsed = GetSlotResp.from_json(raw)
    assert isinstance(parsed, GetSlotResp)
    assert parsed.value == 1234


def test_get_slot_leader() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": "ENvAW7JScgYq6o4zKZwewtkzzJgDzuJAFxYasvmEQdpS",
  "id": 1
}"""
    parsed = GetSlotLeaderResp.from_json(raw)
    assert isinstance(parsed, GetSlotLeaderResp)
    assert parsed.value == Pubkey.from_string(
        "ENvAW7JScgYq6o4zKZwewtkzzJgDzuJAFxYasvmEQdpS"
    )


def test_get_slot_leaders() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": [
    "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
    "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
    "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
    "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
    "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
    "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
    "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
    "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
    "DWvDTSh3qfn88UoQTEKRV2JnLt5jtJAVoiCo3ivtMwXP",
    "DWvDTSh3qfn88UoQTEKRV2JnLt5jtJAVoiCo3ivtMwXP"
  ],
  "id": 1
}"""
    parsed = GetSlotLeadersResp.from_json(raw)
    assert isinstance(parsed, GetSlotLeadersResp)
    assert parsed.value == [
        Pubkey.from_string(p)
        for p in (
            "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
            "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
            "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
            "ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
            "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
            "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
            "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
            "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
            "DWvDTSh3qfn88UoQTEKRV2JnLt5jtJAVoiCo3ivtMwXP",
            "DWvDTSh3qfn88UoQTEKRV2JnLt5jtJAVoiCo3ivtMwXP",
        )
    ]


def test_get_stake_activation() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": { "active": 197717120, "inactive": 0, "state": "active" },
  "id": 1
}"""
    parsed = GetStakeActivationResp.from_json(raw)
    assert isinstance(parsed, GetStakeActivationResp)
    assert parsed.value == RpcStakeActivation(
        state=StakeActivationState.Active, active=197717120, inactive=0
    )


def test_get_supply() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": {
      "circulating": 16000,
      "nonCirculating": 1000000,
      "nonCirculatingAccounts": [
        "FEy8pTbP5fEoqMV1GdTz83byuA8EKByqYat1PKDgVAq5",
        "9huDUZfxoJ7wGMTffUE7vh1xePqef7gyrLJu9NApncqA",
        "3mi1GmwEE3zo2jmfDuzvjSX9ovRXsDUKHvsntpkhuLJ9",
        "BYxEJTDerkaRWBem3XgnVcdhppktBXa2HbkHPKj2Ui4Z"
      ],
      "total": 1016000
    }
  },
  "id": 1
}"""
    parsed = GetSupplyResp.from_json(raw)
    assert isinstance(parsed, GetSupplyResp)
    assert parsed.value == RpcSupply(
        total=1016000,
        circulating=16000,
        non_circulating=1000000,
        non_circulating_accounts=[
            Pubkey.from_string(s)
            for s in (
                "FEy8pTbP5fEoqMV1GdTz83byuA8EKByqYat1PKDgVAq5",
                "9huDUZfxoJ7wGMTffUE7vh1xePqef7gyrLJu9NApncqA",
                "3mi1GmwEE3zo2jmfDuzvjSX9ovRXsDUKHvsntpkhuLJ9",
                "BYxEJTDerkaRWBem3XgnVcdhppktBXa2HbkHPKj2Ui4Z",
            )
        ],
    )


def test_get_token_account_balance() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": {
      "amount": "9864",
      "decimals": 2,
      "uiAmount": 98.64,
      "uiAmountString": "98.64"
    },
    "id": 1
  }
}"""
    parsed = GetTokenAccountBalanceResp.from_json(raw)
    assert isinstance(parsed, GetTokenAccountBalanceResp)
    assert parsed.value == UiTokenAmount(
        amount="9864", decimals=2, ui_amount=98.64, ui_amount_string="98.64"
    )


def test_get_token_accounts_by_owner_json_parsed() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": [
      {
        "account": {
          "data": {
            "program": "spl-token",
            "parsed": {
              "accountType": "account",
              "info": {
                "tokenAmount": {
                  "amount": "1",
                  "decimals": 1,
                  "uiAmount": 0.1,
                  "uiAmountString": "0.1"
                },
                "delegate": "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
                "delegatedAmount": {
                  "amount": "1",
                  "decimals": 1,
                  "uiAmount": 0.1,
                  "uiAmountString": "0.1"
                },
                "state": "initialized",
                "isNative": false,
                "mint": "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E",
                "owner": "4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F"
              },
              "type": "account"
            },
            "space": 165
          },
          "executable": false,
          "lamports": 1726080,
          "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "rentEpoch": 4
        },
        "pubkey": "C2gJg6tKpQs41PRS1nC8aw3ZKNZK3HQQZGVrDFDup5nx"
      }
    ]
  },
  "id": 1
}"""
    parsed = GetTokenAccountsByOwnerJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetTokenAccountsByOwnerJsonParsedResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccountJsonParsed)
    assert val.pubkey == Pubkey.from_string(
        "C2gJg6tKpQs41PRS1nC8aw3ZKNZK3HQQZGVrDFDup5nx"
    )
    acc = val.account
    assert isinstance(acc, AccountJSON)
    assert acc.lamports == 1726080
    data = acc.data
    assert isinstance(data, ParsedAccount)
    assert data.program == "spl-token"
    assert isinstance(data.parsed, str)
    assert data.space == 165
    assert acc.owner == Pubkey.from_string(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    )


def test_get_token_accounts_by_owner_base64() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "apiVersion": "1.10.34",
            "slot": 147478898
        },
        "value": [
            {
                "account": {
                    "data": [
                        "xvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWFKA06zRYoxr7jEHagAc7BsbLa6ckZFMVzMh+LY4w/Hky/M7fw+BQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                        "base64"
                    ],
                    "executable": false,
                    "lamports": 2039280,
                    "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                    "rentEpoch": 341
                },
                "pubkey": "5urjqaUDYeHiSiiTkRph6aqYU94GrsBXXSMosxT9b3dF"
            }
        ]
    },
    "id": 1
}"""
    parsed = GetTokenAccountsByOwnerResp.from_json(raw)
    assert isinstance(parsed, GetTokenAccountsByOwnerResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccount)
    assert val.pubkey == Pubkey.from_string(
        "5urjqaUDYeHiSiiTkRph6aqYU94GrsBXXSMosxT9b3dF"
    )
    acc = val.account
    expected_lamports = 2039280
    expected_owner = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
    expected_data = b64decode(
        "xvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWFKA06zRYoxr7jEHagAc7BsbLa6ckZFMVzMh+LY4w/Hky/M7fw+BQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    )
    assert acc == Account(
        lamports=expected_lamports,
        data=expected_data,
        owner=expected_owner,
        executable=False,
        rent_epoch=341,
    )


def test_get_token_accounts_by_delegate_json_parsed() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": [
      {
        "account": {
          "data": {
            "program": "spl-token",
            "parsed": {
              "info": {
                "tokenAmount": {
                  "amount": "1",
                  "decimals": 1,
                  "uiAmount": 0.1,
                  "uiAmountString": "0.1"
                },
                "delegate": "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
                "delegatedAmount": {
                  "amount": "1",
                  "decimals": 1,
                  "uiAmount": 0.1,
                  "uiAmountString": "0.1"
                },
                "state": "initialized",
                "isNative": false,
                "mint": "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E",
                "owner": "CnPoSPKXu7wJqxe59Fs72tkBeALovhsCxYeFwPCQH9TD"
              },
              "type": "account"
            },
            "space": 165
          },
          "executable": false,
          "lamports": 1726080,
          "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "rentEpoch": 4
        },
        "pubkey": "28YTZEwqtMHWrhWcvv34se7pjS7wctgqzCPB3gReCFKp"
      }
    ]
  },
  "id": 1
}"""
    parsed = GetTokenAccountsByDelegateJsonParsedResp.from_json(raw)
    assert isinstance(parsed, GetTokenAccountsByDelegateJsonParsedResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccountJsonParsed)
    assert val.pubkey == Pubkey.from_string(
        "28YTZEwqtMHWrhWcvv34se7pjS7wctgqzCPB3gReCFKp"
    )
    acc = val.account
    assert isinstance(acc, AccountJSON)
    assert acc.executable is False
    assert acc.owner == Pubkey.from_string(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    )
    assert acc.rent_epoch == 4
    assert acc.lamports == 1726080
    data = acc.data
    assert data.program == "spl-token"
    assert isinstance(data.parsed, str)
    assert data.space == 165


def test_get_token_accounts_by_delegate_base64() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "apiVersion": "1.10.34",
            "slot": 147478898
        },
        "value": [
            {
                "account": {
                    "data": [
                        "xvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWFKA06zRYoxr7jEHagAc7BsbLa6ckZFMVzMh+LY4w/Hky/M7fw+BQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                        "base64"
                    ],
                    "executable": false,
                    "lamports": 2039280,
                    "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                    "rentEpoch": 341
                },
                "pubkey": "5urjqaUDYeHiSiiTkRph6aqYU94GrsBXXSMosxT9b3dF"
            }
        ]
    },
    "id": 1
}"""
    parsed = GetTokenAccountsByDelegateResp.from_json(raw)
    assert isinstance(parsed, GetTokenAccountsByDelegateResp)
    val = parsed.value[0]
    assert isinstance(val, RpcKeyedAccount)
    assert val.pubkey == Pubkey.from_string(
        "5urjqaUDYeHiSiiTkRph6aqYU94GrsBXXSMosxT9b3dF"
    )
    acc = val.account
    expected_lamports = 2039280
    expected_owner = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
    expected_data = b64decode(
        "xvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWFKA06zRYoxr7jEHagAc7BsbLa6ckZFMVzMh+LY4w/Hky/M7fw+BQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    )
    assert acc == Account(
        lamports=expected_lamports,
        data=expected_data,
        owner=expected_owner,
        executable=False,
        rent_epoch=341,
    )


def test_get_token_largest_accounts() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": [
      {
        "address": "FYjHNoFtSQ5uijKrZFyYAxvEr87hsKXkXcxkcmkBAf4r",
        "amount": "771",
        "decimals": 2,
        "uiAmount": 7.71,
        "uiAmountString": "7.71"
      },
      {
        "address": "BnsywxTcaYeNUtzrPxQUvzAWxfzZe3ZLUJ4wMMuLESnu",
        "amount": "229",
        "decimals": 2,
        "uiAmount": 2.29,
        "uiAmountString": "2.29"
      }
    ]
  },
  "id": 1
}"""
    parsed = GetTokenLargestAccountsResp.from_json(raw)
    assert isinstance(parsed, GetTokenLargestAccountsResp)
    val = parsed.value
    assert val[0] == RpcTokenAccountBalance(
        address=Pubkey.from_string("FYjHNoFtSQ5uijKrZFyYAxvEr87hsKXkXcxkcmkBAf4r"),
        amount=UiTokenAmount(
            decimals=2,
            ui_amount=7.71,
            amount="771",
            ui_amount_string="7.71",
        ),
    )


def test_get_token_supply() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 1114
    },
    "value": {
      "amount": "100000",
      "decimals": 2,
      "uiAmount": 1000,
      "uiAmountString": "1000"
    }
  },
  "id": 1
}"""
    parsed = GetTokenSupplyResp.from_json(raw)
    assert isinstance(parsed, GetTokenSupplyResp)
    val = parsed.value
    assert val == UiTokenAmount(
        amount="100000", decimals=2, ui_amount=1000, ui_amount_string="1000"
    )


@mark.parametrize(
    "path",
    [
        "get_transaction_json_encoding.json",
        "get_transaction_base64_encoding.json",
        "get_transaction_json_parsed_encoding.json",
    ],
)
def test_get_transaction(path: str) -> None:
    raw = (Path(__file__).parent / "data" / path).read_text()
    parsed = GetTransactionResp.from_json(raw)
    assert isinstance(parsed, GetTransactionResp)
    encoded = parsed.value
    assert encoded is not None
    assert encoded.block_time == 1661417105
    assert encoded.slot == 147558327
    assert encoded is not None
    tx = encoded.transaction
    meta = tx.meta
    expected_meta = UiTransactionStatusMeta(
        err=None,
        fee=5000,
        pre_balances=[65966805251, 28420419522, 1169280, 143487360, 1],
        post_balances=[65966800251, 28420419522, 1169280, 143487360, 1],
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
    expected_signature = Signature.from_string(
        "4zExa66hUSr28i2ma6EJwEsm3nZo4VFryEyRYE5Zrr4Q6dZpP1ctzsoMqUhLnp6iHGPp5MB722rZXRBKg927WXN9"
    )
    assert tx.transaction.signatures[0] == expected_signature
    expected_account_keys = [
        Pubkey.from_string(
            "5p8qKVyKthA9DUb1rwQDzjcmTkaZdwN97J3LiaEywUjd",
        ),
        Pubkey.from_string(
            "EsEtxoyhFTgfvudcy2VwwQJ1qA6BScLUW39PKpYczuxF",
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
        "2NiTTzGXE7kW66iwM4FaoB7xMidgzMXZkh7k4AeagnW8"
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
            json_data = '{"info":{"clockSysvar":"SysvarC1ock11111111111111111111111111111111","slotHashesSysvar":"SysvarS1otHashes111111111111111111111111111","vote":{"hash":"EGPfU6nPLtV76PrQrsUKAmkLKf2q9prrzGtdn8xLmXqP","slots":[147558324, 147558325],"timestamp":1661417104},"voteAccount":"EsEtxoyhFTgfvudcy2VwwQJ1qA6BScLUW39PKpYczuxF","voteAuthority":"5p8qKVyKthA9DUb1rwQDzjcmTkaZdwN97J3LiaEywUjd"},"type":"vote"}'
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
                    data="29z5mr1JoRmJYQ6zAMYeoMgmako3Wwu22EBXCfkjkPzNMYFWT8NYE9mLqfhirugcv2CiNqvhCiGLwzpvtUuowKoy1kjyyq",
                )
            ]
    else:
        assert isinstance(encoded_tx, VersionedTransaction)
        assert isinstance(msg, Message)
        # don't need so many assertions here since we already have tests for Message


def test_get_transaction_count() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 268, "id": 1 }'
    parsed = GetTransactionCountResp.from_json(raw)
    assert isinstance(parsed, GetTransactionCountResp)
    assert parsed.value == 268


def test_get_version() -> None:
    raw = '{ "jsonrpc": "2.0", "result": { "solana-core": "1.10.32" }, "id": 1 }'
    parsed = GetVersionResp.from_json(raw)
    assert isinstance(parsed, GetVersionResp)
    assert parsed.value == RpcVersionInfo(solana_core="1.10.32")


def test_vote_accounts() -> None:
    raw = (Path(__file__).parent / "data" / "get_vote_accounts.json").read_text()
    parsed = GetVoteAccountsResp.from_json(raw)
    assert isinstance(parsed, GetVoteAccountsResp)
    val = parsed.value
    assert isinstance(val, RpcVoteAccountStatus)
    current = val.current[0]
    delinquent = val.delinquent[0]
    expected_delinquent = RpcVoteAccountInfo(
        commission=100,
        epoch_vote_account=True,
        epoch_credits=[
            (
                154,
                80087,
                0,
            ),
            (
                155,
                207429,
                80087,
            ),
        ],
        node_pubkey=Pubkey.from_string("ECTTH7S5UVJeC5C5WxH64KMdpUVJ9yrmMdWwEd8vcFU6"),
        last_vote=67089657,
        activated_stake=2272912627,
        vote_pubkey=Pubkey.from_string("GaZ5Pqr1GN5paSeuvkXHJnLsvjbAQGZgZrkjkPRnSp1s"),
        root_slot=67089626,
    )
    assert expected_delinquent.commission == delinquent.commission
    assert expected_delinquent.epoch_vote_account == delinquent.epoch_vote_account
    assert expected_delinquent.epoch_credits == delinquent.epoch_credits
    assert expected_delinquent.node_pubkey == delinquent.node_pubkey
    assert expected_delinquent.last_vote == delinquent.last_vote
    assert expected_delinquent.activated_stake == delinquent.activated_stake
    assert expected_delinquent.vote_pubkey == delinquent.vote_pubkey
    assert expected_delinquent.root_slot == delinquent.root_slot
    assert delinquent == expected_delinquent
    expected_current = RpcVoteAccountInfo(
        commission=85,
        epoch_vote_account=True,
        epoch_credits=[
            (360, 30653476, 30296700),
            (361, 31017670, 30653476),
            (362, 31382495, 31017670),
            (363, 31741125, 31382495),
            (364, 31968581, 31741125),
        ],
        node_pubkey=Pubkey.from_string("7LH3HCmvnJRvHvzinbDerTNQ2GvLvdnukdx1dQ26aCFt"),
        last_vote=157522003,
        activated_stake=15864630107818,
        vote_pubkey=Pubkey.from_string("F95vVhuyAjAtmXbg2EnNVWKkD5yQsDS5S83Uw1TUDcZm"),
        root_slot=157521972,
    )
    assert current == expected_current


def test_is_blockhash_valid() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "slot": 2483
    },
    "value": false
  },
  "id": 1
}"""
    parsed = IsBlockhashValidResp.from_json(raw)
    assert isinstance(parsed, IsBlockhashValidResp)
    assert parsed.value is False


def test_minimum_ledger_slot() -> None:
    raw = '{ "jsonrpc": "2.0", "result": 1234, "id": 1 }'
    parsed = MinimumLedgerSlotResp.from_json(raw)
    assert isinstance(parsed, MinimumLedgerSlotResp)
    assert parsed.value == 1234


def test_request_airdrop() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW",
  "id": 1
}"""
    parsed = RequestAirdropResp.from_json(raw)
    assert isinstance(parsed, RequestAirdropResp)
    assert parsed.value == Signature.from_string(
        "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"
    )


def test_validator_exit() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": true,
  "id": 1
}"""
    parsed = ValidatorExitResp.from_json(raw)
    assert isinstance(parsed, ValidatorExitResp)
    assert parsed.value is True


def test_send_transaction() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "result": "2id3YC2jK9G5Wo2phDx4gJVAew8DcY5NAojnVuao8rkxwPYPe8cSwE5GzhEgJA2y8fVjDEo6iR6ykBvDxrTQrtpb",
  "id": 1
}"""
    parsed = SendTransactionResp.from_json(raw)
    assert isinstance(parsed, SendTransactionResp)
    assert parsed.value == Signature.from_string(
        "2id3YC2jK9G5Wo2phDx4gJVAew8DcY5NAojnVuao8rkxwPYPe8cSwE5GzhEgJA2y8fVjDEo6iR6ykBvDxrTQrtpb"
    )


def test_simulate_transaction() -> None:
    raw = """{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "apiVersion": "1.10.34",
            "slot": 147616013
        },
        "value": {
            "accounts": null,
            "err": {
                "InstructionError": [
                    0,
                    {
                        "Custom": 0
                    }
                ]
            },
            "logs": [
                "Program Vote111111111111111111111111111111111111111 invoke [1]",
                "Program Vote111111111111111111111111111111111111111 failed: custom program error: 0x0"
            ],
            "unitsConsumed": 0,
            "returnData": null
        }
    },
    "id": "00f783f3-2ab0-42cd-80c8-8cdd14732f45"
}"""
    parsed = SimulateTransactionResp.from_json(raw)
    assert isinstance(parsed, SimulateTransactionResp)
    assert parsed.value == RpcSimulateTransactionResult(
        err=TransactionErrorInstructionError(0, InstructionErrorCustom(0)),
        logs=[
            "Program Vote111111111111111111111111111111111111111 invoke [1]",
            "Program Vote111111111111111111111111111111111111111 failed: custom program error: 0x0",
        ],
        units_consumed=0,
    )


def test_batch() -> None:
    parsed: List[Union[GetBlockHeightResp, GetFirstAvailableBlockResp]] = [
        GetBlockHeightResp(1233),
        GetFirstAvailableBlockResp(1),
    ]
    raw = batch_to_json(parsed)
    assert (
        batch_from_json(raw, [GetBlockHeightResp, GetFirstAvailableBlockResp]) == parsed
    )


def test_account_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "accountNotification",
  "params": {
    "result": {
      "context": {
        "slot": 5199307
      },
      "value": {
        "data": [
          "11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHPXHRDEHrBesJhZyqnnq9qJeUuF7WHxiuLuL5twc38w2TXNLxnDbjmuR",
          "base58"
        ],
        "executable": false,
        "lamports": 33594,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 635
      }
    },
    "subscription": 23784
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, AccountNotification)
    result = parsed.result
    assert isinstance(result, AccountNotificationResult)
    assert isinstance(result.value, Account)


def test_account_notification_json_parsed() -> None:
    raw_sub = '{"result":{"context":{"apiVersion":"1.10.25","slot":140702417},"value":{"data":{"parsed":{"info":{"isNative":false,"mint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","owner":"vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg","state":"initialized","tokenAmount":{"amount":"36010000000","decimals":6,"uiAmount":36010.0,"uiAmountString":"36010"}},"type":"account"},"program":"spl-token","space":165},"executable":false,"lamports":2039280,"owner":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","rentEpoch":325}}, "subscription":1}'
    parsed_sub = AccountNotificationJsonParsed.from_json(raw_sub)
    assert isinstance(parsed_sub, AccountNotificationJsonParsed)


def test_block_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "blockNotification",
  "params": {
    "result": {
      "context": {
        "slot": 112301554
      },
      "value": {
        "slot": 112301554,
        "block": {
          "previousBlockhash": "GJp125YAN4ufCSUvZJVdCyWQJ7RPWMmwxoyUQySydZA",
          "blockhash": "6ojMHjctdqfB55JDpEpqfHnP96fiaHEcvzEQ2NNcxzHP",
          "parentSlot": 112301553,
          "transactions": [
            {
              "transaction": [
                "OpltwoUvWxYi1P2U8vbIdE/aPntjYo5Aa0VQ2JJyeJE2g9Vvxk8dDGgFMruYfDu8/IfUWb0REppTe7IpAuuLRgIBAAkWnj4KHRpEWWW7gvO1c0BHy06wZi2g7/DLqpEtkRsThAXIdBbhXCLvltw50ZnjDx2hzw74NVn49kmpYj2VZHQJoeJoYJqaKcvuxCi/2i4yywedcVNDWkM84Iuw+cEn9/ROCrXY4qBFI9dveEERQ1c4kdU46xjxj9Vi+QXkb2Kx45QFVkG4Y7HHsoS6WNUiw2m4ffnMNnOVdF9tJht7oeuEfDMuUEaO7l9JeUxppCvrGk3CP45saO51gkwVYEgKzhpKjCx3rgsYxNR81fY4hnUQXSbbc2Y55FkwgRBpVvQK7/+clR4Gjhd3L4y+OtPl7QF93Akg1LaU9wRMs5nvfDFlggqI9PqJl+IvVWrNRdBbPS8LIIhcwbRTkSbqlJQWxYg3Bo2CTVbw7rt1ZubuHWWp0mD/UJpLXGm2JprWTePNULzHu67sfqaWF99LwmwjTyYEkqkRt1T0Je5VzHgJs0N5jY4iIU9K3lMqvrKOIn/2zEMZ+ol2gdgjshx+sphIyhw65F3J/Dbzk04LLkK+CULmN571Y+hFlXF2ke0BIuUG6AUF+4214Cu7FXnqo3rkxEHDZAk0lRrAJ8X/Z+iwuwI5cgbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCpDLAp8axcEkaQkLDKRoWxqp8XLNZSKial7Rk+ELAVVKWoWLRXRZ+OIggu0OzMExvVLE5VHqy71FNHq4gGitkiKYNFWSLIE4qGfdFLZXy/6hwS+wq9ewjikCpd//C9BcCL7Wl0iQdUslxNVCBZHnCoPYih9JXvGefOb9WWnjGy14sG9j70+RSVx6BlkFELWwFvIlWR/tHn3EhHAuL0inS2pwX7ZQTAU6gDVaoqbR2EiJ47cKoPycBNvHLoKxoY9AZaBjPl6q8SKQJSFyFd9n44opAgI6zMTjYF/8Ok4VpXEESp3QaoUyTI9sOJ6oFP6f4dwnvQelgXS+AEfAsHsKXxGAIUDQENAgMEBQAGBwgIDg8IBJCER3QXl1AVDBADCQoOAAQLERITDAjb7ugh3gOuTy==",
                "base64"
              ],
              "meta": {
                "err": null,
                "status": {
                  "Ok": null
                },
                "fee": 5000,
                "preBalances": [
                  1758510880, 2067120, 1566000, 1461600, 2039280, 2039280,
                  1900080, 1865280, 0, 3680844220, 2039280
                ],
                "postBalances": [
                  1758505880, 2067120, 1566000, 1461600, 2039280, 2039280,
                  1900080, 1865280, 0, 3680844220, 2039280
                ],
                "innerInstructions": [
                  {
                    "index": 0,
                    "instructions": [
                      {
                        "programIdIndex": 13,
                        "accounts": [1, 15, 3, 4, 2, 14],
                        "data": "21TeLgZXNbtHXVBzCaiRmH"
                      },
                      {
                        "programIdIndex": 14,
                        "accounts": [3, 4, 1],
                        "data": "6qfC8ic7Aq99"
                      },
                      {
                        "programIdIndex": 13,
                        "accounts": [1, 15, 3, 5, 2, 14],
                        "data": "21TeLgZXNbsn4QEpaSEr3q"
                      },
                      {
                        "programIdIndex": 14,
                        "accounts": [3, 5, 1],
                        "data": "6LC7BYyxhFRh"
                      }
                    ]
                  },
                  {
                    "index": 1,
                    "instructions": [
                      {
                        "programIdIndex": 14,
                        "accounts": [4, 3, 0],
                        "data": "7aUiLHFjSVdZ"
                      },
                      {
                        "programIdIndex": 19,
                        "accounts": [17, 18, 16, 9, 11, 12, 14],
                        "data": "8kvZyjATKQWYxaKR1qD53V"
                      },
                      {
                        "programIdIndex": 14,
                        "accounts": [9, 11, 18],
                        "data": "6qfC8ic7Aq99"
                      }
                    ]
                  }
                ],
                "logMessages": [
                  "Program QMNeHCGYnLVDn1icRAfQZpjPLBNkfGbSKRB83G5d8KB invoke [1]",
                  "Program QMWoBmAyJLAsA1Lh9ugMTw2gciTihncciphzdNzdZYV invoke [2]"
                ],
                "preTokenBalances": [
                  {
                    "accountIndex": 4,
                    "mint": "iouQcQBAiEXe6cKLS85zmZxUqaCqBdeHFpqKoSz615u",
                    "uiTokenAmount": {
                      "uiAmount": null,
                      "decimals": 6,
                      "amount": "0",
                      "uiAmountString": "0"
                    },
                    "owner": "LieKvPRE8XeX3Y2xVNHjKlpAScD12lYySBVQ4HqoJ5op",
                    "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                  },
                  {
                    "accountIndex": 5,
                    "mint": "iouQcQBAiEXe6cKLS85zmZxUqaCqBdeHFpqKoSz615u",
                    "uiTokenAmount": {
                      "uiAmount": 11513.0679,
                      "decimals": 6,
                      "amount": "11513067900",
                      "uiAmountString": "11513.0679"
                    },
                    "owner": "rXhAofQCT7NN9TUqigyEAUzV1uLL4boeD8CRkNBSkYk",
                    "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                  },
                  {
                    "accountIndex": 10,
                    "mint": "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1",
                    "uiTokenAmount": {
                      "uiAmount": null,
                      "decimals": 6,
                      "amount": "0",
                      "uiAmountString": "0"
                    },
                    "owner": "CL9wkGFT3SZRRNa9dgaovuRV7jrVVigBUZ6DjcgySsCU",
                    "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
                  },
                  {
                    "accountIndex": 11,
                    "mint": "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1",
                    "uiTokenAmount": {
                      "uiAmount": 15138.514093,
                      "decimals": 6,
                      "amount": "15138514093",
                      "uiAmountString": "15138.514093"
                    },
                    "owner": "LieKvPRE8XeX3Y2xVNHjKlpAScD12lYySBVQ4HqoJ5op",
                    "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
                  }
                ],
                "postTokenBalances": [
                  {
                    "accountIndex": 4,
                    "mint": "iouQcQBAiEXe6cKLS85zmZxUqaCqBdeHFpqKoSz615u",
                    "uiTokenAmount": {
                      "uiAmount": null,
                      "decimals": 6,
                      "amount": "0",
                      "uiAmountString": "0"
                    },
                    "owner": "LieKvPRE8XeX3Y2xVNHjKlpAScD12lYySBVQ4HqoJ5op",
                    "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                  },
                  {
                    "accountIndex": 5,
                    "mint": "iouQcQBAiEXe6cKLS85zmZxUqaCqBdeHFpqKoSz615u",
                    "uiTokenAmount": {
                      "uiAmount": 11513.103028,
                      "decimals": 6,
                      "amount": "11513103028",
                      "uiAmountString": "11513.103028"
                    },
                    "owner": "rXhAofQCT7NN9TUqigyEAUzV1uLL4boeD8CRkNBSkYk",
                    "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                  },
                  {
                    "accountIndex": 10,
                    "mint": "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1",
                    "uiTokenAmount": {
                      "uiAmount": null,
                      "decimals": 6,
                      "amount": "0",
                      "uiAmountString": "0"
                    },
                    "owner": "CL9wkGFT3SZRRNa9dgaovuRV7jrVVigBUZ6DjcgySsCU",
                    "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
                  },
                  {
                    "accountIndex": 11,
                    "mint": "Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1",
                    "uiTokenAmount": {
                      "uiAmount": 15489.767829,
                      "decimals": 6,
                      "amount": "15489767829",
                      "uiAmountString": "15489.767829"
                    },
                    "owner": "BeiHVPRE8XeX3Y2xVNrSsTpAScH94nYySBVQ4HqgN9at",
                    "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
                  }
                ],
                "rewards": []
              }
            }
          ],
          "blockTime": 1639926816,
          "blockHeight": 101210751
        },
        "err": null
      }
    },
    "subscription": 14
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, BlockNotification)
    result = parsed.result
    assert isinstance(result, BlockNotificationResult)
    assert isinstance(result.value, RpcBlockUpdate)


def test_logs_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "logsNotification",
  "params": {
    "result": {
      "context": {
        "slot": 5208469
      },
      "value": {
        "signature": "5h6xBEauJ3PK6SWCZ1PGjBvj8vDdWG3KpwATGy1ARAXFSDwt8GFXM7W5Ncn16wmqokgpiKRLuS83KUxyZyv2sUYv",
        "err": null,
        "logs": [
          "BPF program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri success"
        ]
      }
    },
    "subscription": 24040
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, LogsNotification)
    result = parsed.result
    assert isinstance(result, LogsNotificationResult)
    assert isinstance(result.value, RpcLogsResponse)


def test_program_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "programNotification",
  "params": {
    "result": {
      "context": {
        "slot": 5208469
      },
      "value": {
        "pubkey": "H4vnBqifaSACnKa7acsxstsY1iV1bvJNxsCY7enrd1hq",
        "account": {
          "data": [
            "11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHPXHRDEHrBesJhZyqnnq9qJeUuF7WHxiuLuL5twc38w2TXNLxnDbjmuR",
            "base58"
          ],
          "executable": false,
          "lamports": 33594,
          "owner": "11111111111111111111111111111111",
          "rentEpoch": 636
        }
      }
    },
    "subscription": 24040
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, ProgramNotification)
    result = parsed.result
    assert isinstance(result, ProgramNotificationResult)
    assert isinstance(result.value, RpcKeyedAccount)


def test_signature_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "signatureNotification",
  "params": {
    "result": {
      "context": {
        "slot": 5207624
      },
      "value": {
        "err": null
      }
    },
    "subscription": 24006
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, SignatureNotification)
    result = parsed.result
    assert isinstance(result, SignatureNotificationResult)
    assert isinstance(result.value, RpcSignatureResponse)


def test_slot_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "slotNotification",
  "params": {
    "result": {
      "parent": 75,
      "root": 44,
      "slot": 76
    },
    "subscription": 0
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, SlotNotification)
    result = parsed.result
    assert isinstance(result, SlotInfo)


def test_slot_update_notification() -> None:
    raw = """{
  "jsonrpc": "2.0",
  "method": "slotsUpdatesNotification",
  "params": {
    "result": {
      "parent": 75,
      "slot": 76,
      "timestamp": 1625081266243,
      "type": "optimisticConfirmation"
    },
    "subscription": 0
  }
}"""
    parsed = parse_notification(raw)
    assert isinstance(parsed, SlotUpdateNotification)
    result = parsed.result
    assert isinstance(result, SlotUpdateOptimisticConfirmation)


def test_vote_notification() -> None:
    raw = '{"jsonrpc":"2.0","method":"voteNotification","params":{"result":{"votePubkey":"2rQ2oMoB29Ge8pWuPB7pgc4tGTj5Ppzdqd53ThYPAtU1","slots":[214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244],"hash":"5qj25gJJWnzpq5SGKPVpgw84NoNV4qNDbhTgchCUqu42","timestamp":1664066781,"signature":"mz5NhjFjAs5r9J74ndChvYGxbXhnBRUz5UULkSsaDos1SmqoFJrRC16LYtVX73y42RTYEBhGbUpRx6umyxEzRM1"},"subscription":3}}'
    parsed = parse_notification(raw)
    assert isinstance(parsed, VoteNotification)
    result = parsed.result
    assert isinstance(result, RpcVote)
    assert result.timestamp == 1664066781


def test_parse_ws_message() -> None:
    raw_err = '{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid param: WrongSize"},"id":1}'
    parsed_err = parse_websocket_message(raw_err)
    assert isinstance(parsed_err, SubscriptionError)
    assert isinstance(parsed_err.error, RpcError)
    raw_ok = '{ "jsonrpc": "2.0", "result": 23784, "id": 3 }'
    parsed_ok = parse_websocket_message(raw_ok)
    assert isinstance(parsed_ok, SubscriptionResult)
    assert parsed_ok.result == 23784
    assert parsed_ok.id == 3
    raw_notification = '{ "jsonrpc": "2.0", "method": "rootNotification", "params": { "result": 4, "subscription": 0 } }'
    parsed_notification = parse_websocket_message(raw_notification)
    assert isinstance(parsed_notification, RootNotification)
    assert parsed_notification.result == 4
