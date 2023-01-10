#![allow(deprecated)]
use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    sanitize::Sanitize,
    signature::Signature as SignatureOriginal,
    transaction::{
        get_nonce_pubkey_from_instruction, uses_durable_nonce, Legacy as LegacyOriginal,
        Transaction as TransactionOriginal, TransactionVersion as TransactionVersionOriginal,
        VersionedTransaction as VersionedTransactionOriginal,
    },
};
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_traits::{
    handle_py_err, impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    CommonMethodsCore, RichcmpEqualityOnly,
};

use crate::{
    convert_instructions, convert_optional_pubkey,
    hash::Hash as SolderHash,
    instruction::{CompiledInstruction, Instruction},
    message::{Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Signature,
    signature::{originals_into_solders, solders_into_originals},
    signer::Signer,
    signer::SignerVec,
};

/// An atomic transaction
///
/// The ``__init__`` method signs a versioned message to
/// create a signed transaction.
///
/// Args:
///     message (Message | MessageV0): The message to sign.
///     keypairs (Sequence[Keypair | Presigner]): The keypairs that are to sign the transaction.
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction", subclass)]
pub struct VersionedTransaction(VersionedTransactionOriginal);

impl From<Transaction> for VersionedTransaction {
    fn from(t: Transaction) -> Self {
        VersionedTransactionOriginal::from(TransactionOriginal::from(t)).into()
    }
}

impl RichcmpEqualityOnly for VersionedTransaction {}
pybytes_general_via_bincode!(VersionedTransaction);
py_from_bytes_general_via_bincode!(VersionedTransaction);
impl_display!(VersionedTransaction);
solders_traits::common_methods_default!(VersionedTransaction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl VersionedTransaction {
    #[new]
    pub fn new(message: VersionedMessage, keypairs: Vec<Signer>) -> PyResult<Self> {
        handle_py_err(VersionedTransactionOriginal::try_new(
            message.into(),
            &SignerVec(keypairs),
        ))
    }

    /// Message | MessageV0: The transaction message.
    #[getter]
    pub fn message(&self) -> VersionedMessage {
        self.0.message.clone().into()
    }

    /// List[Signature]: The transaction signatures.
    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        originals_into_solders(self.0.signatures.clone())
    }

    #[setter]
    fn set_signatures(&mut self, signatures: Vec<Signature>) {
        self.0.signatures = solders_into_originals(signatures);
    }

    /// Create a fully-signed transaction from a message and its signatures.
    ///
    /// Args:
    ///     message (Message | MessageV0): The transaction message.
    ///     signatures (Sequence[Signature]): The message's signatures.
    ///
    /// Returns:
    ///     Transaction: The signed transaction.
    ///
    /// Example:
    ///
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import Instruction
    ///     >>> from solders.message import MessageV0
    ///     >>> from solders.hash import Hash
    ///     >>> from solders.keypair import Keypair
    ///     >>> from solders.transaction import VersionedTransaction
    ///     >>> payer = Keypair()
    ///     >>> program_id = Pubkey.default()
    ///     >>> instructions = [Instruction(program_id, bytes([]), [])]
    ///     >>> recent_blockhash = Hash.new_unique()
    ///     >>> message = MessageV0.try_compile(payer.pubkey(), instructions, [], recent_blockhash)
    ///     >>> tx = VersionedTransaction(message, [payer])
    ///     >>> assert VersionedTransaction.populate(message, tx.signatures) == tx
    ///
    #[staticmethod]
    pub fn populate(message: VersionedMessage, signatures: Vec<Signature>) -> Self {
        VersionedTransactionOriginal {
            signatures: signatures.into_iter().map(|s| s.into()).collect(),
            message: message.into(),
        }
        .into()
    }

    /// Sanity checks the Transaction properties.
    pub fn sanitize(&self, require_static_program_ids: bool) -> PyResult<()> {
        handle_py_err(self.0.sanitize(require_static_program_ids))
    }

    /// Returns the version of the transaction.
    ///
    /// Returns:
    ///     Legacy | int: Transaction version.
    pub fn version(&self) -> TransactionVersion {
        self.0.version().into()
    }

    /// Returns a legacy transaction if the transaction message is legacy.
    ///
    /// Returns:
    ///     Optional[Transaction]: The legacy transaction.
    pub fn into_legacy_transaction(&self) -> Option<Transaction> {
        self.0.clone().into_legacy_transaction().map(|t| t.into())
    }

    /// Verify the transaction and hash its message
    pub fn verify_and_hash_message(&self) -> PyResult<SolderHash> {
        handle_py_err(self.0.verify_and_hash_message())
    }

    /// Verify the transaction and return a list of verification results
    pub fn verify_with_results(&self) -> Vec<bool> {
        self.0.verify_with_results()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Return a new default transaction.
    ///
    /// Returns:
    ///     VersionedTransaction: The default transaction.
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Convert a legacy transaction to a VersionedTransaction.
    ///
    /// Returns:
    ///     VersionedTransaction: The versioned tx.
    #[staticmethod]
    pub fn from_legacy(tx: Transaction) -> Self {
        Self::from(tx)
    }

    /// Returns true if transaction begins with a valid advance nonce instruction.
    ///
    /// Returns:
    ///     bool
    pub fn uses_durable_nonce(&self) -> bool {
        self.0.uses_durable_nonce()
    }
}

#[pyclass(module = "solders.transaction", subclass)]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize, From, Into)]
/// An atomically-commited sequence of instructions.
///
/// While :class:`~solders.instruction.Instruction`\s are the basic unit of computation in Solana,
/// they are submitted by clients in :class:`~solders.transaction.Transaction`\s containing one or
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
///     >>> tx = Transaction([payer], message, blockhash)
///
pub struct Transaction(TransactionOriginal);

#[richcmp_eq_only]
#[common_methods]
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
        originals_into_solders(self.0.signatures.clone())
    }

    #[setter]
    fn set_signatures(&mut self, signatures: Vec<Signature>) {
        self.0.signatures = solders_into_originals(signatures);
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
    pub fn new_unsigned(message: Message) -> Self {
        TransactionOriginal::new_unsigned(message.into()).into()
    }

    #[staticmethod]
    /// Create an unsigned transaction from a list of :class:`~solders.instruction.Instruction`\s.
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
    ///     >>> tx = Transaction.new_with_payer([instruction], payer.pubkey())
    ///
    pub fn new_with_payer(instructions: Vec<Instruction>, payer: Option<&Pubkey>) -> Self {
        TransactionOriginal::new_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer),
        )
        .into()
    }

    #[staticmethod]
    /// Create a fully-signed transaction from a list of :class:`~solders.instruction.Instruction`\s.
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
        payer: Option<Pubkey>,
        signing_keypairs: Vec<Signer>,
        recent_blockhash: SolderHash,
    ) -> Self {
        TransactionOriginal::new_signed_with_payer(
            &convert_instructions(instructions),
            convert_optional_pubkey(payer.as_ref()),
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
    /// Create a fully-signed transaction from a message and its signatures.
    ///
    /// Args:
    ///     message (Message): The transaction message.
    ///     signatures (Sequence[Signature]): The message's signatures.
    ///
    /// Returns:
    ///     Message: The signed transaction.
    ///
    /// Example:
    ///
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
    ///     >>> assert tx == Transaction.populate(tx.message, tx.signatures)
    ///
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

    /// Get the data for an instruction at the given index.
    ///
    /// Args:
    ///     instruction_index (int): index into the ``instructions`` vector of the transaction's ``message``.
    ///
    /// Returns:
    ///     bytes: The instruction data.
    ///
    pub fn data(&self, instruction_index: usize) -> &[u8] {
        self.0.data(instruction_index)
    }

    /// Get the :class:`~solders.pubkey.Pubkey` of an account required by one of the instructions in
    /// the transaction.
    ///
    /// Returns ``None`` if `instruction_index` is greater than or equal to the
    /// number of instructions in the transaction; or if `accounts_index` is
    /// greater than or equal to the number of accounts in the instruction.
    ///
    /// Args:
    ///     instruction_index (int): index into the ``instructions`` vector of the transaction's ``message``.
    ///     account_index (int): index into the ``acounts`` list of the message's ``compiled_instructions``.
    ///
    /// Returns:
    ///     Optional[Pubkey]: The account key.
    ///
    pub fn key(&self, instruction_index: usize, accounts_index: usize) -> Option<Pubkey> {
        self.0
            .key(instruction_index, accounts_index)
            .map(Pubkey::from)
    }

    /// Get the :class:`~solders.pubkey.Pubkey` of a signing account required by one of the
    /// instructions in the transaction.
    ///
    /// The transaction does not need to be signed for this function to return a
    /// signing account's pubkey.
    ///
    /// Returns ``None`` if the indexed account is not required to sign the
    /// transaction. Returns ``None`` if the [`signatures`] field does not contain
    /// enough elements to hold a signature for the indexed account (this should
    /// only be possible if `Transaction` has been manually constructed).
    ///
    /// Returns `None` if `instruction_index` is greater than or equal to the
    /// number of instructions in the transaction; or if `accounts_index` is
    /// greater than or equal to the number of accounts in the instruction.
    ///
    /// Args:
    ///     instruction_index (int): index into the ``instructions`` vector of the transaction's ``message``.
    ///     account_index (int): index into the ``acounts`` list of the message's ``compiled_instructions``.
    ///
    /// Returns:
    ///     Optional[Pubkey]: The account key.
    ///
    pub fn signer_key(&self, instruction_index: usize, accounts_index: usize) -> Option<Pubkey> {
        self.0
            .signer_key(instruction_index, accounts_index)
            .map(Pubkey::from)
    }

    /// Return the serialized message data to sign.
    ///
    /// Returns:
    ///     bytes: The serialized message data.
    ///
    pub fn message_data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.message_data())
    }

    /// Sign the transaction, returning any errors.
    ///
    /// This method fully signs a transaction with all required signers, which
    /// must be present in the ``keypairs`` list. To sign with only some of the
    /// required signers, use :meth:`Transaction.partial_sign`.
    ///
    /// If ``recent_blockhash`` is different than recorded in the transaction message's
    /// ``recent_blockhash``] field, then the message's ``recent_blockhash`` will be updated
    /// to the provided ``recent_blockhash``, and any prior signatures will be cleared.
    ///
    ///
    /// **Errors:**
    ///
    /// Signing will fail if some required signers are not provided in
    /// ``keypairs``; or, if the transaction has previously been partially signed,
    /// some of the remaining required signers are not provided in ``keypairs``.
    /// In other words, the transaction must be fully signed as a result of
    /// calling this function.
    ///
    /// Signing will fail for any of the reasons described in the documentation
    /// for :meth:`Transaction.partial_sign`.
    ///
    /// Args:
    ///     keypairs (Sequence[Keypair | Presigner]): The signers for the transaction.
    ///     recent_blockhash (Hash): The id of a recent ledger entry.
    ///
    pub fn sign(&mut self, keypairs: Vec<Signer>, recent_blockhash: SolderHash) -> PyResult<()> {
        handle_py_err(
            self.0
                .try_sign(&SignerVec(keypairs), recent_blockhash.into()),
        )
    }

    /// Sign the transaction with a subset of required keys, returning any errors.
    ///
    /// Unlike :meth:`Transaction.sign`, this method does not require all
    /// keypairs to be provided, allowing a transaction to be signed in multiple
    /// steps.
    ///
    /// It is permitted to sign a transaction with the same keypair multiple
    /// times.
    ///
    /// If ``recent_blockhash`` is different than recorded in the transaction message's
    /// ``recent_blockhash`` field, then the message's ``recent_blockhash`` will be updated
    /// to the provided ``recent_blockhash``, and any prior signatures will be cleared.
    ///
    /// **Errors:**
    ///
    /// Signing will fail if
    ///
    /// - The transaction's :class:`~solders.message.Message` is malformed such that the number of
    ///   required signatures recorded in its header
    ///   (``num_required_signatures``) is greater than the length of its
    ///   account keys (``account_keys``).
    /// - Any of the provided signers in ``keypairs`` is not a required signer of
    ///   the message.
    /// - Any of the signers is a :class:`~solders.presigner.Presigner`, and its provided signature is
    ///   incorrect.
    ///
    /// Args:
    ///     keypairs (Sequence[Keypair | Presigner]): The signers for the transaction.
    ///     recent_blockhash (Hash): The id of a recent ledger entry.
    ///     
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

    /// Verifies that all signers have signed the message.
    ///
    /// Raises:
    ///     TransactionError: if the check fails.
    pub fn verify(&self) -> PyResult<()> {
        handle_py_err(self.0.verify())
    }

    /// Verify the transaction and hash its message.
    ///
    /// Returns:
    ///     Hash: The blake3 hash of the message.
    ///
    /// Raises:
    ///     TransactionError: if the check fails.
    pub fn verify_and_hash_message(&self) -> PyResult<SolderHash> {
        handle_py_err(self.0.verify_and_hash_message())
    }

    /// Verifies that all signers have signed the message.
    ///
    /// Returns:
    ///     list[bool]: a list with the length of required signatures, where each element is either ``True`` if that signer has signed, or ``False`` if not.
    ///
    pub fn verify_with_results(&self) -> Vec<bool> {
        self.0.verify_with_results()
    }

    /// Get the positions of the pubkeys in account_keys associated with signing keypairs.
    ///
    /// Args:
    ///     pubkeys (Sequence[Pubkey]): The pubkeys to find.
    ///     
    ///     Returns:
    ///         list[Optional[int]]: The pubkey positions.
    ///
    pub fn get_signing_keypair_positions(
        &self,
        pubkeys: Vec<Pubkey>,
    ) -> PyResult<Vec<Option<usize>>> {
        let converted_pubkeys: Vec<PubkeyOriginal> =
            pubkeys.into_iter().map(PubkeyOriginal::from).collect();
        handle_py_err(self.0.get_signing_keypair_positions(&converted_pubkeys))
    }

    /// Replace all the signatures and pubkeys.
    ///
    /// Args:
    ///     signers (Sequence[Tuple[Pubkey, Signature]]): The replacement pubkeys and signatures.
    ///
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

    /// Check if the transaction has been signed.
    ///
    /// Returns:
    ///     bool: True if the transaction has been signed.
    ///
    pub fn is_signed(&self) -> bool {
        self.0.is_signed()
    }

    /// See https://docs.rs/solana-sdk/latest/solana_sdk/transaction/fn.uses_durable_nonce.html
    pub fn uses_durable_nonce(&self) -> Option<CompiledInstruction> {
        uses_durable_nonce(&self.0).map(|x| CompiledInstruction::from(x.clone()))
    }

    /// Sanity checks the Transaction properties.
    pub fn sanitize(&self) -> PyResult<()> {
        handle_py_err(self.0.sanitize())
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Return a new default transaction.
    ///
    /// Returns:
    ///     Transaction: The default transaction.
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    /// Deserialize a serialized ``Transaction`` object.
    ///
    /// Args:
    ///     data (bytes): the serialized ``Transaction``.
    ///
    /// Returns:
    ///     Transaction: the deserialized ``Transaction``.
    ///
    /// Example:
    ///     >>> from solders.transaction import Transaction
    ///     >>> tx = Transaction.default()
    ///     >>> assert Transaction.from_bytes(bytes(tx)) == tx
    ///
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }

    /// Deprecated in the Solana Rust SDK, expose here only for testing.
    pub fn get_nonce_pubkey_from_instruction(&self, ix: &CompiledInstruction) -> Option<Pubkey> {
        get_nonce_pubkey_from_instruction(ix.as_ref(), self.as_ref()).map(Pubkey::from)
    }
}

impl RichcmpEqualityOnly for Transaction {}
pybytes_general_via_bincode!(Transaction);
py_from_bytes_general_via_bincode!(Transaction);
impl_display!(Transaction);
solders_traits::common_methods_default!(Transaction);

impl AsRef<TransactionOriginal> for Transaction {
    fn as_ref(&self) -> &TransactionOriginal {
        &self.0
    }
}

/// Transaction version type that serializes to the string "legacy"
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.transaction")]
pub enum Legacy {
    Legacy,
}

impl From<Legacy> for LegacyOriginal {
    fn from(x: Legacy) -> Self {
        match x {
            Legacy::Legacy => Self::Legacy,
        }
    }
}

impl From<LegacyOriginal> for Legacy {
    fn from(x: LegacyOriginal) -> Self {
        match x {
            LegacyOriginal::Legacy => Self::Legacy,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, EnumIntoPy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TransactionVersion {
    Legacy(Legacy),
    Number(u8),
}

impl From<TransactionVersion> for TransactionVersionOriginal {
    fn from(v: TransactionVersion) -> Self {
        match v {
            TransactionVersion::Legacy(x) => Self::Legacy(x.into()),
            TransactionVersion::Number(n) => Self::Number(n),
        }
    }
}

impl From<TransactionVersionOriginal> for TransactionVersion {
    fn from(v: TransactionVersionOriginal) -> Self {
        match v {
            TransactionVersionOriginal::Legacy(x) => Self::Legacy(x.into()),
            TransactionVersionOriginal::Number(n) => Self::Number(n),
        }
    }
}
