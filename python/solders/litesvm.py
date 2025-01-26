"""The Solana LiteSVM library."""
from pathlib import Path
from typing import List, Optional, Sequence, Tuple, Union

from .solders import (
    Account,
    Clock,
    ComputeBudget,
    EpochRewards,
    EpochSchedule,
    FeatureSet,
    Hash,
    Pubkey,
    Rent,
    Signature,
    SlotHistory,
    StakeHistory,
    Transaction,
    VersionedTransaction,
)
from .solders import LiteSVM as _LiteSVM
from .transaction_metadata import SimulateResult, TransactionResult


class LiteSVM:
    """The main class in the litesvm library.

    Use this to send transactions, query accounts and configure the runtime.
    """

    def __init__(self) -> None:
        """Create a new LiteSVM instance with standard functionality enabled."""
        inner = _LiteSVM()
        self._inner = inner

    @staticmethod
    def default() -> "LiteSVM":
        """Create a new LiteSVM instance with minimal functionality enabled."""
        svm = LiteSVM()
        inner = _LiteSVM.default()
        svm._inner = inner
        return svm

    def with_compute_budget(self, budget: ComputeBudget) -> "LiteSVM":
        """Set the compute budget.

        Args:
            budget: The new compute budget

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_compute_budget(budget)
        return self

    def with_sigverify(self, sigverify: bool) -> "LiteSVM":
        """Enable or disable sigverify.

        Args:
            sigverify: if false, transaction signatures will not be checked.

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_sigverify(sigverify)
        return self

    def with_blockhash_check(self, check: bool) -> "LiteSVM":
        """Enables or disables transaction blockhash checking.

        Args:
            check: If false, the blockhash check will be skipped

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_blockhash_check(check)
        return self

    def with_sysvars(self) -> "LiteSVM":
        """Sets up the standard sysvars.

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_sysvars()
        return self

    def with_builtins(self, feature_set: Optional[FeatureSet] = None) -> "LiteSVM":
        """Adds the standard builtin programs.

        Args:
            feature_set: if provided, decides what builtins to add based on what
                features are active

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_builtins(feature_set)
        return self

    def with_lamports(self, lamports: int) -> "LiteSVM":
        """Changes the initial lamports in LiteSVM's airdrop account.

        Args:
            lamports: The number of lamports to set in the airdrop account

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_lamports(lamports)
        return self

    def with_spl_programs(self) -> "LiteSVM":
        """Adds the standard SPL programs.

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_spl_programs()
        return self

    def with_transaction_history(self, capacity: int) -> "LiteSVM":
        """Changes the capacity of the transaction history.

        Args:
            capacity: How many transactions to store in history.
                Set this to 0 to disable transaction history
                and allow duplicate transactions.

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_transaction_history(capacity)
        return self

    def with_log_bytes_limit(self, limit: Optional[int] = None) -> "LiteSVM":
        """Set a limit for transaction logs, beyond which they will be truncated.

        Args:
            limit: The limit in bytes. If None, no limit is enforced.

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_log_bytes_limit(limit)
        return self

    def with_precompiles(self, feature_set: Optional[FeatureSet] = None) -> "LiteSVM":
        """Adds the standard precompiles.

        Args:
            feature_set: if provided, decides what precompiles to add based on what
                features are active

        Returns:
            The modified LiteSVM instance
        """
        self._inner.set_precompiles(feature_set)
        return self

    def minimum_balance_for_rent_exemption(self, data_len: int) -> int:
        """Calculates the minimum balance required to make an account rent exempt.

        Args:
            data_len: The number of bytes in the account.

        Returns:
            The required balance in lamports
        """
        return self._inner.minimum_balance_for_rent_exemption(data_len)

    def get_account(self, address: Pubkey) -> Optional[Account]:
        """Return the account at the given address.

        If the account is not found, None is returned.

        Args:
            address: The account address to look up.

        Returns:
            The account object, if the account exists.
        """
        return self._inner.get_account(address)

    def set_account(self, address: Pubkey, account: Account) -> None:
        """Create or overwrite an account, subverting normal runtime checks.

        This method exists to make it easier to set up artificial situations
        that would be difficult to replicate by sending individual transactions.
        Beware that it can be used to create states that would not be reachable
        by sending transactions!

        Args:
            address: The address to write to.
            account: The account object to write.
        """
        self._inner.set_account(address, account)

    def get_balance(self, address: Pubkey) -> Optional[int]:
        """Gets the balance of the provided account address.

        Args:
            address: The account address.

        Returns:
            The account's balance in lamports.
        """
        return self._inner.get_balance(address)

    def latest_blockhash(self) -> Hash:
        """Gets the latest blockhash.

        Since LiteSVM doesn't have blocks, this is an arbitrary
        value controlled by LiteSVM.

        Returns:
            The designated latest blockhash.
        """
        return self._inner.latest_blockhash()

    def get_transaction(self, signature: Signature) -> Optional[TransactionResult]:
        """Gets a transaction from the transaction history.

        Args:
            signature: The transaction signature

        Returns:
            The transaction, if it is found in the history.
        """
        return self._inner.get_transaction(signature)

    def airdrop(self, address: Pubkey, lamports: int) -> TransactionResult:
        """Airdrops the lamport amount specified to the given address.

        Args:
            address: The airdrop recipient.
            lamports: The amount to airdrop.

        Returns:
            The transaction result.
        """
        return self._inner.airdrop(address, lamports)

    def add_program_from_file(self, program_id: Pubkey, path: Path) -> None:
        """Adds an SBF program to the test environment from the file specified.

        Args:
            program_id: The program ID.
            path: The path to the .so file.
        """
        return self._inner.add_program_from_file(program_id, path)

    def add_program(self, program_id: Pubkey, program_bytes: bytes) -> None:
        """Adds an SBF program to the test environment.

        Args:
            program_id: The program ID.
            program_bytes: The raw bytes of the compiled program.
        """
        return self._inner.add_program(program_id, program_bytes)

    def send_transaction(
        self, tx: Union[Transaction, VersionedTransaction]
    ) -> TransactionResult:
        """Processes a transaction and returns the result.

        Args:
            tx: The transaction to send.

        Returns:
            TransactionMetadata if transaction succeeds, else FailedTransactionMetadata
        """
        return self._inner.send_transaction(tx)

    def simulate_transaction(
        self, tx: Union[Transaction, VersionedTransaction]
    ) -> SimulateResult:
        """Simulates a transaction.

        Args:
            tx: The transaction to simulate

        Returns:
            SimulatedTransactionInfo if sim succeeds, else FailedTransactionMetadata
        """
        return self._inner.simulate_transaction(tx)

    def expire_blockhash(self) -> None:
        """Expires the current blockhash.

        The return value of `latest_blockhash()` will be different after calling this.
        """
        self._inner.expire_blockhash()

    def warp_to_slot(self, slot: int) -> None:
        """Warps the clock to the specified slot.

        This is a convenience wrapper around `set_clock()`.

        Args:
            slot: The new slot.
        """
        self._inner.warp_to_slot(slot)

    def get_clock(self) -> Clock:
        """Get the cluster clock.

        Returns:
            the clock object.
        """
        return self._inner.get_clock()

    def set_clock(self, clock: Clock) -> None:
        """Overwrite the clock sysvar.

        Args:
            clock: The clock object.
        """
        self._inner.set_clock(clock)

    def get_epoch_rewards(self) -> EpochRewards:
        """Get the EpochRewards sysvar.

        Returns:
            the EpochRewards object.
        """
        return self._inner.get_epoch_rewards()

    def set_epoch_rewards(self, rewards: EpochRewards) -> None:
        """Overwrite the EpochRewards sysvar.

        Args:
            rewards: The EpochRewards object.
        """
        self._inner.set_epoch_rewards(rewards)

    def get_epoch_schedule(self) -> EpochSchedule:
        """Get the EpochSchedule sysvar.

        Returns:
            the EpochSchedule object.
        """
        return self._inner.get_epoch_schedule()

    def set_epoch_schedule(self, schedule: EpochSchedule) -> None:
        """Overwrite the EpochSchedule sysvar.

        Args:
            schedule: The EpochSchedule object.
        """
        self._inner.set_epoch_schedule(schedule)

    def get_last_restart_slot(self) -> int:
        """Get the last restart slot sysvar.

        Returns:
            the last restart slot.
        """
        return self._inner.get_last_restart_slot()

    def set_last_restart_slot(self, slot: int) -> None:
        """Overwrite the last restart slot sysvar.

        Args:
            slot: The last restart slot.
        """
        self._inner.set_last_restart_slot(slot)

    def get_rent(self) -> Rent:
        """Get the cluster rent.

        Returns:
            The rent object.
        """
        return self._inner.get_rent()

    def set_rent(self, rent: Rent) -> None:
        """Overwrite the rent sysvar.

        Args:
            rent: The new rent object.
        """
        self._inner.set_rent(rent)

    def get_slot_hashes(self) -> List[Tuple[int, Hash]]:
        """Get the SlotHashes sysvar.

        Returns:
            The SlotHash array.
        """
        return self._inner.get_slot_hashes()

    def set_slot_hashes(self, hashes: Sequence[Tuple[int, Hash]]) -> None:
        """Overwrite the SlotHashes sysvar.

        Args:
            hashes: The SlotHash array.
        """
        self._inner.set_slot_hashes(hashes)

    def get_slot_history(self) -> SlotHistory:
        """Get the SlotHistory sysvar.

        Returns:
            The SlotHistory object.
        """
        return self._inner.get_slot_history()

    def set_slot_history(self, history: SlotHistory) -> None:
        """Overwrite the SlotHistory sysvar.

        Args:
            history: The SlotHistory object
        """
        self._inner.set_slot_history(history)

    def get_stake_history(self) -> StakeHistory:
        """Get the StakeHistory sysvar.

        Returns:
            The StakeHistory object.
        """
        return self._inner.get_stake_history()

    def set_stake_history(self, history: StakeHistory) -> None:
        """Overwrite the StakeHistory sysvar.

        Args:
            history: The StakeHistory object
        """
        self._inner.set_stake_history(history)


__all__ = ["FeatureSet", "LiteSVM"]
