#![allow(clippy::too_many_arguments)]
use derive_more::{From, Into};
extern crate base64;
use pythonize::{depythonize, pythonize};
use solders_primitives::{message::MessageHeader, pubkey::Pubkey, signature::Signature};
use solders_traits::{handle_py_value_err, RichcmpEqualityOnly};
use std::fmt::Display;
use std::str::FromStr;

use crate::{
    account_decoder::UiTokenAmount,
    commitment_config::CommitmentConfig,
    tmp_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta as EncodedConfirmedTransactionWithStatusMetaOriginal,
        EncodedTransaction as EncodedTransactionOriginal,
        EncodedTransactionWithStatusMeta as EncodedTransactionWithStatusMetaOriginal,
        ParsedAccount as ParsedAccountOriginal, ParsedInstruction as ParsedInstructionOriginal,
        Reward as RewardOriginal, RewardType as RewardTypeOriginal,
        TransactionBinaryEncoding as TransactionBinaryEncodingOriginal,
        TransactionConfirmationStatus as TransactionConfirmationStatusOriginal,
        TransactionStatus as TransactionStatusOriginal,
        UiAddressTableLookup as UiAddressTableLookupOriginal,
        UiCompiledInstruction as UiCompiledInstructionOriginal,
        UiConfirmedBlock as UiConfirmedBlockOriginal,
        UiInnerInstructions as UiInnerInstructionsOriginal, UiInstruction as UiInstructionOriginal,
        UiLoadedAddresses as UiLoadedAddressesOriginal, UiMessage as UiMessageOriginal,
        UiParsedInstruction as UiParsedInstructionOriginal,
        UiParsedMessage as UiParsedMessageOriginal,
        UiPartiallyDecodedInstruction as UiPartiallyDecodedInstructionOriginal,
        UiRawMessage as UiRawMessageOriginal, UiTransaction as UiTransactionOriginal,
        UiTransactionStatusMeta as UiTransactionStatusMetaOriginal,
        UiTransactionTokenBalance as UiTransactionTokenBalanceOriginal,
    },
    SolderHash,
};
use pyo3::{
    prelude::*,
    types::{PyBytes, PyTuple},
    PyTypeInfo,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::{
    clock::UnixTimestamp, instruction::InstructionError as InstructionErrorOriginal,
    slot_history::Slot, transaction::TransactionError as TransactionErrorOriginal,
    transaction_context::TransactionReturnData as TransactionReturnDataOriginal,
};
use solders_macros::{common_methods, enum_original_mapping, richcmp_eq_only, EnumIntoPy};
use solders_primitives::transaction::{TransactionVersion, VersionedTransaction};

macro_rules! transaction_status_boilerplate {
    ($name:ident) => {
        impl RichcmpEqualityOnly for $name {}
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        solders_traits::pybytes_general_via_bincode!($name);
        solders_traits::py_from_bytes_general_via_bincode!($name);
        solders_traits::common_methods_default!($name);
    };
}

pub(crate) use transaction_status_boilerplate;

/// Encoding options for transaction data.
#[pyclass(module = "solders.transaction_status")]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UiTransactionEncoding {
    Binary, // Legacy. Retained for RPC backwards compatibility
    Base64,
    Base58,
    Json,
    JsonParsed,
}

impl Default for UiTransactionEncoding {
    fn default() -> Self {
        Self::Base64
    }
}

/// Levels of transaction detail to return in RPC requests.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.transaction_status")]
pub enum TransactionDetails {
    Full,
    Signatures,
    #[serde(rename = "none")]
    None_,
}

impl Default for TransactionDetails {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(TransactionBinaryEncodingOriginal)]
#[pyclass(module = "solders.transaction_status")]
pub enum TransactionBinaryEncoding {
    Base58,
    Base64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiCompiledInstruction(UiCompiledInstructionOriginal);

transaction_status_boilerplate!(UiCompiledInstruction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiCompiledInstruction {
    #[new]
    fn new(program_id_index: u8, accounts: Vec<u8>, data: String) -> Self {
        UiCompiledInstructionOriginal {
            program_id_index,
            accounts,
            data,
        }
        .into()
    }

    #[getter]
    pub fn program_id_index(&self) -> u8 {
        self.0.program_id_index
    }

    #[getter]
    pub fn accounts<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.accounts)
    }

    #[getter]
    pub fn data(&self) -> String {
        self.0.data.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiAddressTableLookup(UiAddressTableLookupOriginal);

transaction_status_boilerplate!(UiAddressTableLookup);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiAddressTableLookup {
    #[new]
    fn new(account_key: Pubkey, writable_indexes: Vec<u8>, readonly_indexes: Vec<u8>) -> Self {
        UiAddressTableLookupOriginal {
            account_key: account_key.to_string(),
            writable_indexes,
            readonly_indexes,
        }
        .into()
    }

    #[getter]
    pub fn account_key(&self) -> Pubkey {
        Pubkey::from_str(&self.0.account_key).unwrap()
    }

    #[getter]
    pub fn writable_indexes<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.writable_indexes)
    }

    #[getter]
    pub fn readonly_indexes<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.readonly_indexes)
    }
}

/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiRawMessage(UiRawMessageOriginal);

transaction_status_boilerplate!(UiRawMessage);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiRawMessage {
    #[new]
    fn new(
        header: MessageHeader,
        account_keys: Vec<Pubkey>,
        recent_blockhash: SolderHash,
        instructions: Vec<UiCompiledInstruction>,
        address_table_lookups: Option<Vec<UiAddressTableLookup>>,
    ) -> Self {
        UiRawMessageOriginal {
            header: header.into(),
            account_keys: account_keys.into_iter().map(|p| p.to_string()).collect(),
            recent_blockhash: recent_blockhash.to_string(),
            instructions: instructions.into_iter().map(|ix| ix.into()).collect(),
            address_table_lookups: address_table_lookups
                .map(|v| v.into_iter().map(|a| a.into()).collect()),
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
            .iter()
            .map(|s| Pubkey::from_str(s).unwrap())
            .collect()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> SolderHash {
        SolderHash::from_str(&self.0.recent_blockhash).unwrap()
    }

    #[getter]
    pub fn instructions(&self) -> Vec<UiCompiledInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(|ix| ix.into())
            .collect()
    }

    #[getter]
    pub fn address_table_lookups(&self) -> Option<Vec<UiAddressTableLookup>> {
        self.0
            .address_table_lookups
            .clone()
            .map(|v| v.into_iter().map(UiAddressTableLookup::from).collect())
    }
}

/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct ParsedAccount(ParsedAccountOriginal);

transaction_status_boilerplate!(ParsedAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ParsedAccount {
    #[new]
    fn new(pubkey: Pubkey, writable: bool, signer: bool) -> Self {
        ParsedAccountOriginal {
            pubkey: pubkey.to_string(),
            writable,
            signer,
        }
        .into()
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.pubkey).unwrap()
    }

    #[getter]
    pub fn writable(&self) -> bool {
        self.0.writable
    }

    #[getter]
    pub fn signer(&self) -> bool {
        self.0.signer
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct ParsedInstruction(ParsedInstructionOriginal);

transaction_status_boilerplate!(ParsedInstruction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ParsedInstruction {
    #[new]
    fn new(program: String, program_id: Pubkey, parsed: &PyAny) -> PyResult<Self> {
        let value = handle_py_value_err(depythonize::<Value>(parsed))?;
        Ok(ParsedInstructionOriginal {
            program,
            program_id: program_id.to_string(),
            parsed: value,
        }
        .into())
    }

    #[getter]
    pub fn program(&self) -> String {
        self.0.program.clone()
    }

    #[getter]
    pub fn program_id(&self) -> Pubkey {
        Pubkey::from_str(&self.0.program_id).unwrap()
    }

    #[getter]
    pub fn parsed(&self, py: Python<'_>) -> PyResult<PyObject> {
        handle_py_value_err(pythonize(py, &self.0.parsed))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiPartiallyDecodedInstruction(UiPartiallyDecodedInstructionOriginal);

transaction_status_boilerplate!(UiPartiallyDecodedInstruction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiPartiallyDecodedInstruction {
    #[new]
    fn new(program_id: Pubkey, accounts: Vec<Pubkey>, data: String) -> Self {
        UiPartiallyDecodedInstructionOriginal {
            program_id: program_id.to_string(),
            accounts: accounts.into_iter().map(|a| a.to_string()).collect(),
            data,
        }
        .into()
    }

    #[getter]
    pub fn program_id(&self) -> Pubkey {
        Pubkey::from_str(&self.0.program_id).unwrap()
    }

    #[getter]
    pub fn accounts(&self) -> Vec<Pubkey> {
        self.0
            .accounts
            .clone()
            .into_iter()
            .map(|a| Pubkey::from_str(&a).unwrap())
            .collect()
    }

    #[getter]
    pub fn data(&self) -> String {
        self.0.data.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, EnumIntoPy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiParsedInstruction {
    Parsed(ParsedInstruction),
    PartiallyDecoded(UiPartiallyDecodedInstruction),
}

impl From<UiParsedInstruction> for UiParsedInstructionOriginal {
    fn from(ix: UiParsedInstruction) -> Self {
        match ix {
            UiParsedInstruction::Parsed(p) => Self::Parsed(p.into()),
            UiParsedInstruction::PartiallyDecoded(p) => Self::PartiallyDecoded(p.into()),
        }
    }
}

impl From<UiParsedInstructionOriginal> for UiParsedInstruction {
    fn from(ix: UiParsedInstructionOriginal) -> Self {
        match ix {
            UiParsedInstructionOriginal::Parsed(p) => Self::Parsed(p.into()),
            UiParsedInstructionOriginal::PartiallyDecoded(p) => Self::PartiallyDecoded(p.into()),
        }
    }
}

/// A duplicate representation of an Instruction for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, EnumIntoPy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiInstruction {
    Compiled(UiCompiledInstruction),
    Parsed(UiParsedInstruction),
}

impl From<UiInstruction> for UiInstructionOriginal {
    fn from(ix: UiInstruction) -> Self {
        match ix {
            UiInstruction::Compiled(c) => Self::Compiled(c.into()),
            UiInstruction::Parsed(p) => Self::Parsed(p.into()),
        }
    }
}

impl From<UiInstructionOriginal> for UiInstruction {
    fn from(ix: UiInstructionOriginal) -> Self {
        match ix {
            UiInstructionOriginal::Compiled(c) => Self::Compiled(c.into()),
            UiInstructionOriginal::Parsed(p) => Self::Parsed(p.into()),
        }
    }
}

/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiParsedMessage(UiParsedMessageOriginal);

transaction_status_boilerplate!(UiParsedMessage);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiParsedMessage {
    #[new]
    fn new(
        account_keys: Vec<ParsedAccount>,
        recent_blockhash: SolderHash,
        instructions: Vec<UiInstruction>,
        address_table_lookups: Option<Vec<UiAddressTableLookup>>,
    ) -> Self {
        UiParsedMessageOriginal {
            account_keys: account_keys.into_iter().map(|p| p.into()).collect(),
            recent_blockhash: recent_blockhash.to_string(),
            instructions: instructions.into_iter().map(|ix| ix.into()).collect(),
            address_table_lookups: address_table_lookups
                .map(|v| v.into_iter().map(|a| a.into()).collect()),
        }
        .into()
    }

    #[getter]
    pub fn account_keys(&self) -> Vec<ParsedAccount> {
        self.0
            .account_keys
            .clone()
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> SolderHash {
        SolderHash::from_str(&self.0.recent_blockhash).unwrap()
    }

    #[getter]
    pub fn instructions(&self) -> Vec<UiInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(|ix| ix.into())
            .collect()
    }

    #[getter]
    pub fn address_table_lookups(&self) -> Option<Vec<UiAddressTableLookup>> {
        self.0
            .address_table_lookups
            .clone()
            .map(|v| v.into_iter().map(|a| a.into()).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, EnumIntoPy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiMessage {
    Parsed(UiParsedMessage),
    Raw(UiRawMessage),
}

impl From<UiMessageOriginal> for UiMessage {
    fn from(m: UiMessageOriginal) -> Self {
        match m {
            UiMessageOriginal::Parsed(msg) => Self::Parsed(msg.into()),
            UiMessageOriginal::Raw(msg) => Self::Raw(msg.into()),
        }
    }
}

impl From<UiMessage> for UiMessageOriginal {
    fn from(m: UiMessage) -> Self {
        match m {
            UiMessage::Parsed(msg) => Self::Parsed(msg.into()),
            UiMessage::Raw(msg) => Self::Raw(msg.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransaction(UiTransactionOriginal);

transaction_status_boilerplate!(UiTransaction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTransaction {
    #[new]
    fn new(signatures: Vec<Signature>, message: UiMessage) -> Self {
        UiTransactionOriginal {
            signatures: signatures.into_iter().map(|s| s.to_string()).collect(),
            message: message.into(),
        }
        .into()
    }

    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        self.0
            .signatures
            .iter()
            .map(|s| Signature::from_str(s).unwrap())
            .collect()
    }

    #[getter]
    pub fn message(&self) -> UiMessage {
        self.0.message.clone().into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, EnumIntoPy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EncodedVersionedTransaction {
    Binary(VersionedTransaction),
    Json(UiTransaction),
}

impl From<EncodedTransaction> for EncodedVersionedTransaction {
    fn from(e: EncodedTransaction) -> Self {
        match e {
            EncodedTransaction::LegacyBinary(..) | EncodedTransaction::Binary(..) => Self::Binary(
                VersionedTransaction::from(EncodedTransactionOriginal::from(e).decode().unwrap()),
            ),
            EncodedTransaction::Json(u) => Self::Json(u),
        }
    }
}

impl From<EncodedVersionedTransaction> for EncodedTransaction {
    fn from(e: EncodedVersionedTransaction) -> Self {
        match e {
            EncodedVersionedTransaction::Binary(v) => Self::Binary(
                base64::encode(bincode::serialize(&v).unwrap()),
                TransactionBinaryEncoding::Base64,
            ),
            EncodedVersionedTransaction::Json(u) => Self::Json(u),
        }
    }
}

impl From<EncodedVersionedTransaction> for EncodedTransactionOriginal {
    fn from(e: EncodedVersionedTransaction) -> Self {
        EncodedTransaction::from(e).into()
    }
}

impl From<EncodedTransactionOriginal> for EncodedVersionedTransaction {
    fn from(e: EncodedTransactionOriginal) -> Self {
        EncodedTransaction::from(e).into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EncodedTransaction {
    LegacyBinary(String), // Old way of expressing base-58, retained for RPC backwards compatibility
    Binary(String, TransactionBinaryEncoding),
    Json(UiTransaction),
}

impl From<EncodedTransactionOriginal> for EncodedTransaction {
    fn from(e: EncodedTransactionOriginal) -> Self {
        match e {
            EncodedTransactionOriginal::LegacyBinary(s) => Self::LegacyBinary(s),
            EncodedTransactionOriginal::Binary(s, b) => Self::Binary(s, b.into()),
            EncodedTransactionOriginal::Json(t) => Self::Json(t.into()),
        }
    }
}

impl From<EncodedTransaction> for EncodedTransactionOriginal {
    fn from(e: EncodedTransaction) -> Self {
        match e {
            EncodedTransaction::LegacyBinary(s) => Self::LegacyBinary(s),
            EncodedTransaction::Binary(s, b) => Self::Binary(s, b.into()),
            EncodedTransaction::Json(t) => Self::Json(t.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiInnerInstructions(UiInnerInstructionsOriginal);

transaction_status_boilerplate!(UiInnerInstructions);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiInnerInstructions {
    #[new]
    pub fn new(index: u8, instructions: Vec<UiInstruction>) -> Self {
        UiInnerInstructionsOriginal {
            index,
            instructions: instructions.into_iter().map(|ix| ix.into()).collect(),
        }
        .into()
    }

    #[getter]
    pub fn index(&self) -> u8 {
        self.0.index
    }

    #[getter]
    pub fn instructions(&self) -> Vec<UiInstruction> {
        self.0
            .instructions
            .clone()
            .into_iter()
            .map(|ix| ix.into())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiLoadedAddresses(UiLoadedAddressesOriginal);

transaction_status_boilerplate!(UiLoadedAddresses);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiLoadedAddresses {
    #[new]
    pub fn new(writable: Vec<Pubkey>, readonly: Vec<Pubkey>) -> Self {
        UiLoadedAddressesOriginal {
            writable: writable.iter().map(|x| x.to_string()).collect(),
            readonly: readonly.iter().map(|x| x.to_string()).collect(),
        }
        .into()
    }

    #[getter]
    pub fn writable(&self) -> Vec<Pubkey> {
        self.0
            .writable
            .iter()
            .map(|x| Pubkey::from_str(x).unwrap())
            .collect()
    }

    #[getter]
    pub fn readonly(&self) -> Vec<Pubkey> {
        self.0
            .readonly
            .iter()
            .map(|x| Pubkey::from_str(x).unwrap())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransactionTokenBalance(UiTransactionTokenBalanceOriginal);

transaction_status_boilerplate!(UiTransactionTokenBalance);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTransactionTokenBalance {
    #[new]
    pub fn new(
        account_index: u8,
        mint: Pubkey,
        ui_token_amount: UiTokenAmount,
        owner: Option<Pubkey>,
        program_id: Option<Pubkey>,
    ) -> Self {
        UiTransactionTokenBalanceOriginal {
            account_index,
            mint: mint.to_string(),
            ui_token_amount: ui_token_amount.into(),
            owner: owner.map(|x| x.to_string()),
            program_id: program_id.map(|x| x.to_string()),
        }
        .into()
    }

    #[getter]
    pub fn account_index(&self) -> u8 {
        self.0.account_index
    }

    #[getter]
    pub fn mint(&self) -> Pubkey {
        Pubkey::from_str(&self.0.mint).unwrap()
    }

    #[getter]
    pub fn ui_token_amount(&self) -> UiTokenAmount {
        self.0.ui_token_amount.clone().into()
    }

    #[getter]
    pub fn owner(&self) -> Option<Pubkey> {
        self.0.owner.clone().map(|x| Pubkey::from_str(&x).unwrap())
    }

    #[getter]
    pub fn program_id(&self) -> Option<Pubkey> {
        self.0
            .clone()
            .program_id
            .map(|x| Pubkey::from_str(&x).unwrap())
    }
}

#[pyclass(module = "solders.transaction_status")]
#[enum_original_mapping(RewardTypeOriginal)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionReturnData(TransactionReturnDataOriginal);
transaction_status_boilerplate!(TransactionReturnData);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionReturnData {
    #[new]
    pub fn new(program_id: Pubkey, data: Vec<u8>) -> Self {
        TransactionReturnDataOriginal {
            program_id: program_id.into(),
            data,
        }
        .into()
    }

    #[getter]
    pub fn program_id(&self) -> Pubkey {
        self.0.program_id.into()
    }

    #[getter]
    pub fn data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.data)
    }
}

/// A duplicate representation of TransactionStatusMeta with `err` field
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransactionStatusMeta(UiTransactionStatusMetaOriginal);
transaction_status_boilerplate!(UiTransactionStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTransactionStatusMeta {
    #[pyo3(
        signature = (err, fee, pre_balances, post_balances, inner_instructions=None, log_messages=None, pre_token_balances=None, post_token_balances=None, rewards=None, loaded_addresses=None, return_data=None)
    )]
    #[new]
    pub fn new(
        err: Option<TransactionErrorType>,
        fee: u64,
        pre_balances: Vec<u64>,
        post_balances: Vec<u64>,
        inner_instructions: Option<Vec<UiInnerInstructions>>,
        log_messages: Option<Vec<String>>,
        pre_token_balances: Option<Vec<UiTransactionTokenBalance>>,
        post_token_balances: Option<Vec<UiTransactionTokenBalance>>,
        rewards: Option<Rewards>,
        loaded_addresses: Option<UiLoadedAddresses>,
        return_data: Option<TransactionReturnData>,
    ) -> Self {
        UiTransactionStatusMetaOriginal {
            err: err.map(|e| e.into()),
            status: Ok(()),
            fee,
            pre_balances,
            post_balances,
            inner_instructions: inner_instructions
                .map(|v| v.into_iter().map(|ix| ix.into()).collect()),
            log_messages,
            pre_token_balances: pre_token_balances
                .map(|v| v.into_iter().map(|bal| bal.into()).collect()),
            post_token_balances: post_token_balances
                .map(|v| v.into_iter().map(|bal| bal.into()).collect()),
            rewards: rewards.map(|v| v.into_iter().map(|r| r.into()).collect()),
            loaded_addresses: loaded_addresses.map(|a| a.into()),
            return_data: return_data.map(|r| r.into()),
        }
        .into()
    }

    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.0.err.clone().map(|e| e.into())
    }
    #[getter]
    pub fn fee(&self) -> u64 {
        self.0.fee
    }
    #[getter]
    pub fn pre_balances(&self) -> Vec<u64> {
        self.0.pre_balances.clone()
    }
    #[getter]
    pub fn post_balances(&self) -> Vec<u64> {
        self.0.post_balances.clone()
    }
    #[getter]
    pub fn inner_instructions(&self) -> Option<Vec<UiInnerInstructions>> {
        self.0
            .inner_instructions
            .clone()
            .map(|v| v.into_iter().map(|ix| ix.into()).collect())
    }
    #[getter]
    pub fn log_messages(&self) -> Option<Vec<String>> {
        self.0.log_messages.clone()
    }
    #[getter]
    pub fn pre_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        self.0
            .pre_token_balances
            .clone()
            .map(|v| v.into_iter().map(|bal| bal.into()).collect())
    }
    #[getter]
    pub fn post_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        self.0
            .post_token_balances
            .clone()
            .map(|v| v.into_iter().map(|bal| bal.into()).collect())
    }
    #[getter]
    pub fn rewards(&self) -> Option<Rewards> {
        self.0
            .rewards
            .clone()
            .map(|v| v.into_iter().map(|r| r.into()).collect())
    }
    #[getter]
    pub fn loaded_addresses(&self) -> Option<UiLoadedAddresses> {
        self.0.loaded_addresses.clone().map(|a| a.into())
    }
    #[getter]
    pub fn return_data(&self) -> Option<TransactionReturnData> {
        self.0.return_data.clone().map(|r| r.into())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct EncodedTransactionWithStatusMeta(EncodedTransactionWithStatusMetaOriginal);

transaction_status_boilerplate!(EncodedTransactionWithStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EncodedTransactionWithStatusMeta {
    #[new]
    pub fn new(
        transaction: EncodedVersionedTransaction,
        meta: Option<UiTransactionStatusMeta>,
        version: Option<TransactionVersion>,
    ) -> Self {
        EncodedTransactionWithStatusMetaOriginal {
            transaction: transaction.into(),
            meta: meta.map(|m| m.into()),
            version: version.map(|v| v.into()),
        }
        .into()
    }

    #[getter]
    pub fn transaction(&self) -> EncodedVersionedTransaction {
        self.0.transaction.clone().into()
    }

    #[getter]
    pub fn meta(&self) -> Option<UiTransactionStatusMeta> {
        self.0.meta.clone().map(|t| t.into())
    }

    #[getter]
    pub fn version(&self) -> Option<TransactionVersion> {
        self.0.version.clone().map(|v| v.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct InstructionErrorCustom(pub u32);

transaction_status_boilerplate!(InstructionErrorCustom);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl InstructionErrorCustom {
    #[new]
    pub fn new(code: u32) -> Self {
        Self(code)
    }

    #[getter]
    pub fn code(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct InstructionErrorBorshIO(pub String);
transaction_status_boilerplate!(InstructionErrorBorshIO);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl InstructionErrorBorshIO {
    #[new]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    #[getter]
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[pyclass(module = "solders.transaction_status")]
pub enum InstructionErrorFieldless {
    GenericError,
    InvalidArgument,
    InvalidInstructionData,
    InvalidAccountData,
    AccountDataTooSmall,
    InsufficientFunds,
    IncorrectProgramId,
    MissingRequiredSignature,
    AccountAlreadyInitialized,
    UninitializedAccount,
    UnbalancedInstruction,
    ModifiedProgramId,
    ExternalAccountLamportSpend,
    ExternalAccountDataModified,
    ReadonlyLamportChange,
    ReadonlyDataModified,
    DuplicateAccountIndex,
    ExecutableModified,
    RentEpochModified,
    NotEnoughAccountKeys,
    AccountDataSizeChanged,
    AccountNotExecutable,
    AccountBorrowFailed,
    AccountBorrowOutstanding,
    DuplicateAccountOutOfSync,
    InvalidError,
    ExecutableDataModified,
    ExecutableLamportChange,
    ExecutableAccountNotRentExempt,
    UnsupportedProgramId,
    CallDepth,
    MissingAccount,
    ReentrancyNotAllowed,
    MaxSeedLengthExceeded,
    InvalidSeeds,
    InvalidRealloc,
    ComputationalBudgetExceeded,
    PrivilegeEscalation,
    ProgramEnvironmentSetupFailure,
    ProgramFailedToComplete,
    ProgramFailedToCompile,
    Immutable,
    IncorrectAuthority,
    AccountNotRentExempt,
    InvalidAccountOwner,
    ArithmeticOverflow,
    UnsupportedSysvar,
    IllegalOwner,
    MaxAccountsDataSizeExceeded,
    MaxAccountsExceeded,
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
pub enum InstructionErrorTagged {
    Custom(InstructionErrorCustom),
    BorshIoError(InstructionErrorBorshIO),
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
#[serde(untagged)]
pub enum InstructionErrorType {
    Fieldless(InstructionErrorFieldless),
    Tagged(InstructionErrorTagged),
}

impl Default for InstructionErrorType {
    fn default() -> Self {
        Self::Fieldless(InstructionErrorFieldless::GenericError)
    }
}

impl From<InstructionErrorType> for InstructionErrorOriginal {
    fn from(w: InstructionErrorType) -> Self {
        match w {
            InstructionErrorType::Tagged(t) => match t {
                InstructionErrorTagged::Custom(custom) => Self::Custom(custom.0),
                InstructionErrorTagged::BorshIoError(borsh_io) => Self::BorshIoError(borsh_io.0),
            },
            InstructionErrorType::Fieldless(f) => match f {
                InstructionErrorFieldless::GenericError => Self::GenericError,
                InstructionErrorFieldless::InvalidArgument => Self::InvalidArgument,
                InstructionErrorFieldless::InvalidInstructionData => Self::InvalidInstructionData,
                InstructionErrorFieldless::InvalidAccountData => Self::InvalidAccountData,
                InstructionErrorFieldless::AccountDataTooSmall => Self::AccountDataTooSmall,
                InstructionErrorFieldless::InsufficientFunds => Self::InsufficientFunds,
                InstructionErrorFieldless::IncorrectProgramId => Self::IncorrectProgramId,
                InstructionErrorFieldless::MissingRequiredSignature => {
                    Self::MissingRequiredSignature
                }
                InstructionErrorFieldless::AccountAlreadyInitialized => {
                    Self::AccountAlreadyInitialized
                }
                InstructionErrorFieldless::UninitializedAccount => Self::UninitializedAccount,
                InstructionErrorFieldless::UnbalancedInstruction => Self::UnbalancedInstruction,
                InstructionErrorFieldless::ModifiedProgramId => Self::ModifiedProgramId,
                InstructionErrorFieldless::ExternalAccountLamportSpend => {
                    Self::ExternalAccountLamportSpend
                }
                InstructionErrorFieldless::ExternalAccountDataModified => {
                    Self::ExternalAccountDataModified
                }
                InstructionErrorFieldless::ReadonlyLamportChange => Self::ReadonlyLamportChange,
                InstructionErrorFieldless::ReadonlyDataModified => Self::ReadonlyDataModified,
                InstructionErrorFieldless::DuplicateAccountIndex => Self::DuplicateAccountIndex,
                InstructionErrorFieldless::ExecutableModified => Self::ExecutableModified,
                InstructionErrorFieldless::RentEpochModified => Self::RentEpochModified,
                InstructionErrorFieldless::NotEnoughAccountKeys => Self::NotEnoughAccountKeys,
                InstructionErrorFieldless::AccountDataSizeChanged => Self::AccountDataSizeChanged,
                InstructionErrorFieldless::AccountNotExecutable => Self::AccountNotExecutable,
                InstructionErrorFieldless::AccountBorrowFailed => Self::AccountBorrowFailed,
                InstructionErrorFieldless::AccountBorrowOutstanding => {
                    Self::AccountBorrowOutstanding
                }
                InstructionErrorFieldless::DuplicateAccountOutOfSync => {
                    Self::DuplicateAccountOutOfSync
                }
                InstructionErrorFieldless::InvalidError => Self::InvalidError,
                InstructionErrorFieldless::ExecutableDataModified => Self::ExecutableDataModified,
                InstructionErrorFieldless::ExecutableLamportChange => Self::ExecutableLamportChange,
                InstructionErrorFieldless::ExecutableAccountNotRentExempt => {
                    Self::ExecutableAccountNotRentExempt
                }
                InstructionErrorFieldless::UnsupportedProgramId => Self::UnsupportedProgramId,
                InstructionErrorFieldless::CallDepth => Self::CallDepth,
                InstructionErrorFieldless::MissingAccount => Self::MissingAccount,
                InstructionErrorFieldless::ReentrancyNotAllowed => Self::ReentrancyNotAllowed,
                InstructionErrorFieldless::MaxSeedLengthExceeded => Self::MaxSeedLengthExceeded,
                InstructionErrorFieldless::InvalidSeeds => Self::InvalidSeeds,
                InstructionErrorFieldless::InvalidRealloc => Self::InvalidRealloc,
                InstructionErrorFieldless::ComputationalBudgetExceeded => {
                    Self::ComputationalBudgetExceeded
                }
                InstructionErrorFieldless::PrivilegeEscalation => Self::PrivilegeEscalation,
                InstructionErrorFieldless::ProgramEnvironmentSetupFailure => {
                    Self::ProgramEnvironmentSetupFailure
                }
                InstructionErrorFieldless::ProgramFailedToComplete => Self::ProgramFailedToComplete,
                InstructionErrorFieldless::ProgramFailedToCompile => Self::ProgramFailedToCompile,
                InstructionErrorFieldless::Immutable => Self::Immutable,
                InstructionErrorFieldless::IncorrectAuthority => Self::IncorrectAuthority,
                InstructionErrorFieldless::AccountNotRentExempt => Self::AccountNotRentExempt,
                InstructionErrorFieldless::InvalidAccountOwner => Self::InvalidAccountOwner,
                InstructionErrorFieldless::ArithmeticOverflow => Self::ArithmeticOverflow,
                InstructionErrorFieldless::UnsupportedSysvar => Self::UnsupportedSysvar,
                InstructionErrorFieldless::IllegalOwner => Self::IllegalOwner,
                InstructionErrorFieldless::MaxAccountsDataSizeExceeded => {
                    Self::MaxAccountsDataSizeExceeded
                }
                InstructionErrorFieldless::MaxAccountsExceeded => Self::MaxAccountsExceeded,
            },
        }
    }
}

impl From<InstructionErrorOriginal> for InstructionErrorType {
    fn from(e: InstructionErrorOriginal) -> Self {
        match e {
            InstructionErrorOriginal::Custom(code) => {
                Self::Tagged(InstructionErrorTagged::Custom(InstructionErrorCustom(code)))
            }
            InstructionErrorOriginal::BorshIoError(val) => Self::Tagged(
                InstructionErrorTagged::BorshIoError(InstructionErrorBorshIO(val)),
            ),
            InstructionErrorOriginal::GenericError => {
                Self::Fieldless(InstructionErrorFieldless::GenericError)
            }
            InstructionErrorOriginal::InvalidArgument => {
                Self::Fieldless(InstructionErrorFieldless::InvalidArgument)
            }
            InstructionErrorOriginal::InvalidInstructionData => {
                Self::Fieldless(InstructionErrorFieldless::InvalidInstructionData)
            }
            InstructionErrorOriginal::InvalidAccountData => {
                Self::Fieldless(InstructionErrorFieldless::InvalidAccountData)
            }
            InstructionErrorOriginal::AccountDataTooSmall => {
                Self::Fieldless(InstructionErrorFieldless::AccountDataTooSmall)
            }
            InstructionErrorOriginal::InsufficientFunds => {
                Self::Fieldless(InstructionErrorFieldless::InsufficientFunds)
            }
            InstructionErrorOriginal::IncorrectProgramId => {
                Self::Fieldless(InstructionErrorFieldless::IncorrectProgramId)
            }
            InstructionErrorOriginal::MissingRequiredSignature => {
                Self::Fieldless(InstructionErrorFieldless::MissingRequiredSignature)
            }
            InstructionErrorOriginal::AccountAlreadyInitialized => {
                Self::Fieldless(InstructionErrorFieldless::AccountAlreadyInitialized)
            }
            InstructionErrorOriginal::UninitializedAccount => {
                Self::Fieldless(InstructionErrorFieldless::UninitializedAccount)
            }
            InstructionErrorOriginal::UnbalancedInstruction => {
                Self::Fieldless(InstructionErrorFieldless::UnbalancedInstruction)
            }
            InstructionErrorOriginal::ModifiedProgramId => {
                Self::Fieldless(InstructionErrorFieldless::ModifiedProgramId)
            }
            InstructionErrorOriginal::ExternalAccountLamportSpend => {
                Self::Fieldless(InstructionErrorFieldless::ExternalAccountLamportSpend)
            }
            InstructionErrorOriginal::ExternalAccountDataModified => {
                Self::Fieldless(InstructionErrorFieldless::ExternalAccountDataModified)
            }
            InstructionErrorOriginal::ReadonlyLamportChange => {
                Self::Fieldless(InstructionErrorFieldless::ReadonlyLamportChange)
            }
            InstructionErrorOriginal::ReadonlyDataModified => {
                Self::Fieldless(InstructionErrorFieldless::ReadonlyDataModified)
            }
            InstructionErrorOriginal::DuplicateAccountIndex => {
                Self::Fieldless(InstructionErrorFieldless::DuplicateAccountIndex)
            }
            InstructionErrorOriginal::ExecutableModified => {
                Self::Fieldless(InstructionErrorFieldless::ExecutableModified)
            }
            InstructionErrorOriginal::RentEpochModified => {
                Self::Fieldless(InstructionErrorFieldless::RentEpochModified)
            }
            InstructionErrorOriginal::NotEnoughAccountKeys => {
                Self::Fieldless(InstructionErrorFieldless::NotEnoughAccountKeys)
            }
            InstructionErrorOriginal::AccountDataSizeChanged => {
                Self::Fieldless(InstructionErrorFieldless::AccountDataSizeChanged)
            }
            InstructionErrorOriginal::AccountNotExecutable => {
                Self::Fieldless(InstructionErrorFieldless::AccountNotExecutable)
            }
            InstructionErrorOriginal::AccountBorrowFailed => {
                Self::Fieldless(InstructionErrorFieldless::AccountBorrowFailed)
            }
            InstructionErrorOriginal::AccountBorrowOutstanding => {
                Self::Fieldless(InstructionErrorFieldless::AccountBorrowOutstanding)
            }
            InstructionErrorOriginal::DuplicateAccountOutOfSync => {
                Self::Fieldless(InstructionErrorFieldless::DuplicateAccountOutOfSync)
            }
            InstructionErrorOriginal::InvalidError => {
                Self::Fieldless(InstructionErrorFieldless::InvalidError)
            }
            InstructionErrorOriginal::ExecutableDataModified => {
                Self::Fieldless(InstructionErrorFieldless::ExecutableDataModified)
            }
            InstructionErrorOriginal::ExecutableLamportChange => {
                Self::Fieldless(InstructionErrorFieldless::ExecutableLamportChange)
            }
            InstructionErrorOriginal::ExecutableAccountNotRentExempt => {
                Self::Fieldless(InstructionErrorFieldless::ExecutableAccountNotRentExempt)
            }
            InstructionErrorOriginal::UnsupportedProgramId => {
                Self::Fieldless(InstructionErrorFieldless::UnsupportedProgramId)
            }
            InstructionErrorOriginal::CallDepth => {
                Self::Fieldless(InstructionErrorFieldless::CallDepth)
            }
            InstructionErrorOriginal::MissingAccount => {
                Self::Fieldless(InstructionErrorFieldless::MissingAccount)
            }
            InstructionErrorOriginal::ReentrancyNotAllowed => {
                Self::Fieldless(InstructionErrorFieldless::ReentrancyNotAllowed)
            }
            InstructionErrorOriginal::MaxSeedLengthExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxSeedLengthExceeded)
            }
            InstructionErrorOriginal::InvalidSeeds => {
                Self::Fieldless(InstructionErrorFieldless::InvalidSeeds)
            }
            InstructionErrorOriginal::InvalidRealloc => {
                Self::Fieldless(InstructionErrorFieldless::InvalidRealloc)
            }
            InstructionErrorOriginal::ComputationalBudgetExceeded => {
                Self::Fieldless(InstructionErrorFieldless::ComputationalBudgetExceeded)
            }
            InstructionErrorOriginal::PrivilegeEscalation => {
                Self::Fieldless(InstructionErrorFieldless::PrivilegeEscalation)
            }
            InstructionErrorOriginal::ProgramEnvironmentSetupFailure => {
                Self::Fieldless(InstructionErrorFieldless::ProgramEnvironmentSetupFailure)
            }
            InstructionErrorOriginal::ProgramFailedToComplete => {
                Self::Fieldless(InstructionErrorFieldless::ProgramFailedToComplete)
            }
            InstructionErrorOriginal::ProgramFailedToCompile => {
                Self::Fieldless(InstructionErrorFieldless::ProgramFailedToCompile)
            }
            InstructionErrorOriginal::Immutable => {
                Self::Fieldless(InstructionErrorFieldless::Immutable)
            }
            InstructionErrorOriginal::IncorrectAuthority => {
                Self::Fieldless(InstructionErrorFieldless::IncorrectAuthority)
            }
            InstructionErrorOriginal::AccountNotRentExempt => {
                Self::Fieldless(InstructionErrorFieldless::AccountNotRentExempt)
            }
            InstructionErrorOriginal::InvalidAccountOwner => {
                Self::Fieldless(InstructionErrorFieldless::InvalidAccountOwner)
            }
            InstructionErrorOriginal::ArithmeticOverflow => {
                Self::Fieldless(InstructionErrorFieldless::ArithmeticOverflow)
            }
            InstructionErrorOriginal::UnsupportedSysvar => {
                Self::Fieldless(InstructionErrorFieldless::UnsupportedSysvar)
            }
            InstructionErrorOriginal::IllegalOwner => {
                Self::Fieldless(InstructionErrorFieldless::IllegalOwner)
            }
            InstructionErrorOriginal::MaxAccountsDataSizeExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxAccountsDataSizeExceeded)
            }
            InstructionErrorOriginal::MaxAccountsExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxAccountsExceeded)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionErrorInstructionError(pub (u8, InstructionErrorType));
transaction_status_boilerplate!(TransactionErrorInstructionError);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionErrorInstructionError {
    #[new]
    pub fn new(index: u8, err: InstructionErrorType) -> Self {
        Self((index, err))
    }

    #[getter]
    pub fn index(&self) -> u8 {
        self.0 .0
    }

    #[getter]
    pub fn err(&self) -> InstructionErrorType {
        self.0 .1.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionErrorDuplicateInstruction(pub u8);
transaction_status_boilerplate!(TransactionErrorDuplicateInstruction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionErrorDuplicateInstruction {
    #[new]
    pub fn new(index: u8) -> Self {
        Self(index)
    }

    #[getter]
    pub fn index(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionErrorInsufficientFundsForRent {
    #[pyo3(get)]
    account_index: u8,
}
transaction_status_boilerplate!(TransactionErrorInsufficientFundsForRent);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionErrorInsufficientFundsForRent {
    #[new]
    pub fn new(account_index: u8) -> Self {
        Self { account_index }
    }
}

#[pyclass(module = "solders.transaction_status")]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransactionErrorFieldless {
    AccountInUse,
    AccountLoadedTwice,
    AccountNotFound,
    ProgramAccountNotFound,
    InsufficientFundsForFee,
    InvalidAccountForFee,
    AlreadyProcessed,
    BlockhashNotFound,
    CallChainTooDeep,
    MissingSignatureForFee,
    InvalidAccountIndex,
    SignatureFailure,
    InvalidProgramForExecution,
    SanitizeFailure,
    ClusterMaintenance,
    AccountBorrowOutstanding,
    WouldExceedMaxBlockCostLimit,
    UnsupportedVersion,
    InvalidWritableAccount,
    WouldExceedMaxAccountCostLimit,
    WouldExceedAccountDataBlockLimit,
    TooManyAccountLocks,
    AddressLookupTableNotFound,
    InvalidAddressLookupTableOwner,
    InvalidAddressLookupTableData,
    InvalidAddressLookupTableIndex,
    InvalidRentPayingAccount,
    WouldExceedMaxVoteCostLimit,
    WouldExceedAccountDataTotalLimit,
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
pub enum TransactionErrorTypeTagged {
    InstructionError(TransactionErrorInstructionError),
    DuplicateInstruction(TransactionErrorDuplicateInstruction),
    InsufficientFundsForRent(TransactionErrorInsufficientFundsForRent),
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
#[serde(untagged)]
pub enum TransactionErrorType {
    Fieldless(TransactionErrorFieldless),
    Tagged(TransactionErrorTypeTagged),
}

impl Default for TransactionErrorType {
    fn default() -> Self {
        Self::Fieldless(TransactionErrorFieldless::AccountInUse)
    }
}

impl From<TransactionErrorType> for TransactionErrorOriginal {
    fn from(w: TransactionErrorType) -> Self {
        match w {
            TransactionErrorType::Tagged(t) => match t {
                TransactionErrorTypeTagged::InstructionError(e) => {
                    Self::InstructionError(e.0 .0, e.0 .1.into())
                }
                TransactionErrorTypeTagged::DuplicateInstruction(e) => {
                    Self::DuplicateInstruction(e.0)
                }
                TransactionErrorTypeTagged::InsufficientFundsForRent(e) => {
                    Self::InsufficientFundsForRent {
                        account_index: e.account_index,
                    }
                }
            },
            TransactionErrorType::Fieldless(f) => match f {
                TransactionErrorFieldless::AccountInUse => Self::AccountInUse,
                TransactionErrorFieldless::AccountLoadedTwice => Self::AccountLoadedTwice,
                TransactionErrorFieldless::AccountNotFound => Self::AccountNotFound,
                TransactionErrorFieldless::ProgramAccountNotFound => Self::ProgramAccountNotFound,
                TransactionErrorFieldless::InsufficientFundsForFee => Self::InsufficientFundsForFee,
                TransactionErrorFieldless::InvalidAccountForFee => Self::InvalidAccountForFee,
                TransactionErrorFieldless::AlreadyProcessed => Self::AlreadyProcessed,
                TransactionErrorFieldless::BlockhashNotFound => Self::BlockhashNotFound,
                TransactionErrorFieldless::CallChainTooDeep => Self::CallChainTooDeep,
                TransactionErrorFieldless::MissingSignatureForFee => Self::MissingSignatureForFee,
                TransactionErrorFieldless::InvalidAccountIndex => Self::InvalidAccountIndex,
                TransactionErrorFieldless::SignatureFailure => Self::SignatureFailure,
                TransactionErrorFieldless::InvalidProgramForExecution => {
                    Self::InvalidProgramForExecution
                }
                TransactionErrorFieldless::SanitizeFailure => Self::SanitizeFailure,
                TransactionErrorFieldless::ClusterMaintenance => Self::ClusterMaintenance,
                TransactionErrorFieldless::AccountBorrowOutstanding => {
                    Self::AccountBorrowOutstanding
                }
                TransactionErrorFieldless::WouldExceedMaxBlockCostLimit => {
                    Self::WouldExceedMaxBlockCostLimit
                }
                TransactionErrorFieldless::UnsupportedVersion => Self::UnsupportedVersion,
                TransactionErrorFieldless::InvalidWritableAccount => Self::InvalidWritableAccount,
                TransactionErrorFieldless::WouldExceedMaxAccountCostLimit => {
                    Self::WouldExceedMaxAccountCostLimit
                }
                TransactionErrorFieldless::WouldExceedAccountDataBlockLimit => {
                    Self::WouldExceedAccountDataBlockLimit
                }
                TransactionErrorFieldless::TooManyAccountLocks => Self::TooManyAccountLocks,
                TransactionErrorFieldless::AddressLookupTableNotFound => {
                    Self::AddressLookupTableNotFound
                }
                TransactionErrorFieldless::InvalidAddressLookupTableOwner => {
                    Self::InvalidAddressLookupTableOwner
                }
                TransactionErrorFieldless::InvalidAddressLookupTableData => {
                    Self::InvalidAddressLookupTableData
                }
                TransactionErrorFieldless::InvalidAddressLookupTableIndex => {
                    Self::InvalidAddressLookupTableIndex
                }
                TransactionErrorFieldless::InvalidRentPayingAccount => {
                    Self::InvalidRentPayingAccount
                }
                TransactionErrorFieldless::WouldExceedMaxVoteCostLimit => {
                    Self::WouldExceedMaxVoteCostLimit
                }
                TransactionErrorFieldless::WouldExceedAccountDataTotalLimit => {
                    Self::WouldExceedAccountDataTotalLimit
                }
            },
        }
    }
}

impl From<TransactionErrorOriginal> for TransactionErrorType {
    fn from(w: TransactionErrorOriginal) -> Self {
        match w {
            TransactionErrorOriginal::InstructionError(index, err) => {
                Self::Tagged(TransactionErrorTypeTagged::InstructionError(
                    TransactionErrorInstructionError((index, err.into())),
                ))
            }
            TransactionErrorOriginal::DuplicateInstruction(index) => {
                Self::Tagged(TransactionErrorTypeTagged::DuplicateInstruction(
                    TransactionErrorDuplicateInstruction(index),
                ))
            }
            TransactionErrorOriginal::InsufficientFundsForRent { account_index } => {
                Self::Tagged(TransactionErrorTypeTagged::InsufficientFundsForRent(
                    TransactionErrorInsufficientFundsForRent { account_index },
                ))
            }
            TransactionErrorOriginal::AccountInUse => {
                Self::Fieldless(TransactionErrorFieldless::AccountInUse)
            }
            TransactionErrorOriginal::AccountLoadedTwice => {
                Self::Fieldless(TransactionErrorFieldless::AccountLoadedTwice)
            }
            TransactionErrorOriginal::AccountNotFound => {
                Self::Fieldless(TransactionErrorFieldless::AccountNotFound)
            }
            TransactionErrorOriginal::ProgramAccountNotFound => {
                Self::Fieldless(TransactionErrorFieldless::ProgramAccountNotFound)
            }
            TransactionErrorOriginal::InsufficientFundsForFee => {
                Self::Fieldless(TransactionErrorFieldless::InsufficientFundsForFee)
            }
            TransactionErrorOriginal::InvalidAccountForFee => {
                Self::Fieldless(TransactionErrorFieldless::InvalidAccountForFee)
            }
            TransactionErrorOriginal::AlreadyProcessed => {
                Self::Fieldless(TransactionErrorFieldless::AlreadyProcessed)
            }
            TransactionErrorOriginal::BlockhashNotFound => {
                Self::Fieldless(TransactionErrorFieldless::BlockhashNotFound)
            }
            TransactionErrorOriginal::CallChainTooDeep => {
                Self::Fieldless(TransactionErrorFieldless::CallChainTooDeep)
            }
            TransactionErrorOriginal::MissingSignatureForFee => {
                Self::Fieldless(TransactionErrorFieldless::MissingSignatureForFee)
            }
            TransactionErrorOriginal::InvalidAccountIndex => {
                Self::Fieldless(TransactionErrorFieldless::InvalidAccountIndex)
            }
            TransactionErrorOriginal::SignatureFailure => {
                Self::Fieldless(TransactionErrorFieldless::SignatureFailure)
            }
            TransactionErrorOriginal::InvalidProgramForExecution => {
                Self::Fieldless(TransactionErrorFieldless::InvalidProgramForExecution)
            }
            TransactionErrorOriginal::SanitizeFailure => {
                Self::Fieldless(TransactionErrorFieldless::SanitizeFailure)
            }
            TransactionErrorOriginal::ClusterMaintenance => {
                Self::Fieldless(TransactionErrorFieldless::ClusterMaintenance)
            }
            TransactionErrorOriginal::AccountBorrowOutstanding => {
                Self::Fieldless(TransactionErrorFieldless::AccountBorrowOutstanding)
            }
            TransactionErrorOriginal::WouldExceedMaxBlockCostLimit => {
                Self::Fieldless(TransactionErrorFieldless::WouldExceedMaxBlockCostLimit)
            }
            TransactionErrorOriginal::UnsupportedVersion => {
                Self::Fieldless(TransactionErrorFieldless::UnsupportedVersion)
            }
            TransactionErrorOriginal::InvalidWritableAccount => {
                Self::Fieldless(TransactionErrorFieldless::InvalidWritableAccount)
            }
            TransactionErrorOriginal::WouldExceedMaxAccountCostLimit => {
                Self::Fieldless(TransactionErrorFieldless::WouldExceedMaxAccountCostLimit)
            }
            TransactionErrorOriginal::WouldExceedAccountDataBlockLimit => {
                Self::Fieldless(TransactionErrorFieldless::WouldExceedAccountDataBlockLimit)
            }
            TransactionErrorOriginal::TooManyAccountLocks => {
                Self::Fieldless(TransactionErrorFieldless::TooManyAccountLocks)
            }
            TransactionErrorOriginal::AddressLookupTableNotFound => {
                Self::Fieldless(TransactionErrorFieldless::AddressLookupTableNotFound)
            }
            TransactionErrorOriginal::InvalidAddressLookupTableOwner => {
                Self::Fieldless(TransactionErrorFieldless::InvalidAddressLookupTableOwner)
            }
            TransactionErrorOriginal::InvalidAddressLookupTableData => {
                Self::Fieldless(TransactionErrorFieldless::InvalidAddressLookupTableData)
            }
            TransactionErrorOriginal::InvalidAddressLookupTableIndex => {
                Self::Fieldless(TransactionErrorFieldless::InvalidAddressLookupTableIndex)
            }
            TransactionErrorOriginal::InvalidRentPayingAccount => {
                Self::Fieldless(TransactionErrorFieldless::InvalidRentPayingAccount)
            }
            TransactionErrorOriginal::WouldExceedMaxVoteCostLimit => {
                Self::Fieldless(TransactionErrorFieldless::WouldExceedMaxVoteCostLimit)
            }
            TransactionErrorOriginal::WouldExceedAccountDataTotalLimit => {
                Self::Fieldless(TransactionErrorFieldless::WouldExceedAccountDataTotalLimit)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct Reward(RewardOriginal);

transaction_status_boilerplate!(Reward);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Reward {
    #[new]
    pub fn new(
        pubkey: Pubkey,
        lamports: i64,
        post_balance: u64, // Account balance in lamports after `lamports` was applied
        reward_type: Option<RewardType>,
        commission: Option<u8>,
    ) -> Self {
        RewardOriginal {
            pubkey: pubkey.to_string(),
            lamports,
            post_balance,
            reward_type: reward_type.map(|r| r.into()),
            commission,
        }
        .into()
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.pubkey).unwrap()
    }

    #[getter]
    pub fn lamports(&self) -> i64 {
        self.0.lamports
    }

    #[getter]
    pub fn post_balance(&self) -> u64 {
        self.0.post_balance
    }

    #[getter]
    pub fn reward_type(&self) -> Option<RewardType> {
        self.0.reward_type.map(|r| r.into())
    }

    #[getter]
    pub fn commission(&self) -> Option<u8> {
        self.0.commission
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(TransactionConfirmationStatusOriginal)]
#[pyclass(module = "solders.transaction_status")]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}

pub type Rewards = Vec<Reward>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionStatus(TransactionStatusOriginal);

transaction_status_boilerplate!(TransactionStatus);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionStatus {
    #[new]
    pub fn new(
        slot: Slot,
        confirmations: Option<usize>,
        status: Option<TransactionErrorType>,
        err: Option<TransactionErrorType>,
        confirmation_status: Option<TransactionConfirmationStatus>,
    ) -> Self {
        TransactionStatusOriginal {
            slot,
            confirmations,
            status: status.map_or(Ok(()), |e| Err(e.into())),
            err: err.map(Into::into),
            confirmation_status: confirmation_status.map(Into::into),
        }
        .into()
    }

    #[getter]
    pub fn slot(&self) -> Slot {
        self.0.slot
    }
    #[getter]
    pub fn confirmations(&self) -> Option<usize> {
        self.0.confirmations
    }
    #[getter]
    pub fn status(&self) -> Option<TransactionErrorType> {
        self.0
            .status
            .clone()
            .map_or_else(|e| Some(e.into()), |_s| None)
    }
    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.0.err.clone().map(Into::into)
    }
    #[getter]
    pub fn confirmation_status(&self) -> Option<TransactionConfirmationStatus> {
        self.0.confirmation_status.clone().map(Into::into)
    }

    pub fn satisfies_commitment(&self, commitment_config: CommitmentConfig) -> bool {
        self.0.satisfies_commitment(commitment_config.into())
    }

    pub fn find_confirmation_status(&self) -> TransactionConfirmationStatus {
        self.0.confirmation_status().into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct EncodedConfirmedTransactionWithStatusMeta(
    EncodedConfirmedTransactionWithStatusMetaOriginal,
);

transaction_status_boilerplate!(EncodedConfirmedTransactionWithStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EncodedConfirmedTransactionWithStatusMeta {
    #[new]
    pub fn new(
        slot: Slot,
        transaction: EncodedTransactionWithStatusMeta,
        block_time: Option<UnixTimestamp>,
    ) -> Self {
        EncodedConfirmedTransactionWithStatusMetaOriginal {
            slot,
            transaction: transaction.into(),
            block_time,
        }
        .into()
    }

    #[getter]
    pub fn slot(&self) -> Slot {
        self.0.slot
    }

    #[getter]
    pub fn transaction(&self) -> EncodedTransactionWithStatusMeta {
        self.0.transaction.clone().into()
    }

    #[getter]
    pub fn block_time(&self) -> Option<UnixTimestamp> {
        self.0.block_time
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiConfirmedBlock(UiConfirmedBlockOriginal);

transaction_status_boilerplate!(UiConfirmedBlock);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiConfirmedBlock {
    #[new]
    pub fn new(
        previous_blockhash: SolderHash,
        blockhash: SolderHash,
        parent_slot: Slot,
        transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
        signatures: Option<Vec<Signature>>,
        rewards: Option<Rewards>,
        block_time: Option<UnixTimestamp>,
        block_height: Option<u64>,
    ) -> Self {
        UiConfirmedBlockOriginal {
            previous_blockhash: previous_blockhash.to_string(),
            blockhash: blockhash.to_string(),
            parent_slot,
            transactions: transactions.map(|txs| txs.into_iter().map(|tx| tx.into()).collect()),
            signatures: signatures.map(|sigs| sigs.iter().map(|sig| sig.to_string()).collect()),
            rewards: rewards.map(|v| v.into_iter().map(|r| r.into()).collect()),
            block_time,
            block_height,
        }
        .into()
    }

    #[getter]
    pub fn previous_blockhash(&self) -> SolderHash {
        self.0.previous_blockhash.parse().unwrap()
    }

    #[getter]
    pub fn blockhash(&self) -> SolderHash {
        self.0.blockhash.parse().unwrap()
    }

    #[getter]
    pub fn parent_slot(&self) -> Slot {
        self.0.parent_slot
    }

    #[getter]
    pub fn transactions(&self) -> Option<Vec<EncodedTransactionWithStatusMeta>> {
        self.0
            .transactions
            .clone()
            .map(|txs| txs.into_iter().map(|tx| tx.into()).collect())
    }
    #[getter]
    pub fn signatures(&self) -> Option<Vec<Signature>> {
        self.0
            .signatures
            .clone()
            .map(|sigs| sigs.iter().map(|sig| sig.parse().unwrap()).collect())
    }
    #[getter]
    pub fn rewards(&self) -> Option<Rewards> {
        self.0
            .rewards
            .clone()
            .map(|v| v.into_iter().map(|r| r.into()).collect())
    }
    #[getter]
    pub fn block_time(&self) -> Option<UnixTimestamp> {
        self.0.block_time
    }
    #[getter]
    pub fn block_height(&self) -> Option<u64> {
        self.0.block_height
    }
}

pub fn create_transaction_status_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "transaction_status")?;
    m.add_class::<TransactionDetails>()?;
    m.add_class::<UiTransactionEncoding>()?;
    m.add_class::<TransactionBinaryEncoding>()?;
    m.add_class::<UiCompiledInstruction>()?;
    m.add_class::<UiAddressTableLookup>()?;
    m.add_class::<UiRawMessage>()?;
    m.add_class::<ParsedAccount>()?;
    m.add_class::<ParsedInstruction>()?;
    m.add_class::<UiPartiallyDecodedInstruction>()?;
    m.add_class::<UiParsedMessage>()?;
    m.add_class::<UiTransaction>()?;
    m.add_class::<UiInnerInstructions>()?;
    m.add_class::<UiLoadedAddresses>()?;
    m.add_class::<UiTransactionTokenBalance>()?;
    m.add_class::<RewardType>()?;
    m.add_class::<TransactionReturnData>()?;
    m.add_class::<UiTransactionStatusMeta>()?;
    m.add_class::<EncodedTransactionWithStatusMeta>()?;
    m.add_class::<InstructionErrorCustom>()?;
    m.add_class::<InstructionErrorBorshIO>()?;
    m.add_class::<InstructionErrorFieldless>()?;
    m.add_class::<TransactionErrorInstructionError>()?;
    m.add_class::<TransactionErrorDuplicateInstruction>()?;
    m.add_class::<TransactionErrorInsufficientFundsForRent>()?;
    m.add_class::<TransactionErrorFieldless>()?;
    m.add_class::<Reward>()?;
    m.add_class::<TransactionConfirmationStatus>()?;
    m.add_class::<TransactionStatus>()?;
    m.add_class::<EncodedConfirmedTransactionWithStatusMeta>()?;
    m.add_class::<UiConfirmedBlock>()?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let ui_parsed_instruction_members = vec![
        ParsedInstruction::type_object(py),
        UiPartiallyDecodedInstruction::type_object(py),
    ];
    m.add(
        "UiParsedInstruction",
        union.get_item(PyTuple::new(py, ui_parsed_instruction_members.clone()))?,
    )?;
    let mut ui_instruction_members = vec![UiCompiledInstruction::type_object(py)];
    ui_instruction_members.extend(ui_parsed_instruction_members);
    m.add(
        "UiInstruction",
        union.get_item(PyTuple::new(py, ui_instruction_members))?,
    )?;
    m.add(
        "UiMessage",
        union.get_item(PyTuple::new(
            py,
            vec![
                UiParsedMessage::type_object(py),
                UiRawMessage::type_object(py),
            ],
        ))?,
    )?;
    m.add(
        "EncodedVersionedTransaction",
        union.get_item(PyTuple::new(
            py,
            vec![
                VersionedTransaction::type_object(py),
                UiTransaction::type_object(py),
            ],
        ))?,
    )?;
    m.add(
        "InstructionErrorType",
        union.get_item(PyTuple::new(
            py,
            vec![
                InstructionErrorFieldless::type_object(py),
                InstructionErrorCustom::type_object(py),
                InstructionErrorBorshIO::type_object(py),
            ],
        ))?,
    )?;
    m.add(
        "TransactionErrorType",
        union.get_item(PyTuple::new(
            py,
            vec![
                TransactionErrorFieldless::type_object(py),
                TransactionErrorInstructionError::type_object(py),
                TransactionErrorDuplicateInstruction::type_object(py),
                TransactionErrorInsufficientFundsForRent::type_object(py),
            ],
        ))?,
    )?;
    Ok(m)
}
