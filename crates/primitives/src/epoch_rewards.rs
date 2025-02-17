use {
    pyo3::prelude::*,
    serde::{Deserialize, Serialize},
    solana_epoch_rewards::EpochRewards as EpochRewardsOriginal,
    solders_hash::Hash as Blockhash,
    solders_traits_core::transaction_status_boilerplate,
};

/// A type to hold data for the EpochRewards sysvar.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[pyclass(module = "solders.epoch_rewards", subclass)]
pub struct EpochRewards(pub EpochRewardsOriginal);

transaction_status_boilerplate!(EpochRewards);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl EpochRewards {
    #[new]
    pub fn new(
        distribution_starting_block_height: u64,
        num_partitions: u64,
        parent_blockhash: Blockhash,
        total_points: u128,
        total_rewards: u64,
        distributed_rewards: u64,
        active: bool,
    ) -> Self {
        Self(EpochRewardsOriginal {
            distribution_starting_block_height,
            num_partitions,
            parent_blockhash: parent_blockhash.0,
            total_points,
            total_rewards,
            distributed_rewards,
            active,
        })
    }

    /// int: The starting block height of the rewards distribution in the current
    /// epoch
    #[getter]
    pub fn distribution_starting_block_height(&self) -> u64 {
        self.0.distribution_starting_block_height
    }

    #[setter]
    pub fn set_distribution_starting_block_height(&mut self, val: u64) {
        self.0.distribution_starting_block_height = val;
    }

    /// int: Number of partitions in the rewards distribution in the current epoch,
    /// used to generate an EpochRewardsHasher
    #[getter]
    pub fn num_partitions(&self) -> u64 {
        self.0.num_partitions
    }

    #[setter]
    pub fn set_num_partitions(&mut self, val: u64) {
        self.0.num_partitions = val;
    }

    /// Blockhash: The blockhash of the parent block of the first block in the epoch, used
    /// to seed an EpochRewardsHasher
    #[getter]
    pub fn parent_blockhash(&self) -> Blockhash {
        Blockhash(self.0.parent_blockhash)
    }

    #[setter]
    pub fn set_parent_blockhash(&mut self, val: Blockhash) {
        self.0.parent_blockhash = val.0;
    }

    /// int: The total rewards points calculated for the current epoch, where points
    /// equals the sum of (delegated stake * credits observed) for all
    /// delegations
    #[getter]
    pub fn total_points(&self) -> u128 {
        self.0.total_points
    }

    #[setter]
    pub fn set_total_points(&mut self, val: u128) {
        self.0.total_points = val
    }

    /// int: The total rewards calculated for the current epoch. This may be greater
    /// than the total `distributed_rewards` at the end of the rewards period,
    /// due to rounding and inability to deliver rewards smaller than 1 lamport.
    #[getter]
    pub fn total_rewards(&self) -> u64 {
        self.0.total_rewards
    }

    #[setter]
    pub fn set_total_rewards(&mut self, val: u64) {
        self.0.total_rewards = val;
    }

    /// int: The rewards currently distributed for the current epoch, in lamports
    #[getter]
    pub fn distributed_rewards(&self) -> u64 {
        self.0.distributed_rewards
    }

    #[setter]
    pub fn set_distributed_rewards(&mut self, val: u64) {
        self.0.distributed_rewards = val;
    }

    /// bool: Whether the rewards period (including calculation and distribution) is
    /// active
    #[getter]
    pub fn active(&self) -> bool {
        self.0.active
    }

    #[setter]
    pub fn set_active(&mut self, val: bool) {
        self.0.active = val;
    }
}

pub fn include_epoch_rewards(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EpochRewards>()?;
    Ok(())
}
