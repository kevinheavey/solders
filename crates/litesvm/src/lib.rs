use {
    litesvm::LiteSVM as LiteSVMOriginal,
    pyo3::{exceptions::PyFileNotFoundError, prelude::*},
    solders_account::Account,
    solders_compute_budget::ComputeBudget,
    solders_hash::Hash as Blockhash,
    solders_primitives::{
        clock::Clock, epoch_rewards::EpochRewards, epoch_schedule::EpochSchedule, rent::Rent,
        slot_history::SlotHistory, stake_history::StakeHistory,
    },
    solders_pubkey::Pubkey,
    solders_signature::Signature,
    solders_traits::to_py_err,
    solders_traits_core::RichcmpEqualityOnly,
    solders_transaction::TransactionType,
    std::{
        collections::{HashMap, HashSet},
        path::PathBuf,
    },
    transaction_metadata::{SimulateResult, TransactionResult},
    {
        solana_account::Account as AccountOriginal, solana_clock::Clock as ClockOriginal,
        solana_epoch_rewards::EpochRewards as EpochRewardsOriginal,
        solana_epoch_schedule::EpochSchedule as EpochScheduleOriginal,
        solana_feature_set::FeatureSet as FeatureSetOriginal,
        solana_last_restart_slot::LastRestartSlot, solana_rent::Rent as RentOriginal,
        solana_slot_hashes::SlotHashes, solana_slot_history::SlotHistory as SlotHistoryOriginal,
        solana_stake_interface::stake_history::StakeHistory as StakeHistoryOriginal,
    },
};
pub mod transaction_metadata;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(module = "solders.litesvm", subclass)]
pub struct FeatureSet(pub(crate) FeatureSetOriginal);

impl RichcmpEqualityOnly for FeatureSet {}

#[solders_macros::richcmp_eq_only]
#[pymethods]
impl FeatureSet {
    /// Create a new FeatureSet.
    ///
    /// Args:
    ///     active (Dict[Pubkey, int]): a mapping of feature IDs to the slots at which they were activated.
    ///     inactive (Set[Pubkey]): a set of inactive feature IDs.
    ///
    /// Returns:
    ///     FeatureSet: The FeatureSet object.
    #[new]
    pub fn new(active: HashMap<Pubkey, u64>, inactive: HashSet<Pubkey>) -> Self {
        let active_inner = active.into_iter().map(|x| (x.0 .0, x.1)).collect();
        let inactive_inner = inactive.into_iter().map(|x| x.0).collect();
        Self(FeatureSetOriginal {
            active: active_inner,
            inactive: inactive_inner,
        })
    }

    /// Create a new FeatureSet with no featues enabled.
    ///
    /// Returns:
    ///     FeatureSet: The FeatureSet object.
    #[pyo3(name = "default")]
    #[staticmethod]
    pub fn new_default() -> Self {
        Self(FeatureSetOriginal::default())
    }

    /// Create a new FeatureSet with all featues enabled.
    ///
    /// Returns:
    ///     FeatureSet: The FeatureSet object.
    #[staticmethod]
    pub fn all_enabled() -> Self {
        Self(FeatureSetOriginal::all_enabled())
    }

    /// Check if a given feature is active.
    ///
    /// Args:
    ///     feature_id (Pubkey): The feature ID.
    ///
    /// Returns
    ///     bool: True if the feature is active.
    ///
    pub fn is_active(&self, feature_id: Pubkey) -> bool {
        self.0.is_active(&feature_id.0)
    }

    /// Find the slot at which a feature was activated.
    ///
    /// Args:
    ///     feature_id (Pubkey): The feature ID.
    ///
    /// Returns
    ///     Optional[int]: The activated slot, if it exists.
    ///
    pub fn activated_slot(&self, feature_id: Pubkey) -> Option<u64> {
        self.0.activated_slot(&feature_id.0)
    }

    /// Dict[Pubkey, int]: Mapping of feature IDs to the slots at which they were activated.
    #[getter]
    pub fn active(&self) -> HashMap<Pubkey, u64> {
        self.0
            .active
            .clone()
            .into_iter()
            .map(|x| (Pubkey::from(x.0), x.1))
            .collect()
    }

    #[setter]
    pub fn set_active(&mut self, val: HashMap<Pubkey, u64>) {
        let inner = val.into_iter().map(|x| (x.0 .0, x.1)).collect();
        self.0.active = inner;
    }

    /// Set[Pubkey]: The inactive feature IDs.
    #[getter]
    pub fn inactive(&self) -> HashSet<Pubkey> {
        self.0.inactive.clone().into_iter().map(Pubkey).collect()
    }

    #[setter]
    pub fn set_inactive(&mut self, val: HashSet<Pubkey>) {
        let inner = val.into_iter().map(|x| x.0).collect();
        self.0.inactive = inner;
    }
}

#[pyclass(module = "solders.litesvm", subclass)]
pub struct LiteSVM(LiteSVMOriginal);

#[pymethods]
impl LiteSVM {
    #[allow(clippy::new_without_default)]
    #[new]
    pub fn new() -> Self {
        Self(LiteSVMOriginal::new())
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self(LiteSVMOriginal::default())
    }

    pub fn set_compute_budget(&mut self, budget: &ComputeBudget) {
        self.0.set_compute_budget(budget.0);
    }

    /// Enables or disables sigverify
    pub fn set_sigverify(&mut self, sigverify: bool) {
        self.0.set_sigverify(sigverify);
    }

    pub fn set_blockhash_check(&mut self, check: bool) {
        self.0.set_blockhash_check(check);
    }

    pub fn set_sysvars(&mut self) {
        self.0.set_sysvars()
    }

    #[pyo3(signature = (feature_set=None))]
    pub fn set_builtins(&mut self, feature_set: Option<&FeatureSet>) {
        self.0.set_builtins(feature_set.map(|x| x.0.clone()));
    }

    pub fn set_lamports(&mut self, lamports: u64) {
        self.0.set_lamports(lamports)
    }

    /// Includes the standard SPL programs
    pub fn set_spl_programs(&mut self) {
        self.0.set_spl_programs();
    }

    pub fn set_transaction_history(&mut self, capacity: usize) {
        self.0.set_transaction_history(capacity)
    }

    #[pyo3(signature = (limit=None))]
    pub fn set_log_bytes_limit(&mut self, limit: Option<usize>) {
        self.0.set_log_bytes_limit(limit);
    }

    #[pyo3(signature = (feature_set=None))]
    pub fn set_precompiles(&mut self, feature_set: Option<&FeatureSet>) {
        self.0.set_precompiles(feature_set.map(|x| x.0.clone()));
    }

    pub fn minimum_balance_for_rent_exemption(&self, data_len: usize) -> u64 {
        self.0.minimum_balance_for_rent_exemption(data_len)
    }

    pub fn get_account(&self, pubkey: Pubkey) -> Option<Account> {
        self.0.get_account(&pubkey.0).map(Account::from)
    }

    pub fn set_account(&mut self, pubkey: Pubkey, data: &Account) -> PyResult<()> {
        self.0
            .set_account(pubkey.0, AccountOriginal::from(data.clone()))
            .map_err(to_py_err)
    }

    pub fn get_balance(&self, pubkey: Pubkey) -> Option<u64> {
        self.0.get_balance(&pubkey.0)
    }

    pub fn latest_blockhash(&self) -> Blockhash {
        Blockhash(self.0.latest_blockhash())
    }

    pub fn get_transaction(&self, signature: Signature) -> Option<TransactionResult> {
        self.0
            .get_transaction(&signature.0)
            .map(|x| x.clone().into())
    }

    pub fn airdrop(&mut self, pubkey: Pubkey, lamports: u64) -> TransactionResult {
        self.0.airdrop(&pubkey.0, lamports).into()
    }

    pub fn add_program_from_file(&mut self, program_id: Pubkey, path: PathBuf) -> PyResult<()> {
        let res = self
            .0
            .add_program_from_file(program_id.0, path.to_str().unwrap());
        res.map_err(|e| PyFileNotFoundError::new_err(e.to_string()))
    }

    /// Adds am SBF program to the test environment.
    pub fn add_program(&mut self, program_id: Pubkey, program_bytes: &[u8]) {
        self.0.add_program(program_id.0, program_bytes)
    }

    pub fn send_transaction(&mut self, tx: TransactionType) -> TransactionResult {
        let res = self.0.send_transaction(tx);
        TransactionResult::from(res)
    }

    pub fn simulate_transaction(&mut self, tx: TransactionType) -> SimulateResult {
        let res = self.0.simulate_transaction(tx);
        SimulateResult::from(res)
    }

    pub fn expire_blockhash(&mut self) {
        self.0.expire_blockhash()
    }

    pub fn warp_to_slot(&mut self, slot: u64) {
        self.0.warp_to_slot(slot)
    }

    pub fn get_compute_budget(&self) -> Option<ComputeBudget> {
        self.0.get_compute_budget().map(ComputeBudget)
    }

    pub fn get_clock(&self) -> Clock {
        Clock(self.0.get_sysvar::<ClockOriginal>())
    }

    pub fn set_clock(&mut self, clock: &Clock) {
        self.0.set_sysvar(&clock.0)
    }

    pub fn get_rent(&self) -> Rent {
        Rent(self.0.get_sysvar::<RentOriginal>())
    }

    pub fn set_rent(&mut self, rent: &Rent) {
        self.0.set_sysvar(&rent.0)
    }

    pub fn get_epoch_rewards(&self) -> EpochRewards {
        EpochRewards(self.0.get_sysvar::<EpochRewardsOriginal>())
    }

    pub fn set_epoch_rewards(&mut self, rewards: &EpochRewards) {
        self.0.set_sysvar(&rewards.0)
    }

    pub fn get_epoch_schedule(&self) -> EpochSchedule {
        EpochSchedule(self.0.get_sysvar::<EpochScheduleOriginal>())
    }

    pub fn set_epoch_schedule(&mut self, schedule: &EpochSchedule) {
        self.0.set_sysvar(&schedule.0)
    }

    pub fn get_last_restart_slot(&self) -> u64 {
        self.0.get_sysvar::<LastRestartSlot>().last_restart_slot
    }

    pub fn set_last_restart_slot(&mut self, slot: u64) {
        self.0.set_sysvar::<LastRestartSlot>(&LastRestartSlot {
            last_restart_slot: slot,
        })
    }

    pub fn get_slot_hashes(&self) -> Vec<(u64, Blockhash)> {
        let fetched = self.0.get_sysvar::<SlotHashes>();
        fetched
            .slot_hashes()
            .iter()
            .map(|x| (x.0, Blockhash::from(x.1)))
            .collect()
    }

    pub fn set_slot_hashes(&mut self, hashes: Vec<(u64, Blockhash)>) {
        let mut intermediate: Vec<(u64, solana_hash::Hash)> = Vec::with_capacity(hashes.len());
        for h in hashes {
            let converted_hash = h.1 .0;
            intermediate.push((h.0, converted_hash));
        }
        let converted = SlotHashes::from_iter(intermediate);
        self.0.set_sysvar::<SlotHashes>(&converted);
    }

    pub fn get_slot_history(&self) -> SlotHistory {
        SlotHistory(self.0.get_sysvar::<SlotHistoryOriginal>())
    }

    pub fn set_slot_history(&mut self, history: &SlotHistory) {
        self.0.set_sysvar::<SlotHistoryOriginal>(&history.0)
    }

    pub fn get_stake_history(&self) -> StakeHistory {
        StakeHistory(self.0.get_sysvar::<StakeHistoryOriginal>())
    }

    pub fn set_stake_history(&mut self, history: &StakeHistory) {
        self.0.set_sysvar::<StakeHistoryOriginal>(&history.0)
    }
}

pub fn include_litesvm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FeatureSet>()?;
    m.add_class::<LiteSVM>()?;
    Ok(())
}
