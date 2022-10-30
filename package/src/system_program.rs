use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::{exceptions::PyValueError, prelude::*};
use solana_sdk::{
    instruction::Instruction as InstructionOriginal,
    pubkey::Pubkey as PubkeyOriginal,
    system_instruction::{
        advance_nonce_account as advance_nonce_account_original, allocate as allocate_original,
        allocate_with_seed as allocate_with_seed_original, assign as assign_original,
        assign_with_seed as assign_with_seed_original,
        authorize_nonce_account as authorize_nonce_account_original,
        create_account as create_account_original,
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
use solders_traits::handle_py_err;

use crate::{Instruction, Pubkey};

fn convert_instructions_from_original(ixs: Vec<InstructionOriginal>) -> Vec<Instruction> {
    ixs.into_iter().map(Instruction::from).collect()
}

pub fn create_system_program_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let system_program_mod = PyModule::new(py, "_system_program")?;
    system_program_mod.add("ID", Pubkey(system_program::ID))?;
    let funcs = [
        wrap_pyfunction!(create_account, system_program_mod)?,
        wrap_pyfunction!(decode_create_account, system_program_mod)?,
        wrap_pyfunction!(create_account_with_seed, system_program_mod)?,
        wrap_pyfunction!(decode_create_account_with_seed, system_program_mod)?,
        wrap_pyfunction!(assign, system_program_mod)?,
        wrap_pyfunction!(decode_assign, system_program_mod)?,
        wrap_pyfunction!(assign_with_seed, system_program_mod)?,
        wrap_pyfunction!(decode_assign_with_seed, system_program_mod)?,
        wrap_pyfunction!(transfer, system_program_mod)?,
        wrap_pyfunction!(decode_transfer, system_program_mod)?,
        wrap_pyfunction!(transfer_with_seed, system_program_mod)?,
        wrap_pyfunction!(decode_transfer_with_seed, system_program_mod)?,
        wrap_pyfunction!(allocate, system_program_mod)?,
        wrap_pyfunction!(decode_allocate, system_program_mod)?,
        wrap_pyfunction!(allocate_with_seed, system_program_mod)?,
        wrap_pyfunction!(decode_allocate_with_seed, system_program_mod)?,
        wrap_pyfunction!(transfer_many, system_program_mod)?,
        wrap_pyfunction!(create_nonce_account, system_program_mod)?,
        wrap_pyfunction!(initialize_nonce_account, system_program_mod)?,
        wrap_pyfunction!(decode_initialize_nonce_account, system_program_mod)?,
        wrap_pyfunction!(create_nonce_account_with_seed, system_program_mod)?,
        wrap_pyfunction!(advance_nonce_account, system_program_mod)?,
        wrap_pyfunction!(decode_advance_nonce_account, system_program_mod)?,
        wrap_pyfunction!(withdraw_nonce_account, system_program_mod)?,
        wrap_pyfunction!(decode_withdraw_nonce_account, system_program_mod)?,
        wrap_pyfunction!(authorize_nonce_account, system_program_mod)?,
        wrap_pyfunction!(decode_authorize_nonce_account, system_program_mod)?,
    ];
    for func in funcs {
        system_program_mod.add_function(func)?;
    }
    Ok(system_program_mod)
}

#[derive(FromPyObject, IntoPyObject)]
pub struct CreateAccountParams {
    from_pubkey: Pubkey,
    to_pubkey: Pubkey,
    lamports: u64,
    space: u64,
    owner: Pubkey,
}

#[pyfunction]
pub fn decode_create_account(instruction: Instruction) -> PyResult<CreateAccountParams> {
    let keys = instruction.0.accounts;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::CreateAccount {
            lamports,
            space,
            owner,
        } => Ok(CreateAccountParams {
            from_pubkey: keys[0].pubkey.into(),
            to_pubkey: keys[1].pubkey.into(),
            lamports,
            space,
            owner: owner.into(),
        }),
        _ => Err(PyValueError::new_err("Not a CreateAccount instruction")),
    }
}

#[pyfunction]
pub fn create_account(params: CreateAccountParams) -> Instruction {
    create_account_original(
        params.from_pubkey.as_ref(),
        params.to_pubkey.as_ref(),
        params.lamports,
        params.space,
        params.owner.as_ref(),
    )
    .into()
}

#[derive(FromPyObject, IntoPyObject)]
pub struct CreateAccountWithSeedParams {
    from_pubkey: Pubkey,
    to_pubkey: Pubkey,
    base: Pubkey,
    seed: String,
    lamports: u64,
    space: u64,
    owner: Pubkey,
}

#[pyfunction]
pub fn create_account_with_seed(params: CreateAccountWithSeedParams) -> Instruction {
    create_account_with_seed_original(
        params.from_pubkey.as_ref(),
        params.to_pubkey.as_ref(),
        params.base.as_ref(),
        &params.seed,
        params.lamports,
        params.space,
        params.owner.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn decode_create_account_with_seed(
    instruction: Instruction,
) -> PyResult<CreateAccountWithSeedParams> {
    let keys = instruction.0.accounts;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::CreateAccountWithSeed {
            base,
            seed,
            lamports,
            space,
            owner,
        } => Ok(CreateAccountWithSeedParams {
            from_pubkey: keys[0].pubkey.into(),
            to_pubkey: keys[1].pubkey.into(),
            base: base.into(),
            seed,
            lamports,
            space,
            owner: owner.into(),
        }),
        _ => Err(PyValueError::new_err(
            "Not a CreateAccountWithSeed instruction",
        )),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct AssignParams {
    pubkey: Pubkey,
    owner: Pubkey,
}

#[pyfunction]
pub fn assign(params: AssignParams) -> Instruction {
    assign_original(params.pubkey.as_ref(), params.owner.as_ref()).into()
}

#[pyfunction]
pub fn decode_assign(instruction: Instruction) -> PyResult<AssignParams> {
    let pubkey = instruction.0.accounts[0].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::Assign { owner } => Ok(AssignParams {
            pubkey: pubkey.into(),
            owner: owner.into(),
        }),
        _ => Err(PyValueError::new_err("Not an Assign instruction")),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct AssignWithSeedParams {
    address: Pubkey,
    base: Pubkey,
    seed: String,
    owner: Pubkey,
}

#[pyfunction]
pub fn assign_with_seed(params: AssignWithSeedParams) -> Instruction {
    assign_with_seed_original(
        params.address.as_ref(),
        params.base.as_ref(),
        &params.seed,
        params.owner.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn decode_assign_with_seed(instruction: Instruction) -> PyResult<AssignWithSeedParams> {
    let address = instruction.0.accounts[0].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::AssignWithSeed { base, seed, owner } => {
            Ok(AssignWithSeedParams {
                address: address.into(),
                base: base.into(),
                seed,
                owner: owner.into(),
            })
        }
        _ => Err(PyValueError::new_err("Not an AssignWithSeed instruction")),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct TransferParams {
    from_pubkey: Pubkey,
    to_pubkey: Pubkey,
    lamports: u64,
}

#[pyfunction]
pub fn transfer(params: TransferParams) -> Instruction {
    transfer_original(
        params.from_pubkey.as_ref(),
        params.to_pubkey.as_ref(),
        params.lamports,
    )
    .into()
}

#[pyfunction]
pub fn decode_transfer(instruction: Instruction) -> PyResult<TransferParams> {
    let keys = instruction.0.accounts;
    let from_pubkey = keys[0].pubkey;
    let to_pubkey = keys[1].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::Transfer { lamports } => Ok(TransferParams {
            from_pubkey: from_pubkey.into(),
            to_pubkey: to_pubkey.into(),
            lamports,
        }),
        _ => Err(PyValueError::new_err("Not a Transfer instruction")),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct TransferWithSeedParams {
    from_pubkey: Pubkey,
    from_base: Pubkey,
    from_seed: String,
    from_owner: Pubkey,
    to_pubkey: Pubkey,
    lamports: u64,
}

#[pyfunction]
pub fn transfer_with_seed(params: TransferWithSeedParams) -> Instruction {
    transfer_with_seed_original(
        params.from_pubkey.as_ref(),
        params.from_base.as_ref(),
        params.from_seed,
        params.from_owner.as_ref(),
        params.to_pubkey.as_ref(),
        params.lamports,
    )
    .into()
}

#[pyfunction]
pub fn decode_transfer_with_seed(instruction: Instruction) -> PyResult<TransferWithSeedParams> {
    let keys = instruction.0.accounts;
    let from_pubkey = keys[0].pubkey;
    let from_base = keys[1].pubkey;
    let to_pubkey = keys[2].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::TransferWithSeed {
            lamports,
            from_seed,
            from_owner,
        } => Ok(TransferWithSeedParams {
            from_pubkey: from_pubkey.into(),
            from_base: from_base.into(),
            to_pubkey: to_pubkey.into(),
            from_seed,
            from_owner: from_owner.into(),
            lamports,
        }),
        _ => Err(PyValueError::new_err("Not a TransferWithSeed instruction")),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct AllocateParams {
    pubkey: Pubkey,
    space: u64,
}

#[pyfunction]
pub fn allocate(params: AllocateParams) -> Instruction {
    allocate_original(params.pubkey.as_ref(), params.space).into()
}

#[pyfunction]
pub fn decode_allocate(instruction: Instruction) -> PyResult<AllocateParams> {
    let pubkey = instruction.0.accounts[0].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::Allocate { space } => Ok(AllocateParams {
            pubkey: pubkey.into(),
            space,
        }),
        _ => Err(PyValueError::new_err("Not an Allocate instruction")),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct AllocateWithSeedParams {
    address: Pubkey,
    base: Pubkey,
    seed: String,
    space: u64,
    owner: Pubkey,
}

#[pyfunction]
pub fn allocate_with_seed(params: AllocateWithSeedParams) -> Instruction {
    allocate_with_seed_original(
        params.address.as_ref(),
        params.base.as_ref(),
        &params.seed,
        params.space,
        params.owner.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn decode_allocate_with_seed(instruction: Instruction) -> PyResult<AllocateWithSeedParams> {
    let address = instruction.0.accounts[0].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::AllocateWithSeed {
            base,
            seed,
            space,
            owner,
        } => Ok(AllocateWithSeedParams {
            address: address.into(),
            base: base.into(),
            seed,
            space,
            owner: owner.into(),
        }),
        _ => Err(PyValueError::new_err("Not an AllocateWithSeed instruction")),
    }
}

#[pyfunction]
/// Create new Transfer instructions to many destinations.
///
/// Args:
///     from_pubkey (Pubkey): The sender pubkey.
///     to_lamports (Sequence[tuple[int, Pubkey]]): The lamports to transfer to each pubkey.
///
/// Returns:
///     list[Instruction]: The Transfer instructions.
///
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
/// Generate instructions to create and initialize a nonce account.
///
/// Args:
///     from_pubkey (Pubkey): The account that will transfer
///         lamports to the created nonce account.
///     nonce_pubkey (Pubkey): Nonce account which will be
///         created and initialized.
///     authority (Pubkey): Pubkey to set as authority of the
///         initialized nonce account.
///     lamports (int): Amount of lamports to transfer to
///         the created account.
///
/// Returns:
///     tuple[Instruction, Instruction]: The CreateAccount instruction and the InitializeNonceAccount instruction.
///
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

#[derive(FromPyObject, IntoPyObject)]
pub struct InitializeNonceAccountParams {
    nonce_pubkey: Pubkey,
    authority: Pubkey,
}

#[pyfunction]
pub fn initialize_nonce_account(params: InitializeNonceAccountParams) -> Instruction {
    create_nonce_account(
        &Pubkey::default(),
        &params.nonce_pubkey,
        &params.authority,
        0,
    )
    .1
}

#[pyfunction]
pub fn decode_initialize_nonce_account(
    instruction: Instruction,
) -> PyResult<InitializeNonceAccountParams> {
    let nonce_pubkey = instruction.0.accounts[0].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::InitializeNonceAccount(authority) => {
            Ok(InitializeNonceAccountParams {
                authority: authority.into(),
                nonce_pubkey: nonce_pubkey.into(),
            })
        }
        _ => Err(PyValueError::new_err(
            "Not an InitializeNonceAccount instruction",
        )),
    }
}

#[pyfunction]
/// Generate instructions to create a nonce account with seed.
///
/// Args:
///     from_pubkey (Pubkey): The account that will transfer
///         lamports to the created nonce account.
///     nonce_pubkey (Pubkey): Nonce account which will be
///         created and initialized. Must be pre-calculated
///         with :meth:`~solders.pubkey.Pubkey.create_with_seed`
///     base (Pubkey): Base public key to use to derive the
///         address of the created account. Must be the same
///         as the base key used to create ``nonce_pubkey``.
///     seed (str): Seed to use to derive the address of the created account.
///         Must be the same as the seed used to create ``nonce_pubkey``.
///     authority (Pubkey): Pubkey to set as authority of the
///         initialized nonce account.
///     lamports (int): Amount of lamports to transfer to
///         the created account.
///
/// Returns:
///     tuple[Instruction, Instruction]: The CreateAccountWithSeed instruction and the InitializeNonceAccount instruction.
///
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

#[derive(FromPyObject, IntoPyObject)]
pub struct AdvanceNonceAccountParams {
    nonce_pubkey: Pubkey,
    authorized_pubkey: Pubkey,
}

#[pyfunction]
pub fn advance_nonce_account(params: AdvanceNonceAccountParams) -> Instruction {
    advance_nonce_account_original(
        params.nonce_pubkey.as_ref(),
        params.authorized_pubkey.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn decode_advance_nonce_account(
    instruction: Instruction,
) -> PyResult<AdvanceNonceAccountParams> {
    let keys = instruction.0.accounts;
    let nonce_pubkey = keys[0].pubkey;
    let authorized_pubkey = keys[2].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::AdvanceNonceAccount => Ok(AdvanceNonceAccountParams {
            authorized_pubkey: authorized_pubkey.into(),
            nonce_pubkey: nonce_pubkey.into(),
        }),
        _ => Err(PyValueError::new_err(
            "Not an AdvanceNonceAccount instruction",
        )),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct WithdrawNonceAccountParams {
    nonce_pubkey: Pubkey,
    authorized_pubkey: Pubkey,
    to_pubkey: Pubkey,
    lamports: u64,
}

#[pyfunction]
pub fn withdraw_nonce_account(params: WithdrawNonceAccountParams) -> Instruction {
    withdraw_nonce_account_original(
        params.nonce_pubkey.as_ref(),
        params.authorized_pubkey.as_ref(),
        params.to_pubkey.as_ref(),
        params.lamports,
    )
    .into()
}

#[pyfunction]
pub fn decode_withdraw_nonce_account(
    instruction: Instruction,
) -> PyResult<WithdrawNonceAccountParams> {
    let keys = instruction.0.accounts;
    let nonce_pubkey = keys[0].pubkey;
    let to_pubkey = keys[1].pubkey;
    let authorized_pubkey = keys[4].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::WithdrawNonceAccount(lamports) => {
            Ok(WithdrawNonceAccountParams {
                authorized_pubkey: authorized_pubkey.into(),
                nonce_pubkey: nonce_pubkey.into(),
                to_pubkey: to_pubkey.into(),
                lamports,
            })
        }
        _ => Err(PyValueError::new_err(
            "Not a WithdrawNonceAccount instruction",
        )),
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub struct AuthorizeNonceAccountParams {
    nonce_pubkey: Pubkey,
    authorized_pubkey: Pubkey,
    new_authority: Pubkey,
}

#[pyfunction]
pub fn authorize_nonce_account(params: AuthorizeNonceAccountParams) -> Instruction {
    authorize_nonce_account_original(
        params.nonce_pubkey.as_ref(),
        params.authorized_pubkey.as_ref(),
        params.new_authority.as_ref(),
    )
    .into()
}

#[pyfunction]
pub fn decode_authorize_nonce_account(
    instruction: Instruction,
) -> PyResult<AuthorizeNonceAccountParams> {
    let keys = instruction.0.accounts;
    let nonce_pubkey = keys[0].pubkey;
    let authorized_pubkey = keys[1].pubkey;
    let parsed_data = handle_py_err(bincode::deserialize::<SystemInstructionOriginal>(
        instruction.0.data.as_slice(),
    ))?;
    match parsed_data {
        SystemInstructionOriginal::AuthorizeNonceAccount(new_authority) => {
            Ok(AuthorizeNonceAccountParams {
                nonce_pubkey: nonce_pubkey.into(),
                authorized_pubkey: authorized_pubkey.into(),
                new_authority: new_authority.into(),
            })
        }
        _ => Err(PyValueError::new_err(
            "Not an AuthorizeNonceAccount instruction",
        )),
    }
}
