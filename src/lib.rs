//! These docstrings are written for Python users.
//!
//! If you're viewing them on docs.rs, the formatting won't make much sense.
use account::create_account_mod;
use address_lookup_table_account::create_address_lookup_table_account_mod;
use commitment_config::{CommitmentConfig, CommitmentLevel};
use pyo3::prelude::*;
use rpc::create_rpc_mod;
use solders_primitives::instruction::{AccountMeta, CompiledInstruction, Instruction};
use solders_traits::{BincodeError, CborError, ParseHashError, SerdeJSONError, SignerError};
use std::collections::HashMap;
use system_program::create_system_program_mod;
use sysvar::create_sysvar_mod;
use transaction_status::create_transaction_status_mod;
pub mod message;
use message::create_message_mod;
pub mod transaction;
use transaction::create_transaction_mod;
pub mod account_decoder;
use account_decoder::create_account_decoder_mod;
pub mod account;
pub mod address_lookup_table_account;
pub mod commitment_config;
pub mod epoch_schedule;
pub mod rpc;
pub mod system_program;
pub mod sysvar;
mod tmp_account_decoder;
mod tmp_transaction_status;
pub mod transaction_status;
use epoch_schedule::create_epoch_schedule_mod;
use solders_primitives::{
    hash::Hash as SolderHash, keypair::Keypair, null_signer::NullSigner, presigner::Presigner,
    pubkey::Pubkey, signature::Signature,
};

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
    let rpc_mod = create_rpc_mod(py)?;
    let commitment_config_mod = PyModule::new(py, "commitment_config")?;
    commitment_config_mod.add_class::<CommitmentConfig>()?;
    commitment_config_mod.add_class::<CommitmentLevel>()?;
    let transaction_status_mod = create_transaction_status_mod(py)?;
    let account_decoder_mod = create_account_decoder_mod(py)?;
    let account_mod = create_account_mod(py)?;
    let epoch_schedule_mod = create_epoch_schedule_mod(py)?;
    let address_lookup_table_account_mod = create_address_lookup_table_account_mod(py)?;
    let submodules = [
        errors_mod,
        hash_mod,
        instruction_mod,
        keypair_mod,
        message_mod,
        null_signer_mod,
        presigner_mod,
        pubkey_mod,
        signature_mod,
        transaction_mod,
        system_program_mod,
        sysvar_mod,
        rpc_mod,
        commitment_config_mod,
        transaction_status_mod,
        account_decoder_mod,
        account_mod,
        address_lookup_table_account_mod,
        epoch_schedule_mod,
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
