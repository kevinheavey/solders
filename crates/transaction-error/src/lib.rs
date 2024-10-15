use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::InstructionError as InstructionErrorOriginal,
    transaction::TransactionError as TransactionErrorOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_traits_core::transaction_status_boilerplate;

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
    MaxAccountsDataAllocationsExceeded,
    MaxAccountsExceeded,
    MaxInstructionTraceLengthExceeded,
    BuiltinProgramsMustConsumeComputeUnits,
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
                InstructionErrorFieldless::MaxAccountsDataAllocationsExceeded => {
                    Self::MaxAccountsDataAllocationsExceeded
                }
                InstructionErrorFieldless::MaxAccountsExceeded => Self::MaxAccountsExceeded,
                InstructionErrorFieldless::MaxInstructionTraceLengthExceeded => {
                    Self::MaxInstructionTraceLengthExceeded
                }
                InstructionErrorFieldless::BuiltinProgramsMustConsumeComputeUnits => {
                    Self::BuiltinProgramsMustConsumeComputeUnits
                }
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
            InstructionErrorOriginal::MaxAccountsDataAllocationsExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxAccountsDataAllocationsExceeded)
            }
            InstructionErrorOriginal::MaxAccountsExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxAccountsExceeded)
            }
            InstructionErrorOriginal::MaxInstructionTraceLengthExceeded => {
                Self::Fieldless(InstructionErrorFieldless::MaxInstructionTraceLengthExceeded)
            }
            InstructionErrorOriginal::BuiltinProgramsMustConsumeComputeUnits => {
                Self::Fieldless(InstructionErrorFieldless::BuiltinProgramsMustConsumeComputeUnits)
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionErrorProgramExecutionTemporarilyRestricted {
    #[pyo3(get)]
    account_index: u8,
}
transaction_status_boilerplate!(TransactionErrorProgramExecutionTemporarilyRestricted);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionErrorProgramExecutionTemporarilyRestricted {
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
    MaxLoadedAccountsDataSizeExceeded,
    ResanitizationNeeded,
    InvalidLoadedAccountsDataSizeLimit,
    UnbalancedTransaction,
    ProgramCacheHitMaxLimit,
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
pub enum TransactionErrorTypeTagged {
    InstructionError(TransactionErrorInstructionError),
    DuplicateInstruction(TransactionErrorDuplicateInstruction),
    InsufficientFundsForRent(TransactionErrorInsufficientFundsForRent),
    ProgramExecutionTemporarilyRestricted(TransactionErrorProgramExecutionTemporarilyRestricted),
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
                TransactionErrorTypeTagged::ProgramExecutionTemporarilyRestricted(e) => {
                    Self::ProgramExecutionTemporarilyRestricted {
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
                TransactionErrorFieldless::MaxLoadedAccountsDataSizeExceeded => {
                    Self::MaxLoadedAccountsDataSizeExceeded
                }
                TransactionErrorFieldless::ResanitizationNeeded => Self::ResanitizationNeeded,
                TransactionErrorFieldless::InvalidLoadedAccountsDataSizeLimit => {
                    Self::InvalidLoadedAccountsDataSizeLimit
                }
                TransactionErrorFieldless::UnbalancedTransaction => Self::UnbalancedTransaction,
                TransactionErrorFieldless::ProgramCacheHitMaxLimit => Self::ProgramCacheHitMaxLimit,
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
            TransactionErrorOriginal::ProgramExecutionTemporarilyRestricted { account_index } => {
                Self::Tagged(
                    TransactionErrorTypeTagged::ProgramExecutionTemporarilyRestricted(
                        TransactionErrorProgramExecutionTemporarilyRestricted { account_index },
                    ),
                )
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
            TransactionErrorOriginal::MaxLoadedAccountsDataSizeExceeded => {
                Self::Fieldless(TransactionErrorFieldless::MaxLoadedAccountsDataSizeExceeded)
            }
            TransactionErrorOriginal::ResanitizationNeeded => {
                Self::Fieldless(TransactionErrorFieldless::ResanitizationNeeded)
            }
            TransactionErrorOriginal::InvalidLoadedAccountsDataSizeLimit => {
                Self::Fieldless(TransactionErrorFieldless::InvalidLoadedAccountsDataSizeLimit)
            }
            TransactionErrorOriginal::UnbalancedTransaction => {
                Self::Fieldless(TransactionErrorFieldless::UnbalancedTransaction)
            }
            TransactionErrorOriginal::ProgramCacheHitMaxLimit => {
                Self::Fieldless(TransactionErrorFieldless::ProgramCacheHitMaxLimit)
            }
        }
    }
}
