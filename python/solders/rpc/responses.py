from typing import Any, List, Sequence, TypeVar, Union

from ..solders import (
    AccountNotification,
    AccountNotificationJsonParsed,
    AccountNotificationJsonParsedResult,
    AccountNotificationResult,
    BlockNotification,
    BlockNotificationResult,
    BlockStoreError,
    GetAccountInfoJsonParsedResp,
    GetAccountInfoMaybeJsonParsedResp,
    GetAccountInfoResp,
    GetBalanceResp,
    GetBlockCommitmentResp,
    GetBlockHeightResp,
    GetBlockProductionResp,
    GetBlockResp,
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
    GetMultipleAccountsJsonParsedResp,
    GetMultipleAccountsMaybeJsonParsedResp,
    GetMultipleAccountsResp,
    GetProgramAccountsJsonParsedResp,
    GetProgramAccountsMaybeJsonParsedResp,
    GetProgramAccountsResp,
    GetProgramAccountsWithContextJsonParsedResp,
    GetProgramAccountsWithContextMaybeJsonParsedResp,
    GetProgramAccountsWithContextResp,
    GetRecentPerformanceSamplesResp,
    GetSignaturesForAddressResp,
    GetSignatureStatusesResp,
    GetSlotLeaderResp,
    GetSlotLeadersResp,
    GetSlotResp,
    GetSupplyResp,
    GetTokenAccountBalanceResp,
    GetTokenAccountsByDelegateJsonParsedResp,
    GetTokenAccountsByDelegateResp,
    GetTokenAccountsByOwnerJsonParsedResp,
    GetTokenAccountsByOwnerResp,
    GetTokenLargestAccountsResp,
    GetTokenSupplyResp,
    GetTransactionCountResp,
    GetTransactionResp,
    GetVersionResp,
    GetVoteAccountsResp,
    IsBlockhashValidResp,
    LogsNotification,
    LogsNotificationResult,
    MinimumLedgerSlotResp,
    ProgramNotification,
    ProgramNotificationJsonParsed,
    ProgramNotificationJsonParsedResult,
    ProgramNotificationResult,
    RequestAirdropResp,
    RootNotification,
    RpcAccountBalance,
    RpcBlockCommitment,
    RpcBlockhash,
    RpcBlockProduction,
    RpcBlockProductionRange,
    RpcBlockUpdate,
    RpcConfirmedTransactionStatusWithSignature,
    RpcContactInfo,
    RpcIdentity,
    RpcInflationGovernor,
    RpcInflationRate,
    RpcInflationReward,
    RpcKeyedAccount,
    RpcKeyedAccountJsonParsed,
    RpcLogsResponse,
    RpcPerfSample,
    RpcResponseContext,
    RpcSignatureResponse,
    RpcSimulateTransactionResult,
    RpcSnapshotSlotInfo,
    RpcSupply,
    RpcTokenAccountBalance,
    RpcVersionInfo,
    RpcVote,
    RpcVoteAccountInfo,
    RpcVoteAccountStatus,
    SendTransactionResp,
    SignatureNotification,
    SignatureNotificationResult,
    SimulateTransactionResp,
    SlotInfo,
    SlotNotification,
    SlotTransactionStats,
    SlotUpdateCompleted,
    SlotUpdateCreatedBank,
    SlotUpdateDead,
    SlotUpdateFirstShredReceived,
    SlotUpdateFrozen,
    SlotUpdateNotification,
    SlotUpdateOptimisticConfirmation,
    SlotUpdateRoot,
    SubscriptionError,
    SubscriptionResult,
    UnsubscribeResult,
    ValidatorExitResp,
    VoteNotification,
    parse_notification,
    parse_websocket_message,
)
from ..solders import (
    batch_responses_from_json as _batch_from_json,
)
from ..solders import (
    batch_responses_to_json as _batch_to_json,
)
from .errors import (
    BlockCleanedUpMessage,
    BlockNotAvailableMessage,
    BlockStatusNotAvailableYetMessage,
    InternalErrorMessage,
    InvalidParamsMessage,
    InvalidRequestMessage,
    KeyExcludedFromSecondaryIndexMessage,
    LongTermStorageSlotSkippedMessage,
    MethodNotFoundMessage,
    MinContextSlotNotReachedMessage,
    NodeUnhealthyMessage,
    ParseErrorMessage,
    RpcCustomErrorFieldless,
    ScanErrorMessage,
    SendTransactionPreflightFailureMessage,
    SlotSkippedMessage,
    TransactionPrecompileVerificationFailureMessage,
    UnsupportedTransactionVersion,
    UnsupportedTransactionVersionMessage,
)

Notification = Union[
    AccountNotification,
    AccountNotificationJsonParsed,
    BlockNotification,
    LogsNotification,
    ProgramNotification,
    ProgramNotificationJsonParsed,
    SignatureNotification,
    SlotNotification,
    SlotUpdateNotification,
    RootNotification,
    VoteNotification,
]

RpcBlockUpdateError = Union[UnsupportedTransactionVersion, BlockStoreError]

WebsocketMessage = Union[
    Notification, SubscriptionResult, SubscriptionError, UnsubscribeResult
]

RPCError = Union[
    RpcCustomErrorFieldless,
    BlockCleanedUpMessage,
    SendTransactionPreflightFailureMessage,
    BlockNotAvailableMessage,
    NodeUnhealthyMessage,
    TransactionPrecompileVerificationFailureMessage,
    SlotSkippedMessage,
    LongTermStorageSlotSkippedMessage,
    KeyExcludedFromSecondaryIndexMessage,
    ScanErrorMessage,
    BlockStatusNotAvailableYetMessage,
    MinContextSlotNotReachedMessage,
    UnsupportedTransactionVersionMessage,
    ParseErrorMessage,
    InvalidRequestMessage,
    MethodNotFoundMessage,
    InvalidParamsMessage,
    InternalErrorMessage,
]

T = TypeVar("T")
Resp = Union[RPCError, T]

SlotUpdate = Union[
    SlotUpdateFirstShredReceived,
    SlotUpdateCompleted,
    SlotUpdateCreatedBank,
    SlotUpdateDead,
    SlotUpdateOptimisticConfirmation,
    SlotUpdateRoot,
    SlotUpdateFrozen,
]

RPCResult = Union[
    RPCError,
    GetAccountInfoResp,
    GetAccountInfoJsonParsedResp,
    GetAccountInfoMaybeJsonParsedResp,
    GetBalanceResp,
    GetBlockProductionResp,
    GetBlockResp,
    GetBlockCommitmentResp,
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
    GetMultipleAccountsMaybeJsonParsedResp,
    GetProgramAccountsWithContextResp,
    GetProgramAccountsResp,
    GetProgramAccountsWithContextJsonParsedResp,
    GetProgramAccountsJsonParsedResp,
    GetProgramAccountsMaybeJsonParsedResp,
    GetProgramAccountsWithContextMaybeJsonParsedResp,
    GetRecentPerformanceSamplesResp,
    GetSignaturesForAddressResp,
    GetSignatureStatusesResp,
    GetSlotResp,
    GetSlotLeaderResp,
    GetSlotLeadersResp,
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
    RpcVersionInfo,
    GetVoteAccountsResp,
    IsBlockhashValidResp,
    MinimumLedgerSlotResp,
    RequestAirdropResp,
    SendTransactionResp,
    SimulateTransactionResp,
]


def batch_to_json(resps: Sequence[RPCResult]) -> str:
    """Serialize a list of response objects into a single batch response JSON.

    Args:
        resps: A list of response objects.

    Returns:
        str: The batch JSON string.

    Example:
        >>> from solders.rpc.responses import batch_to_json, GetBlockHeightResp, GetFirstAvailableBlockResp
        >>> batch_to_json([GetBlockHeightResp(1233), GetFirstAvailableBlockResp(1)])
        '[{"id":0,"jsonrpc":"2.0","result":1233},{"id":0,"jsonrpc":"2.0","result":1}]'

    """
    return _batch_to_json(resps)


def batch_from_json(raw: str, parsers: Sequence[Any]) -> List[RPCResult]:
    """Deserialize a batch request JSON string into a list of request objects.

    Args:
        raw (str): The batch JSON string.
        parsers (Sequence): The classes to parse.

    Returns:
        A list of response objects.

    Example:
        >>> from solders.rpc.responses import batch_from_json, GetBlockHeightResp, GetFirstAvailableBlockResp
        >>> raw = '[{ "jsonrpc": "2.0", "result": 1233, "id": 1 },{ "jsonrpc": "2.0", "result": 111, "id": 1 }]'
        >>> batch_from_json(raw, [GetBlockHeightResp, GetFirstAvailableBlockResp])
        [GetBlockHeightResp(
            1233,
        ), GetFirstAvailableBlockResp(
            111,
        )]

    """
    return _batch_from_json(raw, parsers)


__all__ = [
    "RpcResponseContext",
    "GetAccountInfoResp",
    "GetAccountInfoJsonParsedResp",
    "GetAccountInfoMaybeJsonParsedResp",
    "GetBalanceResp",
    "RpcBlockCommitment",
    "GetBlockCommitmentResp",
    "GetBlockHeightResp",
    "RpcBlockProductionRange",
    "RpcBlockProduction",
    "GetBlockProductionResp",
    "GetBlockResp",
    "GetBlocksResp",
    "GetBlocksWithLimitResp",
    "GetBlockTimeResp",
    "RpcContactInfo",
    "GetClusterNodesResp",
    "GetEpochInfoResp",
    "GetEpochScheduleResp",
    "GetFeeForMessageResp",
    "GetFirstAvailableBlockResp",
    "GetGenesisHashResp",
    "GetHealthResp",
    "RpcSimulateTransactionResult",
    "RpcSnapshotSlotInfo",
    "GetHighestSnapshotSlotResp",
    "RpcIdentity",
    "GetIdentityResp",
    "RpcInflationGovernor",
    "GetInflationGovernorResp",
    "RpcInflationRate",
    "GetInflationRateResp",
    "RpcInflationReward",
    "GetInflationRewardResp",
    "RpcAccountBalance",
    "GetLargestAccountsResp",
    "RpcBlockhash",
    "GetLatestBlockhashResp",
    "GetLeaderScheduleResp",
    "GetMaxRetransmitSlotResp",
    "GetMaxShredInsertSlotResp",
    "GetMinimumBalanceForRentExemptionResp",
    "GetMultipleAccountsResp",
    "GetMultipleAccountsJsonParsedResp",
    "GetMultipleAccountsMaybeJsonParsedResp",
    "RpcKeyedAccount",
    "RpcKeyedAccountJsonParsed",
    "GetProgramAccountsWithContextResp",
    "GetProgramAccountsWithContextJsonParsedResp",
    "GetProgramAccountsResp",
    "GetProgramAccountsJsonParsedResp",
    "GetProgramAccountsWithContextMaybeJsonParsedResp",
    "GetProgramAccountsMaybeJsonParsedResp",
    "RpcPerfSample",
    "GetRecentPerformanceSamplesResp",
    "RpcConfirmedTransactionStatusWithSignature",
    "GetSignaturesForAddressResp",
    "GetSignatureStatusesResp",
    "GetSlotResp",
    "GetSlotLeaderResp",
    "GetSlotLeadersResp",
    "RpcSupply",
    "GetSupplyResp",
    "GetTokenAccountBalanceResp",
    "GetTokenAccountsByDelegateResp",
    "GetTokenAccountsByDelegateJsonParsedResp",
    "GetTokenAccountsByOwnerResp",
    "GetTokenAccountsByOwnerJsonParsedResp",
    "RpcTokenAccountBalance",
    "GetTokenLargestAccountsResp",
    "GetTokenSupplyResp",
    "GetTransactionResp",
    "GetTransactionCountResp",
    "RpcVersionInfo",
    "GetVersionResp",
    "RpcVoteAccountInfo",
    "RpcVoteAccountStatus",
    "GetVoteAccountsResp",
    "IsBlockhashValidResp",
    "MinimumLedgerSlotResp",
    "RequestAirdropResp",
    "ValidatorExitResp",
    "SendTransactionResp",
    "SimulateTransactionResp",
    "RpcLogsResponse",
    "RpcVote",
    "SlotTransactionStats",
    "SlotUpdateFirstShredReceived",
    "SlotUpdateCompleted",
    "SlotUpdateCreatedBank",
    "SlotUpdateDead",
    "SlotUpdateOptimisticConfirmation",
    "SlotUpdateRoot",
    "SlotUpdateFrozen",
    "AccountNotificationResult",
    "AccountNotification",
    "AccountNotificationJsonParsedResult",
    "AccountNotificationJsonParsed",
    "BlockNotificationResult",
    "BlockNotification",
    "LogsNotificationResult",
    "LogsNotification",
    "ProgramNotificationResult",
    "ProgramNotification",
    "ProgramNotificationJsonParsedResult",
    "ProgramNotificationJsonParsed",
    "RpcSignatureResponse",
    "SignatureNotificationResult",
    "SignatureNotification",
    "SlotInfo",
    "SlotNotification",
    "SlotUpdateNotification",
    "RootNotification",
    "VoteNotification",
    "SubscriptionResult",
    "SubscriptionError",
    "BlockStoreError",
    "RpcBlockUpdate",
    "UnsubscribeResult",
    "batch_to_json",
    "batch_from_json",
    "parse_notification",
    "parse_websocket_message",
    "Notification",
    "RpcBlockUpdateError",
    "WebsocketMessage",
    "RPCError",
    "Resp",
    "SlotUpdate",
    "RPCResult",
]
