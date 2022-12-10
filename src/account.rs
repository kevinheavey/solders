#![allow(clippy::redundant_closure)]
use std::str::FromStr;

use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};
use solana_sdk::{account::Account as AccountOriginal, clock::Epoch};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_primitives::pubkey::Pubkey;
use solders_traits::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    RichcmpEqualityOnly,
};

use crate::{
    account_decoder::ParsedAccount,
    tmp_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding},
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, From, Into)]
pub struct Account(AccountOriginal);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Account {
    #[new]
    #[pyo3(signature = (lamports, data, owner, executable = false, rent_epoch = Epoch::default()))]
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

solders_traits::common_methods_default!(Account);
impl RichcmpEqualityOnly for Account {}

impl TryFrom<UiAccount> for Account {
    type Error = &'static str;
    fn try_from(acc: UiAccount) -> Result<Self, Self::Error> {
        let decoded = acc
            .decode::<AccountOriginal>()
            .ok_or("Cannot decode JsonParsed here.")?;
        Ok(decoded.into())
    }
}

impl From<Account> for UiAccount {
    fn from(acc: Account) -> Self {
        let underlying = acc.0;
        Self {
            lamports: underlying.lamports,
            data: UiAccountData::Binary(base64::encode(underlying.data), UiAccountEncoding::Base64),
            owner: underlying.owner.to_string(),
            executable: underlying.executable,
            rent_epoch: underlying.rent_epoch,
        }
    }
}

/// An Account with data that is stored on chain, where the data is parsed as a JSON string.
///
/// Args:
///     lamports (int): Lamports in the account.
///     data (solders.account_decoder.ParsedAccount): Data held in this account.
///     owner (Pubkey): The program that owns this account. If executable, the program that loads this account.
///     executable (bool): Whether this account's data contains a loaded program (and is now read-only). Defaults to False.
///     epoch_info (int): The epoch at which this account will next owe rent. Defaults to 0.
///
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.account", subclass)]
pub struct AccountJSON {
    /// int: Lamports in the account.
    #[pyo3(get)]
    pub lamports: u64,
    /// solders.account_decoder.ParsedAccount: Data held in this account.
    #[pyo3(get)]
    pub data: ParsedAccount,
    /// Pubkey: The program that owns this account. If executable, the program that loads this account.
    #[pyo3(get)]
    pub owner: Pubkey,
    /// bool: Whether this account's data contains a loaded program (and is now read-only).
    #[pyo3(get)]
    pub executable: bool,
    /// int: The epoch at which this account will next owe rent.
    #[pyo3(get)]
    pub rent_epoch: Epoch,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AccountJSON {
    #[new]
    #[pyo3(signature = (lamports, data, owner, executable=false, rent_epoch=Epoch::default()))]
    pub fn new(
        lamports: u64,
        data: ParsedAccount,
        owner: Pubkey,
        executable: bool,
        rent_epoch: Epoch,
    ) -> Self {
        Self {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        }
    }
}

impl std::fmt::Display for AccountJSON {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
pybytes_general_via_bincode!(AccountJSON);
py_from_bytes_general_via_bincode!(AccountJSON);

solders_traits::common_methods_default!(AccountJSON);
impl RichcmpEqualityOnly for AccountJSON {}

impl TryFrom<UiAccount> for AccountJSON {
    type Error = String;
    fn try_from(acc: UiAccount) -> Result<Self, Self::Error> {
        if let UiAccountData::Json(parsed_account) = acc.data {
            Ok(Self {
                lamports: acc.lamports,
                data: parsed_account.into(),
                owner: Pubkey::from_str(&acc.owner).unwrap(),
                executable: acc.executable,
                rent_epoch: acc.rent_epoch,
            })
        } else {
            Err(format!(
                "Expected UiAccountData::Json, found {:?}",
                acc.data
            ))
        }
    }
}

impl From<AccountJSON> for UiAccount {
    fn from(acc: AccountJSON) -> Self {
        Self {
            lamports: acc.lamports,
            data: UiAccountData::Json(acc.data.into()),
            owner: acc.owner.to_string(),
            executable: acc.executable,
            rent_epoch: acc.rent_epoch,
        }
    }
}

pub(crate) fn create_account_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "account")?;
    m.add_class::<Account>()?;
    m.add_class::<AccountJSON>()?;
    Ok(m)
}
