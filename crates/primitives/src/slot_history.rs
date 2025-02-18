use {
    bv::BitVec,
    pyo3::prelude::*,
    serde::{Deserialize, Serialize},
    solana_slot_history::{Check, SlotHistory as SlotHistoryOriginal},
    solders_macros::enum_original_mapping,
    solders_traits_core::transaction_status_boilerplate,
};

#[derive(Debug, PartialEq)]
#[pyclass(module = "solders.slot_history", eq, eq_int)]
#[enum_original_mapping(Check)]
pub enum SlotHistoryCheck {
    Future,
    TooOld,
    Found,
    NotFound,
}

/// A bitvector indicating which slots are present in the past epoch.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[pyclass(module = "solders.slot_history", subclass)]
pub struct SlotHistory(pub SlotHistoryOriginal);

transaction_status_boilerplate!(SlotHistory);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl SlotHistory {
    #[new]
    pub fn new(bits: Vec<u64>, next_slot: u64) -> Self {
        let bits_converted: BitVec<u64> = BitVec::from(bits.to_vec());
        Self(SlotHistoryOriginal {
            bits: bits_converted,
            next_slot,
        })
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self(SlotHistoryOriginal::default())
    }

    #[getter]
    pub fn bits(&self) -> Vec<u64> {
        self.0.bits.clone().into_boxed_slice().to_vec()
    }

    #[setter]
    pub fn set_bits(&mut self, bits: Vec<u64>) {
        let bits_converted: BitVec<u64> = BitVec::from(bits.to_vec());
        self.0.bits = bits_converted;
    }

    #[getter]
    pub fn next_slot(&self) -> u64 {
        self.0.next_slot
    }

    #[setter]
    pub fn set_next_slot(&mut self, slot: u64) {
        self.0.next_slot = slot;
    }

    pub fn add(&mut self, slot: u64) {
        self.0.add(slot);
    }

    pub fn check(&self, slot: u64) -> SlotHistoryCheck {
        SlotHistoryCheck::from(self.0.check(slot))
    }

    pub fn oldest(&self) -> u64 {
        self.0.oldest()
    }

    pub fn newest(&self) -> u64 {
        self.0.newest()
    }
}

pub fn include_slot_history(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SlotHistoryCheck>()?;
    m.add_class::<SlotHistory>()?;
    Ok(())
}
