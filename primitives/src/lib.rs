use crate::pubkey::Pubkey;
use instruction::Instruction;
use solana_sdk::{
    instruction::Instruction as InstructionOriginal, pubkey::Pubkey as PubkeyOriginal,
};

pub mod address_lookup_table_account;
pub mod hash;
pub mod instruction;
pub mod keypair;
pub mod message;
pub mod null_signer;
pub mod presigner;
pub mod pubkey;
pub mod signature;
pub mod signer;
pub mod transaction;

pub fn convert_optional_pubkey(pubkey: Option<&Pubkey>) -> Option<&PubkeyOriginal> {
    pubkey.map(|p| p.as_ref())
}

pub fn convert_instructions(instructions: Vec<Instruction>) -> Vec<InstructionOriginal> {
    instructions
        .into_iter()
        .map(solana_sdk::instruction::Instruction::from)
        .collect()
}
