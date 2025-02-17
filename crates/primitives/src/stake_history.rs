use {
    pyo3::prelude::*,
    serde::{Deserialize, Serialize},
    solana_stake_interface::stake_history::{
        StakeHistory as StakeHistoryOriginal, StakeHistoryEntry as StakeHistoryEntryOriginal,
    },
    solders_traits_core::transaction_status_boilerplate,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[pyclass(module = "solders.stake_history", subclass)]
pub struct StakeHistoryEntry(pub(crate) StakeHistoryEntryOriginal);

transaction_status_boilerplate!(StakeHistoryEntry);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl StakeHistoryEntry {
    #[new]
    pub fn new(effective: u64, activating: u64, deactivating: u64) -> Self {
        Self(StakeHistoryEntryOriginal {
            effective,
            activating,
            deactivating,
        })
    }

    /// effective stake at this epoch
    #[getter]
    pub fn effective(&self) -> u64 {
        self.0.effective
    }

    #[setter]
    pub fn set_effective(&mut self, val: u64) {
        self.0.effective = val;
    }

    /// sum of portion of stakes not fully warmed up
    #[getter]
    pub fn activating(&self) -> u64 {
        self.0.activating
    }

    #[setter]
    pub fn set_activating(&mut self, val: u64) {
        self.0.effective = val;
    }

    /// requested to be cooled down, not fully deactivated yet
    #[getter]
    pub fn deactivating(&self) -> u64 {
        self.0.deactivating
    }

    #[setter]
    pub fn set_deactivating(&mut self, val: u64) {
        self.0.effective = val;
    }
}

/// A type to hold data for the StakeHistory sysvar.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[pyclass(module = "solders.stake_history", subclass)]
pub struct StakeHistory(pub StakeHistoryOriginal);

transaction_status_boilerplate!(StakeHistory);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl StakeHistory {
    #[allow(clippy::new_without_default)]
    #[new]
    pub fn new() -> Self {
        Self(StakeHistoryOriginal::default())
    }

    pub fn get(&self, epoch: u64) -> Option<StakeHistoryEntry> {
        self.0.get(epoch).map(|x| StakeHistoryEntry(x.clone()))
    }

    pub fn add(&mut self, epoch: u64, entry: &StakeHistoryEntry) {
        self.0.add(epoch, entry.clone().0)
    }
}

pub fn include_stake_history(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StakeHistoryEntry>()?;
    m.add_class::<StakeHistory>()?;
    Ok(())
}
