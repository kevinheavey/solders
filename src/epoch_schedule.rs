use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    clock::{Epoch, Slot},
    epoch_schedule::EpochSchedule as EpochScheduleOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only};

use solders_traits::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};

/// Configuration for epochs and slots.
///
/// Args:
///     slots_per_epoch (int): The maximum number of slots in each epoch.
///
#[pyclass(module = "solders.epoch_schedule", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, From, Into)]
pub struct EpochSchedule(EpochScheduleOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EpochSchedule {
    #[new]
    pub fn new(slots_per_epoch: u64) -> Self {
        EpochScheduleOriginal::new(slots_per_epoch).into()
    }

    /// int: The maximum number of slots in each epoch.
    #[getter]
    pub fn slots_per_epoch(&self) -> u64 {
        self.0.slots_per_epoch
    }

    /// int: A number of slots before beginning of an epoch to calculate a leader schedule for that epoch.
    #[getter]
    pub fn leader_schedule_slot_offset(&self) -> u64 {
        self.0.leader_schedule_slot_offset
    }

    /// bool: Whether epochs start short and grow
    #[getter]
    pub fn warmup(&self) -> bool {
        self.0.warmup
    }

    /// int: Basically ``log2(slots_per_epoch) - log2(MINIMUM_SLOTS_PER_EPOCH)``
    #[getter]
    pub fn first_normal_epoch(&self) -> u64 {
        self.0.first_normal_epoch
    }

    /// int: Basically ``MINIMUM_SLOTS_PER_EPOCH * (2.pow(first_normal_epoch) - 1)``
    #[getter]
    pub fn first_normal_slot(&self) -> u64 {
        self.0.first_normal_slot
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default EpochSchedule.
    ///
    /// Returns:
    ///     EpochSchedule: The default EpochSchedule.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }

    #[staticmethod]
    pub fn without_warmup() -> Self {
        EpochScheduleOriginal::without_warmup().into()
    }

    #[staticmethod]
    pub fn custom(slots_per_epoch: u64, leader_schedule_slot_offset: u64, warmup: bool) -> Self {
        EpochScheduleOriginal::custom(slots_per_epoch, leader_schedule_slot_offset, warmup).into()
    }

    /// Get the length of the given epoch (in slots).
    pub fn get_slots_in_epoch(&self, epoch: Epoch) -> u64 {
        self.0.get_slots_in_epoch(epoch)
    }

    /// Get the epoch for which the given slot should save off
    /// information about stakers.
    pub fn get_leader_schedule_epoch(&self, slot: Slot) -> Epoch {
        self.0.get_leader_schedule_epoch(slot)
    }

    /// Get epoch for the given slot
    pub fn get_epoch(&self, slot: Slot) -> Epoch {
        self.0.get_epoch(slot)
    }

    /// get epoch and offset into the epoch for the given slot
    pub fn get_epoch_and_slot_index(&self, slot: Slot) -> (Epoch, u64) {
        self.0.get_epoch_and_slot_index(slot)
    }

    pub fn get_first_slot_in_epoch(&self, epoch: Epoch) -> Slot {
        self.0.get_first_slot_in_epoch(epoch)
    }

    pub fn get_last_slot_in_epoch(&self, epoch: Epoch) -> Slot {
        self.0.get_last_slot_in_epoch(epoch)
    }
}

impl_display!(EpochSchedule);
pybytes_general_via_bincode!(EpochSchedule);
py_from_bytes_general_via_bincode!(EpochSchedule);
solders_traits::common_methods_default!(EpochSchedule);
impl RichcmpEqualityOnly for EpochSchedule {}

pub(crate) fn create_epoch_schedule_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "epoch_schedule")?;
    m.add_class::<EpochSchedule>()?;
    Ok(m)
}
