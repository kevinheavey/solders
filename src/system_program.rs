use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::prelude::*;
use solana_sdk::{
    instruction::Instruction as InstructionOriginal,
    pubkey::Pubkey as PubkeyOriginal,
    system_instruction::{
        advance_nonce_account as advance_nonce_account_original, allocate as allocate_original,
        allocate_with_seed as allocate_with_seed_original, assign as assign_original,
        assign_with_seed as assign_with_seed_original, create_account as create_account_original,
        create_account_with_seed as create_account_with_seed_original,
        create_nonce_account as create_nonce_account_original,
        create_nonce_account_with_seed as create_nonce_account_with_seed_original,
        transfer as transfer_original, transfer_many as transfer_many_original,
        transfer_with_seed as transfer_with_seed_original,
        withdraw_nonce_account as withdraw_nonce_account_original,
        SystemInstruction as SystemInstructionOriginal,
    },
    system_program,
};

use crate::{Instruction, Pubkey};

fn convert_instructions_from_original(ixs: Vec<InstructionOriginal>) -> Vec<Instruction> {
    ixs.into_iter().map(Instruction::from).collect()
}

#[derive(FromPyObject, IntoPyObject)]
pub struct CreateAccountParams {
    from_pubkey: Pubkey,
    to_pubkey: Pubkey,
    lamports: u64,
    space: u64,
    owner: Pubkey,
}

pub fn create_system_program_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let system_program_mod = PyModule::new(py, "system_program")?;
    system_program_mod.add("ID", Pubkey(system_program::ID))?;
    let funcs = [
        wrap_pyfunction!(create_account, system_program_mod)?,
        wrap_pyfunction!(create_account_with_seed, system_program_mod)?,
        wrap_pyfunction!(assign, system_program_mod)?,
        wrap_pyfunction!(assign_with_seed, system_program_mod)?,
        wrap_pyfunction!(transfer, system_program_mod)?,
        wrap_pyfunction!(transfer_with_seed, system_program_mod)?,
        wrap_pyfunction!(allocate, system_program_mod)?,
        wrap_pyfunction!(allocate_with_seed, system_program_mod)?,
        wrap_pyfunction!(transfer_many, system_program_mod)?,
        wrap_pyfunction!(create_nonce_account, system_program_mod)?,
        wrap_pyfunction!(create_nonce_account_with_seed, system_program_mod)?,
        wrap_pyfunction!(advance_nonce_account, system_program_mod)?,
        wrap_pyfunction!(withdraw_nonce_account, system_program_mod)?,
    ];
    for func in funcs {
        system_program_mod.add_function(func)?;
    }
    Ok(system_program_mod)
}

#[pyclass(module = "solders", subclass)]
pub struct SystemProgram;

// #[pyfunction]
// pub fn decode_create_account(data: &[u8]) -> CreateAccountParams {
//     let deser = bincode::deserialize::<SystemInstructionOriginal>(data).unwrap();
// }

#[pyfunction]
pub fn create_account(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Instruction {
    create_account_original(
        from_pubkey.as_ref(),
        to_pubkey.as_ref(),
        lamports,
        space,
        owner.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn create_account_with_seed(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey, // must match create_with_seed(base, seed, owner)
    base: &Pubkey,
    seed: &str,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Instruction {
    create_account_with_seed_original(
        from_pubkey.as_ref(),
        to_pubkey.as_ref(),
        base.as_ref(),
        seed,
        lamports,
        space,
        owner.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn assign(pubkey: &Pubkey, owner: &Pubkey) -> Instruction {
    assign_original(pubkey.as_ref(), owner.as_ref()).into()
}

#[pyfunction]
pub fn assign_with_seed(
    address: &Pubkey, // must match create_with_seed(base, seed, owner)
    base: &Pubkey,
    seed: &str,
    owner: &Pubkey,
) -> Instruction {
    assign_with_seed_original(address.as_ref(), base.as_ref(), seed, owner.as_ref()).into()
}

#[pyfunction]
pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, lamports: u64) -> Instruction {
    transfer_original(from_pubkey.as_ref(), to_pubkey.as_ref(), lamports).into()
}

#[pyfunction]
pub fn transfer_with_seed(
    from_pubkey: &Pubkey, // must match create_with_seed(base, seed, owner)
    from_base: &Pubkey,
    from_seed: &str,
    from_owner: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
) -> Instruction {
    transfer_with_seed_original(
        from_pubkey.as_ref(),
        from_base.as_ref(),
        from_seed.into(),
        from_owner.as_ref(),
        to_pubkey.as_ref(),
        lamports,
    )
    .into()
}
#[pyfunction]
pub fn allocate(pubkey: &Pubkey, space: u64) -> Instruction {
    allocate_original(pubkey.as_ref(), space).into()
}
#[pyfunction]
pub fn allocate_with_seed(
    address: &Pubkey, // must match create_with_seed(base, seed, owner)
    base: &Pubkey,
    seed: &str,
    space: u64,
    owner: &Pubkey,
) -> Instruction {
    allocate_with_seed_original(address.as_ref(), base.as_ref(), seed, space, owner.as_ref()).into()
}

#[pyfunction]
pub fn transfer_many(from_pubkey: &Pubkey, to_lamports: Vec<(Pubkey, u64)>) -> Vec<Instruction> {
    let to_lamports_converted: Vec<(PubkeyOriginal, u64)> = to_lamports
        .into_iter()
        .map(|x| (PubkeyOriginal::from(x.0), x.1))
        .collect();
    convert_instructions_from_original(transfer_many_original(
        from_pubkey.as_ref(),
        &to_lamports_converted,
    ))
}

#[pyfunction]
pub fn create_nonce_account(
    from_pubkey: &Pubkey,
    nonce_pubkey: &Pubkey,
    authority: &Pubkey,
    lamports: u64,
) -> (Instruction, Instruction) {
    let ixs = create_nonce_account_original(
        from_pubkey.as_ref(),
        nonce_pubkey.as_ref(),
        authority.as_ref(),
        lamports,
    );
    (ixs[0].clone().into(), ixs[1].clone().into())
}

#[pyfunction]
pub fn create_nonce_account_with_seed(
    from_pubkey: &Pubkey,
    nonce_pubkey: &Pubkey,
    base: &Pubkey,
    seed: &str,
    authority: &Pubkey,
    lamports: u64,
) -> (Instruction, Instruction) {
    let ixs = create_nonce_account_with_seed_original(
        from_pubkey.as_ref(),
        nonce_pubkey.as_ref(),
        base.as_ref(),
        seed,
        authority.as_ref(),
        lamports,
    );
    (ixs[0].clone().into(), ixs[1].clone().into())
}

#[pyfunction]
pub fn advance_nonce_account(nonce_pubkey: &Pubkey, authorized_pubkey: &Pubkey) -> Instruction {
    advance_nonce_account_original(nonce_pubkey.as_ref(), authorized_pubkey.as_ref()).into()
}

#[pyfunction]
pub fn withdraw_nonce_account(
    nonce_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
) -> Instruction {
    withdraw_nonce_account_original(
        nonce_pubkey.as_ref(),
        authorized_pubkey.as_ref(),
        to_pubkey.as_ref(),
        lamports,
    )
    .into()
}
