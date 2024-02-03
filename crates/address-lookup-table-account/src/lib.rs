use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_program::{
    address_lookup_table::{
        instruction::derive_lookup_table_address as derive_lookup_table_address_original,
        AddressLookupTableAccount as AddressLookupTableAccountOriginal,
    },
    pubkey::Pubkey as PubkeyOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_pubkey::Pubkey;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};

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

solders_traits_core::common_methods_default!(AddressLookupTableAccount);
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

#[pyfunction]
pub fn derive_lookup_table_address(
    authority_address: Pubkey,
    recent_block_slot: u64,
) -> (Pubkey, u8) {
    let (lookup_table_address, bump_seed) =
        derive_lookup_table_address_original(&authority_address.into(), recent_block_slot);
    (lookup_table_address.into(), bump_seed)
}
