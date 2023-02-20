use derive_more::{From, Into};
use pyo3::prelude::*;
use solders_primitives::{pubkey::Pubkey, hash::Hash as SolderHash, keypair::Keypair, transaction::VersionedTransaction};
use solders_traits::{BanksClientError, to_py_err};
use {
    solana_program_test::{ProgramTest, BanksClient as BanksClientOriginal},
    solana_sdk::{
        bpf_loader_upgradeable::{self, UpgradeableLoaderState},
        signature::Signer,
        transaction::Transaction,
    },
};
use crate::account::Account;



#[pyclass(module = "solders.program_test", subclass)]
#[derive(From, Into)]
pub struct BanksClient(BanksClientOriginal);

#[pymethods]
impl BanksClient {
    pub fn send_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0;
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying.send_transaction(tx_inner).await.map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    pub fn process_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: VersionedTransaction,
    ) -> PyResult<&'p PyAny> {
        let tx_inner = transaction.0.into_legacy_transaction().unwrap();
        let mut underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = underlying.send_transaction(tx_inner).await.map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }
}

fn new_program_test(programs: Option<Vec<(&str, Pubkey)>>, compute_max_units: Option<u64>, transaction_account_lock_limit: Option<usize>, use_bpf_jit: Option<bool>, accounts: Option<Vec<(Pubkey, Account)>>) -> ProgramTest {
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
pub fn start<'p>(py: Python<'p>, programs: Option<Vec<(&str, Pubkey)>>, compute_max_units: Option<u64>, transaction_account_lock_limit: Option<usize>, use_bpf_jit: Option<bool>, accounts: Option<Vec<(Pubkey, Account)>>) -> PyResult<&'p PyAny> {
    let pt = new_program_test(programs, compute_max_units, transaction_account_lock_limit, use_bpf_jit, accounts);
    pyo3_asyncio::tokio::future_into_py(py, async move {
            let inner = pt.start().await;
            let tup = (BanksClient::from(inner.0), Keypair::from(inner.1), SolderHash::from(inner.2));
            let res: PyResult<PyObject> = Python::with_gil(|py| Ok(tup.into_py(py)));
            res
        })
}

async fn hi() {
    // Arrange
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::default().start().await;

    let buffer_keypair = Keypair::new().0;
    let upgrade_authority_keypair = Keypair::new().0;

    let rent = banks_client.get_rent().await.unwrap();
    let buffer_rent = rent.minimum_balance(UpgradeableLoaderState::size_of_programdata(1));

    let create_buffer_instructions = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        &upgrade_authority_keypair.pubkey(),
        buffer_rent,
        1,
    )
    .unwrap();

    let mut transaction =
        Transaction::new_with_payer(&create_buffer_instructions[..], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &buffer_keypair], recent_blockhash);

    // Act
    banks_client.process_transaction(transaction).await.unwrap();

    // Assert
    let buffer_account = banks_client
        .get_account(buffer_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_ne!(buffer_account.owner, bpf_loader_upgradeable::id());
}

#[pyfunction]
fn call_hi(py: Python<'_>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        hi().await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

pub(crate) fn create_program_test_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "program_test")?;
    m.add("BanksClientError", py.get_type::<BanksClientError>())?;
    m.add_function(wrap_pyfunction!(start, m)?)?;
    Ok(m)
}
