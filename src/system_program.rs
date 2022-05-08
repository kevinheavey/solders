use pyo3::prelude::*;
use solana_sdk::{
    instruction::Instruction as InstructionOriginal,
    pubkey::Pubkey as PubkeyOriginal,
    system_instruction::{
        advance_nonce_account, allocate, allocate_with_seed, assign, assign_with_seed,
        create_account, create_account_with_seed, create_nonce_account,
        create_nonce_account_with_seed, transfer, transfer_many, transfer_with_seed,
        withdraw_nonce_account,
    },
};

use crate::{Instruction, Pubkey};

fn convert_instructions_from_original(ixs: Vec<InstructionOriginal>) -> Vec<Instruction> {
    ixs.into_iter().map(Instruction::from).collect()
}

#[pyclass]
pub struct SystemProgram;

#[pymethods]
impl SystemProgram {
    #[staticmethod]
    pub fn create_account(
        from_pubkey: &Pubkey,
        to_pubkey: &Pubkey,
        lamports: u64,
        space: u64,
        owner: &Pubkey,
    ) -> Instruction {
        create_account(
            from_pubkey.as_ref(),
            to_pubkey.as_ref(),
            lamports,
            space,
            owner.as_ref(),
        )
        .into()
    }

    #[staticmethod]
    pub fn create_account_with_seed(
        from_pubkey: &Pubkey,
        to_pubkey: &Pubkey, // must match create_with_seed(base, seed, owner)
        base: &Pubkey,
        seed: &str,
        lamports: u64,
        space: u64,
        owner: &Pubkey,
    ) -> Instruction {
        create_account_with_seed(
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

    #[staticmethod]
    pub fn assign(pubkey: &Pubkey, owner: &Pubkey) -> Instruction {
        assign(pubkey.as_ref(), owner.as_ref()).into()
    }

    #[staticmethod]
    pub fn assign_with_seed(
        address: &Pubkey, // must match create_with_seed(base, seed, owner)
        base: &Pubkey,
        seed: &str,
        owner: &Pubkey,
    ) -> Instruction {
        assign_with_seed(address.as_ref(), base.as_ref(), seed, owner.as_ref()).into()
    }

    #[staticmethod]
    pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, lamports: u64) -> Instruction {
        transfer(from_pubkey.as_ref(), to_pubkey.as_ref(), lamports).into()
    }

    #[staticmethod]
    pub fn transfer_with_seed(
        from_pubkey: &Pubkey, // must match create_with_seed(base, seed, owner)
        from_base: &Pubkey,
        from_seed: &str,
        from_owner: &Pubkey,
        to_pubkey: &Pubkey,
        lamports: u64,
    ) -> Instruction {
        transfer_with_seed(
            from_pubkey.as_ref(),
            from_base.as_ref(),
            from_seed.into(),
            from_owner.as_ref(),
            to_pubkey.as_ref(),
            lamports,
        )
        .into()
    }
    #[staticmethod]
    pub fn allocate(pubkey: &Pubkey, space: u64) -> Instruction {
        allocate(pubkey.as_ref(), space).into()
    }
    #[staticmethod]
    pub fn allocate_with_seed(
        address: &Pubkey, // must match create_with_seed(base, seed, owner)
        base: &Pubkey,
        seed: &str,
        space: u64,
        owner: &Pubkey,
    ) -> Instruction {
        allocate_with_seed(address.as_ref(), base.as_ref(), seed, space, owner.as_ref()).into()
    }

    #[staticmethod]
    pub fn transfer_many(
        from_pubkey: &Pubkey,
        to_lamports: Vec<(Pubkey, u64)>,
    ) -> Vec<Instruction> {
        let to_lamports_converted: Vec<(PubkeyOriginal, u64)> = to_lamports
            .into_iter()
            .map(|x| (PubkeyOriginal::from(x.0), x.1))
            .collect();
        convert_instructions_from_original(transfer_many(
            from_pubkey.as_ref(),
            &to_lamports_converted,
        ))
    }

    #[staticmethod]
    pub fn create_nonce_account_with_seed(
        from_pubkey: &Pubkey,
        nonce_pubkey: &Pubkey,
        base: &Pubkey,
        seed: &str,
        authority: &Pubkey,
        lamports: u64,
    ) -> (Instruction, Instruction) {
        let ixs = create_nonce_account_with_seed(
            from_pubkey.as_ref(),
            nonce_pubkey.as_ref(),
            base.as_ref(),
            seed,
            authority.as_ref(),
            lamports,
        );
        (ixs[0].clone().into(), ixs[1].clone().into())
    }

    #[staticmethod]
    pub fn create_nonce_account(
        from_pubkey: &Pubkey,
        nonce_pubkey: &Pubkey,
        authority: &Pubkey,
        lamports: u64,
    ) -> (Instruction, Instruction) {
        let ixs = create_nonce_account(
            from_pubkey.as_ref(),
            nonce_pubkey.as_ref(),
            authority.as_ref(),
            lamports,
        );
        (ixs[0].clone().into(), ixs[1].clone().into())
    }

    #[staticmethod]
    pub fn advance_nonce_account(nonce_pubkey: &Pubkey, authorized_pubkey: &Pubkey) -> Instruction {
        advance_nonce_account(nonce_pubkey.as_ref(), authorized_pubkey.as_ref()).into()
    }

    #[staticmethod]
    pub fn withdraw_nonce_account(
        nonce_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        to_pubkey: &Pubkey,
        lamports: u64,
    ) -> Instruction {
        withdraw_nonce_account(
            nonce_pubkey.as_ref(),
            authorized_pubkey.as_ref(),
            to_pubkey.as_ref(),
            lamports,
        )
        .into()
    }
}
