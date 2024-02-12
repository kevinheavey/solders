//! These docstrings are written for Python users.
//!
//! If you're viewing them on docs.rs, the formatting won't make much sense.
use address_lookup_table_account::create_address_lookup_table_account_mod;
use pyo3::prelude::*;
#[cfg(feature = "ring")]
use rpc::create_rpc_mod;
#[cfg(feature = "ring")]
use solders_account::create_account_mod;
use solders_instruction::{AccountMeta, CompiledInstruction, Instruction};
use solders_system_program::create_system_program_mod;
use solders_token::create_token_mod;
use solders_traits::{BincodeError, CborError, ParseHashError, SerdeJSONError, SignerError};
#[cfg(feature = "ring")]
use solders_transaction_status::create_transaction_status_mod;
use std::collections::HashMap;
use sysvar::create_sysvar_mod;
pub mod message;
use message::create_message_mod;
pub mod transaction;
#[cfg(feature = "ring")]
use solders_account_decoder::create_account_decoder_mod;
use transaction::create_transaction_mod;
pub mod address_lookup_table_account;
#[cfg(feature = "ring")]
pub mod rpc;
pub mod sysvar;
use solders_commitment_config::{CommitmentConfig, CommitmentLevel};
use solders_compute_budget::create_compute_budget_mod;
use solders_epoch_info::create_epoch_info_mod;
use solders_hash::Hash as SolderHash;
use solders_keypair::{null_signer::NullSigner, presigner::Presigner, Keypair};
use solders_primitives::{
    clock::create_clock_mod, epoch_schedule::create_epoch_schedule_mod, rent::create_rent_mod,
};
use solders_pubkey::Pubkey;
use solders_signature::Signature;

#[pymodule]
fn solders(py: Python, m: &PyModule) -> PyResult<()> {
    let hash_mod = PyModule::new(py, "hash")?;
    hash_mod.add_class::<SolderHash>()?;
    hash_mod.add("ParseHashError", py.get_type::<ParseHashError>())?;
    let instruction_mod = PyModule::new(py, "instruction")?;
    instruction_mod.add_class::<AccountMeta>()?;
    instruction_mod.add_class::<Instruction>()?;
    instruction_mod.add_class::<CompiledInstruction>()?;
    let pubkey_mod = PyModule::new(py, "pubkey")?;
    pubkey_mod.add_class::<Pubkey>()?;
    let keypair_mod = PyModule::new(py, "keypair")?;
    keypair_mod.add_class::<Keypair>()?;
    let signature_mod = PyModule::new(py, "signature")?;
    signature_mod.add_class::<Signature>()?;
    let message_mod = create_message_mod(py)?;
    let null_signer_mod = PyModule::new(py, "null_signer")?;
    null_signer_mod.add_class::<NullSigner>()?;
    let transaction_mod = create_transaction_mod(py)?;
    let system_program_mod = create_system_program_mod(py)?;
    let sysvar_mod = create_sysvar_mod(py)?;
    let presigner_mod = PyModule::new(py, "presigner")?;
    presigner_mod.add_class::<Presigner>()?;
    let errors_mod = PyModule::new(py, "errors")?;
    errors_mod.add("BincodeError", py.get_type::<BincodeError>())?;
    errors_mod.add("SignerError", py.get_type::<SignerError>())?;
    errors_mod.add("CborError", py.get_type::<CborError>())?;
    errors_mod.add("SerdeJSONError", py.get_type::<SerdeJSONError>())?;
    #[cfg(feature = "ring")]
    let rpc_mod = create_rpc_mod(py)?;
    let commitment_config_mod = PyModule::new(py, "commitment_config")?;
    commitment_config_mod.add_class::<CommitmentConfig>()?;
    commitment_config_mod.add_class::<CommitmentLevel>()?;
    #[cfg(feature = "ring")]
    let transaction_status_mod = create_transaction_status_mod(py)?;
    #[cfg(feature = "ring")]
    let account_decoder_mod = create_account_decoder_mod(py)?;
    #[cfg(feature = "ring")]
    let account_mod = create_account_mod(py)?;
    let epoch_schedule_mod = create_epoch_schedule_mod(py)?;
    let address_lookup_table_account_mod = create_address_lookup_table_account_mod(py)?;
    #[cfg(feature = "bankrun")]
    let bankrun_mod = solders_bankrun::create_bankrun_mod(py)?;
    let clock_mod = create_clock_mod(py)?;
    let rent_mod = create_rent_mod(py)?;
    let epoch_info_mod = create_epoch_info_mod(py)?;
    let compute_budget_mod = create_compute_budget_mod(py)?;
    let token_mod = create_token_mod(py)?;
    let submodules = [
        #[cfg(feature = "ring")]
        account_mod,
        #[cfg(feature = "ring")]
        account_decoder_mod,
        address_lookup_table_account_mod,
        #[cfg(feature = "bankrun")]
        bankrun_mod,
        clock_mod,
        commitment_config_mod,
        compute_budget_mod,
        epoch_info_mod,
        epoch_schedule_mod,
        errors_mod,
        hash_mod,
        instruction_mod,
        keypair_mod,
        message_mod,
        null_signer_mod,
        presigner_mod,
        pubkey_mod,
        rent_mod,
        #[cfg(feature = "ring")]
        rpc_mod,
        signature_mod,
        system_program_mod,
        sysvar_mod,
        token_mod,
        transaction_mod,
        #[cfg(feature = "ring")]
        transaction_status_mod,
    ];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("solders.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        m.add_submodule(submod)?;
    }
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
