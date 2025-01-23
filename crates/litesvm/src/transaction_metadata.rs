use {
    litesvm::types::{
        FailedTransactionMetadata as FailedTransactionMetadataOriginal,
        SimulatedTransactionInfo as SimulatedTransactionInfoOriginal,
        TransactionMetadata as TransactionMetadataOriginal,
    },
    pyo3::prelude::*,
    serde::{Deserialize, Serialize},
    solana_sdk::{
        account::Account as AccountOriginal,
        inner_instruction::InnerInstruction as InnerInstructionOriginal,
    },
    solders_account::Account,
    solders_instruction::CompiledInstruction,
    solders_pubkey::Pubkey,
    solders_signature::Signature,
    solders_traits_core::transaction_status_boilerplate,
    solders_transaction_error::TransactionErrorType,
    solders_transaction_return_data::TransactionReturnData,
};

#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InnerInstruction(InnerInstructionOriginal);

transaction_status_boilerplate!(InnerInstruction);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl InnerInstruction {
    pub fn instruction(&self) -> CompiledInstruction {
        CompiledInstruction(self.0.instruction.clone())
    }

    pub fn stack_height(&self) -> u8 {
        self.0.stack_height
    }
}

#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionMetadata(pub(crate) TransactionMetadataOriginal);

transaction_status_boilerplate!(TransactionMetadata);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl TransactionMetadata {
    pub fn signature(&self) -> Signature {
        Signature(self.0.signature)
    }

    pub fn logs(&self) -> Vec<String> {
        self.0.logs.clone()
    }

    pub fn inner_instructions(&self) -> Vec<Vec<InnerInstruction>> {
        self.0
            .inner_instructions
            .clone()
            .into_iter()
            .map(|outer| outer.into_iter().map(InnerInstruction).collect())
            .collect()
    }

    pub fn compute_units_consumed(&self) -> u64 {
        self.0.compute_units_consumed
    }

    pub fn return_data(&self) -> TransactionReturnData {
        TransactionReturnData(self.0.return_data.clone())
    }
}

#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FailedTransactionMetadata(pub(crate) FailedTransactionMetadataOriginal);

transaction_status_boilerplate!(FailedTransactionMetadata);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl FailedTransactionMetadata {
    pub fn err(&self) -> TransactionErrorType {
        self.0.err.clone().into()
    }

    pub fn meta(&self) -> TransactionMetadata {
        TransactionMetadata(self.0.meta.clone())
    }
}

#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimulatedTransactionInfo(pub(crate) SimulatedTransactionInfoOriginal);

transaction_status_boilerplate!(SimulatedTransactionInfo);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl SimulatedTransactionInfo {
    pub fn meta(&self) -> TransactionMetadata {
        TransactionMetadata(self.0.meta.clone())
    }

    pub fn post_accounts(&self) -> Vec<(Pubkey, Account)> {
        self.0
            .post_accounts
            .clone()
            .into_iter()
            .map(|x| (Pubkey(x.0), Account::from(AccountOriginal::from(x.1))))
            .collect()
    }
}
