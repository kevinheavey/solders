use pyo3::{prelude::*, pyclass::CompareOp};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{
        CompiledInstruction as CompiledInstructionOriginal, Instruction as InstructionOriginal,
    },
    message::{
        legacy::Message as MessageOriginal, MessageHeader as MessageHeaderOriginal,
        MESSAGE_HEADER_LENGTH,
    },
    pubkey::Pubkey as PubkeyOriginal,
    signer::{keypair::Keypair as KeypairOriginal, signers::Signers, Signer},
    transaction::Transaction as TransactionOriginal,
};

use crate::{
    convert_instructions, convert_optional_pubkey, handle_py_value_err, CompiledInstruction,
    Instruction, Keypair, Message, Pubkey, RichcmpEqualityOnly, SolderHash,
};

fn convert_keypairs(keypairs: &Vec<Keypair>) -> Vec<&KeypairOriginal> {
    keypairs.iter().map(|x| x.as_ref()).collect()
}

#[pyclass]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
pub struct Transaction(TransactionOriginal);

#[pymethods]
impl Transaction {
    #[new]
    pub fn new(
        from_keypairs: Vec<Keypair>,
        message: &Message,
        recent_blockhash: SolderHash,
    ) -> Self {
        let underlying_keypairs = convert_keypairs(&from_keypairs);
        TransactionOriginal::new(
            &underlying_keypairs,
            message.into(),
            recent_blockhash.into(),
        )
        .into()
    }

    #[staticmethod]
    pub fn new_unsigned(message: Message) -> Self {
        TransactionOriginal::new_unsigned(message.into()).into()
    }

    #[staticmethod]
    pub fn new_with_payer(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        TransactionOriginal::new_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer),
        )
        .into()
    }

    #[staticmethod]
    pub fn new_signed_with_payer(
        instructions: Vec<Instruction>,
        payer: Option<&Pubkey>,
        signing_keypairs: Vec<Keypair>,
        recent_blockhash: SolderHash,
    ) -> Self {
        TransactionOriginal::new_signed_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer),
            &convert_keypairs(&signing_keypairs),
            recent_blockhash.into(),
        )
        .into()
    }
}

impl From<TransactionOriginal> for Transaction {
    fn from(tx: TransactionOriginal) -> Self {
        Self(tx)
    }
}
