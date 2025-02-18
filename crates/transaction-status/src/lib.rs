#![allow(clippy::too_many_arguments)]
use derive_more::{From, Into};
extern crate base64;
use pythonize::{depythonize, pythonize};
use solders_account_decoder::UiTokenAmount;
use solders_hash::Hash as SolderHash;
use solders_message::MessageHeader;
use solders_pubkey::Pubkey;
use solders_signature::Signature;
use solders_traits_core::{
    common_methods_default, handle_py_value_err, py_from_bytes_general_via_bincode,
    pybytes_general_via_bincode, richcmp_type_error, transaction_status_boilerplate,
    RichcmpEqualityOnly,
};
use solders_transaction_confirmation_status::TransactionConfirmationStatus;
use solders_transaction_error::{
    InstructionErrorBorshIO, InstructionErrorCustom, InstructionErrorFieldless,
    TransactionErrorDuplicateInstruction, TransactionErrorFieldless,
    TransactionErrorInstructionError, TransactionErrorInsufficientFundsForRent,
    TransactionErrorProgramExecutionTemporarilyRestricted, TransactionErrorType,
};
use solders_transaction_return_data::TransactionReturnData;
use solders_transaction_status_enums::{TransactionDetails, UiTransactionEncoding};
use solders_transaction_status_struct::TransactionStatus;

use std::str::FromStr;

use pyo3::{prelude::*, pyclass::CompareOp, IntoPyObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_reward_info::RewardType as RewardTypeOriginal;
use solana_transaction_status_client_types::{
    EncodedTransaction as EncodedTransactionOriginal,
    EncodedTransactionWithStatusMeta as EncodedTransactionWithStatusMetaOriginal,
    ParsedAccount as ParsedAccountOriginal, ParsedAccountSource as ParsedAccountSourceOriginal,
    ParsedInstruction as ParsedInstructionOriginal, Reward as RewardOriginal,
    TransactionBinaryEncoding as TransactionBinaryEncodingOriginal,
    UiAccountsList as UiAccountsListOriginal, UiAddressTableLookup as UiAddressTableLookupOriginal,
    UiCompiledInstruction as UiCompiledInstructionOriginal,
    UiConfirmedBlock as UiConfirmedBlockOriginal,
    UiInnerInstructions as UiInnerInstructionsOriginal, UiInstruction as UiInstructionOriginal,
    UiLoadedAddresses as UiLoadedAddressesOriginal, UiMessage as UiMessageOriginal,
    UiParsedInstruction as UiParsedInstructionOriginal, UiParsedMessage as UiParsedMessageOriginal,
    UiPartiallyDecodedInstruction as UiPartiallyDecodedInstructionOriginal,
    UiRawMessage as UiRawMessageOriginal, UiTransaction as UiTransactionOriginal,
    UiTransactionReturnData, UiTransactionStatusMeta as UiTransactionStatusMetaOriginal,
    UiTransactionTokenBalance as UiTransactionTokenBalanceOriginal,
};
use solders_macros::{common_methods, enum_original_mapping, richcmp_eq_only};
use solders_transaction::{TransactionVersion, VersionedTransaction};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(TransactionBinaryEncodingOriginal)]
#[pyclass(module = "solders.transaction_status", eq, eq_int)]
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
    #[pyo3(signature = (program_id_index, accounts, data, stack_height=None))]
    #[new]
    fn new(
        program_id_index: u8,
        accounts: Vec<u8>,
        data: String,
        stack_height: Option<u32>,
    ) -> Self {
        UiCompiledInstructionOriginal {
            program_id_index,
            accounts,
            data,
            stack_height,
        }
        .into()
    }

    #[getter]
    pub fn program_id_index(&self) -> u8 {
        self.0.program_id_index
    }

    #[getter]
    pub fn accounts(&self) -> Vec<u8> {
        self.0.accounts.clone()
    }

    #[getter]
    pub fn data(&self) -> String {
        self.0.data.clone()
    }

    #[getter]
    pub fn stack_height(&self) -> Option<u32> {
        self.0.stack_height
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
    pub fn writable_indexes(&self) -> Vec<u8> {
        self.0.writable_indexes.clone()
    }

    #[getter]
    pub fn readonly_indexes(&self) -> Vec<u8> {
        self.0.readonly_indexes.clone()
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
    #[pyo3(signature = (header, account_keys, recent_blockhash, instructions, address_table_lookups=None))]
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

#[pyclass(module = "solders.transaction_status", eq, eq_int)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(ParsedAccountSourceOriginal)]
pub enum ParsedAccountSource {
    Transaction,
    LookupTable,
}
/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct ParsedAccountTxStatus(ParsedAccountOriginal);

transaction_status_boilerplate!(ParsedAccountTxStatus);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl ParsedAccountTxStatus {
    #[pyo3(signature = (pubkey, writable, signer, source=None))]
    #[new]
    fn new(
        pubkey: Pubkey,
        writable: bool,
        signer: bool,
        source: Option<ParsedAccountSource>,
    ) -> Self {
        ParsedAccountOriginal {
            pubkey: pubkey.0.to_string(),
            writable,
            signer,
            source: source.map(Into::into),
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

    #[getter]
    pub fn source(&self) -> Option<ParsedAccountSource> {
        self.0.source.clone().map(Into::into)
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
    #[pyo3(signature = (program, program_id, parsed, stack_height=None))]
    #[new]
    fn new(
        program: String,
        program_id: Pubkey,
        parsed: Bound<'_, PyAny>,
        stack_height: Option<u32>,
    ) -> PyResult<Self> {
        let value = handle_py_value_err(depythonize::<Value>(&parsed))?;
        Ok(ParsedInstructionOriginal {
            program,
            program_id: program_id.to_string(),
            parsed: value,
            stack_height,
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

    #[getter]
    pub fn stack_height(&self) -> Option<u32> {
        self.0.stack_height
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
    #[pyo3(signature = (program_id, accounts, data, stack_height=None))]
    #[new]
    fn new(
        program_id: Pubkey,
        accounts: Vec<Pubkey>,
        data: String,
        stack_height: Option<u32>,
    ) -> Self {
        UiPartiallyDecodedInstructionOriginal {
            program_id: program_id.to_string(),
            accounts: accounts.into_iter().map(|a| a.to_string()).collect(),
            data,
            stack_height,
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

    #[getter]
    pub fn stack_height(&self) -> Option<u32> {
        self.0.stack_height
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, IntoPyObject)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, IntoPyObject)]
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
    #[pyo3(signature = (account_keys, recent_blockhash, instructions, address_table_lookups=None))]
    #[new]
    fn new(
        account_keys: Vec<ParsedAccountTxStatus>,
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
    pub fn account_keys(&self) -> Vec<ParsedAccountTxStatus> {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, IntoPyObject)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject, IntoPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EncodedVersionedTransaction {
    Binary(VersionedTransaction),
    Json(UiTransaction),
    Accounts(UiAccountsList),
}

impl From<EncodedTransaction> for EncodedVersionedTransaction {
    fn from(e: EncodedTransaction) -> Self {
        match e {
            EncodedTransaction::LegacyBinary(..) | EncodedTransaction::Binary(..) => Self::Binary(
                VersionedTransaction::from(EncodedTransactionOriginal::from(e).decode().unwrap()),
            ),
            EncodedTransaction::Json(u) => Self::Json(u),
            EncodedTransaction::Accounts(u) => Self::Accounts(u),
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
            EncodedVersionedTransaction::Accounts(u) => Self::Accounts(u),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiAccountsList(UiAccountsListOriginal);

transaction_status_boilerplate!(UiAccountsList);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiAccountsList {
    #[new]
    pub fn new(signatures: Vec<Signature>, account_keys: Vec<ParsedAccountTxStatus>) -> Self {
        UiAccountsListOriginal {
            signatures: signatures.into_iter().map(|s| s.to_string()).collect(),
            account_keys: account_keys.into_iter().map(Into::into).collect(),
        }
        .into()
    }

    #[getter]
    pub fn signatures(&self) -> Vec<Signature> {
        self.0
            .signatures
            .clone()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    #[getter]
    pub fn account_keys(&self) -> Vec<ParsedAccountTxStatus> {
        self.0
            .account_keys
            .clone()
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EncodedTransaction {
    LegacyBinary(String), // Old way of expressing base-58, retained for RPC backwards compatibility
    Binary(String, TransactionBinaryEncoding),
    Json(UiTransaction),
    Accounts(UiAccountsList),
}

impl From<EncodedTransactionOriginal> for EncodedTransaction {
    fn from(e: EncodedTransactionOriginal) -> Self {
        match e {
            EncodedTransactionOriginal::LegacyBinary(s) => Self::LegacyBinary(s),
            EncodedTransactionOriginal::Binary(s, b) => Self::Binary(s, b.into()),
            EncodedTransactionOriginal::Json(t) => Self::Json(t.into()),
            EncodedTransactionOriginal::Accounts(a) => Self::Accounts(a.into()),
        }
    }
}

impl From<EncodedTransaction> for EncodedTransactionOriginal {
    fn from(e: EncodedTransaction) -> Self {
        match e {
            EncodedTransaction::LegacyBinary(s) => Self::LegacyBinary(s),
            EncodedTransaction::Binary(s, b) => Self::Binary(s, b.into()),
            EncodedTransaction::Json(t) => Self::Json(t.into()),
            EncodedTransaction::Accounts(t) => Self::Accounts(t.into()),
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
    #[pyo3(signature = (account_index, mint, ui_token_amount, owner=None, program_id=None))]
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
            owner: owner.map(|x| x.to_string()).into(),
            program_id: program_id.map(|x| x.to_string()).into(),
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
        let maybe_key: Option<String> = self.0.owner.clone().into();
        maybe_key.map(|x| Pubkey::from_str(&x).unwrap())
    }

    #[getter]
    pub fn program_id(&self) -> Option<Pubkey> {
        let maybe_id: Option<String> = self.0.clone().program_id.into();
        maybe_id.map(|x| Pubkey::from_str(&x).unwrap())
    }
}

#[pyclass(module = "solders.transaction_status", eq, eq_int)]
#[enum_original_mapping(RewardTypeOriginal)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}
/// A duplicate representation of TransactionStatusMeta with `err` field
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct UiTransactionStatusMeta(UiTransactionStatusMetaOriginal);
impl RichcmpEqualityOnly for UiTransactionStatusMeta {
    fn richcmp(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.compare(other)),
            CompareOp::Ne => Ok(!self.compare(other)),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}

impl UiTransactionStatusMeta {
    fn compare(&self, other: &Self) -> bool {
        self.err() == other.err()
            && self.fee() == other.fee()
            && self.pre_balances() == other.pre_balances()
            && self.post_balances() == other.post_balances()
            && self.inner_instructions() == other.inner_instructions()
            && self.log_messages() == other.log_messages()
            && self.pre_token_balances() == other.pre_token_balances()
            && self.post_token_balances() == other.post_token_balances()
            && self.rewards() == other.rewards()
            && self.loaded_addresses() == other.loaded_addresses()
            && self.return_data() == other.return_data()
            && self.compute_units_consumed() == other.compute_units_consumed()
    }
}

impl std::fmt::Display for UiTransactionStatusMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pybytes_general_via_bincode!(UiTransactionStatusMeta);
py_from_bytes_general_via_bincode!(UiTransactionStatusMeta);
common_methods_default!(UiTransactionStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl UiTransactionStatusMeta {
    #[pyo3(
        signature = (err, fee, pre_balances, post_balances, inner_instructions=None, log_messages=None, pre_token_balances=None, post_token_balances=None, rewards=None, loaded_addresses=None, return_data=None, compute_units_consumed=None)
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
        compute_units_consumed: Option<u64>,
    ) -> Self {
        UiTransactionStatusMetaOriginal {
            err: err.map(|e| e.into()),
            status: Ok(()),
            fee,
            pre_balances,
            post_balances,
            inner_instructions: inner_instructions
                .map(|v| v.into_iter().map(|ix| ix.into()).collect())
                .into(),
            log_messages: log_messages.into(),
            pre_token_balances: pre_token_balances
                .map(|v| v.into_iter().map(|bal| bal.into()).collect())
                .into(),
            post_token_balances: post_token_balances
                .map(|v| v.into_iter().map(|bal| bal.into()).collect())
                .into(),
            rewards: rewards
                .map(|v| v.into_iter().map(|r| r.into()).collect())
                .into(),
            loaded_addresses: loaded_addresses.map(|a| a.into()).into(),
            return_data: return_data.map(|r| r.into()).into(),
            compute_units_consumed: compute_units_consumed.into(),
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
        let maybe_instructions: Option<Vec<UiInnerInstructionsOriginal>> =
            self.0.inner_instructions.clone().into();
        maybe_instructions.map(|v| v.into_iter().map(|ix| ix.into()).collect())
    }
    #[getter]
    pub fn log_messages(&self) -> Option<Vec<String>> {
        self.0.log_messages.clone().into()
    }
    #[getter]
    pub fn pre_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        let maybe_balances: Option<Vec<UiTransactionTokenBalanceOriginal>> =
            self.0.pre_token_balances.clone().into();
        maybe_balances.map(|v| v.into_iter().map(|bal| bal.into()).collect())
    }
    #[getter]
    pub fn post_token_balances(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        let maybe_balances: Option<Vec<UiTransactionTokenBalanceOriginal>> =
            self.0.post_token_balances.clone().into();
        maybe_balances.map(|v| v.into_iter().map(|bal| bal.into()).collect())
    }
    #[getter]
    pub fn rewards(&self) -> Option<Rewards> {
        let maybe_rewards: Option<Vec<RewardOriginal>> = self.0.rewards.clone().into();
        maybe_rewards.map(|v| v.into_iter().map(|r| r.into()).collect())
    }
    #[getter]
    pub fn loaded_addresses(&self) -> Option<UiLoadedAddresses> {
        let maybe_addresses: Option<UiLoadedAddressesOriginal> =
            self.0.loaded_addresses.clone().into();
        maybe_addresses.map(UiLoadedAddresses::from)
    }
    #[getter]
    pub fn return_data(&self) -> Option<TransactionReturnData> {
        let maybe_underlying: Option<UiTransactionReturnData> = self.0.return_data.clone().into();
        maybe_underlying.map(|r| r.into())
    }
    #[getter]
    pub fn compute_units_consumed(&self) -> Option<u64> {
        self.0.compute_units_consumed.clone().into()
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
    #[pyo3(signature = (transaction, meta=None, version=None))]
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
pub struct Reward(RewardOriginal);

transaction_status_boilerplate!(Reward);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Reward {
    #[pyo3(signature = (pubkey, lamports, post_balance, reward_type=None, commission=None))]
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

pub type Rewards = Vec<Reward>;

// the one in transaction_status is missing Clone
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct EncodedConfirmedTransactionWithStatusMeta {
    #[pyo3(get)]
    pub slot: u64,
    #[serde(flatten)]
    #[pyo3(get)]
    pub transaction: EncodedTransactionWithStatusMeta,
    #[pyo3(get)]
    pub block_time: Option<i64>,
}

transaction_status_boilerplate!(EncodedConfirmedTransactionWithStatusMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EncodedConfirmedTransactionWithStatusMeta {
    #[pyo3(signature = (slot, transaction, block_time=None))]
    #[new]
    pub fn new(
        slot: u64,
        transaction: EncodedTransactionWithStatusMeta,
        block_time: Option<i64>,
    ) -> Self {
        Self {
            slot,
            transaction,
            block_time,
        }
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
    #[pyo3(signature = (previous_blockhash, blockhash, parent_slot, transactions=None, signatures=None, rewards=None, block_time=None, block_height=None, num_reward_partitions=None))]
    #[new]
    pub fn new(
        previous_blockhash: SolderHash,
        blockhash: SolderHash,
        parent_slot: u64,
        transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
        signatures: Option<Vec<Signature>>,
        rewards: Option<Rewards>,
        block_time: Option<i64>,
        block_height: Option<u64>,
        num_reward_partitions: Option<u64>,
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
            num_reward_partitions,
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
    pub fn parent_slot(&self) -> u64 {
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
    pub fn block_time(&self) -> Option<i64> {
        self.0.block_time
    }
    #[getter]
    pub fn block_height(&self) -> Option<u64> {
        self.0.block_height
    }
}

pub fn include_transaction_status(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TransactionDetails>()?;
    m.add_class::<UiTransactionEncoding>()?;
    m.add_class::<TransactionBinaryEncoding>()?;
    m.add_class::<UiCompiledInstruction>()?;
    m.add_class::<UiAddressTableLookup>()?;
    m.add_class::<UiRawMessage>()?;
    m.add_class::<ParsedAccountSource>()?;
    m.add_class::<ParsedAccountTxStatus>()?;
    m.add_class::<ParsedInstruction>()?;
    m.add_class::<UiPartiallyDecodedInstruction>()?;
    m.add_class::<UiParsedMessage>()?;
    m.add_class::<UiTransaction>()?;
    m.add_class::<UiInnerInstructions>()?;
    m.add_class::<UiLoadedAddresses>()?;
    m.add_class::<UiAccountsList>()?;
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
    m.add_class::<TransactionErrorProgramExecutionTemporarilyRestricted>()?;
    m.add_class::<TransactionErrorFieldless>()?;
    m.add_class::<Reward>()?;
    m.add_class::<TransactionConfirmationStatus>()?;
    m.add_class::<TransactionStatus>()?;
    m.add_class::<EncodedConfirmedTransactionWithStatusMeta>()?;
    m.add_class::<UiConfirmedBlock>()?;
    Ok(())
}
