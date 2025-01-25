use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solders_macros::richcmp_eq_only;
use solders_pubkey::Pubkey;
use solders_traits_core::RichcmpEqualityOnly;

/// Fieldless filters for ``logsSubscribe``.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", eq, eq_int)]
pub enum RpcTransactionLogsFilter {
    All,
    AllWithVotes,
}

/// ``mentions`` filter for ``logsSubscribe``.
///
/// Args:
///     pubkey (Pubkey): Subscribe to all transactions that mention the provided Pubkey.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionLogsFilterMentions(pub Vec<String>);

#[richcmp_eq_only]
#[pymethods]
impl RpcTransactionLogsFilterMentions {
    #[new]
    pub fn new(pubkey: &Pubkey) -> Self {
        Self(vec![pubkey.to_string()])
    }

    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0[0]).unwrap()
    }
}

impl RichcmpEqualityOnly for RpcTransactionLogsFilterMentions {}

/// ``mint`` filter for ``getTokenAccountsBy*`` methods.
///
/// Args:
///     mint (Pubkey):  Pubkey of the specific token Mint to limit accounts to.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTokenAccountsFilterMint(pub Pubkey);

#[richcmp_eq_only]
#[pymethods]
impl RpcTokenAccountsFilterMint {
    #[new]
    pub fn new(mint: Pubkey) -> Self {
        Self(mint)
    }

    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    pub fn mint(&self) -> Pubkey {
        self.0
    }
}

impl RichcmpEqualityOnly for RpcTokenAccountsFilterMint {}

/// ``programId`` filter for ``getTokenAccountsBy*`` methods.
///
/// Args:
///     program_id (Pubkey):   Pubkey of the Token program that owns the accounts.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTokenAccountsFilterProgramId(pub Pubkey);

#[richcmp_eq_only]
#[pymethods]
impl RpcTokenAccountsFilterProgramId {
    #[new]
    pub fn new(program_id: Pubkey) -> Self {
        Self(program_id)
    }

    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    pub fn program_id(&self) -> Pubkey {
        self.0
    }
}

impl RichcmpEqualityOnly for RpcTokenAccountsFilterProgramId {}

/// Filter for ``blockSubscribe``.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", eq, eq_int)]
pub enum RpcBlockSubscribeFilter {
    All,
}

/// ``mentions`` filter for ``blockSubscribe``.
///
/// Args:
///     pubkey (Pubkey): Return only transactions that mention the provided pubkey.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcBlockSubscribeFilterMentions(pub String);

#[richcmp_eq_only]
#[pymethods]
impl RpcBlockSubscribeFilterMentions {
    #[new]
    pub fn new(pubkey: &Pubkey) -> Self {
        Self(pubkey.to_string())
    }

    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0).unwrap()
    }
}

impl RichcmpEqualityOnly for RpcBlockSubscribeFilterMentions {}
