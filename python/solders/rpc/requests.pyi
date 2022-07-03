from typing import Optional, Sequence, List
from solders.pubkey import Pubkey
from solders.rpc.config import (
    RpcAccountInfoConfig,
    RpcContextConfig,
    RpcBlockConfig,
    RpcEpochConfig,
)
from solders.commitment_config import CommitmentLevel
from solders.message import Message

class GetAccountInfo:
    def __init__(
        pubkey: Pubkey,
        config: Optional[RpcAccountInfoConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def pubkey(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcAccountInfoConfig]: ...

class GetBalance:
    def __init__(
        pubkey: Pubkey,
        config: Optional[RpcContextConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def pubkey(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetBlock:
    def __init__(
        slot: int, config: Optional[RpcBlockConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def slot(self) -> int: ...
    @property
    def config(self) -> Optional[RpcBlockConfig]: ...

class GetBlockHeight:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetBlockProduction:
    def __init__(
        config: Optional[RpcBlockProductionConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcBlockProductionConfig]: ...

class GetBlockCommitment:
    def __init__(slot: int, id: Optional[int] = None): ...
    @property
    def slot(self) -> int: ...

class GetBlocks:
    def __init__(
        start: int,
        end: Optional[int] = None,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def start(self) -> int: ...
    @property
    def end(self) -> Optional[int]: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetBlocksWithLimit:
    def __init__(
        start: int,
        limit: Optional[int] = None,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def start(self) -> int: ...
    @property
    def limit(self) -> Optional[int]: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetBlockTime:
    def __init__(slot: int, id: Optional[int] = None): ...
    @property
    def slot(self) -> int: ...

class GetEpochInfo:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetFeeForMessage:
    def __init__(
        message: Message,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def message(self) -> Message: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetInflationGovernor:
    def __init__(
        commitment: Optional[CommitmentLevel] = None, id: Optional[int] = None
    ): ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetInflationReward:
    def __init__(
        addresses: Sequence[Pubkey],
        config: Optional[RpcEpochConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def addresses(self) -> List[Pubkey]: ...
    @property
    def config(self) -> Optional[RpcEpochConfig]: ...

class GetLargestAccounts:
    def __init__(
        commitment: Optional[CommitmentLevel] = None,
        filter_: Optional[RpcLargestAccountsFilter] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...
    @property
    def filter_(self) -> Optional[RpcLargestAccountsFilter]: ...

class GetLatestBlockhash:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetLeaderSchedule:
    def __init__(
        slot: Optional[int] = None,
        config: Optional[RpcLeaderScheduleConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def slot(self) -> Optional[int]: ...
    @property
    def config(self) -> Optional[RpcLeaderScheduleConfig]: ...

class GetMinimumBalanceForRentExemption:
    def __init__(
        length: int,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def length(self) -> int: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetMultipleAccounts:
    def __init__(
        accounts: Sequence[Pubkey],
        config: Optional[RpcAccountInfoConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def accounts(self) -> List[Pubkey]: ...
    @property
    def config(self) -> Optional[RpcAccountInfoConfig]: ...

class GetProgramAccounts:
    def __init__(
        program: Pubkey,
        config: Optional[RpcProgramAccountsConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def program(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcProgramAccountsConfig]: ...

class GetRecentPerformanceSamples:
    def __init__(limit: Optional[int] = None, id: Optional[int] = None): ...
    @property
    def limit(self) -> Optional[int]: ...

class GetSignaturesForAddress:
    def __init__(
        address: Pubkey,
        config: Optional[RpcSignaturesForAddressConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def address(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcSignaturesForAddressConfig]: ...

class GetSignatureStatuses:
    def __init__(
        signatures: Sequence[Signature],
        config: Optional[RpcSignatureStatusConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def signatures(self) -> List[Signature]: ...
    @property
    def config(self) -> Optional[RpcSignatureStatusConfig]: ...

class GetSlot:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetSlotLeader:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetSlotLeaders:
    def __init__(start: int, limit: int, id: Optional[int] = None): ...
    @property
    def start(self) -> int: ...
    @property
    def limit(self) -> int: ...

class GetStakeActivation:
    def __init__(
        account: Pubkey,
        config: Optional[RpcEpochConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def account(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcEpochConfig]: ...

class GetSupply:
    def __init__(
        config: Optional[RpcSupplyConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcSupplyConfig]: ...

class GetTokenAccountBalance:
    def __init__(
        account: Pubkey,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def account(self) -> Pubkey: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetTokenAccountsByDelegate:
    def __init__(
        account: Pubkey,
        filter_: RpcTokenAccountsFilterWrapper,
        config: Optional[RpcAccountInfoConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def account(self) -> Pubkey: ...
    @property
    def filter_(self) -> RpcTokenAccountsFilterWrapper: ...
    @property
    def config(self) -> Optional[RpcAccountInfoConfig]: ...

class GetTokenAccountsByOwner:
    def __init__(
        account: Pubkey,
        filter_: RpcTokenAccountsFilterWrapper,
        config: Optional[RpcAccountInfoConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def account(self) -> Pubkey: ...
    @property
    def filter_(self) -> RpcTokenAccountsFilterWrapper: ...
    @property
    def config(self) -> Optional[RpcAccountInfoConfig]: ...

class GetTokenLargestAccounts:
    def __init__(
        mint: Pubkey,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def mint(self) -> Pubkey: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetTokenSupply:
    def __init__(
        mint: Pubkey,
        commitment: Optional[CommitmentLevel] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def mint(self) -> Pubkey: ...
    @property
    def commitment(self) -> Optional[CommitmentLevel]: ...

class GetTransaction:
    def __init__(
        signature: Signature,
        config: Optional[RpcTransactionConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def signature(self) -> Signature: ...
    @property
    def config(self) -> Optional[RpcTransactionConfig]: ...

class GetTransactionCount:
    def __init__(
        config: Optional[RpcContextConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class GetVoteAccounts:
    def __init__(
        config: Optional[RpcGetVoteAccountsConfig] = None, id: Optional[int] = None
    ): ...
    @property
    def config(self) -> Optional[RpcGetVoteAccountsConfig]: ...

class IsBlockhashValid:
    def __init__(
        blockhash: SolderHash,
        config: Optional[RpcContextConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def blockhash(self) -> SolderHash: ...
    @property
    def config(self) -> Optional[RpcContextConfig]: ...

class RequestAirdrop:
    def __init__(
        pubkey: Pubkey,
        lamports: int,
        config: Optional[RpcRequestAirdropConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def pubkey(self) -> Pubkey: ...
    @property
    def lamports(self) -> int: ...
    @property
    def config(self) -> Optional[RpcRequestAirdropConfig]: ...

class SendTransaction:
    def __init__(
        tx: Transaction,
        config: Optional[RpcSendTransactionConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def tx(self) -> Transaction: ...
    @property
    def config(self) -> Optional[RpcSendTransactionConfig]: ...

class SimulateTransaction:
    def __init__(
        tx: Transaction,
        config: Optional[RpcSimulateTransactionConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def tx(self) -> Transaction: ...
    @property
    def config(self) -> Optional[RpcSimulateTransactionConfig]: ...

class AccountSubscribe:
    def __init__(
        account: Pubkey,
        config: Optional[RpcAccountInfoConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def account(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcAccountInfoConfig]: ...

class BlockSubscribe:
    def __init__(
        filter_: RpcBlockSubscribeFilterWrapper,
        config: Optional[RpcBlockSubscribeConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def filter_(self) -> RpcBlockSubscribeFilterWrapper: ...
    @property
    def config(self) -> Optional[RpcBlockSubscribeConfig]: ...

class LogsSubscribe:
    def __init__(
        filter_: TransactionLogsFilterWrapper,
        config: Optional[RpcTransactionLogsConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def filter_(self) -> TransactionLogsFilterWrapper: ...
    @property
    def config(self) -> Optional[RpcTransactionLogsConfig]: ...

class ProgramSubscribe:
    def __init__(
        program: Pubkey,
        config: Optional[RpcProgramAccountsConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def program(self) -> Pubkey: ...
    @property
    def config(self) -> Optional[RpcProgramAccountsConfig]: ...

class SignatureSubscribe:
    def __init__(
        signature: Signature,
        config: Optional[RpcSignatureSubscribeConfig] = None,
        id: Optional[int] = None,
    ): ...
    @property
    def signature(self) -> Signature: ...
    @property
    def config(self) -> Optional[RpcSignatureSubscribeConfig]: ...

def batch_to_json(reqs: Sequence[__Body]) -> str: ...
def batch_from_json(raw: str) -> List[__Body]: ...
