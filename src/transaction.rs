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
    signature::Signature as SignatureOriginal,
    signer::{keypair::Keypair as KeypairOriginal, signers::Signers, Signer},
    transaction::{uses_durable_nonce, Transaction as TransactionOriginal},
};

use crate::{
    convert_instructions, convert_optional_pubkey, handle_py_value_err, CompiledInstruction,
    Instruction, Keypair, Message, Pubkey, RichcmpEqualityOnly, Signature, SolderHash,
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

    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        self.0
            .signatures
            .clone()
            .into_iter()
            .map(Signature::from)
            .collect()
    }

    #[getter]
    pub fn message(&self) -> Message {
        self.0.message.clone().into()
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

    #[staticmethod]
    pub fn new_with_compiled_instructions(
        from_keypairs: Vec<Keypair>,
        keys: Vec<Pubkey>,
        recent_blockhash: SolderHash,
        program_ids: Vec<Pubkey>,
        instructions: Vec<CompiledInstruction>,
    ) -> Self {
        let converted_keypairs = &convert_keypairs(&from_keypairs);
        let converted_keys: Vec<PubkeyOriginal> =
            keys.into_iter().map(PubkeyOriginal::from).collect();
        let converted_program_ids: Vec<PubkeyOriginal> =
            program_ids.into_iter().map(PubkeyOriginal::from).collect();
        let converted_instructions = instructions
            .into_iter()
            .map(solana_sdk::instruction::CompiledInstruction::from)
            .collect();
        TransactionOriginal::new_with_compiled_instructions(
            converted_keypairs,
            &converted_keys[..],
            recent_blockhash.into(),
            converted_program_ids,
            converted_instructions,
        )
        .into()
    }

    pub fn data(&self, instruction_index: usize) -> &[u8] {
        self.0.data(instruction_index)
    }

    pub fn key(&self, instruction_index: usize, accounts_index: usize) -> Option<Pubkey> {
        self.0
            .key(instruction_index, accounts_index)
            .map(Pubkey::from)
    }

    pub fn signer_key(&self, instruction_index: usize, accounts_index: usize) -> Option<Pubkey> {
        self.0
            .signer_key(instruction_index, accounts_index)
            .map(Pubkey::from)
    }

    pub fn message_data(&self) -> Vec<u8> {
        self.0.message_data()
    }

    pub fn sign(&mut self, keypairs: Vec<Keypair>, recent_blockhash: SolderHash) -> PyResult<()> {
        let converted_keypairs = convert_keypairs(&keypairs);
        handle_py_value_err(
            self.0
                .try_sign(&converted_keypairs, recent_blockhash.into()),
        )
    }

    pub fn try_partial_sign(
        &mut self,
        keypairs: Vec<Keypair>,
        recent_blockhash: SolderHash,
    ) -> PyResult<()> {
        let converted_keypairs = convert_keypairs(&keypairs);
        handle_py_value_err(
            self.0
                .try_partial_sign(&converted_keypairs, recent_blockhash.into()),
        )
    }

    pub fn verify(&self) -> PyResult<()> {
        handle_py_value_err(self.0.verify())
    }

    pub fn verify_and_hash_message(&self) -> PyResult<SolderHash> {
        handle_py_value_err(self.0.verify_and_hash_message())
    }

    pub fn verify_with_results(&self) -> Vec<bool> {
        self.0.verify_with_results()
    }

    pub fn get_signing_keypair_positions(
        &self,
        pubkeys: Vec<Pubkey>,
    ) -> PyResult<Vec<Option<usize>>> {
        let converted_pubkeys: Vec<PubkeyOriginal> =
            pubkeys.into_iter().map(PubkeyOriginal::from).collect();
        handle_py_value_err(self.0.get_signing_keypair_positions(&converted_pubkeys[..]))
    }

    pub fn replace_signatures(&mut self, signers: Vec<(Pubkey, Signature)>) -> PyResult<()> {
        let converted_signers: Vec<(PubkeyOriginal, SignatureOriginal)> = signers
            .into_iter()
            .map(|(pubkey, signature)| {
                (
                    PubkeyOriginal::from(pubkey),
                    SignatureOriginal::from(signature),
                )
            })
            .collect();
        handle_py_value_err(self.0.replace_signatures(&converted_signers[..]))
    }

    pub fn uses_durable_nonce(&self) -> Option<CompiledInstruction> {
        uses_durable_nonce(&self.0).map(|x| CompiledInstruction::from(x.clone()))
    }
}

impl From<TransactionOriginal> for Transaction {
    fn from(tx: TransactionOriginal) -> Self {
        Self(tx)
    }
}
