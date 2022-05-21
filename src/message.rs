use pyo3::{prelude::*, pyclass::CompareOp, types::PyBytes};
use solana_sdk::{
    instruction::CompiledInstruction as CompiledInstructionOriginal,
    message::{
        legacy::Message as MessageOriginal, MessageHeader as MessageHeaderOriginal,
        MESSAGE_HEADER_LENGTH,
    },
    pubkey::Pubkey as PubkeyOriginal,
};

use crate::{
    convert_instructions, convert_optional_pubkey, handle_py_err, CompiledInstruction, Instruction,
    Pubkey, RichcmpEqualityOnly, SolderHash,
};

#[pyclass(module = "solders", subclass)]
#[derive(PartialEq, Eq, Debug, Default)]
/// Describes the organization of a :class:`Message`'s account keys.
///
/// Every :class:`~solders.instruction.Instruction` specifies which accounts it may reference, or
/// otherwise requires specific permissions of. Those specifications are:
/// whether the account is read-only, or read-write; and whether the account
/// must have signed the transaction containing the instruction.
///
/// Whereas an individual ``Instruction`` contains a list of all accounts they may
/// access, along with their required permissions, a ``Message`` contains a
/// single shared flat list of *all* accounts required by *all* instructions in
/// a transaction. When building a ``Message``, this flat list is created and
/// each ``Instruction`` is converted to :class:`~solders.instruction.CompiledInstruction`. Each
/// ``CompiledInstruction`` then references by index the accounts they require in
/// the single shared account list.
///
/// The shared account list is ordered by the permissions required of the accounts:
///
/// * accounts that are writable and signers
/// * accounts that are read-only and signers
/// * accounts that are writable and not signers
/// * accounts that are read-only and not signers
///
/// Given this ordering, the fields of ``MessageHeader`` describe which accounts
/// in a transaction require which permissions.
///
/// When multiple transactions access the same read-only accounts, the runtime
/// may process them in parallel, in a single
/// `PoH <https://docs.solana.com/cluster/synchronization>`_ entry.
/// Transactions that access the same read-write accounts are processed sequentially.
///
/// Args:
///     num_required_signatures (int): The number of signatures required for this message
///         to be considered valid. The signers of those signatures must match the
///         first ``num_required_signatures`` of :attr:`Message.account_keys`.
///     num_readonly_signed_accounts (int): The last ``num_readonly_signed_accounts`` of
///         the signed keys are read-only accounts.
///     num_readonly_unsigned_accounts (int): The last ``num_readonly_unsigned_accounts``
///         of the unsigned keys are read-only accounts.
pub struct MessageHeader(MessageHeaderOriginal);

#[pymethods]
impl MessageHeader {
    #[classattr]
    const LENGTH: usize = MESSAGE_HEADER_LENGTH;

    #[new]
    pub fn new(
        num_required_signatures: u8,
        num_readonly_signed_accounts: u8,
        num_readonly_unsigned_accounts: u8,
    ) -> Self {
        MessageHeaderOriginal {
            num_required_signatures,
            num_readonly_signed_accounts,
            num_readonly_unsigned_accounts,
        }
        .into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default ``MessageHeader``.
    ///
    /// Returns:
    ///     MessageHeader: default ``MessageHeader``.
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn num_required_signatures(&self) -> u8 {
        self.0.num_required_signatures
    }

    #[getter]
    pub fn num_readonly_signed_accounts(&self) -> u8 {
        self.0.num_readonly_signed_accounts
    }

    #[getter]
    pub fn num_readonly_unsigned_accounts(&self) -> u8 {
        self.0.num_readonly_unsigned_accounts
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<MessageHeaderOriginal> for MessageHeader {
    fn from(h: MessageHeaderOriginal) -> Self {
        Self(h)
    }
}

#[pyclass(module = "solders", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Message(MessageOriginal);

#[pymethods]
impl Message {
    #[new]
    pub fn new(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new(&instructions_inner, convert_optional_pubkey(payer)).into()
    }

    #[getter]
    pub fn header(&self) -> MessageHeader {
        self.0.header.into()
    }

    #[getter]
    pub fn account_keys(&self) -> Vec<Pubkey> {
        self.0
            .account_keys
            .clone()
            .into_iter()
            .map(Pubkey::from)
            .collect()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> SolderHash {
        self.0.recent_blockhash.into()
    }

    #[getter]
    pub fn instructions(&self) -> Vec<CompiledInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(CompiledInstruction::from)
            .collect()
    }

    #[staticmethod]
    pub fn new_with_blockhash(
        instructions: Vec<Instruction>,
        payer: Option<&Pubkey>,
        blockhash: &SolderHash,
    ) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new_with_blockhash(
            &instructions_inner,
            convert_optional_pubkey(payer),
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
        let instructions_inner: Vec<CompiledInstructionOriginal> = instructions
            .into_iter()
            .map(CompiledInstructionOriginal::from)
            .collect();
        let account_keys_inner: Vec<PubkeyOriginal> =
            account_keys.into_iter().map(PubkeyOriginal::from).collect();
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

    pub fn hash(&self) -> SolderHash {
        self.0.hash().into()
    }

    #[staticmethod]
    pub fn hash_raw_message(message_bytes: &[u8]) -> SolderHash {
        MessageOriginal::hash_raw_message(message_bytes).into()
    }

    pub fn compile_instruction(&self, ix: &Instruction) -> CompiledInstruction {
        self.0.compile_instruction(ix.as_ref()).into()
    }

    pub fn __bytes__<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.serialize())
    }

    pub fn program_id(&self, instruction_index: usize) -> Option<Pubkey> {
        self.0.program_id(instruction_index).map(Pubkey::from)
    }

    pub fn program_index(&self, instruction_index: usize) -> Option<usize> {
        self.0.program_index(instruction_index)
    }

    pub fn program_ids(&self) -> Vec<Pubkey> {
        self.0.program_ids().into_iter().map(Pubkey::from).collect()
    }

    pub fn is_key_passed_to_program(&self, key_index: usize) -> bool {
        self.0.is_key_passed_to_program(key_index)
    }

    pub fn is_key_called_as_program(&self, key_index: usize) -> bool {
        self.0.is_key_called_as_program(key_index)
    }

    pub fn is_non_loader_key(&self, key_index: usize) -> bool {
        self.0.is_non_loader_key(key_index)
    }

    pub fn program_position(&self, index: usize) -> Option<usize> {
        self.0.program_position(index)
    }

    pub fn maybe_executable(&self, i: usize) -> bool {
        self.0.maybe_executable(i)
    }

    pub fn is_writable(&self, i: usize) -> bool {
        self.0.is_writable(i)
    }

    pub fn is_signer(&self, i: usize) -> bool {
        self.0.is_signer(i)
    }

    pub fn signer_keys(&self) -> Vec<Pubkey> {
        self.0.signer_keys().into_iter().map(Pubkey::from).collect()
    }

    pub fn has_duplicates(&self) -> bool {
        self.0.has_duplicates()
    }

    pub fn is_upgradeable_loader_present(&self) -> bool {
        self.0.is_upgradeable_loader_present()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        handle_py_err(bincode::deserialize::<MessageOriginal>(data))
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
}

impl RichcmpEqualityOnly for Message {}

impl From<MessageOriginal> for Message {
    fn from(message: MessageOriginal) -> Self {
        Self(message)
    }
}

impl From<Message> for MessageOriginal {
    fn from(message: Message) -> Self {
        message.0
    }
}

impl From<&Message> for MessageOriginal {
    fn from(message: &Message) -> Self {
        message.0.clone()
    }
}
