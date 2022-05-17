use crate::Pubkey;
use pyo3::prelude::*;
use solana_sdk::pubkey;

pub fn create_sysvar_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let sysvar_mod = PyModule::new(py, "sysvar")?;
    let sysvars = vec![
        (
            "CLOCK",
            pubkey!("SysvarC1ock11111111111111111111111111111111"),
        ),
        (
            "RECENT_BLOCKHASHES",
            pubkey!("SysvarRecentB1ockHashes11111111111111111111"),
        ),
        (
            "RENT",
            pubkey!("SysvarRent111111111111111111111111111111111"),
        ),
        (
            "REWARDS",
            pubkey!("SysvarRewards111111111111111111111111111111"),
        ),
        (
            "STAKE_HISTORY",
            pubkey!("SysvarStakeHistory1111111111111111111111111"),
        ),
        (
            "EPOCH_SCHEDULE",
            pubkey!("SysvarEpochSchedu1e111111111111111111111111"),
        ),
        (
            "INSTRUCTIONS",
            pubkey!("Sysvar1nstructions1111111111111111111111111"),
        ),
        (
            "SLOT_HASHES",
            pubkey!("SysvarS1otHashes111111111111111111111111111"),
        ),
    ];
    for sysvar in sysvars {
        sysvar_mod.add(sysvar.0, Pubkey(sysvar.1))?
    }
    Ok(sysvar_mod)
}
