#![allow(clippy::redundant_closure)]
use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use solana_account::Account as AccountOriginal;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_pubkey::Pubkey;
use solders_traits_core::{
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode, RichcmpEqualityOnly,
};

use solana_account_decoder_client_types::{UiAccount, UiAccountData, UiAccountEncoding};
use solders_account_decoder::ParsedAccount;

// The Account from solana_sdk doesn't serialize the owner pubkey as base58,
// so we copy it and change that.
/// An Account with data that is stored on chain.
///
/// Args:
///     lamports (int): Lamports in the account.
///     data (bytes): Data held in this account.
///     owner (Pubkey): The program that owns this account. If executable, the program that loads this account.
///     executable (bool): Whether this account's data contains a loaded program (and is now read-only). Defaults to False.
///     epoch_info (int): The epoch at which this account will next owe rent. Defaults to 0.
///
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.account", subclass)]
pub struct Account {
    /// lamports in the account
    #[pyo3(get)]
    pub lamports: u64,
    /// data held in this account
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    #[pyo3(get)]
    pub executable: bool,
    /// the epoch at which this account will next owe rent
    #[pyo3(get)]
    pub rent_epoch: u64,
}

impl From<AccountOriginal> for Account {
    fn from(value: AccountOriginal) -> Self {
        let AccountOriginal {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        } = value;
        Self {
            lamports,
            data,
            owner: owner.into(),
            executable,
            rent_epoch,
        }
    }
}

impl From<Account> for AccountOriginal {
    fn from(value: Account) -> Self {
        let Account {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        } = value;
        Self {
            lamports,
            data,
            owner: owner.into(),
            executable,
            rent_epoch,
        }
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Account {
    #[new]
    #[pyo3(signature = (lamports, data, owner, executable = false, rent_epoch = u64::default()))]
    pub fn new(
        lamports: u64,
        data: &[u8],
        owner: Pubkey,
        executable: bool,
        rent_epoch: u64,
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

    /// bytes: Data held in this account.
    #[getter]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
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

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pybytes_general_via_bincode!(Account);
py_from_bytes_general_via_bincode!(Account);

solders_traits_core::common_methods_default!(Account);
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
        Self {
            lamports: acc.lamports,
            data: UiAccountData::Binary(base64::encode(acc.data), UiAccountEncoding::Base64),
            owner: acc.owner.to_string(),
            executable: acc.executable,
            rent_epoch: acc.rent_epoch,
            space: None,
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
#[serde_as]
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
    #[serde_as(as = "DisplayFromStr")]
    pub owner: Pubkey,
    /// bool: Whether this account's data contains a loaded program (and is now read-only).
    #[pyo3(get)]
    pub executable: bool,
    /// int: The epoch at which this account will next owe rent.
    #[pyo3(get)]
    pub rent_epoch: u64,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AccountJSON {
    #[new]
    #[pyo3(signature = (lamports, data, owner, executable=false, rent_epoch=u64::default()))]
    pub fn new(
        lamports: u64,
        data: ParsedAccount,
        owner: Pubkey,
        executable: bool,
        rent_epoch: u64,
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

solders_traits_core::common_methods_default!(AccountJSON);
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
        let space = acc.data.0.space;
        Self {
            lamports: acc.lamports,
            data: UiAccountData::Json(acc.data.into()),
            owner: acc.owner.to_string(),
            executable: acc.executable,
            rent_epoch: acc.rent_epoch,
            space: Some(space),
        }
    }
}

pub fn include_account(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Account>()?;
    m.add_class::<AccountJSON>()?;
    Ok(())
}
