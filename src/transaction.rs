use pyo3::{prelude::*, pyclass::CompareOp, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    sanitize::Sanitize,
    signature::Signature as SignatureOriginal,
    transaction::{
        get_nonce_pubkey_from_instruction, uses_durable_nonce, Transaction as TransactionOriginal,
    },
};

use crate::{
    convert_instructions, convert_optional_pubkey, handle_py_value_err, signer::SignerVec,
    CompiledInstruction, Instruction, Message, Pubkey, RichcmpEqualityOnly, Signature, Signer,
    SolderHash,
};

#[pyclass]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
pub struct Transaction(TransactionOriginal);

#[pymethods]
impl Transaction {
    #[new]
    pub fn new(
        from_keypairs: Vec<Signer>,
        message: &Message,
        recent_blockhash: SolderHash,
    ) -> Self {
        TransactionOriginal::new(
            &SignerVec(from_keypairs),
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
        signing_keypairs: Vec<Signer>,
        recent_blockhash: SolderHash,
    ) -> Self {
        TransactionOriginal::new_signed_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer),
            &SignerVec(signing_keypairs),
            recent_blockhash.into(),
        )
        .into()
    }

    #[staticmethod]
    pub fn new_with_compiled_instructions(
        from_keypairs: Vec<Signer>,
        keys: Vec<Pubkey>,
        recent_blockhash: SolderHash,
        program_ids: Vec<Pubkey>,
        instructions: Vec<CompiledInstruction>,
    ) -> Self {
        let converted_keys: Vec<PubkeyOriginal> =
            keys.into_iter().map(PubkeyOriginal::from).collect();
        let converted_program_ids: Vec<PubkeyOriginal> =
            program_ids.into_iter().map(PubkeyOriginal::from).collect();
        let converted_instructions = instructions
            .into_iter()
            .map(solana_sdk::instruction::CompiledInstruction::from)
            .collect();
        TransactionOriginal::new_with_compiled_instructions(
            &SignerVec(from_keypairs),
            &converted_keys,
            recent_blockhash.into(),
            converted_program_ids,
            converted_instructions,
        )
        .into()
    }

    #[staticmethod]
    pub fn populate(message: Message, signatures: Vec<Signature>) -> Self {
        (TransactionOriginal {
            message: message.into(),
            signatures: signatures
                .into_iter()
                .map(SignatureOriginal::from)
                .collect(),
        })
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

    pub fn message_data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.message_data())
    }

    pub fn sign(&mut self, keypairs: Vec<Signer>, recent_blockhash: SolderHash) -> PyResult<()> {
        handle_py_value_err(
            self.0
                .try_sign(&SignerVec(keypairs), recent_blockhash.into()),
        )
    }

    pub fn partial_sign(
        &mut self,
        keypairs: Vec<Signer>,
        recent_blockhash: SolderHash,
    ) -> PyResult<()> {
        handle_py_value_err(
            self.0
                .try_partial_sign(&SignerVec(keypairs), recent_blockhash.into()),
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
        handle_py_value_err(self.0.get_signing_keypair_positions(&converted_pubkeys))
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
        handle_py_value_err(self.0.replace_signatures(&converted_signers))
    }

    pub fn is_signed(&self) -> bool {
        self.0.is_signed()
    }

    pub fn uses_durable_nonce(&self) -> Option<CompiledInstruction> {
        uses_durable_nonce(&self.0).map(|x| CompiledInstruction::from(x.clone()))
    }

    pub fn sanitize(&self) -> PyResult<()> {
        handle_py_value_err(self.0.sanitize())
    }

    pub fn serialize<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let as_vec: Vec<u8> = handle_py_value_err(bincode::serialize(&self.0))?;
        Ok(PyBytes::new(py, &as_vec))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn deserialize(data: &[u8]) -> PyResult<Self> {
        handle_py_value_err(bincode::deserialize::<TransactionOriginal>(data))
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        self.richcmp(other, op)
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_nonce_pubkey_from_instruction(&self, ix: &CompiledInstruction) -> Option<Pubkey> {
        get_nonce_pubkey_from_instruction(ix.as_ref(), self.as_ref()).map(Pubkey::from)
    }
}

impl RichcmpEqualityOnly for Transaction {}

impl From<TransactionOriginal> for Transaction {
    fn from(tx: TransactionOriginal) -> Self {
        Self(tx)
    }
}

impl AsRef<TransactionOriginal> for Transaction {
    fn as_ref(&self) -> &TransactionOriginal {
        &self.0
    }
}
