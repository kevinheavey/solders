use crate::Pubkey;
use pyo3::prelude::*;
use solana_sdk::sysvar as sysvar_original;

pub fn create_sysvar_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let sysvar_mod = PyModule::new(py, "_sysvar")?;
    let sysvars = vec![
        ("CLOCK", sysvar_original::clock::ID),
        (
            "RECENT_BLOCKHASHES",
            sysvar_original::recent_blockhashes::ID,
        ),
        ("RENT", sysvar_original::rent::ID),
        ("REWARDS", sysvar_original::rewards::ID),
        ("STAKE_HISTORY", sysvar_original::stake_history::ID),
        ("EPOCH_SCHEDULE", sysvar_original::epoch_schedule::ID),
        ("INSTRUCTIONS", sysvar_original::instructions::ID),
        ("SLOT_HASHES", sysvar_original::slot_hashes::ID),
    ];
    for sysvar in sysvars {
        sysvar_mod.add(sysvar.0, Pubkey(sysvar.1))?
    }
    Ok(sysvar_mod)
}
