#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
use std::collections::HashMap;
use std::str::FromStr;

use camelpaste::paste;
use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TryFromInto};
use solana_account_decoder::{UiAccount, UiAccountData};
use solana_rpc_client_api::response::{
    RpcBlockhash as RpcBlockhashOriginal, RpcIdentity as RpcIdentityOriginal,
    RpcTokenAccountBalance as RpcTokenAccountBalanceOriginal,
    RpcVersionInfo as RpcVersionInfoOriginal, RpcVoteAccountInfo as RpcVoteAccountInfoOriginal,
    RpcVoteAccountStatus as RpcVoteAccountStatusOriginal,
};
use solana_sdk::clock::{Epoch, Slot};
use solders_account::{Account, AccountJSON};
use solders_account_decoder::UiTokenAmount;
use solders_hash::Hash as SolderHash;
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_pubkey::Pubkey;
use solders_transaction_error::TransactionErrorType;

use solders_rpc_response_data_boilerplate::response_data_boilerplate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcResponseContext {
    #[pyo3(get)]
    pub slot: Slot,
    #[pyo3(get)]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub api_version: Option<String>,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcResponseContext {
    #[new]
    pub fn new(slot: Slot, api_version: Option<String>) -> Self {
        Self { slot, api_version }
    }
}

response_data_boilerplate!(RpcResponseContext);

#[macro_export]
macro_rules! contextful_struct_def_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            value: $inner,
        }
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        #[serde_as]
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            #[serde_as(as = $serde_as)]
            value: $inner,
        }
    };
}

#[macro_export]
macro_rules! contextful_struct_def_no_eq {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            value: $inner,
        }
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        #[serde_as]
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            context: RpcResponseContext,
            #[pyo3(get)]
            #[serde_as(as = $serde_as)]
            value: $inner,
        }
    };
}

#[macro_export]
macro_rules! notification_struct_def_outer {
    ($name:ident) => {
        paste! {
            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
            #[pyclass(module = "solders.rpc.responses", subclass)]
            pub struct $name {
                #[pyo3(get)]
                result: [<$name Result>],
                #[pyo3(get)]
                subscription: u64,
            }
        }
    };
}

#[macro_export]
macro_rules! notification_struct_def_outer_no_eq {
    ($name:ident) => {
        paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
            #[pyclass(module = "solders.rpc.responses", subclass)]
            pub struct $name {
                #[pyo3(get)]
                result: [<$name Result>],
                #[pyo3(get)]
                subscription: u64,
            }
        }
    };
}

#[macro_export]
macro_rules! notification_struct_def {
    ($name:ident, $inner:ty) => {
        notification_struct_def_outer!($name);
        paste! {
            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
            #[pyclass(module = "solders.rpc.responses", subclass)]
            pub struct [<$name Result>] {
                #[pyo3(get)]
                context: RpcResponseContext,
                #[pyo3(get)]
                value: $inner,
            }
        }
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        notification_struct_def_outer!($name);
        paste! {
            #[serde_as]
            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
            #[pyclass(module = "solders.rpc.responses", subclass)]
            pub struct [<$name Result>] {
                #[pyo3(get)]
                context: RpcResponseContext,
                #[pyo3(get)]
                #[serde_as(as = $serde_as)]
                value: $inner,
            }
        }
    };
}

#[macro_export]
macro_rules! notification_struct_def_contextless {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
        #[pyclass(module = "solders.rpc.responses", subclass)]
        pub struct $name {
            #[pyo3(get)]
            result: $inner,
            #[pyo3(get)]
            subscription: u64,
        }
    };
}

#[macro_export]
macro_rules! notification_struct_def_no_eq {
    ($name:ident, $inner:ty) => {
        notification_struct_def_outer_no_eq!($name);
        paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
            #[pyclass(module = "solders.rpc.responses", subclass)]
            pub struct [<$name Result>] {
                #[pyo3(get)]
                context: RpcResponseContext,
                #[pyo3(get)]
                value: $inner,
            }
        }
    };
}

#[macro_export]
macro_rules! notification_boilerplate {
    ($name:ident, $inner:ty) => {
        paste! {
            response_data_boilerplate!([<$name Result>]);
            #[common_methods]
            #[pymethods]
            impl [<$name Result>] {
                #[new]
                pub fn new(value: $inner, context: RpcResponseContext) -> Self {
                    Self { value, context }
                }
            }
            response_data_boilerplate!($name);
            #[common_methods]
            #[pymethods]
            impl $name {
                #[new]
                pub fn new(result: [<$name Result>], subscription: u64) -> Self {
                    Self { result, subscription }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! notification_boilerplate_contextless {
    ($name:ident, $inner:ty) => {
        response_data_boilerplate!($name);
        #[common_methods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(result: $inner, subscription: u64) -> Self {
                Self {
                    result,
                    subscription,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! notification {
    ($name:ident, $inner:ty) => {
        notification_struct_def!($name, $inner);
        notification_boilerplate!($name, $inner);
    };
    ($name:ident, $inner:ty, $serde_as:expr) => {
        notification_struct_def!($name, $inner, $serde_as);
        notification_boilerplate!($name, $inner);
    };
}

#[macro_export]
macro_rules! notification_no_eq {
    ($name:ident, $inner:ty) => {
        notification_struct_def_no_eq!($name, $inner);
        notification_boilerplate!($name, $inner);
    };
}

#[macro_export]
macro_rules! notification_contextless {
    ($name:ident, $inner:ty) => {
        notification_struct_def_contextless!($name, $inner);
        notification_boilerplate_contextless!($name, $inner);
    };
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcIdentity(RpcIdentityOriginal);

response_data_boilerplate!(RpcIdentity);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcIdentity {
    #[new]
    pub fn new(identity: Pubkey) -> Self {
        RpcIdentityOriginal {
            identity: identity.to_string(),
        }
        .into()
    }

    /// Pubkey: The current node identity.
    #[getter]
    pub fn identity(&self) -> Pubkey {
        self.0.identity.parse().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcBlockhash(RpcBlockhashOriginal);
//     #[serde_as(as = "DisplayFromStr")]
//     #[pyo3(get)]
//     pub blockhash: SolderHash,
//     #[pyo3(get)]
//     pub last_valid_block_height: u64,
// }

response_data_boilerplate!(RpcBlockhash);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockhash {
    #[new]
    pub fn new(blockhash: SolderHash, last_valid_block_height: u64) -> Self {
        RpcBlockhashOriginal {
            blockhash: blockhash.to_string(),
            last_valid_block_height,
        }
        .into()
    }

    #[getter]
    pub fn blockhash(&self) -> SolderHash {
        self.0.blockhash.parse().unwrap()
    }

    #[getter]
    pub fn last_valid_block_height(&self) -> u64 {
        self.0.last_valid_block_height
    }
}
pub type RpcLeaderSchedule = Option<HashMap<Pubkey, Vec<usize>>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromPyObject, EnumIntoPy)]
#[serde(untagged)]
pub enum AccountMaybeJSON {
    Binary(Account),
    Parsed(AccountJSON),
}

impl From<Account> for AccountMaybeJSON {
    fn from(a: Account) -> Self {
        Self::Binary(a)
    }
}

impl From<AccountJSON> for AccountMaybeJSON {
    fn from(a: AccountJSON) -> Self {
        Self::Parsed(a)
    }
}

impl TryFrom<AccountMaybeJSON> for Account {
    type Error = String;
    fn try_from(acc: AccountMaybeJSON) -> Result<Self, Self::Error> {
        if let AccountMaybeJSON::Binary(account) = acc {
            Ok(account)
        } else {
            Err("Expected Account, found AccountJSON".to_string())
        }
    }
}

impl TryFrom<AccountMaybeJSON> for AccountJSON {
    type Error = String;
    fn try_from(acc: AccountMaybeJSON) -> Result<Self, Self::Error> {
        if let AccountMaybeJSON::Parsed(account) = acc {
            Ok(account)
        } else {
            Err("Expected AccountJSON, found Account".to_string())
        }
    }
}

impl From<UiAccount> for AccountMaybeJSON {
    fn from(u: UiAccount) -> Self {
        match u.data {
            UiAccountData::LegacyBinary(_) => panic!("LegacyBinary data should not appear"),
            UiAccountData::Json(_) => AccountJSON::try_from(u).unwrap().into(),
            UiAccountData::Binary(..) => Account::try_from(u).unwrap().into(),
        }
    }
}

impl From<AccountMaybeJSON> for UiAccount {
    fn from(a: AccountMaybeJSON) -> Self {
        match a {
            AccountMaybeJSON::Binary(acc) => Self::from(acc),
            AccountMaybeJSON::Parsed(acc) => Self::from(acc),
        }
    }
}

// TODO: make the one in solana-rpc-client-api work here.
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccount {
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub pubkey: Pubkey,
    #[serde_as(as = "TryFromInto<UiAccount>")]
    #[pyo3(get)]
    pub account: Account,
}

response_data_boilerplate!(RpcKeyedAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcKeyedAccount {
    #[new]
    pub fn new(pubkey: Pubkey, account: Account) -> Self {
        Self { pubkey, account }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccountJsonParsed {
    #[serde_as(as = "DisplayFromStr")]
    #[pyo3(get)]
    pub pubkey: Pubkey,
    #[serde_as(as = "TryFromInto<UiAccount>")]
    #[pyo3(get)]
    pub account: AccountJSON,
}

response_data_boilerplate!(RpcKeyedAccountJsonParsed);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcKeyedAccountJsonParsed {
    #[new]
    pub fn new(pubkey: Pubkey, account: AccountJSON) -> Self {
        Self { pubkey, account }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromPyObject, EnumIntoPy)]
#[serde(untagged)]
pub enum RpcKeyedAccountMaybeJSON {
    Binary(RpcKeyedAccount),
    Parsed(RpcKeyedAccountJsonParsed),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcTokenAccountBalance(RpcTokenAccountBalanceOriginal);

response_data_boilerplate!(RpcTokenAccountBalance);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTokenAccountBalance {
    #[new]
    pub fn new(address: Pubkey, amount: UiTokenAmount) -> Self {
        RpcTokenAccountBalanceOriginal {
            address: address.to_string(),
            amount: amount.into(),
        }
        .into()
    }

    #[getter]
    pub fn address(&self) -> Pubkey {
        Pubkey::from_str(&self.0.address).unwrap()
    }

    #[getter]
    pub fn amount(&self) -> UiTokenAmount {
        self.0.amount.clone().into()
    }
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVersionInfo(RpcVersionInfoOriginal);

response_data_boilerplate!(RpcVersionInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVersionInfo {
    #[new]
    pub fn new(solana_core: String, feature_set: Option<u32>) -> Self {
        RpcVersionInfoOriginal {
            solana_core,
            feature_set,
        }
        .into()
    }

    #[getter]
    pub fn solana_core(&self) -> String {
        self.0.solana_core.clone()
    }

    #[getter]
    pub fn feature_set(&self) -> Option<u32> {
        self.0.feature_set
    }
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVoteAccountInfo(RpcVoteAccountInfoOriginal);

response_data_boilerplate!(RpcVoteAccountInfo);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVoteAccountInfo {
    #[new]
    pub fn new(
        vote_pubkey: Pubkey,
        node_pubkey: Pubkey,
        activated_stake: u64,
        commission: u8,
        epoch_vote_account: bool,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_vote: u64,
        root_slot: Slot,
    ) -> Self {
        RpcVoteAccountInfoOriginal {
            vote_pubkey: vote_pubkey.to_string(),
            node_pubkey: node_pubkey.to_string(),
            activated_stake,
            commission,
            epoch_vote_account,
            epoch_credits,
            last_vote,
            root_slot,
        }
        .into()
    }
    #[getter]
    pub fn vote_pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.vote_pubkey).unwrap()
    }
    #[getter]
    pub fn node_pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.0.node_pubkey).unwrap()
    }
    #[getter]
    pub fn activated_stake(&self) -> u64 {
        self.0.activated_stake
    }
    #[getter]
    pub fn commission(&self) -> u8 {
        self.0.commission
    }
    #[getter]
    pub fn epoch_vote_account(&self) -> bool {
        self.0.epoch_vote_account
    }
    #[getter]
    pub fn epoch_credits(&self) -> Vec<(Epoch, u64, u64)> {
        self.0.epoch_credits.clone()
    }
    #[getter]
    pub fn last_vote(&self) -> u64 {
        self.0.last_vote
    }
    #[getter]
    pub fn root_slot(&self) -> Slot {
        self.0.root_slot
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, From, Into)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcVoteAccountStatus(RpcVoteAccountStatusOriginal);

response_data_boilerplate!(RpcVoteAccountStatus);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcVoteAccountStatus {
    #[new]
    pub fn new(current: Vec<RpcVoteAccountInfo>, delinquent: Vec<RpcVoteAccountInfo>) -> Self {
        RpcVoteAccountStatusOriginal {
            current: current.into_iter().map(|x| x.into()).collect(),
            delinquent: delinquent.into_iter().map(|x| x.into()).collect(),
        }
        .into()
    }
    #[getter]
    pub fn current(&self) -> Vec<RpcVoteAccountInfo> {
        self.0
            .current
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }

    #[getter]
    pub fn delinquent(&self) -> Vec<RpcVoteAccountInfo> {
        self.0
            .delinquent
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcSignatureResponse {
    #[pyo3(get)]
    err: Option<TransactionErrorType>,
}

response_data_boilerplate!(RpcSignatureResponse);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSignatureResponse {
    #[new]
    pub fn new(err: Option<TransactionErrorType>) -> Self {
        Self { err }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Eq, PartialEq)]
#[pyclass(module = "solders.rpc.responses")]
pub enum BlockStoreError {
    BlockStoreError,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct SubscriptionResult {
    #[serde(skip_deserializing)]
    jsonrpc: solders_rpc_version::V2,
    #[pyo3(get)]
    id: u64,
    #[pyo3(get)]
    result: u64,
}

response_data_boilerplate!(SubscriptionResult);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl SubscriptionResult {
    #[new]
    pub fn new(id: u64, result: u64) -> Self {
        Self {
            id,
            result,
            jsonrpc: solders_rpc_version::V2::default(),
        }
    }
}

notification!(AccountNotification, Account, "TryFromInto<UiAccount>");
notification!(
    AccountNotificationJsonParsed,
    AccountJSON,
    "TryFromInto<UiAccount>"
);
notification!(ProgramNotification, RpcKeyedAccount);
notification!(ProgramNotificationJsonParsed, RpcKeyedAccountJsonParsed);
notification!(SignatureNotification, RpcSignatureResponse);
notification_contextless!(RootNotification, u64);

#[derive(FromPyObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumIntoPy)]
#[serde(untagged)]
pub enum AccountNotificationType {
    JsonParsed(AccountNotificationJsonParsed),
    Binary(AccountNotification),
}

#[derive(FromPyObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumIntoPy)]
#[serde(untagged)]
pub enum ProgramNotificationType {
    Binary(ProgramNotification),
    JsonParsed(ProgramNotificationJsonParsed),
}
