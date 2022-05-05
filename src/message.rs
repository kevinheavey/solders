use pyo3::prelude::*;
use solana_sdk::message::legacy::Message as MessageOriginal;

use crate::{Instruction, Pubkey};

#[pyclass]
#[derive(PartialEq, Debug, Clone)]
pub struct Message(pub MessageOriginal);

impl Message {
    pub fn new(instructions: &[Instruction], payer: Option<&Pubkey>) -> Self {
        let instructions_inner = &instructions
            .iter()
            .map(|x| -> solana_sdk::instruction::Instruction { x.into() })
            .collect::<Vec<_>>()[..];
        MessageOriginal::new(instructions_inner, payer.map(|p| p.as_ref())).into()
    }
}

impl From<MessageOriginal> for Message {
    fn from(message: MessageOriginal) -> Self {
        Self(message)
    }
}
