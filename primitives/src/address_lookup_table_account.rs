use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount as AddressLookupTableAccountOriginal,
    pubkey::Pubkey as PubkeyOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};

use crate::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
#[serde(remote = "AddressLookupTableAccountOriginal")]
struct AddressLookupTableAccountOriginalDef {
    key: PubkeyOriginal,
    addresses: Vec<PubkeyOriginal>,
}

/// The definition of address lookup table accounts as used by ``MessageV0``.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.address_lookup_table_account", subclass)]
pub struct AddressLookupTableAccount(
    #[serde(with = "AddressLookupTableAccountOriginalDef")] AddressLookupTableAccountOriginal,
);

impl_display!(AddressLookupTableAccount);
pybytes_general_via_bincode!(AddressLookupTableAccount);
py_from_bytes_general_via_bincode!(AddressLookupTableAccount);

solders_traits::common_methods_default!(AddressLookupTableAccount);
impl RichcmpEqualityOnly for AddressLookupTableAccount {}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AddressLookupTableAccount {
    #[new]
    pub fn new(key: Pubkey, addresses: Vec<Pubkey>) -> Self {
        AddressLookupTableAccountOriginal {
            key: key.into(),
            addresses: addresses.into_iter().map(|a| a.into()).collect(),
        }
        .into()
    }

    #[getter]
    pub fn key(&self) -> Pubkey {
        self.0.key.into()
    }

    #[getter]
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.0
            .addresses
            .clone()
            .into_iter()
            .map(|a| a.into())
            .collect()
    }
}
