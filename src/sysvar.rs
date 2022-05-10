use crate::Pubkey;
use pyo3::prelude::*;
use solana_sdk::pubkey;

#[derive(Clone, Debug)]
#[pyclass(module = "solders", subclass)]
pub struct Sysvar;

#[pymethods]
impl Sysvar {
    #[classattr]
    const CLOCK: Pubkey = Pubkey(pubkey!("SysvarC1ock11111111111111111111111111111111"));
    #[classattr]
    const RECENT_BLOCKHASHES: Pubkey =
        Pubkey(pubkey!("SysvarRecentB1ockHashes11111111111111111111"));
    #[classattr]
    const RENT: Pubkey = Pubkey(pubkey!("SysvarRent111111111111111111111111111111111"));
    #[classattr]
    const REWARDS: Pubkey = Pubkey(pubkey!("SysvarRewards111111111111111111111111111111"));
    #[classattr]
    const STAKE_HISTORY: Pubkey = Pubkey(pubkey!("SysvarStakeHistory1111111111111111111111111"));
    #[classattr]
    const EPOCH_SCHEDULE: Pubkey = Pubkey(pubkey!("SysvarEpochSchedu1e111111111111111111111111"));
    #[classattr]
    const INSTRUCTIONS: Pubkey = Pubkey(pubkey!("Sysvar1nstructions1111111111111111111111111"));
    #[classattr]
    const SLOT_HASHES: Pubkey = Pubkey(pubkey!("SysvarS1otHashes111111111111111111111111111"));
}
