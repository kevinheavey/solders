use pyo3::prelude::*;
use solana_sdk::{
    instruction::Instruction as InstructionOriginal, message::legacy::Message as MessageOriginal,
    pubkey::Pubkey as PubkeyOriginal,
};

use crate::{Instruction, Pubkey, SolderHash};

fn convert_instructions(instructions: &[Instruction]) -> Vec<InstructionOriginal> {
    instructions
        .iter()
        .map(|x| -> solana_sdk::instruction::Instruction { x.into() })
        .collect::<Vec<_>>()
}

fn convert_otpional_pubkey(pubkey: Option<&Pubkey>) -> Option<&PubkeyOriginal> {
    pubkey.map(|p| p.as_ref())
}
#[pyclass]
#[derive(PartialEq, Debug, Clone)]
pub struct Message(pub MessageOriginal);

impl Message {
    pub fn new(instructions: &[Instruction], payer: Option<&Pubkey>) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new(&instructions_inner[..], convert_otpional_pubkey(payer)).into()
    }

    pub fn new_with_blockhash(
        instructions: &[Instruction],
        payer: Option<&Pubkey>,
        blockhash: &SolderHash,
    ) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new_with_blockhash(
            &instructions_inner[..],
            convert_otpional_pubkey(payer),
            blockhash.as_ref(),
        )
        .into()
    }
}

impl From<MessageOriginal> for Message {
    fn from(message: MessageOriginal) -> Self {
        Self(message)
    }
}
