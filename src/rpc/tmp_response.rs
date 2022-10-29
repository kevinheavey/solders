use {
    crate::{
        tmp_account_decoder::UiAccount,
        tmp_transaction_status::{
            TransactionConfirmationStatus, UiConfirmedBlock, UiTransactionReturnData,
        },
    },
    serde::{Deserialize, Serialize},
    solana_sdk::{
        clock::{Epoch, Slot, UnixTimestamp},
        fee_calculator::{FeeCalculator, FeeRateGovernor},
        hash::Hash,
        inflation::Inflation,
        transaction::{Result, TransactionError},
    },
    std::{collections::HashMap, fmt, net::SocketAddr},
    thiserror::Error,
};

pub const JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP: i64 = -32001;
pub const JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE: i64 = -32002;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_VERIFICATION_FAILURE: i64 = -32003;
pub const JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE: i64 = -32004;
pub const JSON_RPC_SERVER_ERROR_NODE_UNHEALTHY: i64 = -32005;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE: i64 = -32006;
pub const JSON_RPC_SERVER_ERROR_SLOT_SKIPPED: i64 = -32007;
pub const JSON_RPC_SERVER_ERROR_NO_SNAPSHOT: i64 = -32008;
pub const JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED: i64 = -32009;
pub const JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX: i64 = -32010;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_HISTORY_NOT_AVAILABLE: i64 = -32011;
pub const JSON_RPC_SCAN_ERROR: i64 = -32012;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_LEN_MISMATCH: i64 = -32013;
pub const JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET: i64 = -32014;
pub const JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION: i64 = -32015;
pub const JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED: i64 = -32016;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockCommitment<T> {
    pub commitment: Option<T>,
    pub total_stake: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockhashFeeCalculator {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockhash {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFees {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
    pub last_valid_slot: Slot,
    pub last_valid_block_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeprecatedRpcFees {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
    pub last_valid_slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub blockhash: Hash,
    pub fee_calculator: FeeCalculator,
    pub last_valid_block_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFeeCalculator {
    pub fee_calculator: FeeCalculator,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFeeRateGovernor {
    pub fee_rate_governor: FeeRateGovernor,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationGovernor {
    pub initial: f64,
    pub terminal: f64,
    pub taper: f64,
    pub foundation: f64,
    pub foundation_term: f64,
}

impl From<Inflation> for RpcInflationGovernor {
    fn from(inflation: Inflation) -> Self {
        Self {
            initial: inflation.initial,
            terminal: inflation.terminal,
            taper: inflation.taper,
            foundation: inflation.foundation,
            foundation_term: inflation.foundation_term,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationRate {
    pub total: f64,
    pub validator: f64,
    pub foundation: f64,
    pub epoch: Epoch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotInfo {
    pub slot: Slot,
    pub parent: Slot,
    pub root: Slot,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransactionStats {
    pub num_transaction_entries: u64,
    pub num_successful_transactions: u64,
    pub num_failed_transactions: u64,
    pub max_transactions_per_entry: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SlotUpdate {
    FirstShredReceived {
        slot: Slot,
        timestamp: u64,
    },
    Completed {
        slot: Slot,
        timestamp: u64,
    },
    CreatedBank {
        slot: Slot,
        parent: Slot,
        timestamp: u64,
    },
    Frozen {
        slot: Slot,
        timestamp: u64,
        stats: SlotTransactionStats,
    },
    Dead {
        slot: Slot,
        timestamp: u64,
        err: String,
    },
    OptimisticConfirmation {
        slot: Slot,
        timestamp: u64,
    },
    Root {
        slot: Slot,
        timestamp: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RpcSignatureResult {
    ProcessedSignature(ProcessedSignatureResult),
    ReceivedSignature(ReceivedSignatureResult),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcLogsResponse {
    pub signature: String, // Signature as base58 string
    pub err: Option<TransactionError>,
    pub logs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedSignatureResult {
    pub err: Option<TransactionError>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ReceivedSignatureResult {
    ReceivedSignature,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcContactInfo {
    /// Pubkey of the node as a base-58 string
    pub pubkey: String,
    /// Gossip port
    pub gossip: Option<SocketAddr>,
    /// Tpu port
    pub tpu: Option<SocketAddr>,
    /// JSON RPC port
    pub rpc: Option<SocketAddr>,
    /// Software version
    pub version: Option<String>,
    /// First 4 bytes of the FeatureSet identifier
    pub feature_set: Option<u32>,
    /// Shred version
    pub shred_version: Option<u16>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockProductionRange {
    pub first_slot: Slot,
    pub last_slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockProduction {
    /// Map of leader base58 identity pubkeys to a tuple of `(number of leader slots, number of blocks produced)`
    pub by_identity: HashMap<String, (usize, usize)>,
    pub range: RpcBlockProductionRange,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RpcVersionInfo {
    /// The current version of solana-core
    pub solana_core: String,
    /// first 4 bytes of the FeatureSet identifier
    pub feature_set: Option<u32>,
}

impl fmt::Debug for RpcVersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.solana_core)
    }
}

impl fmt::Display for RpcVersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(version) = self.solana_core.split_whitespace().next() {
            // Display just the semver if possible
            write!(f, "{version}")
        } else {
            write!(f, "{}", self.solana_core)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RpcIdentity {
    /// The current node identity pubkey
    pub identity: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVote {
    /// Vote account address, as base-58 encoded string
    pub vote_pubkey: String,
    pub slots: Vec<Slot>,
    pub hash: String,
    pub timestamp: Option<UnixTimestamp>,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountStatus {
    pub current: Vec<RpcVoteAccountInfo>,
    pub delinquent: Vec<RpcVoteAccountInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountInfo {
    /// Vote account address, as base-58 encoded string
    pub vote_pubkey: String,

    /// The validator identity, as base-58 encoded string
    pub node_pubkey: String,

    /// The current stake, in lamports, delegated to this vote account
    pub activated_stake: u64,

    /// An 8-bit integer used as a fraction (commission/MAX_U8) for rewards payout
    pub commission: u8,

    /// Whether this account is staked for the current epoch
    pub epoch_vote_account: bool,

    /// History of how many credits earned by the end of each epoch
    ///   each tuple is (Epoch, credits, prev_credits)
    pub epoch_credits: Vec<(Epoch, u64, u64)>,

    /// Most recent slot voted on by this vote account (0 if no votes exist)
    pub last_vote: u64,

    /// Current root slot for this vote account (0 if not root slot exists)
    pub root_slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSignatureConfirmation {
    pub confirmations: usize,
    pub status: Result<()>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateTransactionResult {
    pub err: Option<TransactionError>,
    pub logs: Option<Vec<String>>,
    pub accounts: Option<Vec<Option<UiAccount>>>,
    pub units_consumed: Option<u64>,
    pub return_data: Option<UiTransactionReturnData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcStorageTurn {
    pub blockhash: String,
    pub slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcAccountBalance {
    pub address: String,
    pub lamports: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSupply {
    pub total: u64,
    pub circulating: u64,
    pub non_circulating: u64,
    pub non_circulating_accounts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StakeActivationState {
    Activating,
    Active,
    Deactivating,
    Inactive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcStakeActivation {
    pub state: StakeActivationState,
    pub active: u64,
    pub inactive: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConfirmedTransactionStatusWithSignature {
    pub signature: String,
    pub slot: Slot,
    pub err: Option<TransactionError>,
    pub memo: Option<String>,
    pub block_time: Option<UnixTimestamp>,
    pub confirmation_status: Option<TransactionConfirmationStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcPerfSample {
    pub slot: Slot,
    pub num_transactions: u64,
    pub num_slots: u64,
    pub sample_period_secs: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationReward {
    pub epoch: Epoch,
    pub effective_slot: Slot,
    pub amount: u64,            // lamports
    pub post_balance: u64,      // lamports
    pub commission: Option<u8>, // Vote account commission when the reward was credited
}

#[derive(Clone, Deserialize, Serialize, Debug, Error, Eq, PartialEq)]
pub enum RpcBlockUpdateError {
    #[error("block store error")]
    BlockStoreError,

    #[error("unsupported transaction version ({0})")]
    UnsupportedTransactionVersion(u8),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockUpdate {
    pub slot: Slot,
    pub block: Option<UiConfirmedBlock>,
    pub err: Option<RpcBlockUpdateError>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpcSnapshotSlotInfo {
    pub full: Slot,
    pub incremental: Option<Slot>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpcPrioritizationFee {
    pub slot: Slot,
    pub prioritization_fee: u64,
}
