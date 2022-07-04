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
    RpcBlockProductionConfigRange,
    RpcLeaderScheduleConfig,
    RpcEpochConfig,
    RpcLargestAccountsFilter,
    RpcSupplyConfig,
    RpcTokenAccountsFilterProgramId,
    RpcProgramAccountsConfig,
    RpcSignaturesForAddressConfig,
)
from solders.rpc.filter import Memcmp
from solders.transaction_status import UiTransactionEncoding, TransactionDetails
from solders.signature import Signature
from solders.message import Message
from solders.commitment_config import CommitmentLevel
from solders.account_decoder import UiAccountEncoding, UiDataSliceConfig
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


def test_get_inflation_governor() -> None:
    req = GetInflationGovernor(CommitmentLevel.Finalized)
    as_json = req.to_json()
    assert GetInflationGovernor.from_json(as_json) == req


def test_get_inflation_rate() -> None:
    req = GetInflationRate(123)
    as_json = req.to_json()
    assert GetInflationRate.from_json(as_json) == req


def test_get_inflation_reward() -> None:
    config = RpcEpochConfig(epoch=1234)
    addresses = [Pubkey.default(), Pubkey.default()]
    req = GetInflationReward(addresses, config)
    as_json = req.to_json()
    assert GetInflationReward.from_json(as_json) == req


def test_get_largest_accounts() -> None:
    commitment = CommitmentLevel.Processed
    filter_ = RpcLargestAccountsFilter.Circulating
    req = GetLargestAccounts(commitment=commitment, filter_=filter_)
    as_json = req.to_json()
    assert GetLargestAccounts.from_json(as_json) == req


def test_get_latest_blockhash() -> None:
    config = RpcContextConfig(commitment=CommitmentLevel.Processed)
    req = GetLatestBlockhash(config)
    as_json = req.to_json()
    assert GetLatestBlockhash.from_json(as_json) == req


def test_get_leader_schedule() -> None:
    config = RpcLeaderScheduleConfig(identity=Pubkey.default())
    req = GetLeaderSchedule(123, config)
    as_json = req.to_json()
    assert GetLeaderSchedule.from_json(as_json) == req


def test_get_max_retransmit_slot() -> None:
    req = GetMaxRetransmitSlot(123)
    as_json = req.to_json()
    assert GetMaxRetransmitSlot.from_json(as_json) == req


def test_get_max_shred_insert_slot() -> None:
    req = GetMaxShredInsertSlot(123)
    as_json = req.to_json()
    assert GetMaxShredInsertSlot.from_json(as_json) == req


def test_get_minimum_balance_for_rent_exemption() -> None:
    req = GetMinimumBalanceForRentExemption(50)
    as_json = req.to_json()
    assert GetMinimumBalanceForRentExemption.from_json(as_json) == req


def test_get_multiple_accounts() -> None:
    encoding = UiAccountEncoding.Base64Zstd
    data_slice = UiDataSliceConfig(10, 8)
    config = RpcAccountInfoConfig(encoding=encoding, data_slice=data_slice)
    accounts = [Pubkey.default(), Pubkey.default()]
    req = GetMultipleAccounts(accounts, config)
    as_json = req.to_json()
    assert GetMultipleAccounts.from_json(as_json) == req


def test_get_program_accounts() -> None:
    acc_info_config = RpcAccountInfoConfig.default()
    filters = [10, Memcmp(offset=10, bytes_=b"123")]
    config = RpcProgramAccountsConfig(acc_info_config, filters)
    req = GetProgramAccounts(Pubkey.default(), config)
    as_json = req.to_json()
    assert GetProgramAccounts.from_json(as_json) == req


def test_get_recent_performance_samples() -> None:
    req = GetRecentPerformanceSamples(5)
    as_json = req.to_json()
    assert GetRecentPerformanceSamples.from_json(as_json) == req


def test_get_signatures_for_address() -> None:
    config = RpcSignaturesForAddressConfig(limit=10)
    req = GetSignaturesForAddress(Pubkey.default(), config)
    as_json = req.to_json()
    assert GetSignaturesForAddress.from_json(as_json) == req


def test_get_signature_statuses() -> None:
    req = GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True))
    as_json = req.to_json()
    assert GetSignatureStatuses.from_json(as_json) == req


def test_get_slot() -> None:
    config = RpcContextConfig(min_context_slot=123)
    req = GetSlot(config)
    as_json = req.to_json()
    assert GetSlot.from_json(as_json) == req


def test_get_slot_leader() -> None:
    config = RpcContextConfig(min_context_slot=123)
    req = GetSlotLeader(config)
    as_json = req.to_json()
    assert GetSlotLeader.from_json(as_json) == req


def test_get_slot_leaders() -> None:
    req = GetSlotLeaders(100, 10)
    as_json = req.to_json()
    assert GetSlotLeaders.from_json(as_json) == req


def test_get_stake_activation() -> None:
    config = RpcEpochConfig(epoch=1234)
    req = GetStakeActivation(Pubkey.default(), config)
    as_json = req.to_json()
    assert GetStakeActivation.from_json(as_json) == req


def test_get_supply() -> None:
    config = RpcSupplyConfig(exclude_non_circulating_accounts_list=True)
    req = GetSupply(config)
    as_json = req.to_json()
    assert GetSupply.from_json(as_json) == req


def test_get_token_account_balance() -> None:
    config = RpcEpochConfig(epoch=1234)
    req = GetTokenAccountBalance(Pubkey.default(), CommitmentLevel.Processed)
    as_json = req.to_json()
    assert GetTokenAccountBalance.from_json(as_json) == req


def test_get_token_accounts_by_delegate() -> None:
    program_filter = RpcTokenAccountsFilterProgramId(
        Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
    )
    config = RpcAccountInfoConfig(min_context_slot=1234)
    req = GetTokenAccountsByDelegate(Pubkey.default(), program_filter, config)
    assert req.filter_ == program_filter
    as_json = req.to_json()
    assert GetTokenAccountsByDelegate.from_json(as_json) == req


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
