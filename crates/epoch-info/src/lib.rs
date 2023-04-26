use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_program::clock::Epoch;
use solana_sdk::epoch_info::EpochInfo as EpochInfoOriginal;
use solders_macros::{common_methods, richcmp_eq_only};

use solders_traits_core::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};

/// Information about the current epoch.
///
/// Args:
///     epoch (int): The current epoch.
///     slot_index (int): The current slot, relative to the start of the current epoch.
///     slots_in_epoch (int): The number of slots in this epoch.
///     absolute_slot (int): The absolute current slot.
///     block_height (int): The current block height.
///     transaction_count (Optional[int]): Total number of transactions processed without error since genesis
///
#[pyclass(module = "solders.epoch_info", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, From, Into)]
pub struct EpochInfo(EpochInfoOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EpochInfo {
    #[new]
    pub fn new(
        epoch: Epoch,
        slot_index: u64,
        slots_in_epoch: u64,
        absolute_slot: u64,
        block_height: u64,
        transaction_count: Option<u64>,
    ) -> Self {
        EpochInfoOriginal {
            epoch,
            slot_index,
            slots_in_epoch,
            absolute_slot,
            block_height,
            transaction_count,
        }
        .into()
    }

    /// int: The current epoch
    #[getter]
    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    /// int: The current slot, relative to the start of the current epoch
    #[getter]
    pub fn slot_index(&self) -> u64 {
        self.0.slot_index
    }

    /// int: The number of slots in this epoch
    #[getter]
    pub fn slots_in_epoch(&self) -> u64 {
        self.0.slots_in_epoch
    }

    /// int: The absolute current slot
    #[getter]
    pub fn absolute_slot(&self) -> u64 {
        self.0.absolute_slot
    }

    /// int: The current block height
    #[getter]
    pub fn block_height(&self) -> u64 {
        self.0.block_height
    }

    /// Optional[int]: Total number of transactions processed without error since genesis
    #[getter]
    pub fn transaction_count(&self) -> Option<u64> {
        self.0.transaction_count
    }
}

impl_display!(EpochInfo);
pybytes_general_via_bincode!(EpochInfo);
py_from_bytes_general_via_bincode!(EpochInfo);
solders_traits_core::common_methods_default!(EpochInfo);
impl RichcmpEqualityOnly for EpochInfo {}

pub fn create_epoch_info_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "epoch_info")?;
    m.add_class::<EpochInfo>()?;
    Ok(m)
}
