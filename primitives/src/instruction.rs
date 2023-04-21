use std::hash::Hasher;

use crate::pubkey::Pubkey;
use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{
        AccountMeta as AccountMetaOriginal, CompiledInstruction as CompiledInstructionOriginal,
        Instruction as InstructionOriginal,
    },
    pubkey::Pubkey as PubkeyOriginal,
};
use solders_macros::{common_methods, pyhash, richcmp_eq_only};

use solders_traits::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    CommonMethodsCore, PyHash, RichcmpEqualityOnly,
};

/// Describes a single account read or written by a program during instruction
/// execution.
///
/// When constructing an :class:`Instruction`, a list of all accounts that may be
/// read or written during the execution of that instruction must be supplied.
/// Any account that may be mutated by the program during execution, either its
/// data or metadata such as held lamports, must be writable.
///
/// Note that because the Solana runtime schedules parallel transaction
/// execution around which accounts are writable, care should be taken that only
/// accounts which actually may be mutated are specified as writable.
///
/// Args:
///     pubkey (Pubkey): An account's public key.
///     is_signer (bool): True if an :class:`Instruction` requires a :class:`~solders.transaction.Transaction`
///         signature matching ``pubkey``.
///     is_writable (bool): True if the account data or metadata may be mutated during program execution.
///
/// Example:
///     >>> from solders.pubkey import Pubkey
///     >>> from solders.instruction import AccountMeta, Instruction
///     >>> from_pubkey = Pubkey.new_unique()
///     >>> to_pubkey = Pubkey.new_unique()
///     >>> program_id = Pubkey.new_unique()
///     >>> instruction_data = bytes([1])
///     >>> accs = [AccountMeta(from_pubkey, is_signer=True, is_writable=True), AccountMeta(to_pubkey, is_signer=True, is_writable=True)]
///     >>> instruction = Instruction(program_id, instruction_data, accs)
///
#[pyclass(module = "solders.instruction", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, From, Into)]
pub struct AccountMeta(AccountMetaOriginal);
#[pyhash]
#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AccountMeta {
    #[new]
    pub fn new(pubkey: &Pubkey, is_signer: bool, is_writable: bool) -> Self {
        let underlying_pubkey = pubkey.into();
        let underlying = if is_writable {
            AccountMetaOriginal::new(underlying_pubkey, is_signer)
        } else {
            AccountMetaOriginal::new_readonly(underlying_pubkey, is_signer)
        };
        underlying.into()
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        self.0.pubkey.into()
    }

    #[getter]
    pub fn is_signer(&self) -> bool {
        self.0.is_signer
    }

    #[getter]
    pub fn is_writable(&self) -> bool {
        self.0.is_writable
    }

    #[staticmethod]
    /// Deserialize a serialized ``AccountMeta`` object.
    ///
    /// Args:
    ///     data (bytes): the serialized ``AccountMeta``.
    ///
    /// Returns:
    ///     AccountMeta: the deserialized ``AccountMeta``.
    ///
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}
pybytes_general_via_bincode!(AccountMeta);
impl RichcmpEqualityOnly for AccountMeta {}
py_from_bytes_general_via_bincode!(AccountMeta);

solders_traits::common_methods_default!(AccountMeta);

impl PyHash for AccountMeta {}
impl_display!(AccountMeta);

#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for AccountMeta {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.pubkey.hash(state);
        self.0.is_signer.hash(state);
        self.0.is_writable.hash(state);
    }
}

#[pyclass(module = "solders.instruction", subclass)]
/// A directive for a single invocation of a Solana program.
///
/// An instruction specifies which program it is calling, which accounts it may
/// read or modify, and additional data that serves as input to the program. One
/// or more instructions are included in transactions submitted by Solana
/// clients. Instructions are also used to describe `cross-program
/// invocations <https://docs.solana.com/developing/programming-model/calling-between-programs/>`_.
///
/// During execution, a program will receive a list of account data as one of
/// its arguments, in the same order as specified during ``Instruction``
/// construction.
///
/// While Solana is agnostic to the format of the instruction data, it has
/// built-in support for serialization via
/// `borsh <https://docs.rs/borsh/latest/borsh/>`_
/// and `bincode <https://docs.rs/bincode/latest/bincode/>`_.
///
/// When constructing an ``Instruction``, a list of all accounts that may be
/// read or written during the execution of that instruction must be supplied as
/// :class:`AccountMeta` values.
///
/// **Specifying Account Metadata**
///
/// Any account whose data may be mutated by the program during execution must
/// be specified as writable. During execution, writing to an account that was
/// not specified as writable will cause the transaction to fail. Writing to an
/// account that is not owned by the program will cause the transaction to fail.
///
/// Any account whose lamport balance may be mutated by the program during
/// execution must be specified as writable. During execution, mutating the
/// lamports of an account that was not specified as writable will cause the
/// transaction to fail. While *subtracting* lamports from an account not owned
/// by the program will cause the transaction to fail, *adding* lamports to any
/// account is allowed, as long is it is mutable.
///
/// Accounts that are not read or written by the program may still be specified
/// in an ``Instruction``'s account list. These will affect scheduling of program
/// execution by the runtime, but will otherwise be ignored.
///
/// When building a transaction, the Solana runtime coalesces all accounts used
/// by all instructions in that transaction, along with accounts and permissions
/// required by the runtime, into a single account list. Some accounts and
/// account permissions required by the runtime to process a transaction are
/// *not* required to be included in an ``Instruction``'s account list. These
/// include:
///
/// * The program ID: it is a separate field of ``Instruction``
/// * The transaction's fee-paying account: it is added during :class:`~solders.message.Message`
///   construction. A program may still require the fee payer as part of the
///   account list if it directly references it.
///
///
/// Programs may require signatures from some accounts, in which case they
/// should be specified as signers during ``Instruction`` construction. The
/// program must still validate during execution that the account is a signer.
///
/// Args:
///     program_id (Pubkey): Pubkey of the program that executes this instruction.
///     data (bytes): Opaque data passed to the program for its own interpretation.
///     accounts (list[AccountMeta]): Metadata describing accounts that should be passed to the program.
///
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, From, Into)]
pub struct Instruction(pub InstructionOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Instruction {
    #[new]
    pub fn new(program_id: &Pubkey, data: &[u8], accounts: Vec<AccountMeta>) -> Self {
        let underlying_accounts: Vec<AccountMetaOriginal> =
            accounts.into_iter().map(|x| x.0).collect();
        let underlying =
            InstructionOriginal::new_with_bytes(program_id.into(), data, underlying_accounts);
        underlying.into()
    }

    #[getter]
    pub fn program_id(&self) -> Pubkey {
        self.0.program_id.into()
    }

    #[getter]
    pub fn data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.data)
    }

    #[getter]
    pub fn accounts(&self) -> Vec<AccountMeta> {
        self.0
            .accounts
            .clone()
            .into_iter()
            .map(AccountMeta)
            .collect()
    }

    #[setter]
    pub fn set_accounts(&mut self, accounts: Vec<AccountMeta>) {
        self.0.accounts = accounts
            .into_iter()
            .map(AccountMetaOriginal::from)
            .collect();
    }

    #[staticmethod]
    /// Deserialize a serialized ``Instruction`` object.
    ///
    /// Args:
    ///     data (bytes): the serialized ``Instruction``.
    ///
    /// Returns:
    ///     Instruction: the deserialized ``Instruction``.
    ///
    /// Example:
    ///     >>> from solders.pubkey import Pubkey
    ///     >>> from solders.instruction import AccountMeta, Instruction
    ///     >>> from_pubkey = Pubkey.new_unique()
    ///     >>> to_pubkey = Pubkey.new_unique()
    ///     >>> program_id = Pubkey.new_unique()
    ///     >>> instruction_data = bytes([1])
    ///     >>> accounts = [AccountMeta(from_pubkey, is_signer=True, is_writable=True), AccountMeta(to_pubkey, is_signer=True, is_writable=True),]
    ///     >>> instruction = Instruction(program_id, instruction_data, accounts)
    ///     >>> serialized = bytes(instruction)
    ///     >>> assert Instruction.from_bytes(serialized) == instruction
    ///
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}
pybytes_general_via_bincode!(Instruction);
impl RichcmpEqualityOnly for Instruction {}
py_from_bytes_general_via_bincode!(Instruction);

solders_traits::common_methods_default!(Instruction);

impl_display!(Instruction);

impl AsRef<InstructionOriginal> for Instruction {
    fn as_ref(&self) -> &InstructionOriginal {
        &self.0
    }
}

/// A compact encoding of an instruction.
///
/// A ``CompiledInstruction`` is a component of a multi-instruction :class:`~solders.message.Message`,
/// which is the core of a Solana transaction. It is created during the
/// construction of ``Message``. Most users will not interact with it directly.
///
/// Args:
///     program_id_index (int): Index into the transaction keys array indicating the
///         program account that executes this instruction.
///     data (bytes): The program input data.
///     accounts (bytes): Ordered indices into the transaction keys array indicating
///         which accounts to pass to the program.
///
#[pyclass(module = "solders.instruction", subclass)]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, From, Into)]
pub struct CompiledInstruction(CompiledInstructionOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl CompiledInstruction {
    #[new]
    pub fn new(program_id_index: u8, data: &[u8], accounts: &[u8]) -> Self {
        CompiledInstructionOriginal::new_from_raw_parts(
            program_id_index,
            data.to_vec(),
            accounts.to_vec(),
        )
        .into()
    }

    /// Return the pubkey of the program that executes this instruction.
    ///
    /// Returns:
    ///     Pubkey: The program ID.
    ///
    pub fn program_id(&self, program_ids: Vec<Pubkey>) -> Pubkey {
        let underlying_pubkeys: Vec<PubkeyOriginal> =
            program_ids.iter().map(PubkeyOriginal::from).collect();
        let underlying = *self.0.program_id(&underlying_pubkeys);
        underlying.into()
    }

    #[getter]
    pub fn program_id_index(&self) -> u8 {
        self.0.program_id_index
    }

    #[getter]
    pub fn accounts<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.accounts)
    }

    #[setter]
    pub fn set_accounts(&mut self, accounts: Vec<u8>) {
        self.0.accounts = accounts
    }

    #[getter]
    pub fn data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.data)
    }

    #[staticmethod]
    /// Deserialize a serialized ``CompiledInstruction`` object.
    ///
    /// Args:
    ///     data (bytes): the serialized ``CompiledInstruction``.
    ///
    /// Returns:
    ///     CompiledInstruction: The deserialized ``CompiledInstruction``.
    ///
    pub fn from_bytes(data: &[u8]) -> PyResult<Self> {
        Self::py_from_bytes(data)
    }
}
pybytes_general_via_bincode!(CompiledInstruction);
impl RichcmpEqualityOnly for CompiledInstruction {}
py_from_bytes_general_via_bincode!(CompiledInstruction);

solders_traits::common_methods_default!(CompiledInstruction);

impl_display!(CompiledInstruction);

impl AsRef<CompiledInstructionOriginal> for CompiledInstruction {
    fn as_ref(&self) -> &CompiledInstructionOriginal {
        &self.0
    }
}
