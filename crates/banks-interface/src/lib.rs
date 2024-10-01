use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_banks_client::TransactionStatus as TransactionStatusBanks;
use solana_banks_interface::{
    BanksTransactionResultWithMetadata, BanksTransactionResultWithSimulation,
    TransactionConfirmationStatus as TransactionConfirmationStatusBanks, TransactionMetadata,
};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits_core::transaction_status_boilerplate;
use solders_transaction_confirmation_status::TransactionConfirmationStatus;
use solders_transaction_error::TransactionErrorType;
use solders_transaction_return_data::TransactionReturnData;
use solders_transaction_status_struct::TransactionStatus;

pub fn confirmation_status_from_banks(
    s: TransactionConfirmationStatusBanks,
) -> TransactionConfirmationStatus {
    match s {
        TransactionConfirmationStatusBanks::Processed => TransactionConfirmationStatus::Processed,
        TransactionConfirmationStatusBanks::Confirmed => TransactionConfirmationStatus::Confirmed,
        TransactionConfirmationStatusBanks::Finalized => TransactionConfirmationStatus::Finalized,
    }
}

pub fn transaction_status_from_banks(t: TransactionStatusBanks) -> TransactionStatus {
    TransactionStatus::new(
        t.slot,
        t.confirmations,
        None,
        t.err.map(Into::into),
        t.confirmation_status.map(confirmation_status_from_banks),
    )
}

/// Transaction metadata.
#[pyclass(module = "solders.bankrun", subclass)]
#[derive(From, Into, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BanksTransactionMeta(pub TransactionMetadata);

transaction_status_boilerplate!(BanksTransactionMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BanksTransactionMeta {
    #[new]
    pub fn new(
        log_messages: Vec<String>,
        compute_units_consumed: u64,
        return_data: Option<TransactionReturnData>,
    ) -> Self {
        TransactionMetadata {
            log_messages,
            compute_units_consumed,
            return_data: return_data.map(Into::into),
        }
        .into()
    }

    /// List[str]: The log messages written during transaction execution.
    #[getter]
    pub fn log_messages(&self) -> Vec<String> {
        self.0.log_messages.clone()
    }

    /// Optional[TransactionReturnData]: The transaction return data, if present.
    #[getter]
    pub fn return_data(&self) -> Option<TransactionReturnData> {
        self.0.return_data.clone().map(Into::into)
    }

    /// int: The number of compute units consumed by the transaction.
    #[getter]
    pub fn compute_units_consumed(&self) -> u64 {
        self.0.compute_units_consumed
    }
}

/// A transaction result.
///
/// Contains transaction metadata, and the transaction error, if there is one.
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.bankrun", subclass)]
pub struct BanksTransactionResultWithMeta(BanksTransactionResultWithMetadata);

transaction_status_boilerplate!(BanksTransactionResultWithMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BanksTransactionResultWithMeta {
    #[new]
    pub fn new(result: Option<TransactionErrorType>, meta: Option<BanksTransactionMeta>) -> Self {
        BanksTransactionResultWithMetadata {
            result: match result {
                None => Ok(()),
                Some(e) => Err(e.into()),
            },
            metadata: meta.map(Into::into),
        }
        .into()
    }

    /// Optional[TransactionErrorType]: The transaction error info, if the transaction failed.
    #[getter]
    pub fn result(&self) -> Option<TransactionErrorType> {
        match self.0.result.clone() {
            Ok(()) => None,
            Err(x) => Some(TransactionErrorType::from(x)),
        }
    }

    /// Optional[BanksTransactionMeta]: The transaction metadata.
    #[getter]
    pub fn meta(&self) -> Option<BanksTransactionMeta> {
        self.0.metadata.clone().map(Into::into)
    }
}

impl From<BanksTransactionResultWithSimulation> for BanksTransactionResultWithMeta {
    fn from(r: BanksTransactionResultWithSimulation) -> Self {
        BanksTransactionResultWithMetadata {
            result: match r.result {
                None => Ok(()),
                Some(x) => x,
            },
            metadata: r.simulation_details.map(|d| TransactionMetadata {
                log_messages: d.logs,
                compute_units_consumed: d.units_consumed,
                return_data: d.return_data,
            }),
        }
        .into()
    }
}
