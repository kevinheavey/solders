use derive_more::{From, Into};
use std::fmt::Display;
use std::str::FromStr;

use crate::{
    message::MessageHeader,
    pubkey::Pubkey,
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    signature::Signature,
    tmp_transaction_status::{
        EncodedTransaction as EncodedTransactionOriginal,
        EncodedTransactionWithStatusMeta as EncodedTransactionWithStatusMetaOriginal,
        ParsedAccount as ParsedAccountOriginal, ParsedInstruction as ParsedInstructionOriginal,
        Reward as RewardOriginal, TransactionBinaryEncoding as TransactionBinaryEncodingOriginal,
        UiAddressTableLookup as UiAddressTableLookupOriginal,
        UiCompiledInstruction as UiCompiledInstructionOriginal,
        UiInnerInstructions as UiInnerInstructionsOriginal, UiInstruction as UiInstructionOriginal,
        UiLoadedAddresses, UiMessage as UiMessageOriginal,
        UiParsedInstruction as UiParsedInstructionOriginal,
        UiParsedMessage as UiParsedMessageOriginal,
        UiPartiallyDecodedInstruction as UiPartiallyDecodedInstructionOriginal,
        UiRawMessage as UiRawMessageOriginal, UiTransaction as UiTransactionOriginal,
        UiTransactionStatusMeta as UiTransactionStatusMetaOriginal, UiTransactionTokenBalance,
    },
    transaction::{TransactionError, TransactionVersion, VersionedTransaction},
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly, SolderHash,
};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::transaction_context::TransactionReturnData;
use solders_macros::{common_methods, enum_original_mapping, richcmp_eq_only};

macro_rules! transaction_status_boilerplate {
    ($name:ident) => {
        impl RichcmpEqualityOnly for $name {}
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        pybytes_general_via_bincode!($name);
        py_from_bytes_general_via_bincode!($name);
        impl<'a> CommonMethods<'a> for $name {}
    };
}

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
    fn new(account_key: String, writable_indexes: Vec<u8>, readonly_indexes: Vec<u8>) -> Self {
        UiAddressTableLookupOriginal {
            account_key,
            writable_indexes,
            readonly_indexes,
        }
        .into()
    }

    #[getter]
    pub fn account_key(&self) -> String {
        self.0.account_key.clone()
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
    fn new(program: String, program_id: Pubkey, parsed: &str) -> Self {
        ParsedInstructionOriginal {
            program,
            program_id: program_id.to_string(),
            parsed: Value::from_str(parsed).unwrap(),
        }
        .into()
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
    pub fn parsed(&self) -> String {
        self.0.parsed.to_string()
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiParsedInstruction {
    Parsed(ParsedInstruction),
    PartiallyDecoded(UiPartiallyDecodedInstruction),
}

impl IntoPy<PyObject> for UiParsedInstruction {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Parsed(m) => m.into_py(py),
            Self::PartiallyDecoded(m) => m.into_py(py),
        }
    }
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiInstruction {
    Compiled(UiCompiledInstruction),
    Parsed(UiParsedInstruction),
}

impl IntoPy<PyObject> for UiInstruction {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Compiled(m) => m.into_py(py),
            Self::Parsed(m) => m.into_py(py),
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
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

impl IntoPy<PyObject> for UiMessage {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Parsed(p) => p.into_py(py),
            Self::Raw(r) => r.into_py(py),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EncodedTransaction {
    LegacyBinary(String), // Old way of expressing base-58, retained for RPC backwards compatibility
    Binary(String, TransactionBinaryEncoding),
    Json(UiTransaction),
}

impl IntoPy<PyObject> for EncodedTransaction {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::LegacyBinary(..) | Self::Binary(..) => {
                VersionedTransaction::from(EncodedTransactionOriginal::from(self).decode().unwrap())
                    .into_py(py)
            }
            Self::Json(u) => u.into_py(py),
        }
    }
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
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

/// A duplicate representation of TransactionStatusMeta with `err` field
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransactionStatusMeta(UiTransactionStatusMetaOriginal);

transaction_status_boilerplate!(UiTransactionStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTransactionStatusMeta {
    #[new]
    pub fn new(
        err: Option<TransactionError>,
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
            err: err.into(),
            status: Ok(()),
            fee,
            pre_balances,
            post_balances,
            inner_instructions: inner_instructions
                .map(|v| v.into_iter().map(|ix| ix.into()).collect()),
            log_messages,
            pre_token_balances: pre_token_balances
                .map(|v| v.into_iter().map(|bal| ix.into()).collect()),
            post_token_balances: post_token_balances
                .map(|v| v.into_iter().map(|bal| ix.into()).collect()),
            rewards: rewards.map(|v| v.into_iter().map(|r| r.into()).collect()),
            loaded_addresses: loaded_addresses.map(|a| a.into()),
            return_data: return_data.map(|r| r.into()),
        }
        .into()
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
        transaction: EncodedTransaction,
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
    pub fn transaction(&self) -> EncodedTransaction {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct Reward(RewardOriginal);

pub type Rewards = Vec<Reward>;

pub fn create_transaction_status_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "transaction_status")?;
    m.add_class::<TransactionDetails>()?;
    m.add_class::<UiTransactionEncoding>()?;
    Ok(m)
}
