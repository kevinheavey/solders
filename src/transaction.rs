#![allow(deprecated)]
use pyo3::{
    create_exception, exceptions::PyException, prelude::*, pyclass::CompareOp, types::PyBytes,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    sanitize::{Sanitize, SanitizeError as SanitizeErrorOriginal},
    signature::Signature as SignatureOriginal,
    transaction::{
        get_nonce_pubkey_from_instruction, uses_durable_nonce, Transaction as TransactionOriginal,
        TransactionError as TransactionErrorOriginal,
    },
};

use crate::{
    convert_instructions, convert_optional_pubkey, handle_py_err, signer::SignerVec,
    CompiledInstruction, Instruction, Message, Pubkey, PyErrWrapper, RichcmpEqualityOnly,
    Signature, Signer, SolderHash,
};

create_exception!(solders, TransactionError, PyException);

impl From<TransactionErrorOriginal> for PyErrWrapper {
    fn from(e: TransactionErrorOriginal) -> Self {
        Self(TransactionError::new_err(e.to_string()))
    }
}

create_exception!(solders, SanitizeError, PyException);

impl From<SanitizeErrorOriginal> for PyErrWrapper {
    fn from(e: SanitizeErrorOriginal) -> Self {
        Self(SanitizeError::new_err(e.to_string()))
    }
}

#[pyclass(module = "solders", subclass)]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
/// An atomically-commited sequence of instructions.
///
/// While :class:`~solders.instruction.Instruction`s are the basic unit of computation in Solana,
/// they are submitted by clients in :class:`~solders.transaction.Transaction`s containing one or
/// more instructions, and signed by one or more signers.
///
///
/// See the `Rust module documentation <https://docs.rs/solana-sdk/latest/solana_sdk/transaction/index.html>`_ for more details about transactions.
///
/// Some constructors accept an optional ``payer``, the account responsible for
/// paying the cost of executing a transaction. In most cases, callers should
/// specify the payer explicitly in these constructors. In some cases though,
/// the caller is not *required* to specify the payer, but is still allowed to:
/// in the :class:`~solders.message.Message` object, the first account is always the fee-payer, so
/// if the caller has knowledge that the first account of the constructed
/// transaction's ``Message`` is both a signer and the expected fee-payer, then
/// redundantly specifying the fee-payer is not strictly required.
///
/// The main ``Transaction()`` constructor creates a fully-signed transaction from a ``Message``.
///
/// Args:
///     from_keypairs (Sequence[Keypair | Presigner]): The keypairs that are to sign the transaction.
///     message (Message): The message to sign.
///     recent_blockhash (Hash): The id of a recent ledger entry.
///
/// Example:
///     >>> from solders.message import Message
///     >>> from solders.keypair import Keypair
///     >>> from solders.instruction import Instruction
///     >>> from solders.hash import Hash
///     >>> from solders.transaction import Transaction
///     >>> from solders.pubkey import Pubkey
///     >>> program_id = Pubkey.default()
///     >>> arbitrary_instruction_data = bytes([1])
///     >>> accounts = []
///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
///     >>> payer = Keypair()
///     >>> message = Message([instruction], payer.pubkey())
///     >>> blockhash = Hash.default()  # replace with a real blockhash
///     >>> tx = Transaction([payer], message, blockhash);
///
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
    /// list[Signature]: A set of signatures of a serialized :class:`~solders.message.Message`,
    /// signed by the first keys of the message's :attr:`~solders.message.Message.account_keys`,
    /// where the number of signatures is equal to ``num_required_signatures`` of the `Message`'s
    /// :class:`~solders.message.MessageHeader`.
    pub fn signatures(&self) -> Vec<Signature> {
        self.0
            .signatures
            .clone()
            .into_iter()
            .map(Signature::from)
            .collect()
    }

    #[getter]
    /// Message: The message to sign.
    pub fn message(&self) -> Message {
        self.0.message.clone().into()
    }

    #[staticmethod]
    /// Create an unsigned transaction from a :class:`~solders.message.Message`.
    ///
    /// Args:
    ///     message (Message): The transaction's message.
    ///
    /// Returns:
    ///     Transaction: The unsigned transaction.
    ///
    /// Example:
    ///     >>> from solders.message import Message
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction, AccountMeta
    ///     >>> from solders.hash import Hash
    ///     >>> from solders.transaction import Transaction
    ///     >>> program_id = Pubkey.default()
    ///     >>> blockhash = Hash.default()  # replace with a real blockhash
    ///     >>> arbitrary_instruction_data = bytes([1])
    ///     >>> accounts: list[AccountMeta] = []
    ///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    ///     >>> payer = Keypair()
    ///     >>> message = Message.new_with_blockhash([instruction], payer.pubkey(), blockhash)
    ///     >>> tx = Transaction.new_unsigned(message)
    ///     >>> tx.sign([payer], tx.message.recent_blockhash)
    ///
    pub fn new_unsigned(message: Message) -> Self {
        TransactionOriginal::new_unsigned(message.into()).into()
    }

    #[staticmethod]
    /// Create an unsigned transaction from a list of :class:`~solders.instruction.Instruction`s.
    ///
    /// Args:
    ///    instructions (Sequence[Instruction]): The instructions to include in the transaction message.
    ///    payer (Optional[Pubkey], optional): The transaction fee payer. Defaults to None.
    ///
    /// Returns:
    ///     Transaction: The unsigned transaction.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.instruction import Instruction
    ///     >>> from solders.transaction import Transaction
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> program_id = Pubkey.default()
    ///     >>> arbitrary_instruction_data = bytes([1])
    ///     >>> accounts = []
    ///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    ///     >>> payer = Keypair()
    ///     >>> tx = Transaction.new_with_payer([instruction], payer)
    ///
    pub fn new_with_payer(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        TransactionOriginal::new_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer),
        )
        .into()
    }

    #[staticmethod]
    /// Create a fully-signed transaction from a list of :class:`~solders.instruction.Instruction`s.
    ///
    /// Args:
    ///    instructions (Sequence[Instruction]): The instructions to include in the transaction message.
    ///    payer (Optional[Pubkey], optional): The transaction fee payer.
    ///    signing_keypairs (Sequence[Keypair | Presigner]): The keypairs that will sign the transaction.
    ///    recent_blockhash (Hash): The id of a recent ledger entry.
    ///    
    /// Returns:
    ///     Transaction: The signed transaction.
    ///
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.instruction import Instruction
    ///     >>> from solders.transaction import Transaction
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> program_id = Pubkey.default()
    ///     >>> arbitrary_instruction_data = bytes([1])
    ///     >>> accounts = []
    ///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    ///     >>> payer = Keypair()
    ///     >>> blockhash = Hash.default()  # replace with a real blockhash
    ///     >>> tx = Transaction.new_signed_with_payer([instruction], payer.pubkey(), [payer], blockhash);
    ///
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
    /// Create a fully-signed transaction from pre-compiled instructions.
    ///
    /// Args:
    ///     from_keypairs (Sequence[Keypair | Presigner]): The keys used to sign the transaction.
    ///     keys (Sequence[Pubkey]): The keys for the transaction.  These are the program state
    ///         instances or lamport recipient keys.
    ///     recent_blockhash (Hash): The PoH hash.
    ///     program_ids (Sequence[Pubkey]): The keys that identify programs used in the `instruction` vector.
    ///     instructions (Sequence[Instruction]): Instructions that will be executed atomically.
    ///
    /// Returns:
    ///     Transaction: The signed transaction.
    ///
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
        handle_py_err(
            self.0
                .try_sign(&SignerVec(keypairs), recent_blockhash.into()),
        )
    }

    pub fn partial_sign(
        &mut self,
        keypairs: Vec<Signer>,
        recent_blockhash: SolderHash,
    ) -> PyResult<()> {
        handle_py_err(
            self.0
                .try_partial_sign(&SignerVec(keypairs), recent_blockhash.into()),
        )
    }

    pub fn verify(&self) -> PyResult<()> {
        handle_py_err(self.0.verify())
    }

    pub fn verify_and_hash_message(&self) -> PyResult<SolderHash> {
        handle_py_err(self.0.verify_and_hash_message())
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
        handle_py_err(self.0.get_signing_keypair_positions(&converted_pubkeys))
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
        handle_py_err(self.0.replace_signatures(&converted_signers))
    }

    pub fn is_signed(&self) -> bool {
        self.0.is_signed()
    }

    pub fn uses_durable_nonce(&self) -> Option<CompiledInstruction> {
        uses_durable_nonce(&self.0).map(|x| CompiledInstruction::from(x.clone()))
    }

    pub fn sanitize(&self) -> PyResult<()> {
        handle_py_err(self.0.sanitize())
    }

    pub fn __bytes__<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let as_vec: Vec<u8> = handle_py_err(bincode::serialize(&self.0))?;
        Ok(PyBytes::new(py, &as_vec))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        handle_py_err(bincode::deserialize::<TransactionOriginal>(data))
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
