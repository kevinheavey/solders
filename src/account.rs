use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::{account::Account as AccountOriginal, clock::Epoch};
use solders_macros::{common_methods, richcmp_eq_only};

use crate::{
    impl_display, pubkey::Pubkey, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};

/// An Account with data that is stored on chain.
///
/// Args:
///     lamports (int): Lamports in the account.
///     data (bytes): Data held in this account.
///     owner (Pubkey): The program that owns this account. If executable, the program that loads this account.
///     executable (bool): Whether this account's data contains a loaded program (and is now read-only). Defaults to False.
///     epoch_info (int): The epoch at which this account will next owe rent. Defaults to 0.
///
#[pyclass(module = "solders.account", subclass)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Account(AccountOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Account {
    #[new]
    #[args(executable = "bool::default()", rent_epoch = "Epoch::default()")]
    pub fn new(
        lamports: u64,
        data: &[u8],
        owner: Pubkey,
        executable: bool,
        rent_epoch: Epoch,
    ) -> Self {
        AccountOriginal {
            lamports,
            data: data.to_vec(),
            owner: owner.into(),
            executable,
            rent_epoch,
        }
        .into()
    }

    /// int: Lamports in the account.
    #[getter]
    pub fn lamports(&self) -> u64 {
        self.0.lamports
    }

    /// bytes: Data held in this account.
    #[getter]
    pub fn data<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &self.0.data)
    }

    /// Pubkey: The program that owns this account. If executable, the program that loads this account.
    #[getter]
    pub fn owner(&self) -> Pubkey {
        self.0.owner.into()
    }

    /// Whether this account's data contains a loaded program (and is now read-only).
    #[getter]
    pub fn executable(&self) -> bool {
        self.0.executable
    }

    /// int: The epoch at which this account will next owe rent.
    #[getter]
    pub fn rent_epoch(&self) -> Epoch {
        self.0.rent_epoch
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default account.
    ///
    /// Returns:
    ///     Account: The default account.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }
}

impl_display!(Account);
pybytes_general_via_bincode!(Account);
py_from_bytes_general_via_bincode!(Account);

impl CommonMethods<'_> for Account {}
impl RichcmpEqualityOnly for Account {}

impl From<AccountOriginal> for Account {
    fn from(a: AccountOriginal) -> Self {
        Self(a)
    }
}

pub(crate) fn create_account_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "account")?;
    m.add_class::<Account>()?;
    Ok(m)
}
