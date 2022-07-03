"""These tests are mainly about getting mypy to check stuff, as it doesn't check doc examples."""

from solders.rpc.requests import (
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockHeight,
    GetBlockProduction,
    GetBlockCommitment,
    GetBlocks,
    GetBlocksWithLimit,
    GetBlockTime,
    GetClusterNodes,
    GetEpochInfo,
    GetEpochSchedule,
    GetFeeForMessage,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
    GetHighestSnapshotSlot,
    GetIdentity,
    GetInflationGovernor,
    GetInflationRate,
    GetInflationReward,
    GetLargestAccounts,
    GetLatestBlockhash,
    GetLeaderSchedule,
    GetMaxRetransmitSlot,
    GetMaxShredInsertSlot,
    GetMinimumBalanceForRentExemption,
    GetMultipleAccounts,
    GetProgramAccounts,
    GetRecentPerformanceSamples,
    GetSignaturesForAddress,
    GetSignatureStatuses,
    GetSlot,
    GetSlotLeader,
    GetSlotLeaders,
    GetStakeActivation,
    GetSupply,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenLargestAccounts,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    IsBlockhashValid,
    MinimumLedgerSlot,
    RequestAirdrop,
    SendTransaction,
    AccountSubscribe,
    BlockSubscribe,
    LogsSubscribe,
    ProgramSubscribe,
    SignatureSubscribe,
    SlotSubscribe,
    SlotsUpdatesSubscribe,
    RootSubscribe,
    VoteSubscribe,
    AccountUnsubscribe,
    BlockUnsubscribe,
    LogsUnsubscribe,
    ProgramUnsubscribe,
    SignatureUnsubscribe,
    SimulateTransaction,
    SlotUnsubscribe,
    SlotsUpdatesUnsubscribe,
    RootUnsubscribe,
    VoteUnsubscribe,
    batch_to_json,
    batch_from_json,
)
from solders.rpc.config import (
    RpcSignatureStatusConfig,
    RpcRequestAirdropConfig,
    RpcContextConfig,
    RpcBlockConfig,
    RpcAccountInfoConfig,
    RpcBlockProductionConfig,
    RpcBlockProductionConfigRange
)
from solders.transaction_status import UiTransactionEncoding, TransactionDetails
from solders.signature import Signature
from solders.message import Message
from solders.commitment_config import CommitmentLevel
from solders.account_decoder import UiAccountEncoding
from solders.pubkey import Pubkey


def test_get_account_info() -> None:
    config = RpcAccountInfoConfig(UiAccountEncoding.Base64)
    req = GetAccountInfo(Pubkey.default(), config)
    as_json = req.to_json()
    assert GetAccountInfo.from_json(as_json) == req


def test_get_balance() -> None:
    config = RpcContextConfig(min_context_slot=1)
    req = GetBalance(Pubkey.default(), config)
    as_json = req.to_json()
    assert GetBalance.from_json(as_json) == req


def test_get_block() -> None:
    config = RpcBlockConfig(
        encoding=UiTransactionEncoding.Base58,
        transaction_details=TransactionDetails.None_,
    )
    req = GetBlock(123, config)
    as_json = req.to_json()
    assert GetBlock.from_json(as_json) == req


def test_get_block_height() -> None:
    config = RpcContextConfig(min_context_slot=123)
    req = GetBlockHeight(config)
    as_json = req.to_json()
    assert GetBlockHeight.from_json(as_json) == req

def test_get_block_production() -> None:
    slot_range = RpcBlockProductionConfigRange(first_slot=10, last_slot=15)
    config = RpcBlockProductionConfig(identity=Pubkey.default(), range=slot_range)
    req = GetBlockProduction(config)
    as_json = req.to_json()
    assert GetBlockProduction.from_json(as_json) == req

def test_get_block_commitment() -> None:
    req = GetBlockCommitment(123)
    as_json = req.to_json()
    assert GetBlockCommitment.from_json(as_json) == req

def test_get_blocks() -> None:
    req = GetBlocks(123, commitment=CommitmentLevel.Processed)
    as_json = req.to_json()
    assert GetBlocks.from_json(as_json) == req

def test_get_blocks_with_limit() -> None:
    req = GetBlocksWithLimit(123, 5, commitment=CommitmentLevel.Processed)
    as_json = req.to_json()
    assert GetBlocksWithLimit.from_json(as_json) == req

def test_get_block_time() -> None:
    req = GetBlockTime(123)
    as_json = req.to_json()
    assert GetBlockTime.from_json(as_json) == req

def test_get_cluster_nodes() -> None:
    req = GetClusterNodes(123)
    as_json = req.to_json()
    assert GetClusterNodes.from_json(as_json) == req

def test_get_epoch_info() -> None:
    config = RpcContextConfig(commitment=CommitmentLevel.Processed)
    req = GetEpochInfo(config)
    as_json = req.to_json()
    assert GetEpochInfo.from_json(as_json) == req

def test_get_epoch_schedule() -> None:
    req = GetEpochSchedule(123)
    as_json = req.to_json()
    assert GetEpochSchedule.from_json(as_json) == req

def test_get_fee_for_message() -> None:
    req = GetFeeForMessage(Message.default(), commitment=CommitmentLevel.Processed)
    as_json = req.to_json()
    assert GetFeeForMessage.from_json(as_json) == req

def test_get_first_available_block() -> None:
    req = GetFirstAvailableBlock(123)
    as_json = req.to_json()
    assert GetFirstAvailableBlock.from_json(as_json) == req

def test_get_genesis_hash() -> None:
    req = GetGenesisHash(123)
    as_json = req.to_json()
    assert GetGenesisHash.from_json(as_json) == req

def test_get_health() -> None:
    req = GetHealth(123)
    as_json = req.to_json()
    assert GetHealth.from_json(as_json) == req

def test_get_highest_snapshot_slot() -> None:
    req = GetHighestSnapshotSlot(123)
    as_json = req.to_json()
    assert GetHighestSnapshotSlot.from_json(as_json) == req

def test_get_identity() -> None:
    req = GetIdentity(123)
    as_json = req.to_json()
    assert GetIdentity.from_json(as_json) == req

def test_get_signature_statuses() -> None:
    req = GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True))
    as_json = req.to_json()
    assert GetSignatureStatuses.from_json(as_json) == req


def test_request_airdrop() -> None:
    req = RequestAirdrop(Pubkey.default(), 1000)
    as_json = req.to_json()
    assert RequestAirdrop.from_json(as_json) == req


def test_batch() -> None:
    reqs = [
        GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True)),
        RequestAirdrop(Pubkey.default(), 1000),
    ]
    as_json = batch_to_json(reqs)
    assert as_json == (
        '[{"method":"getSignatureStatuses","jsonrpc":"2.0","id":0,"params"'
        ':[["1111111111111111111111111111111111111111111111111111111111111111"],'
        '{"searchTransactionHistory":true}]},{"method":"requestAirdrop","jsonrpc":"2.0","id":0,'
        '"params":["11111111111111111111111111111111",1000]}]'
    )
    assert batch_from_json(as_json) == reqs
