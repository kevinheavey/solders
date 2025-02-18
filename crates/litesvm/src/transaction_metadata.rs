use {
    litesvm::types::{
        FailedTransactionMetadata as FailedTransactionMetadataOriginal,
        SimulatedTransactionInfo as SimulatedTransactionInfoOriginal,
        TransactionMetadata as TransactionMetadataOriginal,
        TransactionResult as TransactionResultOriginal,
    },
    pyo3::prelude::*,
    serde::{Deserialize, Serialize},
    solana_account::Account as AccountOriginal,
    solana_message::inner_instruction::InnerInstruction as InnerInstructionOriginal,
    solders_account::Account,
    solders_instruction::CompiledInstruction,
    solders_pubkey::Pubkey,
    solders_signature::Signature,
    solders_traits_core::transaction_status_boilerplate,
    solders_transaction_error::TransactionErrorType,
    solders_transaction_return_data::TransactionReturnData,
};

/// A compiled instruction that was invoked during a
/// transaction instruction.
#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InnerInstruction(InnerInstructionOriginal);

transaction_status_boilerplate!(InnerInstruction);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl InnerInstruction {
    ///
    /// Returns:
    ///     CompiledInstruction: the compiled instruction
    pub fn instruction(&self) -> CompiledInstruction {
        CompiledInstruction(self.0.instruction.clone())
    }

    ///
    /// Returns:
    ///     int: Invocation stack height of this instruction. Starts at 1.
    pub fn stack_height(&self) -> u8 {
        self.0.stack_height
    }
}

/// Information about sent transactions.
#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionMetadata(pub(crate) TransactionMetadataOriginal);

transaction_status_boilerplate!(TransactionMetadata);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl TransactionMetadata {
    ///
    /// Returns:
    ///      Signature: The transaction signature
    pub fn signature(&self) -> Signature {
        Signature(self.0.signature)
    }

    ///
    /// Returns:
    ///     list[str]: The transaction logs.
    pub fn logs(&self) -> Vec<String> {
        self.0.logs.clone()
    }

    ///
    /// Returns:
    ///     list[list[InnerInstruction]]: The transaction's inner instructions.
    pub fn inner_instructions(&self) -> Vec<Vec<InnerInstruction>> {
        self.0
            .inner_instructions
            .clone()
            .into_iter()
            .map(|outer| outer.into_iter().map(InnerInstruction).collect())
            .collect()
    }

    ///
    /// Returns:
    ///     int: The compute units consumed by the transaction.
    pub fn compute_units_consumed(&self) -> u64 {
        self.0.compute_units_consumed
    }

    ///
    /// Returns:
    ///     TransactionReturnData: The transaction return data.
    pub fn return_data(&self) -> TransactionReturnData {
        TransactionReturnData(self.0.return_data.clone())
    }
}

/// Information about failed transactions.
#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FailedTransactionMetadata(pub(crate) FailedTransactionMetadataOriginal);

transaction_status_boilerplate!(FailedTransactionMetadata);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl FailedTransactionMetadata {
    ///
    /// Returns:
    ///     TransactionErrorType: The transaction error.
    pub fn err(&self) -> TransactionErrorType {
        self.0.err.clone().into()
    }

    ///
    /// Returns:
    ///     TransactionMetadata: The transaction metadata.
    pub fn meta(&self) -> TransactionMetadata {
        TransactionMetadata(self.0.meta.clone())
    }
}

/// Information about simulated transactions.
#[pyclass(module = "solders.transaction_metadata", subclass)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimulatedTransactionInfo(pub(crate) SimulatedTransactionInfoOriginal);

transaction_status_boilerplate!(SimulatedTransactionInfo);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl SimulatedTransactionInfo {
    ///
    /// Returns:
    ///     TransactionMetadata: The transaction metadata.
    pub fn meta(&self) -> TransactionMetadata {
        TransactionMetadata(self.0.meta.clone())
    }

    ///
    /// Returns:
    ///     list[tuple[Pubkey, Account]]: Pubkey-Account pairs, showing the state of writable accounts after transaction execution.
    pub fn post_accounts(&self) -> Vec<(Pubkey, Account)> {
        self.0
            .post_accounts
            .clone()
            .into_iter()
            .map(|x| (Pubkey(x.0), Account::from(AccountOriginal::from(x.1))))
            .collect()
    }
}

#[derive(FromPyObject, Clone, PartialEq, Debug, IntoPyObject)]
pub enum TransactionResult {
    Ok(TransactionMetadata),
    Err(FailedTransactionMetadata),
}

impl From<TransactionResultOriginal> for TransactionResult {
    fn from(value: TransactionResultOriginal) -> Self {
        match value {
            TransactionResultOriginal::Err(e) => Self::Err(FailedTransactionMetadata(e)),
            TransactionResultOriginal::Ok(x) => Self::Ok(TransactionMetadata(x)),
        }
    }
}

#[derive(FromPyObject, Clone, PartialEq, Debug, IntoPyObject)]
pub enum SimulateResult {
    Ok(SimulatedTransactionInfo),
    Err(FailedTransactionMetadata),
}

type SimResultOriginal =
    Result<SimulatedTransactionInfoOriginal, FailedTransactionMetadataOriginal>;

impl From<SimResultOriginal> for SimulateResult {
    fn from(value: SimResultOriginal) -> Self {
        match value {
            SimResultOriginal::Err(e) => Self::Err(FailedTransactionMetadata(e)),
            SimResultOriginal::Ok(x) => Self::Ok(SimulatedTransactionInfo(x)),
        }
    }
}

pub fn include_transaction_metadata(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<InnerInstruction>()?;
    m.add_class::<TransactionMetadata>()?;
    m.add_class::<FailedTransactionMetadata>()?;
    m.add_class::<SimulatedTransactionInfo>()?;
    Ok(())
}
