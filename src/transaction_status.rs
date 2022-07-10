use std::str::FromStr;

use crate::{
    message::MessageHeader,
    pubkey::Pubkey,
    signature::Signature,
    tmp_transaction_status::{
        EncodedTransaction as EncodedTransactionOriginal,
        EncodedTransactionWithStatusMeta as EncodedTransactionWithStatusMetaOriginal,
        ParsedAccount as ParsedAccountOriginal, ParsedInstruction as ParsedInstructionOriginal,
        Reward as RewardOriginal, TransactionBinaryEncoding as TransactionBinaryEncodingOriginal,
        UiAddressTableLookup as UiAddressTableLookupOriginal,
        UiCompiledInstruction as UiCompiledInstructionOriginal,
        UiInstruction as UiInstructionOriginal, UiMessage as UiMessageOriginal,
        UiParsedInstruction as UiParsedInstructionOriginal,
        UiParsedMessage as UiParsedMessageOriginal,
        UiPartiallyDecodedInstruction as UiPartiallyDecodedInstructionOriginal,
        UiRawMessage as UiRawMessageOriginal, UiTransaction as UiTransactionOriginal,
        UiTransactionStatusMeta as UiTransactionStatusMetaOriginal,
    },
    transaction::{TransactionVersion, VersionedTransaction},
    SolderHash,
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solders_macros::enum_original_mapping;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiCompiledInstruction(UiCompiledInstructionOriginal);

impl From<UiCompiledInstruction> for UiCompiledInstructionOriginal {
    fn from(u: UiCompiledInstruction) -> Self {
        u.0
    }
}

impl From<UiCompiledInstructionOriginal> for UiCompiledInstruction {
    fn from(u: UiCompiledInstructionOriginal) -> Self {
        Self(u)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiAddressTableLookup(UiAddressTableLookupOriginal);

impl From<UiAddressTableLookupOriginal> for UiAddressTableLookup {
    fn from(u: UiAddressTableLookupOriginal) -> Self {
        Self(u)
    }
}

impl From<UiAddressTableLookup> for UiAddressTableLookupOriginal {
    fn from(u: UiAddressTableLookup) -> Self {
        u.0
    }
}
/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiRawMessage(UiRawMessageOriginal);

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
}

impl From<UiRawMessageOriginal> for UiRawMessage {
    fn from(m: UiRawMessageOriginal) -> Self {
        Self(m)
    }
}

impl From<UiRawMessage> for UiRawMessageOriginal {
    fn from(m: UiRawMessage) -> Self {
        m.0
    }
}

/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct ParsedAccount(ParsedAccountOriginal);

impl From<ParsedAccountOriginal> for ParsedAccount {
    fn from(p: ParsedAccountOriginal) -> Self {
        Self(p)
    }
}

impl From<ParsedAccount> for ParsedAccountOriginal {
    fn from(p: ParsedAccount) -> Self {
        p.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct ParsedInstruction(ParsedInstructionOriginal);

impl From<ParsedInstructionOriginal> for ParsedInstruction {
    fn from(p: ParsedInstructionOriginal) -> Self {
        Self(p)
    }
}

impl From<ParsedInstruction> for ParsedInstructionOriginal {
    fn from(p: ParsedInstruction) -> Self {
        p.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiPartiallyDecodedInstruction(UiPartiallyDecodedInstructionOriginal);

impl From<UiPartiallyDecodedInstructionOriginal> for UiPartiallyDecodedInstruction {
    fn from(p: UiPartiallyDecodedInstructionOriginal) -> Self {
        Self(p)
    }
}

impl From<UiPartiallyDecodedInstruction> for UiPartiallyDecodedInstructionOriginal {
    fn from(p: UiPartiallyDecodedInstruction) -> Self {
        p.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiParsedMessage(UiParsedMessageOriginal);

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
}

impl From<UiParsedMessageOriginal> for UiParsedMessage {
    fn from(m: UiParsedMessageOriginal) -> Self {
        Self(m)
    }
}

impl From<UiParsedMessage> for UiParsedMessageOriginal {
    fn from(m: UiParsedMessage) -> Self {
        m.0
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransaction(UiTransactionOriginal);

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

impl From<UiTransactionOriginal> for UiTransaction {
    fn from(t: UiTransactionOriginal) -> Self {
        Self(t)
    }
}

impl From<UiTransaction> for UiTransactionOriginal {
    fn from(t: UiTransaction) -> Self {
        t.0
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

/// A duplicate representation of TransactionStatusMeta with `err` field
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransactionStatusMeta(UiTransactionStatusMetaOriginal);

impl From<UiTransactionStatusMeta> for UiTransactionStatusMetaOriginal {
    fn from(m: UiTransactionStatusMeta) -> Self {
        m.0
    }
}

impl From<UiTransactionStatusMetaOriginal> for UiTransactionStatusMeta {
    fn from(m: UiTransactionStatusMetaOriginal) -> Self {
        Self(m)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct EncodedTransactionWithStatusMeta(EncodedTransactionWithStatusMetaOriginal);

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

impl From<EncodedTransactionWithStatusMeta> for EncodedTransactionWithStatusMetaOriginal {
    fn from(e: EncodedTransactionWithStatusMeta) -> Self {
        e.0
    }
}

impl From<EncodedTransactionWithStatusMetaOriginal> for EncodedTransactionWithStatusMeta {
    fn from(e: EncodedTransactionWithStatusMetaOriginal) -> Self {
        Self(e)
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
