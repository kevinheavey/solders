use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_clock::{
    Clock as ClockOriginal, Epoch, Slot, UnixTimestamp, DEFAULT_DEV_SLOTS_PER_EPOCH,
    DEFAULT_HASHES_PER_SECOND, DEFAULT_HASHES_PER_TICK, DEFAULT_MS_PER_SLOT,
    DEFAULT_SLOTS_PER_EPOCH, DEFAULT_S_PER_SLOT, DEFAULT_TICKS_PER_SECOND, DEFAULT_TICKS_PER_SLOT,
    FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET, GENESIS_EPOCH, HOLD_TRANSACTIONS_SLOT_OFFSET,
    INITIAL_RENT_EPOCH, MAX_HASH_AGE_IN_SECONDS, MAX_PROCESSING_AGE, MAX_RECENT_BLOCKHASHES,
    MAX_TRANSACTION_FORWARDING_DELAY, MAX_TRANSACTION_FORWARDING_DELAY_GPU, MS_PER_TICK,
    NUM_CONSECUTIVE_LEADER_SLOTS, SECONDS_PER_DAY, TICKS_PER_DAY,
};
use solders_traits_core::transaction_status_boilerplate;

/// A representation of network time.
///
/// All members of ``Clock`` start from 0 upon network boot.
#[pyclass(module = "solders.clock", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, From, Into)]
pub struct Clock(pub ClockOriginal);

transaction_status_boilerplate!(Clock);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl Clock {
    #[new]
    fn new(
        slot: Slot,
        epoch_start_timestamp: UnixTimestamp,
        epoch: Epoch,
        leader_schedule_epoch: Epoch,
        unix_timestamp: UnixTimestamp,
    ) -> Self {
        ClockOriginal {
            slot,
            epoch_start_timestamp,
            epoch,
            leader_schedule_epoch,
            unix_timestamp,
        }
        .into()
    }

    /// int: The current Slot.
    #[getter]
    pub fn slot(&self) -> Slot {
        self.0.slot
    }

    #[setter]
    pub fn set_slot(&mut self, slot: Slot) {
        self.0.slot = slot;
    }

    /// int: The timestamp of the first ``Slot`` in this ``Epoch``.
    #[getter]
    pub fn epoch_start_timestamp(&self) -> UnixTimestamp {
        self.0.epoch_start_timestamp
    }

    #[setter]
    pub fn set_epoch_start_timestamp(&mut self, timestamp: UnixTimestamp) {
        self.0.epoch_start_timestamp = timestamp;
    }

    /// int: The current epoch.
    #[getter]
    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    #[setter]
    pub fn set_epoch(&mut self, epoch: Epoch) {
        self.0.epoch = epoch;
    }

    /// int: The future Epoch for which the leader schedule has most recently been calculated.
    #[getter]
    pub fn leader_schedule_epoch(&self) -> Epoch {
        self.0.leader_schedule_epoch
    }

    #[setter]
    pub fn set_leader_schedule_epoch(&mut self, epoch: Epoch) {
        self.0.leader_schedule_epoch = epoch;
    }

    /// int: The approximate real world time of the current slot.
    #[getter]
    pub fn unix_timestamp(&self) -> UnixTimestamp {
        self.0.unix_timestamp
    }

    #[setter]
    pub fn set_unix_timestamp(&mut self, timestamp: UnixTimestamp) {
        self.0.unix_timestamp = timestamp;
    }
}

pub fn include_clock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Clock>()?;
    m.add("DEFAULT_DEV_SLOTS_PER_EPOCH", DEFAULT_DEV_SLOTS_PER_EPOCH)?;
    m.add("DEFAULT_HASHES_PER_SECOND", DEFAULT_HASHES_PER_SECOND)?;
    m.add("DEFAULT_HASHES_PER_TICK", DEFAULT_HASHES_PER_TICK)?;
    m.add("DEFAULT_MS_PER_SLOT", DEFAULT_MS_PER_SLOT)?;
    m.add("DEFAULT_SLOTS_PER_EPOCH", DEFAULT_SLOTS_PER_EPOCH)?;
    m.add("DEFAULT_S_PER_SLOT", DEFAULT_S_PER_SLOT)?;
    m.add("DEFAULT_TICKS_PER_SECOND", DEFAULT_TICKS_PER_SECOND)?;
    m.add("DEFAULT_TICKS_PER_SLOT", DEFAULT_TICKS_PER_SLOT)?;
    m.add(
        "FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET",
        FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET,
    )?;
    m.add("GENESIS_EPOCH", GENESIS_EPOCH)?;
    m.add(
        "HOLD_TRANSACTIONS_SLOT_OFFSET",
        HOLD_TRANSACTIONS_SLOT_OFFSET,
    )?;
    m.add("INITIAL_RENT_EPOCH", INITIAL_RENT_EPOCH)?;
    m.add("MAX_HASH_AGE_IN_SECONDS", MAX_HASH_AGE_IN_SECONDS)?;
    m.add("MAX_PROCESSING_AGE", MAX_PROCESSING_AGE)?;
    m.add("MAX_RECENT_BLOCKHASHES", MAX_RECENT_BLOCKHASHES)?;
    m.add(
        "MAX_TRANSACTION_FORWARDING_DELAY",
        MAX_TRANSACTION_FORWARDING_DELAY,
    )?;
    m.add(
        "MAX_TRANSACTION_FORWARDING_DELAY_GPU",
        MAX_TRANSACTION_FORWARDING_DELAY_GPU,
    )?;
    m.add("MS_PER_TICK", MS_PER_TICK)?;
    m.add("NUM_CONSECUTIVE_LEADER_SLOTS", NUM_CONSECUTIVE_LEADER_SLOTS)?;
    m.add("SECONDS_PER_DAY", SECONDS_PER_DAY)?;
    m.add("TICKS_PER_DAY", TICKS_PER_DAY)?;
    Ok(())
}
