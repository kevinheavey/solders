from typing import ClassVar, Tuple, Sequence, List
from solders.pubkey import Pubkey
from solders.instruction import Instruction

class SystemProgram:
    ID: ClassVar[Pubkey]
    @staticmethod
    def create_account(
        from_pubkey: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def create_account_with_seed(
        from_pubkey: Pubkey,
        to_pubkey: Pubkey,
        base: Pubkey,
        seed: str,
        lamports: int,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def assign(pubkey: Pubkey, owner: Pubkey) -> Instruction: ...
    @staticmethod
    def assign_with_seed(
        address: Pubkey,
        base: Pubkey,
        seed: str,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def transfer(
        from_pubkey: Pubkey, to_pubkey: Pubkey, lamports: int
    ) -> Instruction: ...
    @staticmethod
    def transfer_with_seed(
        from_pubkey: Pubkey,
        from_base: Pubkey,
        from_seed: str,
        from_owner: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
    ) -> Instruction: ...
    @staticmethod
    def allocate(pubkey: Pubkey, space: int) -> Instruction: ...
    @staticmethod
    def allocate_with_seed(
        address: Pubkey,
        base: Pubkey,
        seed: str,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def transfer_many(
        from_pubkey: Pubkey,
        to_lamports: Sequence[Tuple[Pubkey, int]],
    ) -> List[Instruction]: ...
    @staticmethod
    def create_nonce_account_with_seed(
        from_pubkey: Pubkey,
        nonce_pubkey: Pubkey,
        base: Pubkey,
        seed: str,
        authority: Pubkey,
        lamports: int,
    ) -> Tuple[Instruction, Instruction]: ...
    @staticmethod
    def create_nonce_account(
        from_pubkey: Pubkey,
        nonce_pubkey: Pubkey,
        authority: Pubkey,
        lamports: int,
    ) -> Tuple[Instruction, Instruction]: ...
    @staticmethod
    def advance_nonce_account(
        nonce_pubkey: Pubkey, authorized_pubkey: Pubkey
    ) -> Instruction: ...
    @staticmethod
    def withdraw_nonce_account(
        nonce_pubkey: Pubkey,
        authorized_pubkey: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
    ) -> Instruction: ...