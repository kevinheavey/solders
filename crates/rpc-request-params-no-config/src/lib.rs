use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr, FromInto};
use solders_base64_string::Base64String;
use solders_commitment_config::{CommitmentConfig, CommitmentLevel};
use solders_message::VersionedMessage;
use solders_pubkey::Pubkey;
use solders_rpc_version::V2;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UnsubscribeParams(pub (u64,));

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RequestBase {
    pub jsonrpc: V2,
    pub id: u64,
}

impl RequestBase {
    pub fn new(id: Option<u64>) -> Self {
        Self {
            jsonrpc: V2::TwoPointOh,
            id: id.unwrap_or(0),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlocksParams(
    pub u64,
    #[serde(default)] pub Option<u64>,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub Option<CommitmentLevel>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetFeeForMessageParams(
    #[serde_as(as = "FromInto<Base64String>")] pub VersionedMessage,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    pub Option<CommitmentLevel>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMinimumBalanceForRentExemptionParams(
    pub usize,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    pub Option<CommitmentLevel>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PubkeyAndCommitmentParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    pub Option<CommitmentLevel>,
);
