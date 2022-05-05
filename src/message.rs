use pyo3::prelude::*;
use solana_sdk::{
    instruction::{
        CompiledInstruction as CompiledInstructionOriginal, Instruction as InstructionOriginal,
    },
    message::legacy::Message as MessageOriginal,
    pubkey::Pubkey as PubkeyOriginal,
};

use crate::{CompiledInstruction, Instruction, Pubkey, SolderHash};

fn convert_instructions(instructions: Vec<Instruction>) -> Vec<InstructionOriginal> {
    instructions
        .into_iter()
        .map(|x| -> solana_sdk::instruction::Instruction { x.into() })
        .collect()
}

fn convert_otpional_pubkey(pubkey: Option<&Pubkey>) -> Option<&PubkeyOriginal> {
    pubkey.map(|p| p.as_ref())
}
#[pyclass]
#[derive(PartialEq, Debug, Clone)]
pub struct Message(MessageOriginal);

#[pymethods]
impl Message {
    #[new]
    pub fn new(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new(&instructions_inner[..], convert_otpional_pubkey(payer)).into()
    }

    #[staticmethod]
    pub fn new_with_blockhash(
        instructions: Vec<Instruction>,
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

    #[staticmethod]
    pub fn new_with_compiled_instructions(
        num_required_signatures: u8,
        num_readonly_signed_accounts: u8,
        num_readonly_unsigned_accounts: u8,
        account_keys: Vec<Pubkey>,
        recent_blockhash: SolderHash,
        instructions: Vec<CompiledInstruction>,
    ) -> Self {
        let instructions_inner: Vec<CompiledInstructionOriginal> =
            instructions.into_iter().map(|x| x.into()).collect();
        let account_keys_inner: Vec<PubkeyOriginal> =
            account_keys.into_iter().map(|x| x.into()).collect();
        MessageOriginal::new_with_compiled_instructions(
            num_required_signatures,
            num_readonly_signed_accounts,
            num_readonly_unsigned_accounts,
            account_keys_inner,
            recent_blockhash.into(),
            instructions_inner,
        )
        .into()
    }
}

impl From<MessageOriginal> for Message {
    fn from(message: MessageOriginal) -> Self {
        Self(message)
    }
}
