#![allow(clippy::too_many_arguments)]
use derive_more::{From, Into};
use solders_commitment_config::CommitmentConfig;
use solders_traits_core::transaction_status_boilerplate;
use solders_transaction_confirmation_status::TransactionConfirmationStatus;
use solders_transaction_error::TransactionErrorType;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_status::TransactionStatus as TransactionStatusOriginal;
use solders_macros::{common_methods, richcmp_eq_only};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionStatus(pub TransactionStatusOriginal);

transaction_status_boilerplate!(TransactionStatus);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl TransactionStatus {
    #[new]
    pub fn new(
        slot: u64,
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
    pub fn slot(&self) -> u64 {
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
