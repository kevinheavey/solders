use pyo3::prelude::*;
use solders_rpc_account_info_config::RpcAccountInfoConfig;
use solders_rpc_config_no_filter::{
    RpcBlockConfig, RpcBlockProductionConfig, RpcBlockProductionConfigRange,
    RpcBlockSubscribeConfig, RpcContextConfig, RpcEpochConfig, RpcGetVoteAccountsConfig,
    RpcLargestAccountsConfig, RpcLargestAccountsFilter, RpcLeaderScheduleConfig,
    RpcSignatureSubscribeConfig, RpcSupplyConfig, RpcTransactionConfig, RpcTransactionLogsConfig,
};
use solders_rpc_config_no_rpc_api::{
    RpcBlockSubscribeFilter, RpcBlockSubscribeFilterMentions, RpcTransactionLogsFilter,
    RpcTransactionLogsFilterMentions,
};
use solders_rpc_config_no_rpc_api::{RpcTokenAccountsFilterMint, RpcTokenAccountsFilterProgramId};
use solders_rpc_program_accounts_config::RpcProgramAccountsConfig;
use solders_rpc_request_airdrop_config::RpcRequestAirdropConfig;
use solders_rpc_send_transaction_config::RpcSendTransactionConfig;
use solders_rpc_sig_status_config::RpcSignatureStatusConfig;
use solders_rpc_sigs_for_address_config::RpcSignaturesForAddressConfig;
use solders_rpc_sim_transaction_config::RpcSimulateTransactionConfig;
use solders_rpc_simulate_tx_accounts_config::RpcSimulateTransactionAccountsConfig;

pub fn include_config(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RpcSignatureStatusConfig>()?;
    m.add_class::<RpcSendTransactionConfig>()?;
    m.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    m.add_class::<RpcSimulateTransactionConfig>()?;
    m.add_class::<RpcRequestAirdropConfig>()?;
    m.add_class::<RpcLeaderScheduleConfig>()?;
    m.add_class::<RpcBlockProductionConfigRange>()?;
    m.add_class::<RpcBlockProductionConfig>()?;
    m.add_class::<RpcGetVoteAccountsConfig>()?;
    m.add_class::<RpcLargestAccountsConfig>()?;
    m.add_class::<RpcLargestAccountsFilter>()?;
    m.add_class::<RpcSupplyConfig>()?;
    m.add_class::<RpcEpochConfig>()?;
    m.add_class::<RpcAccountInfoConfig>()?;
    m.add_class::<RpcProgramAccountsConfig>()?;
    m.add_class::<RpcTransactionLogsFilter>()?;
    m.add_class::<RpcTransactionLogsFilterMentions>()?;
    m.add_class::<RpcTransactionLogsConfig>()?;
    m.add_class::<RpcTokenAccountsFilterMint>()?;
    m.add_class::<RpcTokenAccountsFilterProgramId>()?;
    m.add_class::<RpcSignatureSubscribeConfig>()?;
    m.add_class::<RpcBlockSubscribeFilter>()?;
    m.add_class::<RpcBlockSubscribeFilterMentions>()?;
    m.add_class::<RpcBlockSubscribeConfig>()?;
    m.add_class::<RpcSignaturesForAddressConfig>()?;
    m.add_class::<RpcBlockConfig>()?;
    m.add_class::<RpcTransactionConfig>()?;
    m.add_class::<RpcContextConfig>()?;
    Ok(())
}
