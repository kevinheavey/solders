#![allow(clippy::too_many_arguments)]
use derive_more::{From, Into};
extern crate base64;
use solders_pubkey::Pubkey;
use solders_traits_core::transaction_status_boilerplate;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_transaction_context::TransactionReturnData as TransactionReturnDataOriginal;
use solana_transaction_status_client_types::UiTransactionReturnData;
use solders_macros::{common_methods, richcmp_eq_only};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.transaction_status", subclass)]
pub struct TransactionReturnData(pub TransactionReturnDataOriginal);
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
    pub fn data(&self) -> Vec<u8> {
        self.0.data.clone()
    }
}

impl From<TransactionReturnData> for UiTransactionReturnData {
    fn from(t: TransactionReturnData) -> Self {
        TransactionReturnDataOriginal::from(t).into()
    }
}

impl From<UiTransactionReturnData> for TransactionReturnData {
    fn from(r: UiTransactionReturnData) -> Self {
        Self::new(
            r.program_id.parse().unwrap(),
            base64::decode(r.data.0).unwrap(),
        )
    }
}
