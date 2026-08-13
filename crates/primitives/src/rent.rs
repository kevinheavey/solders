// `exemption_threshold` and `burn_percent` are deprecated upstream (solana-rent 4.1)
// but still part of the sysvar layout, so we keep exposing them.
#![allow(deprecated)]
use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rent::{
    Rent as RentOriginal, ACCOUNT_STORAGE_OVERHEAD, DEFAULT_LAMPORTS_PER_BYTE, SIZE as RENT_SIZE,
};
use solders_traits_core::transaction_status_boilerplate;

/// Configuration of network rent.
#[pyclass(from_py_object, module = "solders.rent", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, From, Into)]
pub struct Rent(pub RentOriginal);

transaction_status_boilerplate!(Rent);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl Rent {
    #[classattr]
    const LENGTH: usize = RENT_SIZE;

    #[new]
    #[pyo3(signature = (lamports_per_byte, exemption_threshold=1.0, burn_percent=50))]
    pub fn new(lamports_per_byte: u64, exemption_threshold: f64, burn_percent: u8) -> Self {
        RentOriginal {
            lamports_per_byte,
            exemption_threshold: exemption_threshold.to_le_bytes(),
            burn_percent,
        }
        .into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// int: Rental rate in lamports/byte.
    #[getter]
    pub fn lamports_per_byte(&self) -> u64 {
        self.0.lamports_per_byte
    }

    /// float: Formerly the amount of time (in years) a balance must include rent
    /// for the account to be rent exempt. Retained because it is still part of
    /// the sysvar layout.
    #[getter]
    pub fn exemption_threshold(&self) -> f64 {
        f64::from_le_bytes(self.0.exemption_threshold)
    }

    /// int: Formerly the percentage of collected rent that is burned. Retained
    /// because it is still part of the sysvar layout.
    #[getter]
    pub fn burn_percent(&self) -> u8 {
        self.0.burn_percent
    }

    /// Minimum balance due for rent-exemption of a given account data size.
    ///
    /// Args:
    ///     data_len (int): The account data size.
    ///
    /// Returns:
    ///     int: The minimum balance due.
    pub fn minimum_balance(&self, data_len: usize) -> u64 {
        self.0.minimum_balance(data_len)
    }

    /// Minimum balance due for rent-exemption of a given account data size,
    /// without checking that ``data_len`` is within the maximum permitted size.
    ///
    /// Args:
    ///     data_len (int): The account data size.
    ///
    /// Returns:
    ///     int: The minimum balance due.
    pub fn minimum_balance_unchecked(&self, data_len: usize) -> u64 {
        self.0.minimum_balance_unchecked(data_len)
    }

    /// Minimum balance due for rent-exemption of a given account data size,
    /// or ``None`` if the calculation overflows.
    ///
    /// Args:
    ///     data_len (int): The account data size.
    ///
    /// Returns:
    ///     Optional[int]: The minimum balance due.
    pub fn try_minimum_balance(&self, data_len: usize) -> Option<u64> {
        self.0.try_minimum_balance(data_len)
    }

    /// Whether a given balance and data length would be exempt.
    pub fn is_exempt(&self, balance: u64, data_len: usize) -> bool {
        self.0.is_exempt(balance, data_len)
    }

    /// Creates a ``Rent`` that charges no lamports.
    ///
    /// This is used for testing.
    ///
    #[staticmethod]
    pub fn free() -> Self {
        RentOriginal::free().into()
    }

    /// Creates a ``Rent`` with the given lamports per byte.
    ///
    /// Args:
    ///     lamports_per_byte (int): The lamports per byte.
    ///
    /// Returns:
    ///     Rent: The new Rent object.
    #[staticmethod]
    pub fn with_lamports_per_byte(lamports_per_byte: u64) -> Self {
        RentOriginal::with_lamports_per_byte(lamports_per_byte).into()
    }
}

pub fn include_rent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rent>()?;
    m.add("DEFAULT_LAMPORTS_PER_BYTE", DEFAULT_LAMPORTS_PER_BYTE)?;
    m.add("ACCOUNT_STORAGE_OVERHEAD", ACCOUNT_STORAGE_OVERHEAD)?;
    Ok(())
}
