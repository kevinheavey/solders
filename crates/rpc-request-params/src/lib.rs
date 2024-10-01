use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, skip_serializing_none, DisplayFromStr, FromInto};
use solders_base64_string::Base64String;
use solders_commitment_config::{CommitmentConfig, CommitmentLevel};
use solders_hash::Hash as SolderHash;
use solders_pubkey::Pubkey;
use solders_signature::Signature;

use solana_rpc_client_api::config::{
    RpcBlockSubscribeFilter, RpcTokenAccountsFilter, RpcTransactionLogsFilter,
};
use solders_rpc_account_info_config::RpcAccountInfoConfig;
use solders_rpc_config_no_filter::{
    RpcBlockConfig, RpcBlockSubscribeConfig, RpcBlockSubscribeFilterWrapper, RpcContextConfig,
    RpcEpochConfig, RpcLargestAccountsFilter, RpcLeaderScheduleConfig, RpcSignatureSubscribeConfig,
    RpcTokenAccountsFilterWrapper, RpcTransactionConfig, RpcTransactionLogsConfig,
    TransactionLogsFilterWrapper,
};
use solders_rpc_program_accounts_config::RpcProgramAccountsConfig;
use solders_rpc_request_airdrop_config::RpcRequestAirdropConfig;
use solders_rpc_send_transaction_config::RpcSendTransactionConfig;
use solders_rpc_sig_status_config::RpcSignatureStatusConfig;
use solders_rpc_sigs_for_address_config::RpcSignaturesForAddressConfig;
use solders_rpc_sim_transaction_config::RpcSimulateTransactionConfig;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAccountInfoParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde(default)] pub Option<RpcAccountInfoConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBalanceParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde(default)] pub Option<RpcContextConfig>,
);

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockParams(pub u64, #[serde(default)] pub Option<RpcBlockConfig>);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetInflationRewardParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] pub Vec<Pubkey>,
    #[serde(default)] pub Option<RpcEpochConfig>,
);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLargestAccountsParams(
    #[serde_as(as = "Option<FromInto<CommitmentConfig>>")]
    #[serde(default)]
    pub Option<CommitmentLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub Option<RpcLargestAccountsFilter>,
);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLeaderScheduleParams(
    #[serde(default)] pub Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub Option<RpcLeaderScheduleConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetMultipleAccountsParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] pub Vec<Pubkey>,
    #[serde(default)] pub Option<RpcAccountInfoConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetProgramAccountsParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde(default)] pub Option<RpcProgramAccountsConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignaturesForAddressParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde(default)] pub Option<RpcSignaturesForAddressConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetSignatureStatusesParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] pub Vec<Signature>,
    #[serde(default)] pub Option<RpcSignatureStatusConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetStakeActivationParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde(default)] pub Option<RpcEpochConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTokenAccountsByDelegateParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    #[serde_as(as = "FromInto<RpcTokenAccountsFilter>")] pub RpcTokenAccountsFilterWrapper,
    #[serde(default)] pub Option<RpcAccountInfoConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTransactionParams(
    #[serde_as(as = "DisplayFromStr")] pub Signature,
    #[serde(default)] pub Option<RpcTransactionConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IsBlockhashValidParams(
    #[serde_as(as = "DisplayFromStr")] pub SolderHash,
    #[serde(default)] pub Option<RpcContextConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct RequestAirdropParams(
    #[serde_as(as = "DisplayFromStr")] pub Pubkey,
    pub u64,
    #[serde(default)] pub Option<RpcRequestAirdropConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SendTransactionParams<T: From<Base64String> + Into<Base64String> + Clone>(
    #[serde_as(as = "FromInto<Base64String>")] pub T,
    #[serde(default)] pub Option<RpcSendTransactionConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SendRawTransactionParams(
    #[serde_as(as = "Base64")] pub Vec<u8>,
    #[serde(default)] pub Option<RpcSendTransactionConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SimulateTransactionParams<T: From<Base64String> + Into<Base64String> + Clone>(
    #[serde_as(as = "FromInto<Base64String>")] pub T,
    #[serde(default)] pub Option<RpcSimulateTransactionConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct BlockSubscribeParams(
    #[serde_as(as = "FromInto<RpcBlockSubscribeFilter>")] pub RpcBlockSubscribeFilterWrapper,
    #[serde(default)] pub Option<RpcBlockSubscribeConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct LogsSubscribeParams(
    #[serde_as(as = "FromInto<RpcTransactionLogsFilter>")] pub TransactionLogsFilterWrapper,
    #[serde(default)] pub Option<RpcTransactionLogsConfig>,
);

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct SignatureSubscribeParams(
    #[serde_as(as = "DisplayFromStr")] pub Signature,
    #[serde(default)] pub Option<RpcSignatureSubscribeConfig>,
);
