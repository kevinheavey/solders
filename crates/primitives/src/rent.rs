use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rent::{
    Rent as RentOriginal, RentDue, ACCOUNT_STORAGE_OVERHEAD, DEFAULT_BURN_PERCENT,
    DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
};
use solders_traits_core::transaction_status_boilerplate;

/// Configuration of network rent.
#[pyclass(module = "solders.account", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, From, Into)]
pub struct Rent(pub RentOriginal);

transaction_status_boilerplate!(Rent);

#[solders_macros::richcmp_eq_only]
#[solders_macros::common_methods]
#[pymethods]
impl Rent {
    #[new]
    pub fn new(lamports_per_byte_year: u64, exemption_threshold: f64, burn_percent: u8) -> Self {
        RentOriginal {
            lamports_per_byte_year,
            exemption_threshold,
            burn_percent,
        }
        .into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// int: Rental rate in lamports/byte-year.
    #[getter]
    pub fn lamports_per_byte_year(&self) -> u64 {
        self.0.lamports_per_byte_year
    }

    /// float: Amount of time (in years) a balance must include rent for the account to be rent exempt.
    #[getter]
    pub fn exemption_threshold(&self) -> f64 {
        self.0.exemption_threshold
    }

    /// int: The percentage of collected rent that is burned.
    #[getter]
    pub fn burn_percent(&self) -> u8 {
        self.0.burn_percent
    }

    /// Calculate how much rent to burn from the collected rent.
    ///
    /// The first value returned is the amount burned. The second is the amount
    /// to distribute to validators.
    ///
    /// Args:
    ///     rent_collected (int): The amount of rent collected.
    ///
    /// Returns:
    ///     tuple[int, int]: The amount burned and the amount to distribute to validators.
    pub fn calculate_burn(&self, rent_collected: u64) -> (u64, u64) {
        self.0.calculate_burn(rent_collected)
    }

    /// Minimum balance due for rent-exemption of a given account data size.
    ///
    /// Note: a stripped-down version of this calculation is used in
    /// ``calculate_split_rent_exempt_reserve`` in the stake program. When this
    /// function is updated, eg. when making rent variable, the stake program
    /// will need to be refactored.
    ///
    /// Args:
    ///     data_len (int): The account data size.
    ///
    /// Returns:
    ///     int: The minimum balance due.
    pub fn minimum_balance(&self, data_len: usize) -> u64 {
        self.0.minimum_balance(data_len)
    }

    /// Whether a given balance and data length would be exempt.
    pub fn is_exempt(&self, balance: u64, data_len: usize) -> bool {
        self.0.is_exempt(balance, data_len)
    }

    /// Rent due on account's data length with balance.
    ///
    /// Args:
    ///     balance (int): The account balance.
    ///     data_len (int): The account data length.
    ///     years_elapsed (float): Time elapsed in years.
    ///
    /// Returns:
    ///     Optional[int]: The rent due.
    pub fn due(&self, balance: u64, data_len: usize, years_elapsed: f64) -> Option<u64> {
        match self.0.due(balance, data_len, years_elapsed) {
            RentDue::Exempt => None,
            RentDue::Paying(x) => Some(x),
        }
    }

    /// Rent due for account that is known to be not exempt.
    ///
    /// Args:
    ///     data_len (int): The account data length.
    ///     years_elapsed (float): Time elapsed in years.
    ///
    /// Returns:
    ///     int: The amount due.
    pub fn due_amount(&self, data_len: usize, years_elapsed: f64) -> u64 {
        self.0.due_amount(data_len, years_elapsed)
    }

    /// Creates a ``Rent`` that charges no lamports.
    ///
    /// This is used for testing.
    ///
    #[staticmethod]
    pub fn free() -> Self {
        RentOriginal::free().into()
    }

    /// Creates a ``Rent`` that is scaled based on the number of slots in an epoch.
    ///
    /// This is used for testing.
    #[staticmethod]
    pub fn with_slots_per_epoch(slots_per_epoch: u64) -> Self {
        RentOriginal::with_slots_per_epoch(slots_per_epoch).into()
    }
}

pub fn include_rent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rent>()?;
    m.add(
        "DEFAULT_LAMPORTS_PER_BYTE_YEAR",
        DEFAULT_LAMPORTS_PER_BYTE_YEAR,
    )?;
    m.add("DEFAULT_EXEMPTION_THRESHOLD", DEFAULT_EXEMPTION_THRESHOLD)?;
    m.add("DEFAULT_BURN_PERCENT", DEFAULT_BURN_PERCENT)?;
    m.add("ACCOUNT_STORAGE_OVERHEAD", ACCOUNT_STORAGE_OVERHEAD)?;
    Ok(())
}
