use {
    litesvm::types::{
        FailedTransactionMetadata as FailedTransactionMetadataOriginal,
        SimulatedTransactionInfo as SimulatedTransactionInfoOriginal,
        TransactionMetadata as TransactionMetadataOriginal,
        TransactionResult as TransactionResultOriginal,
    },
    pyo3::{prelude::*, types::PyTuple, PyTypeInfo},
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

#[derive(FromPyObject, Clone, PartialEq, Debug)]
pub enum TransactionResult {
    Ok(TransactionMetadata),
    Err(FailedTransactionMetadata),
}

impl IntoPy<PyObject> for TransactionResult {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Ok(x) => x.into_py(py),
            Self::Err(e) => e.into_py(py),
        }
    }
}

impl From<TransactionResultOriginal> for TransactionResult {
    fn from(value: TransactionResultOriginal) -> Self {
        match value {
            TransactionResultOriginal::Err(e) => Self::Err(FailedTransactionMetadata(e)),
            TransactionResultOriginal::Ok(x) => Self::Ok(TransactionMetadata(x)),
        }
    }
}

#[derive(FromPyObject, Clone, PartialEq, Debug)]
pub enum SimulateResult {
    Ok(SimulatedTransactionInfo),
    Err(FailedTransactionMetadata),
}

impl IntoPy<PyObject> for SimulateResult {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Ok(x) => x.into_py(py),
            Self::Err(e) => e.into_py(py),
        }
    }
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

pub fn create_transaction_metadata_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "transaction_metadata")?;
    m.add_class::<InnerInstruction>()?;
    m.add_class::<TransactionMetadata>()?;
    m.add_class::<FailedTransactionMetadata>()?;
    m.add_class::<SimulatedTransactionInfo>()?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let transaction_result_members = vec![
        TransactionMetadata::type_object(py),
        FailedTransactionMetadata::type_object(py),
    ];
    m.add(
        "TransactionResult",
        union.get_item(PyTuple::new(py, transaction_result_members))?,
    )?;
    let simulate_result_members = vec![
        SimulatedTransactionInfo::type_object(py),
        FailedTransactionMetadata::type_object(py),
    ];
    m.add(
        "SimulateResult",
        union.get_item(PyTuple::new(py, simulate_result_members))?,
    )?;
    Ok(m)
}
