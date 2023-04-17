use crate::{
    account::Account,
    clock::Clock,
    commitment_config::CommitmentLevel,
    rent::Rent,
    transaction_status::{
        transaction_status_boilerplate, TransactionConfirmationStatus, TransactionErrorType,
        TransactionReturnData, TransactionStatus,
    },
};
use derive_more::{From, Into};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_banks_client::{
    BanksClientError as BanksClientErrorOriginal, TransactionStatus as TransactionStatusBanks,
};
use solana_banks_interface::{
    BanksTransactionResultWithMetadata, BanksTransactionResultWithSimulation,
    TransactionConfirmationStatus as TransactionConfirmationStatusBanks, TransactionMetadata,
};
use solders_macros::{common_methods, richcmp_eq_only};
use solders_primitives::{
    hash::Hash as SolderHash, keypair::Keypair, message::Message, pubkey::Pubkey,
    signature::Signature, transaction::VersionedTransaction,
};
use solders_traits::{to_py_err, to_py_value_err, BanksClientError};
use tarpc::context::current;
use {
    solana_program_test::{
        BanksClient as BanksClientOriginal, ProgramTest,
        ProgramTestContext as ProgramTestContextOriginal,
    },
    solana_sdk::{
        account::AccountSharedData, clock::Clock as ClockOriginal,
        commitment_config::CommitmentLevel as CommitmentLevelOriginal, slot_history::Slot,
        transaction::Transaction,
    },
};

macro_rules! async_res {
    ($fut:expr) => {
        $fut.await.map_err(to_py_err)
    };
}

macro_rules! res_to_py_obj {
    ($fut:expr) => {{
        let res = async_res!($fut);
        let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
        pyobj
    }};
}

impl From<TransactionConfirmationStatusBanks> for TransactionConfirmationStatus {
    fn from(s: TransactionConfirmationStatusBanks) -> Self {
        match s {
            TransactionConfirmationStatusBanks::Processed => Self::Processed,
            TransactionConfirmationStatusBanks::Confirmed => Self::Confirmed,
            TransactionConfirmationStatusBanks::Finalized => Self::Finalized,
        }
    }
}

impl From<TransactionStatusBanks> for TransactionStatus {
    fn from(t: TransactionStatusBanks) -> Self {
        Self::new(
            t.slot,
            t.confirmations,
            None,
            t.err.map(Into::into),
            t.confirmation_status.map(Into::into),
        )
    }
}

#[pyclass(module = "solders.program_test", subclass)]
#[derive(From, Into, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BanksTransactionMeta(pub TransactionMetadata);

transaction_status_boilerplate!(BanksTransactionMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BanksTransactionMeta {
    #[new]
    pub fn new(
        log_messages: Vec<String>,
        compute_units_consumed: u64,
        return_data: Option<TransactionReturnData>,
    ) -> Self {
        TransactionMetadata {
            log_messages,
            compute_units_consumed,
            return_data: return_data.map(Into::into),
        }
        .into()
    }

    #[getter]
    pub fn log_messages(&self) -> Vec<String> {
        self.0.log_messages.clone()
    }

    #[getter]
    pub fn return_data(&self) -> Option<TransactionReturnData> {
        self.0.return_data.clone().map(Into::into)
    }

    #[getter]
    pub fn compute_units_consumed(&self) -> u64 {
        self.0.compute_units_consumed
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.program_test", subclass)]
pub struct BanksTransactionResultWithMeta(BanksTransactionResultWithMetadata);

transaction_status_boilerplate!(BanksTransactionResultWithMeta);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl BanksTransactionResultWithMeta {
    #[new]
    pub fn new(result: Option<TransactionErrorType>, meta: Option<BanksTransactionMeta>) -> Self {
        BanksTransactionResultWithMetadata {
            result: match result {
                None => Ok(()),
                Some(e) => Err(e.into()),
            },
            metadata: meta.map(Into::into),
        }
        .into()
    }

    #[getter]
    pub fn result(&self) -> Option<TransactionErrorType> {
        match self.0.result.clone() {
            Ok(()) => None,
            Err(x) => Some(TransactionErrorType::from(x)),
        }
    }

    #[getter]
    pub fn meta(&self) -> Option<BanksTransactionMeta> {
        self.0.metadata.clone().map(Into::into)
    }
}

impl From<BanksTransactionResultWithSimulation> for BanksTransactionResultWithMeta {
    fn from(r: BanksTransactionResultWithSimulation) -> Self {
        BanksTransactionResultWithMetadata {
            result: match r.result {
                None => Ok(()),
                Some(x) => x,
            },
            metadata: r.simulation_details.map(|d| TransactionMetadata {
                log_messages: d.logs,
                compute_units_consumed: d.units_consumed,
                return_data: d.return_data,
            }),
        }
        .into()
    }
}

#[pyclass(module = "solders.program_test", subclass)]
#[derive(From, Into)]
pub struct BanksClient(BanksClientOriginal);

#[pymethods]
impl BanksClient {
    /// Send a transaction and return immediately.
    pub fn send_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0;
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            // let res = underlying.send_transaction(tx_inner).await.map_err(to_py_err);
            // let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            // pyobj
            res_to_py_obj!(underlying.send_transaction(tx_inner))
        })
    }

    /// Send a transaction and return until the transaction has been finalized or rejected.
    pub fn process_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0.into_legacy_transaction().unwrap();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying
                .process_transaction_with_commitment(tx_inner, commitment_inner)
                .await
                .map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Send a transaction and return any preflight (sanitization or simulation) errors, or return
    /// after the transaction has been rejected or reached the given level of commitment.
    pub fn process_transaction_with_preflight<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0.into_legacy_transaction().unwrap();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying
                .process_transaction_with_preflight_and_commitment(tx_inner, commitment_inner)
                .await
                .map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    pub fn process_transaction_with_metadata<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0.into_legacy_transaction().unwrap();
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying
                .process_transaction_with_metadata(tx_inner)
                .await
                .map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| {
                res.map(|x| BanksTransactionResultWithMeta::from(x).into_py(py))
            });
            pyobj
        })
    }

    /// Simulate a transaction at the given commitment level
    pub fn simulate_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0.into_legacy_transaction().unwrap();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying
                .simulate_transaction_with_commitment(tx_inner, commitment_inner)
                .await
                .map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| {
                res.map(|x| BanksTransactionResultWithMeta::from(x).into_py(py))
            });
            pyobj
        })
    }

    /// Return the account at the given address at the slot corresponding to the given
    /// commitment level. If the account is not found, None is returned.
    pub fn get_account<'p>(
        &mut self,
        py: Python<'p>,
        address: Pubkey,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let address_inner = address.0;
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res =
                async_res!(underlying.get_account_with_commitment(address_inner, commitment_inner));
            let pyobj: PyResult<Option<PyObject>> =
                Python::with_gil(|py| res.map(|x| x.map(|acc| Account::from(acc).into_py(py))));
            pyobj
        })
    }

    /// Return the status of a transaction with a signature matching the transaction's first
    /// signature. Return None if the transaction is not found, which may be because the
    /// blockhash was expired or the fee-paying account had insufficient funds to pay the
    /// transaction fee. Note that servers rarely store the full transaction history. This
    /// method may return None if the transaction status has been discarded.
    pub fn get_transaction_status<'p>(
        &mut self,
        py: Python<'p>,
        signature: Signature,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let signature_underlying = signature.0;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_transaction_status(signature_underlying));
            let pyobj: PyResult<Option<PyObject>> = Python::with_gil(|py| {
                res.map(|x| x.map(|s| TransactionStatus::from(s).into_py(py)))
            });
            pyobj
        })
    }

    /// Same as get_transaction_status, but for multiple transactions.
    pub fn get_transaction_statuses<'p>(
        &mut self,
        py: Python<'p>,
        signatures: Vec<Signature>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let signatures_underlying = signatures.iter().map(|x| x.0).collect();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_transaction_statuses(signatures_underlying));
            // let res_converted = res.map(|v| v.iter().map(|o| o.map(TransactionStatus::from)).collect::<Vec<Option<TransactionStatus>>>());
            let pyobj: PyResult<Vec<Option<PyObject>>> = Python::with_gil(|py| {
                res.map(|v| {
                    v.iter()
                        .map(|o| o.clone().map(|t| TransactionStatus::from(t).into_py(py)))
                        .collect()
                })
            });
            pyobj
        })
    }

    pub fn get_slot<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_slot_with_context(current(), commitment_inner));
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    pub fn get_block_height<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res =
                async_res!(underlying.get_block_height_with_context(current(), commitment_inner));
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    pub fn get_rent<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_rent());
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| res.map(|x| Rent::from(x).into_py(py)));
            pyobj
        })
    }

    pub fn get_clock<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_sysvar::<ClockOriginal>());
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| res.map(|x| Clock::from(x).into_py(py)));
            pyobj
        })
    }

    pub fn get_balance<'p>(
        &mut self,
        py: Python<'p>,
        address: Pubkey,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let address_inner = address.0;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res =
                async_res!(underlying.get_balance_with_commitment(address_inner, commitment_inner));
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Returns latest blockhash and last valid block height for given commitment level.
    ///
    /// Returns:
    ///     tuple[Hash, int]: The blockhash and last valid block height.
    pub fn get_latest_blockhash<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_latest_blockhash_with_commitment(commitment_inner));
            let flattened = match res {
                Ok(v) => match v {
                    Some(x) => Ok(x),
                    None => Err(to_py_err(BanksClientErrorOriginal::ClientError(
                        "valid blockhash not found",
                    ))),
                },
                Err(e) => Err(e),
            };
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| flattened.map(|x| (SolderHash::from(x.0), x.1).into_py(py)));
            pyobj
        })
    }

    pub fn get_fee_for_message<'p>(
        &mut self,
        py: Python<'p>,
        message: Message,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let mut underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let message_inner = message.0;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_fee_for_message_with_commitment_and_context(
                current(),
                commitment_inner,
                message_inner
            ));
            let pyobj: PyResult<Option<PyObject>> =
                Python::with_gil(|py| res.map(|x| x.map(|num| num.into_py(py))));
            pyobj
        })
    }
}

fn new_program_test(
    programs: Option<Vec<(&str, Pubkey)>>,
    compute_max_units: Option<u64>,
    transaction_account_lock_limit: Option<usize>,
    use_bpf_jit: Option<bool>,
    accounts: Option<Vec<(Pubkey, Account)>>,
) -> ProgramTest {
    let mut pt = ProgramTest::default();
    pt.prefer_bpf(true);
    if let Some(progs) = programs {
        for prog in progs {
            pt.add_program(prog.0, prog.1.into(), None);
        }
    }
    if let Some(cmu) = compute_max_units {
        pt.set_compute_max_units(cmu);
    }
    if let Some(lock_lim) = transaction_account_lock_limit {
        pt.set_transaction_account_lock_limit(lock_lim);
    }
    if let Some(use_jit) = use_bpf_jit {
        pt.use_bpf_jit(use_jit);
    }
    if let Some(accs) = accounts {
        for acc in accs {
            pt.add_account(acc.0.into(), acc.1.into());
        }
    }
    pt
}

#[pyfunction]
pub fn start<'p>(
    py: Python<'p>,
    programs: Option<Vec<(&str, Pubkey)>>,
    compute_max_units: Option<u64>,
    transaction_account_lock_limit: Option<usize>,
    use_bpf_jit: Option<bool>,
    accounts: Option<Vec<(Pubkey, Account)>>,
) -> PyResult<&'p PyAny> {
    let pt = new_program_test(
        programs,
        compute_max_units,
        transaction_account_lock_limit,
        use_bpf_jit,
        accounts,
    );
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let inner = pt.start_with_context().await;
        let res: PyResult<PyObject> =
            Python::with_gil(|py| Ok(ProgramTestContext(inner).into_py(py)));
        res
    })
}

#[pyclass(module = "solders.program_test", subclass)]
#[derive(From, Into)]
pub struct ProgramTestContext(pub ProgramTestContextOriginal);

#[pymethods]
impl ProgramTestContext {
    #[getter]
    pub fn banks_client(&self) -> BanksClient {
        self.0.banks_client.clone().into()
    }

    #[getter]
    pub fn last_blockhash(&self) -> SolderHash {
        self.0.last_blockhash.into()
    }

    #[getter]
    pub fn payer(&self) -> Keypair {
        Keypair::from_bytes(self.0.payer.to_bytes()).unwrap()
    }

    /// Manually increment vote credits for the current epoch in the specified vote account to simulate validator voting activity
    pub fn increment_vote_account_credits(
        &mut self,
        vote_account_address: &Pubkey,
        number_of_credits: u64,
    ) {
        self.0
            .increment_vote_account_credits(vote_account_address.as_ref(), number_of_credits);
    }

    /// Create or overwrite an account, subverting normal runtime checks.
    ///
    /// This method exists to make it easier to set up artificial situations
    /// that would be difficult to replicate by sending individual transactions.
    /// Beware that it can be used to create states that would not be reachable
    /// by sending transactions!
    pub fn set_account(&mut self, address: &Pubkey, account: Account) {
        self.0
            .set_account(address.as_ref(), &AccountSharedData::from(account.0));
    }

    /// Overwrite the clock sysvar.
    pub fn set_clock(&mut self, clock: &Clock) {
        self.0.set_sysvar(&clock.0)
    }

    /// Overwrite the rent sysvar.
    pub fn set_rent(&mut self, rent: &Rent) {
        self.0.set_sysvar(&rent.0)
    }

    /// Force the working bank ahead to a new slot
    pub fn warp_to_slot(&mut self, warp_slot: Slot) -> PyResult<()> {
        self.0
            .warp_to_slot(warp_slot)
            .map_err(|e| to_py_value_err(&e))
    }
}

pub(crate) fn create_program_test_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "program_test")?;
    m.add("BanksClientError", py.get_type::<BanksClientError>())?;
    m.add_class::<BanksClient>()?;
    m.add_class::<ProgramTestContext>()?;
    m.add_class::<BanksTransactionResultWithMeta>()?;
    m.add_class::<BanksTransactionMeta>()?;
    m.add_function(wrap_pyfunction!(start, m)?)?;
    Ok(m)
}
