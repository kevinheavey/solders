from typing import Sequence, List, Optional
from solders.hash import Hash
from solders.account import Account, AccountJSON
from solders.transaction_status import EncodedTransactionWithStatusMeta
from solders.signature import Signature

class RpcResponseContext:
    slot: int
    api_version: Optional[str]
    def __init__(self, slot: int, api_version: Optional[str] = None) -> None: ...

class GetAccountInfoResp:
    context: RpcResponseContext
    value: Optional[Account]
    def __init__(
        self, value: Optional[Account], context: RpcResponseContext
    ) -> None: ...

class GetAccountInfoJsonParsedResp:
    context: RpcResponseContext
    value: Optional[AccountJSON]
    def __init__(
        self, value: Optional[AccountJSON], context: RpcResponseContext
    ) -> None: ...

class GetBalanceResp:
    context: RpcResponseContext
    value: int
    def __init__(self, value: int, context: RpcResponseContext) -> None: ...

class GetBlockCommitmentResp:
    commitment: Optional[List[int]]
    total_stake: int
    def __init__(
        self, commitment: Optional[Sequence[int]], total_stake: int
    ) -> None: ...

class GetBlockResp:

    previous_blockhash: Hash

    blockhash: Hash

    parent_slot: int

    transactions: Optional[List[EncodedTransactionWithStatusMeta]]

    signatures: Optional[List[Signature]]

    rewards: Optional[Rewards]

    block_time: Optional[int]

    block_height: Optional[int]

    def __init__(
        self,
        previous_blockhash: Hash,
        blockhash: Hash,
        parent_slot: int,
        transactions: Optional[Sequence[EncodedTransactionWithStatusMeta]] = None,
        signatures: Optional[Sequence[Signature]] = None,
        rewards: Optional[Rewards] = None,
        block_time: Optional[int] = None,
        block_height: Optional[int] = None,
    ) -> None: ...
