use derive_more::{From, Into};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_program::clock::Slot;
use solana_program::slot_hashes::SlotHashes as SlotHashesOriginal;
use solana_program::{
    address_lookup_table::{
        instruction::derive_lookup_table_address as derive_lookup_table_address_original,
        state::{
            AddressLookupTable as AddressLookupTableOriginal,
            LookupTableMeta as LookupTableMetaOriginal,
            LookupTableStatus as LookupTableStatusOriginal,
        },
        AddressLookupTableAccount as AddressLookupTableAccountOriginal,
    },
    pubkey::Pubkey as PubkeyOriginal,
};
use solders_hash::Hash;
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_pubkey::Pubkey;
use solders_traits_core::{
    handle_py_value_err, impl_display, py_from_bytes_general_via_bincode,
    pybytes_general_via_bincode, RichcmpEqualityOnly,
};
use std::borrow::Cow;

macro_rules! impl_defaults {
    ($s: ident) => {
        impl_display!($s);
        pybytes_general_via_bincode!($s);
        py_from_bytes_general_via_bincode!($s);

        solders_traits_core::common_methods_default!($s);
        impl RichcmpEqualityOnly for $s {}
    };
}

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

impl_defaults!(AddressLookupTableAccount);

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.address_lookup_table_account", subclass)]
pub struct LookupTableStatusDeactivating(pub usize);
impl_defaults!(LookupTableStatusDeactivating);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl LookupTableStatusDeactivating {
    #[new]
    pub fn new(remaining_blocks: usize) -> Self {
        Self(remaining_blocks)
    }

    #[getter]
    pub fn remaining_slots(&self) -> usize {
        self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, EnumIntoPy, FromPyObject)]
pub enum LookupTableStatusTagged {
    Deactivating(LookupTableStatusDeactivating),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[pyclass(module = "solders.address_lookup_table_account")]
pub enum LookupTableStatusFieldless {
    Activated,
    Deactivated,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, EnumIntoPy, FromPyObject)]
#[serde(untagged)]
pub enum LookupTableStatusType {
    Fieldless(LookupTableStatusFieldless),
    Tagged(LookupTableStatusTagged),
}

impl From<LookupTableStatusOriginal> for LookupTableStatusType {
    fn from(status: LookupTableStatusOriginal) -> Self {
        match status {
            LookupTableStatusOriginal::Activated => {
                Self::Fieldless(LookupTableStatusFieldless::Activated)
            }
            LookupTableStatusOriginal::Deactivated => {
                Self::Fieldless(LookupTableStatusFieldless::Deactivated)
            }
            LookupTableStatusOriginal::Deactivating { remaining_blocks } => {
                Self::Tagged(LookupTableStatusTagged::Deactivating(
                    LookupTableStatusDeactivating::new(remaining_blocks),
                ))
            }
        }
    }
}

impl From<LookupTableStatusType> for LookupTableStatusOriginal {
    fn from(status: LookupTableStatusType) -> Self {
        match status {
            LookupTableStatusType::Fieldless(LookupTableStatusFieldless::Activated) => {
                Self::Activated
            }
            LookupTableStatusType::Fieldless(LookupTableStatusFieldless::Deactivated) => {
                Self::Deactivated
            }
            LookupTableStatusType::Tagged(LookupTableStatusTagged::Deactivating(
                remaining_blocks,
            )) => Self::Deactivating {
                remaining_blocks: remaining_blocks.remaining_slots(),
            },
        }
    }
}

#[pyclass(module = "solders.address_lookup_table_account", subclass)]
#[derive(Debug, PartialEq, Eq, From, Into, Serialize, Deserialize)]
pub struct SlotHashes(SlotHashesOriginal);

impl Clone for SlotHashes {
    fn clone(&self) -> Self {
        SlotHashes(SlotHashesOriginal::new(self.0.slot_hashes()))
    }
}

impl_defaults!(SlotHashes);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl SlotHashes {
    #[new]
    pub fn new(slot_hashes: Vec<(Slot, Hash)>) -> Self {
        SlotHashes(SlotHashesOriginal::new(
            &slot_hashes
                .into_iter()
                .map(|(slot, hash)| (slot, hash.into()))
                .collect::<Vec<_>>(),
        ))
    }
    #[getter]
    pub fn slot_hashes(&self) -> Vec<(Slot, Hash)> {
        self.0
            .slot_hashes()
            .iter()
            .map(|(slot, hash)| (*slot, (*hash).into()))
            .collect()
    }
}

#[pyclass(module = "solders.address_lookup_table_account", subclass)]
#[derive(Clone, Debug, PartialEq, Eq, From, Into, Serialize, Deserialize)]
pub struct LookupTableMeta(LookupTableMetaOriginal);

impl_defaults!(LookupTableMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl LookupTableMeta {
    #[new]
    #[pyo3(signature = (deactivation_slot = u64::MAX, last_extended_slot = 0, last_extended_slot_start_index = 0, authority = None, padding = 0))]
    pub fn new(
        deactivation_slot: u64,
        last_extended_slot: u64,
        last_extended_slot_start_index: u8,
        authority: Option<Pubkey>,
        padding: u16,
    ) -> Self {
        LookupTableMetaOriginal {
            deactivation_slot,
            last_extended_slot,
            last_extended_slot_start_index,
            authority: authority.map(Into::into),
            _padding: padding,
        }
        .into()
    }

    #[getter]
    pub fn deactivation_slot(&self) -> u64 {
        self.0.deactivation_slot
    }

    #[getter]
    pub fn last_extended_slot(&self) -> u64 {
        self.0.last_extended_slot
    }

    #[getter]
    pub fn last_extended_slot_start_index(&self) -> u8 {
        self.0.last_extended_slot_start_index
    }

    #[getter]
    pub fn authority(&self) -> Option<Pubkey> {
        self.0.authority.map(Into::into)
    }

    #[getter]
    pub fn padding(&self) -> u16 {
        self.0._padding
    }

    pub fn is_active(&self, current_slot: Slot, slot_hashes: SlotHashes) -> bool {
        self.0.is_active(current_slot, &slot_hashes.into())
    }

    pub fn status(&self, current_slot: Slot, slot_hashes: SlotHashes) -> LookupTableStatusType {
        self.0.status(current_slot, &slot_hashes.into()).into()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "AddressLookupTableOriginal")]
pub struct AddressLookupTableOriginalDef<'a> {
    meta: LookupTableMetaOriginal,
    addresses: Cow<'a, [PubkeyOriginal]>,
}

#[pyclass(module = "solders.address_lookup_table_account", subclass)]
#[derive(Clone, Debug, PartialEq, From, Into, Serialize, Deserialize)]
pub struct AddressLookupTable(
    #[serde(with = "AddressLookupTableOriginalDef")] AddressLookupTableOriginal<'static>,
);

impl_defaults!(AddressLookupTable);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AddressLookupTable {
    #[new]
    pub fn new(meta: LookupTableMeta, addresses: Vec<Pubkey>) -> Self {
        AddressLookupTableOriginal {
            meta: meta.into(),
            addresses: Cow::from(addresses.into_iter().map(Into::into).collect::<Vec<_>>()),
        }
        .into()
    }

    #[getter]
    pub fn meta(&self) -> LookupTableMeta {
        self.0.meta.clone().into()
    }

    #[getter]
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.0.addresses.iter().map(Into::into).collect()
    }

    pub fn get_active_addresses_len(
        &self,
        current_slot: Slot,
        slot_hashes: SlotHashes,
    ) -> PyResult<usize> {
        handle_py_value_err(
            self.0
                .get_active_addresses_len(current_slot, &slot_hashes.into()),
        )
    }

    pub fn lookup(
        &self,
        current_slot: Slot,
        indexes: Vec<u8>,
        slot_hashes: SlotHashes,
    ) -> PyResult<Vec<Pubkey>> {
        handle_py_value_err(
            self.0
                .lookup(current_slot, indexes.as_slice(), &slot_hashes.into())
                .map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()),
        )
    }

    #[staticmethod]
    pub fn deserialize(data: &[u8]) -> PyResult<Self> {
        let address_looking_table = AddressLookupTableOriginal::deserialize(data)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))?;

        let addresses = Cow::from(
            address_looking_table
                .addresses
                .iter()
                .map(Clone::clone)
                .collect::<Vec<_>>(),
        );
        Ok(AddressLookupTableOriginal {
            meta: address_looking_table.meta,
            addresses,
        }
        .into())
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
