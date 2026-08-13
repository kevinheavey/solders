use derive_more::{From, Into};
use pyo3::{create_exception, exceptions::PyException, prelude::*};
use serde::{Deserialize, Serialize};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits::{handle_py_err, PyErrWrapper};
use solders_traits_core::{
    handle_py_value_err, impl_display, py_from_bytes_general_via_bincode,
    pybytes_general_via_bincode, CommonMethodsCore, PyBytesGeneral, PyFromBytesGeneral,
    RichcmpEqualityOnly,
};
use {
    solana_instruction::Instruction as InstructionOriginal,
    solana_message::{
        compiled_instruction::CompiledInstruction as CompiledInstructionOriginal,
        legacy::Message as MessageOriginal,
        v0::{
            Message as MessageV0Original,
            MessageAddressTableLookup as MessageAddressTableLookupOriginal,
        },
        v1::{
            Message as MessageV1Original, TransactionConfig as TransactionConfigOriginal,
            DEFAULT_HEAP_SIZE, FIXED_HEADER_SIZE, MAX_ADDRESSES, MAX_HEAP_SIZE, MAX_INSTRUCTIONS,
            MAX_SIGNATURES, MAX_TRANSACTION_SIZE, MIN_HEAP_SIZE, SIGNATURE_SIZE, V1_PREFIX,
        },
        AddressLookupTableAccount as AddressLookupTableAccountOriginal,
        MessageHeader as MessageHeaderOriginal, VersionedMessage as VersionedMessageOriginal,
        MESSAGE_HEADER_LENGTH,
    },
    solana_pubkey::Pubkey as PubkeyOriginal,
};

/// Concrete type for the `None` case of the generic reserved-addresses argument.
type NoReservedAddresses = std::collections::HashSet<PubkeyOriginal>;

use solders_address_lookup_table_account::AddressLookupTableAccount;
use solders_hash::Hash as SolderHash;
use solders_instruction::{convert_instructions, CompiledInstruction, Instruction};
use solders_pubkey::{convert_optional_pubkey, Pubkey};

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone, From, Into)]
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

#[richcmp_eq_only]
#[common_methods]
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

    #[staticmethod]
    /// Deserialize a serialized ``MessageHeader`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``MessageHeader``.
    ///
    /// Returns:
    ///     MessageHeader: The deserialized ``MessageHeader``.
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}

impl RichcmpEqualityOnly for MessageHeader {}
pybytes_general_via_bincode!(MessageHeader);
impl_display!(MessageHeader);
py_from_bytes_general_via_bincode!(MessageHeader);
solders_traits_core::common_methods_default!(MessageHeader);

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, From, Into)]
/// A Solana transaction message.
///
/// Some constructors accept an optional `payer`, the account responsible for
/// paying the cost of executing a transaction. In most cases, callers should
/// specify the payer explicitly in these constructors. In some cases though,
/// the caller is not *required* to specify the payer, but is still allowed to:
/// in the ``Message`` object, the first account is always the fee-payer, so if
/// the caller has knowledge that the first account of the constructed
/// transaction's ``Message`` is both a signer and the expected fee-payer, then
/// redundantly specifying the fee-payer is not strictly required.
///
/// Args:
///     instructions (Sequence[Instruction]): The instructions to include in the message.
///     payer (Optional[Pubkey]): The fee payer. Defaults to ``None``.
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
///     >>> tx = Transaction([payer], message, blockhash)
///
pub struct Message(pub MessageOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Message {
    #[pyo3(signature = (instructions, payer=None))]
    #[new]
    pub fn new(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new(&instructions_inner, convert_optional_pubkey(payer)).into()
    }

    #[getter]
    /// MessageHeader: The message header, identifying signed and read-only ``account_keys``.
    pub fn header(&self) -> MessageHeader {
        self.0.header.into()
    }

    #[getter]
    /// list[Pubkey]: All the account keys used by this transaction.
    pub fn account_keys(&self) -> Vec<Pubkey> {
        self.0
            .account_keys
            .clone()
            .into_iter()
            .map(Pubkey::from)
            .collect()
    }

    #[getter]
    /// Hash: The id of a recent ledger entry.
    pub fn recent_blockhash(&self) -> SolderHash {
        self.0.recent_blockhash.into()
    }

    #[getter]
    /// list[CompiledInstruction]: Programs that will be executed in sequence
    /// and committed in one atomic transaction if all succeed.
    pub fn instructions(&self) -> Vec<CompiledInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(CompiledInstruction::from)
            .collect()
    }

    #[pyo3(
        signature = (instructions, payer, blockhash)
    )]
    #[staticmethod]
    /// Create a new message while setting the blockhash.
    ///
    /// Args:
    ///     instructions (Sequence[Instruction]): The instructions to include in the message.
    ///     payer (Optional[Pubkey]): The fee payer. Defaults to ``None``.
    ///     blockhash (Hash): a recent blockhash.
    ///
    /// Returns:
    ///     Message: The message object.
    ///
    /// Example:
    ///     >>> from typing import List
    ///     >>> from solders.message import Message
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction, AccountMeta
    ///     >>> from solders.hash import Hash
    ///     >>> from solders.transaction import Transaction
    ///     >>> program_id = Pubkey.default()
    ///     >>> blockhash = Hash.default()  # replace with a real blockhash
    ///     >>> arbitrary_instruction_data = bytes([1])
    ///     >>> accounts: List[AccountMeta] = []
    ///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    ///     >>> payer = Keypair()
    ///     >>> message = Message.new_with_blockhash([instruction], payer.pubkey(), blockhash)
    ///     >>> tx = Transaction.new_unsigned(message)
    ///     >>> tx.sign([payer], tx.message.recent_blockhash)
    ///
    pub fn new_with_blockhash(
        instructions: Vec<Instruction>,
        payer: Option<Pubkey>,
        blockhash: &SolderHash,
    ) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new_with_blockhash(
            &instructions_inner,
            convert_optional_pubkey(payer.as_ref()),
            blockhash.as_ref(),
        )
        .into()
    }

    #[pyo3(
        signature = (instructions, payer, nonce_account_pubkey, nonce_authority_pubkey)
    )]
    #[staticmethod]
    /// Create a new message for a `nonced transaction <https://docs.solana.com/implemented-proposals/durable-tx-nonces>`_.
    ///
    /// Args:
    ///     instructions (Sequence[Instruction]): The instructions to include in the message.
    ///     payer (Optional[Pubkey]): The fee payer. Defaults to ``None``.
    ///     nonce_account_pubkey (Pubkey): The nonce account pubkey.
    ///     nonce_authority_pubkey (Pubkey): The nonce account authority (for advance and close).
    ///
    /// In this type of transaction, the blockhash is replaced with a *durable
    /// transaction nonce*, allowing for extended time to pass between the
    /// transaction's signing and submission to the blockchain.
    ///
    /// Example:
    ///     >>> from typing import List
    ///     >>> from solders.message import Message
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction, AccountMeta
    ///     >>> from solders.hash import Hash
    ///     >>> from solders.transaction import Transaction
    ///     >>> program_id = Pubkey.default()
    ///     >>> blockhash = Hash.default()  # replace with a real blockhash
    ///     >>> arbitrary_instruction_data = bytes([1])
    ///     >>> accounts: List[AccountMeta] = []
    ///     >>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
    ///     >>> payer = Keypair()
    ///     >>> nonce_account = Pubkey.default()  # replace with a real nonce account
    ///     >>> message = Message.new_with_nonce([instruction], payer.pubkey(), nonce_account, payer.pubkey())
    ///     >>> # This transaction will need to be signed later, using the blockhash stored in the nonce account.
    ///     >>> tx = Transaction.new_unsigned(message)
    ///     
    pub fn new_with_nonce(
        instructions: Vec<Instruction>,
        payer: Option<Pubkey>,
        nonce_account_pubkey: &Pubkey,
        nonce_authority_pubkey: &Pubkey,
    ) -> Self {
        let instructions_inner = convert_instructions(instructions);
        MessageOriginal::new_with_nonce(
            instructions_inner,
            convert_optional_pubkey(payer.as_ref()),
            nonce_account_pubkey.as_ref(),
            nonce_authority_pubkey.as_ref(),
        )
        .into()
    }

    #[staticmethod]
    /// Create a new message by specifying all the fields required for the message, including the :class:`MessageHeader` fields.
    ///
    /// Args:
    ///     num_required_signatures (int): The number of signatures required for this message
    ///         to be considered valid. The signers of those signatures must match the
    ///         first ``num_required_signatures`` of :attr:`Message.account_keys`.
    ///     num_readonly_signed_accounts (int): The last ``num_readonly_signed_accounts`` of
    ///         the signed keys are read-only accounts.
    ///     num_readonly_unsigned_accounts (int): The last ``num_readonly_unsigned_accounts``
    ///         of the unsigned keys are read-only accounts.
    ///     account_keys (list[Pubkey]): All the account keys used by this transaction.
    ///     recent_blockhash (Hash): The id of a recent ledger entry.
    ///     instructions (list[CompiledInstruction]): Programs that will be executed in sequence
    ///         and committed in one atomic transaction if all succeed.
    ///
    /// Returns:
    ///     Message: The message object.
    ///
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

    /// Compute the blake3 hash of this transaction's message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash(&self) -> SolderHash {
        self.0.hash().into()
    }

    #[staticmethod]
    /// Compute the blake3 hash of a raw transaction message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash_raw_message(message_bytes: &[u8]) -> SolderHash {
        MessageOriginal::hash_raw_message(message_bytes).into()
    }

    /// Convert an :class:`~solders.Instruction` into a :class:`~solders.instruction.CompiledInstruction` using ``self.account_keys``.
    ///
    /// Returns:
    ///     CompiledInstruction: The compiled instruction.
    pub fn compile_instruction(&self, ix: &Instruction) -> CompiledInstruction {
        self.0.compile_instruction(ix.as_ref()).into()
    }

    /// Return the program ID of an instruction at a particular index in the message.
    ///
    /// Args:
    ///     instruction_index (int): The position of the instruction in the
    ///         message's list of instructions.
    ///
    /// Returns:
    ///     Pubkey: The program ID.
    ///
    pub fn program_id(&self, instruction_index: usize) -> Option<Pubkey> {
        self.0.program_id(instruction_index).map(Pubkey::from)
    }

    /// Return the ``program_id_index`` of the instruction at ``instruction_index`` in the message.
    ///
    /// Args:
    ///     instruction_index (int): The position of the instruction in the
    ///         message's list of instructions.
    ///
    /// Returns:
    ///     int: The program ID index.
    ///
    pub fn program_index(&self, instruction_index: usize) -> Option<usize> {
        self.0.program_index(instruction_index)
    }

    /// Return the program ID of each instruction in the message.
    ///
    /// Returns:
    ///     list[Pubkey]: The program IDs.
    ///
    pub fn program_ids(&self) -> Vec<Pubkey> {
        self.0.program_ids().into_iter().map(Pubkey::from).collect()
    }

    /// Check if the ``program_id_index`` of any of the message's instructions matches ``key_index``.
    ///
    /// Args:
    ///     key_index (int): The index to check.
    ///
    /// Returns:
    ///     bool: The result of the check.
    ///
    pub fn is_key_called_as_program(&self, key_index: usize) -> bool {
        self.0.is_key_called_as_program(key_index)
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.program_position
    pub fn program_position(&self, index: usize) -> Option<usize> {
        self.0.program_position(index)
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.maybe_executable
    pub fn maybe_executable(&self, i: usize) -> bool {
        self.0.maybe_executable(i)
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.is_signer
    pub fn is_signer(&self, i: usize) -> bool {
        self.0.is_signer(i)
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.signer_keys
    pub fn signer_keys(&self) -> Vec<Pubkey> {
        self.0.signer_keys().into_iter().map(Pubkey::from).collect()
    }

    /// Check if ``account_keys`` has any duplicate keys.
    ///
    /// Returns:
    ///     bool: ``True`` if there are duplicates.
    ///
    pub fn has_duplicates(&self) -> bool {
        self.0.has_duplicates()
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.is_upgradeable_loader_present
    pub fn is_upgradeable_loader_present(&self) -> bool {
        self.0.is_upgradeable_loader_present()
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.is_instruction_account
    pub fn is_instruction_account(&self, key_index: usize) -> bool {
        self.0.is_instruction_account(key_index)
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/message/legacy/struct.Message.html#method.demote_program_id
    pub fn demote_program_id(&self, i: usize) -> bool {
        self.0.demote_program_id(i)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default ``Message``.
    ///
    /// Returns:
    ///     Message: default ``Message``.
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    /// Deserialize a serialized ``Message`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``Message``.
    ///
    /// Returns:
    ///     Message: The deserialized ``Message``.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import AccountMeta, Instruction
    ///     >>> from solders.message import Message
    ///     >>> from_pubkey = Pubkey.new_unique()
    ///     >>> to_pubkey = Pubkey.new_unique()
    ///     >>> program_id = Pubkey.new_unique()
    ///     >>> instruction_data = bytes([1])
    ///     >>> accounts = [AccountMeta(from_pubkey, is_signer=True, is_writable=True), AccountMeta(to_pubkey, is_signer=True, is_writable=True)]
    ///     >>> instruction = Instruction(program_id, instruction_data, accounts)
    ///     >>> message = Message([instruction])
    ///     >>> serialized = bytes(message)
    ///     >>> assert Message.from_bytes(serialized) == message
    ///
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}

impl RichcmpEqualityOnly for Message {}
impl PyBytesGeneral for Message {
    fn pybytes_general(&self) -> Vec<u8> {
        self.0.serialize().clone()
    }
}
impl_display!(Message);
py_from_bytes_general_via_bincode!(Message);
solders_traits_core::common_methods_default!(Message);

impl From<&Message> for MessageOriginal {
    fn from(message: &Message) -> Self {
        message.0.clone()
    }
}

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, From, Into)]
/// Address table lookups describe an on-chain address lookup table to use
/// for loading more readonly and writable accounts in a single tx.
///
/// Args:
///     account_key (Pubkey): Address lookup table account key.
///     writable_indexes (bytes): List of u8 indexes used to load writable account addresses, represented as bytes.
///     readonly_indexes (bytes): List of u8 indexes used to load readonly account addresses, represented as bytes.
///
pub struct MessageAddressTableLookup(pub MessageAddressTableLookupOriginal);

impl RichcmpEqualityOnly for MessageAddressTableLookup {}
pybytes_general_via_bincode!(MessageAddressTableLookup);
impl_display!(MessageAddressTableLookup);
py_from_bytes_general_via_bincode!(MessageAddressTableLookup);
solders_traits_core::common_methods_default!(MessageAddressTableLookup);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl MessageAddressTableLookup {
    #[new]
    pub fn new(account_key: Pubkey, writable_indexes: Vec<u8>, readonly_indexes: Vec<u8>) -> Self {
        MessageAddressTableLookupOriginal {
            account_key: account_key.into(),
            writable_indexes,
            readonly_indexes,
        }
        .into()
    }

    /// Pubkey: Address lookup table account key.
    #[getter]
    pub fn account_key(&self) -> Pubkey {
        self.0.account_key.into()
    }

    /// bytes: List of u8 indexes used to load writable account addresses, represented as bytes.
    #[getter]
    pub fn writable_indexes(&self) -> Vec<u8> {
        self.0.writable_indexes.clone()
    }

    /// bytes: List of u8 indexes used to load readonly account addresses, represented as bytes.
    #[getter]
    pub fn readonly_indexes(&self) -> Vec<u8> {
        self.0.readonly_indexes.clone()
    }
}

create_exception!(
    solders,
    CompileError,
    PyException,
    "Raised when an error is encountered in compiling a message."
);

create_exception!(
    solders,
    MessageError,
    PyException,
    "Umbrella error for ``solders.message.v1.Message`` validation."
);

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, From, Into)]
/// A Solana transaction message (v0).
///
/// This message format supports succinct account loading with
/// on-chain address lookup tables
///
/// Args:
///     header (MessageHeader): The message header, identifying signed and read-only `account_keys`.
///         Header values only describe static `account_keys`, they do not describe
///         any additional account keys loaded via address table lookups.
///     account_keys (Sequence[Pubkey]): List of accounts loaded by this transaction.
///     recent_blockhash (Hash): Hash of a recent block.
///     instructions (Sequence[Instruction]): The instructions to include in the message.
///     address_table_lookups (Sequence[MessageAddressTableLookup]): List of address table lookups used to load additional accounts
///         for this transaction.
///
/// Example:
///     >>> from solders.message import MessageV0, MessageHeader, MessageAddressTableLookup
///     >>> from solders.instruction import CompiledInstruction
///     >>> from solders.hash import Hash
///     >>> from solders.pubkey import Pubkey
///     >>> program_id = Pubkey.default()
///     >>> arbitrary_instruction_data = bytes([1])
///     >>> accounts = []
///     >>> instructions=[CompiledInstruction(program_id_index=4, accounts=bytes([1, 2, 3, 5, 6]), data=bytes([]))]
///     >>> account_keys = [Pubkey.new_unique()]
///     >>> header = MessageHeader(1, 0, 0)
///     >>> lookups = [MessageAddressTableLookup(Pubkey.new_unique(), bytes([1, 2, 3]), bytes([0]))]
///     >>> blockhash = Hash.default()  # replace with a real blockhash
///     >>> message = MessageV0(header, account_keys, blockhash, instructions, lookups)
///
pub struct MessageV0(pub MessageV0Original);

impl RichcmpEqualityOnly for MessageV0 {}
pybytes_general_via_bincode!(MessageV0);
impl_display!(MessageV0);
py_from_bytes_general_via_bincode!(MessageV0);
solders_traits_core::common_methods_default!(MessageV0);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl MessageV0 {
    #[new]
    pub fn new(
        header: MessageHeader,
        account_keys: Vec<Pubkey>,
        recent_blockhash: SolderHash,
        instructions: Vec<CompiledInstruction>,
        address_table_lookups: Vec<MessageAddressTableLookup>,
    ) -> Self {
        MessageV0Original {
            header: header.into(),
            account_keys: account_keys.into_iter().map(|p| p.into()).collect(),
            recent_blockhash: recent_blockhash.into(),
            instructions: instructions.into_iter().map(|ix| ix.into()).collect(),
            address_table_lookups: address_table_lookups
                .into_iter()
                .map(|a| a.into())
                .collect(),
        }
        .into()
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
            .map(|p| p.into())
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
            .map(|p| p.into())
            .collect()
    }

    #[getter]
    pub fn address_table_lookups(&self) -> Vec<MessageAddressTableLookup> {
        self.0
            .address_table_lookups
            .clone()
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    /// Create a signable transaction message from a ``payer`` public key, ``recent_blockhash``,
    /// list of ``instructions``, and a list of ``address_lookup_table_accounts``.
    ///
    /// Args:
    ///     payer (Pubkey): The fee payer.
    ///     instructions (Sequence[Instruction]): The instructions to include in the message.
    ///     address_table_lookups (Sequence[MessageAddressTableLookup]): List of address table lookups used to load additional accounts
    ///         for this transaction.
    ///     recent_blockhash (Hash): Hash of a recent block.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction, AccountMeta
    ///     >>> from solders.message import Message
    ///     >>> from solders.address_lookup_table_account import AddressLookupTableAccount
    ///     >>> from solders.hash import Hash
    ///     >>> keys = [Pubkey.new_unique() for i in range(7)]
    ///     >>> payer = keys[0]
    ///     >>> program_id = keys[6]
    ///     >>> ix_accounts = [AccountMeta(keys[1], True, True), AccountMeta(keys[2], True, False), AccountMeta(keys[3], False, True),AccountMeta(keys[4], False, True),AccountMeta(keys[5], False, False),]
    ///     >>> instructions = [Instruction(program_id, bytes([]), ix_accounts)]
    ///     >>> lookup_acc0 = AddressLookupTableAccount(key=Pubkey.new_unique(), addresses=[keys[4], keys[5], keys[6]])
    ///     >>> lookup_acc1 = AddressLookupTableAccount(key=Pubkey.new_unique(), addresses=[])
    ///     >>> lookup_accs = [lookup_acc0, lookup_acc1]
    ///     >>> recent_blockhash = Hash.new_unique()
    ///     >>> msg = MessageV0.try_compile(payer, instructions, lookup_accs, recent_blockhash)
    ///
    #[staticmethod]
    pub fn try_compile(
        payer: &Pubkey,
        instructions: Vec<Instruction>,
        address_lookup_table_accounts: Vec<AddressLookupTableAccount>,
        recent_blockhash: SolderHash,
    ) -> PyResult<Self> {
        MessageV0Original::try_compile(
            payer.as_ref(),
            &instructions
                .into_iter()
                .map(|ix| ix.into())
                .collect::<Vec<InstructionOriginal>>(),
            &address_lookup_table_accounts
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<AddressLookupTableAccountOriginal>>()[..],
            recent_blockhash.into(),
        )
        .map_or_else(
            |e| {
                Err(PyErr::from(PyErrWrapper(CompileError::new_err(
                    e.to_string(),
                ))))
            },
            |v| Ok(v.into()),
        )
    }

    /// Sanitize message fields and compiled instruction indexes.
    pub fn sanitize(&self) -> PyResult<()> {
        handle_py_err(self.0.sanitize())
    }

    /// Returns true if the account at the specified index is called as a program by an instruction
    pub fn is_key_called_as_program(&self, key_index: usize) -> bool {
        self.0.is_key_called_as_program(key_index)
    }

    /// Returns true if the account at the specified index was requested as writable.
    /// Before loading addresses, we can't demote write locks for dynamically loaded
    /// addresses so this should not be used by the runtime.
    pub fn is_maybe_writable(&self, key_index: usize) -> bool {
        self.0
            .is_maybe_writable_with_reserved_addresses(key_index, None::<&NoReservedAddresses>)
    }

    /// Returns true if the account at the specified index signed this
    /// message.
    pub fn is_signer(&self, index: usize) -> bool {
        VersionedMessageOriginal::from(self.clone()).is_signer(index)
    }

    /// Returns true if the account at the specified index is not invoked as a
    /// program or, if invoked, is passed to a program.
    pub fn is_non_loader_key(&self, key_index: usize) -> bool {
        VersionedMessageOriginal::from(self.clone()).is_non_loader_key(key_index)
    }

    /// Compute the blake3 hash of this transaction's message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash(&self) -> SolderHash {
        VersionedMessageOriginal::from(self.clone()).hash().into()
    }

    #[staticmethod]
    /// Compute the blake3 hash of a raw transaction message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash_raw_message(message_bytes: &[u8]) -> SolderHash {
        VersionedMessageOriginal::hash_raw_message(message_bytes).into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default ``MessageV0``.
    ///
    /// Returns:
    ///     MessageV0: default ``MessageV0``.
    pub fn new_default() -> Self {
        Self::default()
    }
}

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, Serialize, Deserialize, From, Into)]
/// Inline transaction configuration carried by a :class:`MessageV1`.
///
/// V1 messages carry the compute budget settings that legacy and v0 messages
/// have to express as separate ``ComputeBudget`` instructions. Every field is
/// optional; an unset field is simply omitted from the serialized message.
///
/// Args:
///     priority_fee (Optional[int]): The priority fee in micro-lamports. Defaults to ``None``.
///     compute_unit_limit (Optional[int]): The compute unit limit. Defaults to ``None``.
///     loaded_accounts_data_size_limit (Optional[int]): The loaded accounts data size limit
///         in bytes. Defaults to ``None``.
///     heap_size (Optional[int]): The requested heap size in bytes. Defaults to ``None``.
///
/// Example:
///     >>> from solders.message import TransactionConfig
///     >>> config = TransactionConfig(priority_fee=1000, compute_unit_limit=200_000)
///     >>> config.priority_fee
///     1000
///
pub struct TransactionConfig(pub TransactionConfigOriginal);

impl RichcmpEqualityOnly for TransactionConfig {}
pybytes_general_via_bincode!(TransactionConfig);
impl_display!(TransactionConfig);
py_from_bytes_general_via_bincode!(TransactionConfig);
solders_traits_core::common_methods_default!(TransactionConfig);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionConfig {
    #[new]
    #[pyo3(signature = (priority_fee=None, compute_unit_limit=None, loaded_accounts_data_size_limit=None, heap_size=None))]
    pub fn new(
        priority_fee: Option<u64>,
        compute_unit_limit: Option<u32>,
        loaded_accounts_data_size_limit: Option<u32>,
        heap_size: Option<u32>,
    ) -> Self {
        TransactionConfigOriginal {
            priority_fee,
            compute_unit_limit,
            loaded_accounts_data_size_limit,
            heap_size,
        }
        .into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default ``TransactionConfig``, with every field unset.
    ///
    /// Returns:
    ///     TransactionConfig: default ``TransactionConfig``.
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    /// Optional[int]: The priority fee in micro-lamports.
    pub fn priority_fee(&self) -> Option<u64> {
        self.0.priority_fee
    }

    #[getter]
    /// Optional[int]: The compute unit limit.
    pub fn compute_unit_limit(&self) -> Option<u32> {
        self.0.compute_unit_limit
    }

    #[getter]
    /// Optional[int]: The loaded accounts data size limit, in bytes.
    pub fn loaded_accounts_data_size_limit(&self) -> Option<u32> {
        self.0.loaded_accounts_data_size_limit
    }

    #[getter]
    /// Optional[int]: The requested heap size, in bytes.
    pub fn heap_size(&self) -> Option<u32> {
        self.0.heap_size
    }

    /// The number of bytes this config occupies in a serialized ``MessageV1``.
    ///
    /// Returns:
    ///     int: The serialized size.
    pub fn size(&self) -> usize {
        self.0.size()
    }

    #[staticmethod]
    /// Deserialize a serialized ``TransactionConfig`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``TransactionConfig``.
    ///
    /// Returns:
    ///     TransactionConfig: The deserialized ``TransactionConfig``.
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}

#[pyclass(from_py_object, module = "solders.message", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, From, Into)]
/// A Solana transaction message (v1), as specified by SIMD-0385.
///
/// This message format supports transactions up to
/// :attr:`MessageV1.MAX_TRANSACTION_SIZE` bytes and carries its compute budget
/// settings inline via :class:`TransactionConfig`, instead of requiring
/// separate compute budget instructions. Unlike :class:`MessageV0` it does not
/// support address lookup tables.
///
/// Args:
///     header (MessageHeader): The message header, identifying signed and read-only `account_keys`.
///     config (TransactionConfig): The inline transaction configuration.
///     lifetime_specifier (Hash): Hash of a recent block, determining when this transaction expires.
///     account_keys (Sequence[Pubkey]): All account addresses referenced by this message.
///     instructions (Sequence[CompiledInstruction]): The instructions to include in the message.
///
/// Example:
///     >>> from solders.message import MessageV1, MessageHeader, TransactionConfig
///     >>> from solders.instruction import CompiledInstruction
///     >>> from solders.hash import Hash
///     >>> from solders.pubkey import Pubkey
///     >>> instructions = [CompiledInstruction(program_id_index=1, accounts=bytes([0]), data=bytes([]))]
///     >>> account_keys = [Pubkey.new_unique(), Pubkey.new_unique()]
///     >>> header = MessageHeader(1, 0, 1)
///     >>> config = TransactionConfig(compute_unit_limit=200_000)
///     >>> blockhash = Hash.default()  # replace with a real blockhash
///     >>> message = MessageV1(header, config, blockhash, account_keys, instructions)
///
pub struct MessageV1(pub MessageV1Original);

impl RichcmpEqualityOnly for MessageV1 {}
impl_display!(MessageV1);
solders_traits_core::common_methods_default!(MessageV1);

impl PyBytesGeneral for MessageV1 {
    fn pybytes_general(&self) -> Vec<u8> {
        // wincode: v1 is not bincode-serializable. Omits the version prefix,
        // matching the other message types; `to_bytes_versioned` includes it.
        wincode::serialize(&self.0).unwrap()
    }
}

impl PyFromBytesGeneral for MessageV1 {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        handle_py_value_err(wincode::deserialize::<MessageV1Original>(raw))
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl MessageV1 {
    #[new]
    pub fn new(
        header: MessageHeader,
        config: TransactionConfig,
        lifetime_specifier: SolderHash,
        account_keys: Vec<Pubkey>,
        instructions: Vec<CompiledInstruction>,
    ) -> Self {
        MessageV1Original {
            header: header.into(),
            config: config.into(),
            lifetime_specifier: lifetime_specifier.into(),
            account_keys: account_keys.into_iter().map(|p| p.into()).collect(),
            instructions: instructions.into_iter().map(|ix| ix.into()).collect(),
        }
        .into()
    }

    #[getter]
    /// MessageHeader: The message header, identifying signed and read-only ``account_keys``.
    pub fn header(&self) -> MessageHeader {
        self.0.header.into()
    }

    #[getter]
    /// TransactionConfig: The inline transaction configuration.
    pub fn config(&self) -> TransactionConfig {
        self.0.config.into()
    }

    #[getter]
    /// Hash: The lifetime specifier (blockhash) that determines when this transaction expires.
    pub fn lifetime_specifier(&self) -> SolderHash {
        self.0.lifetime_specifier.into()
    }

    #[getter]
    /// Hash: Alias of :attr:`lifetime_specifier`, for parity with the other message types.
    pub fn recent_blockhash(&self) -> SolderHash {
        self.0.lifetime_specifier.into()
    }

    #[getter]
    /// list[Pubkey]: All account addresses referenced by this message.
    pub fn account_keys(&self) -> Vec<Pubkey> {
        self.0
            .account_keys
            .clone()
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    #[getter]
    /// list[CompiledInstruction]: The instructions to execute.
    pub fn instructions(&self) -> Vec<CompiledInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    #[staticmethod]
    #[pyo3(signature = (payer, instructions, recent_blockhash, config=None))]
    /// Create a signable transaction message from a ``payer`` public key, a list
    /// of ``instructions``, a ``recent_blockhash`` and an optional ``config``.
    ///
    /// Args:
    ///     payer (Pubkey): The fee payer.
    ///     instructions (Sequence[Instruction]): The instructions to include in the message.
    ///     recent_blockhash (Hash): Hash of a recent block.
    ///     config (Optional[TransactionConfig]): The inline transaction configuration.
    ///         Defaults to an empty config.
    ///
    /// Returns:
    ///     MessageV1: The message object.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction, AccountMeta
    ///     >>> from solders.message import MessageV1, TransactionConfig
    ///     >>> from solders.hash import Hash
    ///     >>> keys = [Pubkey.new_unique() for i in range(3)]
    ///     >>> payer = keys[0]
    ///     >>> program_id = keys[2]
    ///     >>> ix_accounts = [AccountMeta(keys[1], True, True)]
    ///     >>> instructions = [Instruction(program_id, bytes([]), ix_accounts)]
    ///     >>> recent_blockhash = Hash.new_unique()
    ///     >>> config = TransactionConfig(compute_unit_limit=200_000)
    ///     >>> msg = MessageV1.try_compile(payer, instructions, recent_blockhash, config)
    ///
    pub fn try_compile(
        payer: &Pubkey,
        instructions: Vec<Instruction>,
        recent_blockhash: SolderHash,
        config: Option<TransactionConfig>,
    ) -> PyResult<Self> {
        MessageV1Original::try_compile_with_config(
            payer.as_ref(),
            &instructions
                .into_iter()
                .map(|ix| ix.into())
                .collect::<Vec<InstructionOriginal>>(),
            recent_blockhash.into(),
            config.unwrap_or_default().into(),
        )
        .map_or_else(
            |e| {
                Err(PyErr::from(PyErrWrapper(CompileError::new_err(
                    e.to_string(),
                ))))
            },
            |v| Ok(v.into()),
        )
    }

    /// Pubkey: The fee payer, or ``None`` if the message has no account keys.
    pub fn fee_payer(&self) -> Option<Pubkey> {
        self.0.fee_payer().map(|p| (*p).into())
    }

    /// The serialized size of this message in bytes, excluding the version prefix.
    ///
    /// Returns:
    ///     int: The serialized size.
    pub fn size(&self) -> usize {
        self.0.size()
    }

    /// Sanitize message fields and compiled instruction indexes.
    pub fn sanitize(&self) -> PyResult<()> {
        handle_py_err(VersionedMessageOriginal::from(self.clone()).sanitize())
    }

    /// Check that this message satisfies the v1 format limits.
    ///
    /// Raises:
    ///     MessageError: if the message is invalid.
    pub fn validate(&self) -> PyResult<()> {
        self.0
            .validate()
            .map_err(|e| PyErr::from(PyErrWrapper(MessageError::new_err(e.to_string()))))
    }

    /// Returns true if the account at the specified index is called as a program by an instruction.
    pub fn is_key_called_as_program(&self, key_index: usize) -> bool {
        self.0.is_key_called_as_program(key_index)
    }

    /// Returns true if the account at the specified index was requested as writable.
    pub fn is_maybe_writable(&self, key_index: usize) -> bool {
        self.0
            .is_maybe_writable_with_reserved_addresses(key_index, None::<&NoReservedAddresses>)
    }

    /// Returns true if the account at the specified index signed this message.
    pub fn is_signer(&self, index: usize) -> bool {
        self.0.is_signer(index)
    }

    /// Returns true if the account at the specified index is a writable signer.
    pub fn is_signer_writable(&self, index: usize) -> bool {
        self.0.is_signer_writable(index)
    }

    /// Returns true if the account at the specified index is not invoked as a
    /// program or, if invoked, is passed to a program.
    pub fn is_non_loader_key(&self, key_index: usize) -> bool {
        VersionedMessageOriginal::from(self.clone()).is_non_loader_key(key_index)
    }

    /// See https://docs.rs/solana-message/latest/solana_message/v1/struct.Message.html#method.is_upgradeable_loader_present
    pub fn is_upgradeable_loader_present(&self) -> bool {
        self.0.is_upgradeable_loader_present()
    }

    /// See https://docs.rs/solana-message/latest/solana_message/v1/struct.Message.html#method.demote_program_id
    pub fn demote_program_id(&self, i: usize) -> bool {
        self.0.demote_program_id(i)
    }

    /// Compute the blake3 hash of this transaction's message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash(&self) -> SolderHash {
        VersionedMessageOriginal::from(self.clone()).hash().into()
    }

    #[staticmethod]
    /// Compute the blake3 hash of a raw transaction message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash.
    pub fn hash_raw_message(message_bytes: &[u8]) -> SolderHash {
        VersionedMessageOriginal::hash_raw_message(message_bytes).into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default ``MessageV1``.
    ///
    /// Returns:
    ///     MessageV1: default ``MessageV1``.
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    /// Deserialize a serialized ``MessageV1`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``MessageV1``, without the version prefix byte.
    ///
    /// Returns:
    ///     MessageV1: The deserialized ``MessageV1``.
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromPyObject, IntoPyObject)]
#[serde(from = "VersionedMessageOriginal", into = "VersionedMessageOriginal")]
pub enum VersionedMessage {
    Legacy(Message),
    V0(MessageV0),
    V1(MessageV1),
}

impl From<VersionedMessageOriginal> for VersionedMessage {
    fn from(v: VersionedMessageOriginal) -> Self {
        match v {
            VersionedMessageOriginal::Legacy(m) => Self::Legacy(m.into()),
            VersionedMessageOriginal::V0(m) => Self::V0(m.into()),
            VersionedMessageOriginal::V1(m) => Self::V1(m.into()),
        }
    }
}

impl From<VersionedMessage> for VersionedMessageOriginal {
    fn from(v: VersionedMessage) -> Self {
        match v {
            VersionedMessage::Legacy(m) => Self::Legacy(m.into()),
            VersionedMessage::V0(m) => Self::V0(m.into()),
            VersionedMessage::V1(m) => Self::V1(m.into()),
        }
    }
}

impl From<MessageV0> for VersionedMessage {
    fn from(m: MessageV0) -> Self {
        Self::V0(m)
    }
}

impl From<VersionedMessage> for MessageV0 {
    fn from(v: VersionedMessage) -> Self {
        match v {
            VersionedMessage::V0(m) => m,
            _ => unreachable!(),
        }
    }
}

impl From<MessageV0> for VersionedMessageOriginal {
    fn from(m: MessageV0) -> Self {
        Self::V0(m.into())
    }
}

impl From<VersionedMessageOriginal> for MessageV0 {
    fn from(v: VersionedMessageOriginal) -> Self {
        match v {
            VersionedMessageOriginal::V0(m) => m.into(),
            _ => unreachable!(),
        }
    }
}

impl From<MessageV0Original> for VersionedMessage {
    fn from(m: MessageV0Original) -> Self {
        Self::V0(m.into())
    }
}

impl From<VersionedMessage> for MessageV0Original {
    fn from(v: VersionedMessage) -> Self {
        match v {
            VersionedMessage::V0(m) => m.into(),
            _ => unreachable!(),
        }
    }
}

impl From<MessageV1> for VersionedMessage {
    fn from(m: MessageV1) -> Self {
        Self::V1(m)
    }
}

impl From<VersionedMessage> for MessageV1 {
    fn from(v: VersionedMessage) -> Self {
        match v {
            VersionedMessage::V1(m) => m,
            _ => unreachable!(),
        }
    }
}

impl From<MessageV1> for VersionedMessageOriginal {
    fn from(m: MessageV1) -> Self {
        Self::V1(m.into())
    }
}

impl From<VersionedMessageOriginal> for MessageV1 {
    fn from(v: VersionedMessageOriginal) -> Self {
        match v {
            VersionedMessageOriginal::V1(m) => m.into(),
            _ => unreachable!(),
        }
    }
}

impl From<MessageV1Original> for VersionedMessage {
    fn from(m: MessageV1Original) -> Self {
        Self::V1(m.into())
    }
}

impl From<VersionedMessage> for MessageV1Original {
    fn from(v: VersionedMessage) -> Self {
        match v {
            VersionedMessage::V1(m) => m.into(),
            _ => unreachable!(),
        }
    }
}

/// Serialize a versioned message, with a leading byte indicating whether or not it's a legacy message.
///
/// If you want to serialize without the leading byte, use `bytes(msg)`.
///
/// Args:
///     msg (VersionedMessage): The message to serialize.
///
/// Returns:
///     bytes: the serialized message.
#[pyfunction]
pub fn to_bytes_versioned(msg: VersionedMessage) -> Vec<u8> {
    VersionedMessageOriginal::from(msg).serialize()
}

/// Deserialize a versioned message, where the first byte indicates whether or not it's a legacy message.
///
/// Args:
///     raw (bytes): The serialized message.
///
/// Returns:
///     VersionedMessage: the deserialized message.
#[pyfunction]
pub fn from_bytes_versioned(raw: &[u8]) -> PyResult<VersionedMessage> {
    let deser = wincode::deserialize::<VersionedMessageOriginal>(raw);
    handle_py_value_err(deser)
}

/// Register the ``solana_message::v1`` module-scope constants.
///
/// They are added to the flat extension module under ``V1_``-prefixed names and
/// re-exported without the prefix by ``solders.message.v1``.
pub fn include_v1_constants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("V1_PREFIX", V1_PREFIX)?;
    m.add("V1_MAX_TRANSACTION_SIZE", MAX_TRANSACTION_SIZE)?;
    m.add("V1_MAX_ADDRESSES", MAX_ADDRESSES)?;
    m.add("V1_MAX_INSTRUCTIONS", MAX_INSTRUCTIONS)?;
    m.add("V1_MAX_SIGNATURES", MAX_SIGNATURES)?;
    m.add("V1_DEFAULT_HEAP_SIZE", DEFAULT_HEAP_SIZE)?;
    m.add("V1_MIN_HEAP_SIZE", MIN_HEAP_SIZE)?;
    m.add("V1_MAX_HEAP_SIZE", MAX_HEAP_SIZE)?;
    m.add("V1_FIXED_HEADER_SIZE", FIXED_HEADER_SIZE)?;
    m.add("V1_SIGNATURE_SIZE", SIGNATURE_SIZE)?;
    Ok(())
}
