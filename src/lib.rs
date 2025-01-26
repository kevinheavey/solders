//! These docstrings are written for Python users.
//!
//! If you're viewing them on docs.rs, the formatting won't make much sense.
use address_lookup_table_account::include_address_lookup_table_account;
use pyo3::prelude::*;
#[cfg(feature = "ring")]
use rpc::include_rpc;
#[cfg(feature = "ring")]
use solders_account::include_account;
use solders_instruction::{AccountMeta, CompiledInstruction, Instruction};
#[cfg(feature = "litesvm")]
use solders_litesvm::{include_litesvm, transaction_metadata::include_transaction_metadata};
use solders_system_program::include_system_program;
use solders_token::include_token;
use solders_traits::{BincodeError, CborError, ParseHashError, SerdeJSONError, SignerError};
#[cfg(feature = "ring")]
use solders_transaction_status::include_transaction_status;
use sysvar::include_sysvar;
pub mod message;
use message::include_message;
pub mod transaction;
#[cfg(feature = "ring")]
use solders_account_decoder::include_account_decoder;
use transaction::include_transaction;
pub mod address_lookup_table_account;
#[cfg(feature = "ring")]
pub mod rpc;
pub mod sysvar;
use solders_commitment_config::{CommitmentConfig, CommitmentLevel};
use solders_compute_budget::include_compute_budget;
use solders_epoch_info::include_epoch_info;
use solders_hash::Hash as SolderHash;
use solders_keypair::{null_signer::NullSigner, presigner::Presigner, Keypair};
use solders_primitives::{
    clock::include_clock, epoch_rewards::include_epoch_rewards,
    epoch_schedule::include_epoch_schedule, rent::include_rent, slot_history::include_slot_history,
    stake_history::include_stake_history,
};
use solders_pubkey::Pubkey;
use solders_signature::Signature;

#[pymodule]
fn solders(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SolderHash>()?;
    m.add("ParseHashError", py.get_type::<ParseHashError>())?;
    m.add_class::<AccountMeta>()?;
    m.add_class::<Instruction>()?;
    m.add_class::<CompiledInstruction>()?;
    m.add_class::<Pubkey>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<Signature>()?;
    include_message(m)?;
    m.add_class::<NullSigner>()?;
    include_transaction(m, py)?;
    include_system_program(m)?;
    include_sysvar(m)?;
    m.add_class::<Presigner>()?;
    m.add("BincodeError", py.get_type::<BincodeError>())?;
    m.add("SignerError", py.get_type::<SignerError>())?;
    m.add("CborError", py.get_type::<CborError>())?;
    m.add("SerdeJSONError", py.get_type::<SerdeJSONError>())?;
    #[cfg(feature = "ring")]
    include_rpc(m)?;
    m.add_class::<CommitmentConfig>()?;
    m.add_class::<CommitmentLevel>()?;
    #[cfg(feature = "ring")]
    include_transaction_status(m)?;
    #[cfg(feature = "ring")]
    include_account_decoder(m)?;
    #[cfg(feature = "ring")]
    include_account(m)?;
    include_epoch_schedule(m)?;
    include_address_lookup_table_account(m)?;
    include_clock(m)?;
    include_epoch_rewards(m)?;
    include_slot_history(m)?;
    include_stake_history(m)?;
    include_rent(m)?;
    include_epoch_info(m)?;
    include_compute_budget(m)?;
    include_token(m)?;
    #[cfg(feature = "litesvm")]
    include_transaction_metadata(m)?;
    #[cfg(feature = "litesvm")]
    include_litesvm(m)?;
    Ok(())
}
