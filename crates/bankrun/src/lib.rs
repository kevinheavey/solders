use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono_humanize::{Accuracy, HumanTime, Tense};
use derive_more::{From, Into};
use log::{info, warn};
use pyo3::{
    exceptions::{PyFileNotFoundError, PyValueError},
    prelude::*,
};
use solana_banks_client::BanksClientError as BanksClientErrorOriginal;
use solana_program_test::find_file;
use solders_account::Account;
use solders_banks_interface::{
    transaction_status_from_banks, BanksTransactionMeta, BanksTransactionResultWithMeta,
};
use solders_commitment_config::CommitmentLevel;
use solders_hash::Hash as SolderHash;
use solders_keypair::Keypair;
use solders_message::Message;
use solders_primitives::{clock::Clock, rent::Rent};
use solders_pubkey::Pubkey;
use solders_signature::Signature;
use solders_traits::{to_py_err, BanksClientError};
use solders_traits_core::to_py_value_err;
use solders_transaction::{Transaction, VersionedTransaction};
use tarpc::context::current;
use toml::Table;
use {
    solana_program_test::{
        BanksClient as BanksClientOriginal, ProgramTest,
        ProgramTestContext as ProgramTestContextOriginal,
    },
    solana_sdk::{
        account::{Account as AccountOriginal, AccountSharedData},
        clock::Clock as ClockOriginal,
        commitment_config::CommitmentLevel as CommitmentLevelOriginal,
        slot_history::Slot,
    },
};

macro_rules! async_res {
    ($fut:expr) => {
        $fut.await.map_err(to_py_err)
    };
}

#[derive(FromPyObject, Clone, PartialEq, Eq, Debug)]
pub enum TransactionType {
    Legacy(Transaction),
    Versioned(VersionedTransaction),
}

/// A client for the ledger state, from the perspective of an arbitrary validator.
///
/// The client is used to send transactions and query account data, among other things.
/// Use ``bankrun.start()`` to initialize a BanksClient.
#[pyclass(module = "solders.bankrun", subclass)]
#[derive(From, Into)]
pub struct BanksClient(BanksClientOriginal);

#[pymethods]
impl BanksClient {
    /// Send a transaction and return immediately.
    ///
    /// Args:
    ///     transaction (Transaction | VersionedTransaction): The transaction to send.
    ///
    pub fn send_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: TransactionType,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = match transaction {
                TransactionType::Legacy(t) => underlying.send_transaction(t.0).await,
                TransactionType::Versioned(t) => underlying.send_transaction(t.0).await,
            };
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| res.map_err(to_py_err).map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Process a transaction and return the transaction metadata, raising any errors.
    ///
    /// Args:
    ///     transaction (Transaction | VersionedTransaction): The transaction to send.
    ///
    /// Returns:
    ///     BanksTransactionResultWithMeta: The transaction result and metadata.
    ///
    pub fn process_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: TransactionType,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let awaited = match transaction {
                TransactionType::Legacy(t) => {
                    underlying.process_transaction_with_metadata(t.0).await
                }
                TransactionType::Versioned(t) => {
                    underlying.process_transaction_with_metadata(t.0).await
                }
            };
            let res = awaited.map_err(to_py_err);
            let meta = match res {
                Ok(r) => match r.result {
                    Err(e) => Err(to_py_err(e)),
                    Ok(()) => Ok(BanksTransactionMeta::from(r.metadata.unwrap())),
                },
                Err(e) => Err(e),
            };
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| meta.map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Simulate a transaction at the given commitment level.
    ///
    /// Args:
    ///     transaction (Transaction | VersionedTransaction): The transaction to simulate.
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     BanksTransactionResultWithMeta: The transaction simulation result.
    ///
    pub fn simulate_transaction<'p>(
        &'p mut self,
        py: Python<'p>,
        transaction: TransactionType,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let awaited = match transaction {
                TransactionType::Legacy(t) => {
                    underlying
                        .simulate_transaction_with_commitment(t.0, commitment_inner)
                        .await
                }
                TransactionType::Versioned(t) => {
                    underlying
                        .simulate_transaction_with_commitment(t.0, commitment_inner)
                        .await
                }
            };
            let res = awaited.map_err(to_py_err);
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| {
                res.map(|x| BanksTransactionResultWithMeta::from(x).into_py(py))
            });
            pyobj
        })
    }

    /// Return the account at the given address at the slot corresponding to the given
    /// commitment level. If the account is not found, None is returned.
    ///
    /// Args:
    ///     address (Pubkey): The account address to look up.
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     Optional[Account]: The account object, if the account exists
    ///
    pub fn get_account<'p>(
        &mut self,
        py: Python<'p>,
        address: Pubkey,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let address_inner = address.0;
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res =
                async_res!(underlying.get_account_with_commitment(address_inner, commitment_inner));
            let pyobj: PyResult<Option<PyObject>> =
                Python::with_gil(|py| res.map(|x| x.map(|acc| Account::from(acc).into_py(py))));
            pyobj
        })
    }

    /// Return the status of a transaction with a signature matching the transaction's first
    /// signature.
    ///
    /// Return None if the transaction is not found, which may be because the
    /// blockhash was expired or the fee-paying account had insufficient funds to pay the
    /// transaction fee. Note that servers rarely store the full transaction history. This
    /// method may return None if the transaction status has been discarded.
    ///
    /// Args:
    ///     signature (Signature): The transaction signature (the first signature of the transaction).
    ///
    /// Returns:
    ///     Optional[TransactionStatus]: The transaction status, if found.
    ///
    pub fn get_transaction_status<'p>(
        &mut self,
        py: Python<'p>,
        signature: Signature,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        let signature_underlying = signature.0;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_transaction_status(signature_underlying));
            let pyobj: PyResult<Option<PyObject>> = Python::with_gil(|py| {
                res.map(|x| x.map(|s| transaction_status_from_banks(s).into_py(py)))
            });
            pyobj
        })
    }

    /// Same as ``get_transaction_status``, but for multiple transactions.
    ///
    /// Args:
    ///     signatures (Sequence[Signature]): The transaction signatures.
    ///
    /// Returns:
    ///     List[Optional[TransactionStatus]]: The transaction statuses, if found.
    ///
    pub fn get_transaction_statuses<'p>(
        &mut self,
        py: Python<'p>,
        signatures: Vec<Signature>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        let signatures_underlying = signatures.iter().map(|x| x.0).collect();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_transaction_statuses(signatures_underlying));
            let pyobj: PyResult<Vec<Option<PyObject>>> = Python::with_gil(|py| {
                res.map(|v| {
                    v.iter()
                        .map(|o| {
                            o.clone()
                                .map(|t| transaction_status_from_banks(t).into_py(py))
                        })
                        .collect()
                })
            });
            pyobj
        })
    }

    /// Get the slot that has reached the given commitment level (or the default commitment).
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     int: The current slot.
    ///
    pub fn get_slot<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_slot_with_context(current(), commitment_inner));
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Get the current block height.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     int: The current block height.
    ///
    pub fn get_block_height<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res =
                async_res!(underlying.get_block_height_with_context(current(), commitment_inner));
            let pyobj: PyResult<PyObject> = Python::with_gil(|py| res.map(|x| x.into_py(py)));
            pyobj
        })
    }

    /// Get the cluster rent.
    ///
    /// Returns:
    ///     Rent: The rent object.
    ///  
    pub fn get_rent<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_rent());
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| res.map(|x| Rent::from(x).into_py(py)));
            pyobj
        })
    }

    /// Get the cluster clock.
    ///
    /// Returns:
    ///     Clock: the clock object.
    ///
    pub fn get_clock<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_sysvar::<ClockOriginal>());
            let pyobj: PyResult<PyObject> =
                Python::with_gil(|py| res.map(|x| Clock::from(x).into_py(py)));
            pyobj
        })
    }

    /// Return the balance in lamports of an account at the given address at the slot
    /// corresponding to the given commitment level.
    ///
    /// Args:
    ///     address (Pubkey): The account to look up.
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     int: The account balance in lamports.
    ///
    pub fn get_balance<'p>(
        &mut self,
        py: Python<'p>,
        address: Pubkey,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
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
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     tuple[Hash, int]: The blockhash and last valid block height.
    ///
    pub fn get_latest_blockhash<'p>(
        &mut self,
        py: Python<'p>,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
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

    /// Get the fee in lamports for a given message.
    ///
    /// Args:
    ///     message (Message): The message to check.
    ///     commitment (Optional[CommitmentLevel]): The commitment level to use.
    ///
    /// Returns:
    ///     Optional[int]: The fee for the given message.
    ///
    pub fn get_fee_for_message<'p>(
        &mut self,
        py: Python<'p>,
        message: Message,
        commitment: Option<CommitmentLevel>,
    ) -> PyResult<&'p PyAny> {
        let underlying = self.0.clone();
        let commitment_inner = CommitmentLevelOriginal::from(commitment.unwrap_or_default());
        let message_inner = message.0;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = async_res!(underlying.get_fee_for_message_with_commitment_and_context(
                current(),
                message_inner,
                commitment_inner,
            ));
            let pyobj: PyResult<Option<PyObject>> =
                Python::with_gil(|py| res.map(|x| x.map(|num| num.into_py(py))));
            pyobj
        })
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let path = path.as_ref();
    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Failed to open \"{}\": {}", path.display(), err));

    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)
        .unwrap_or_else(|err| panic!("Failed to read \"{}\": {}", path.display(), err));
    file_data
}

fn default_shared_object_dirs() -> Vec<PathBuf> {
    let mut search_path = vec![];
    if let Ok(bpf_out_dir) = std::env::var("BPF_OUT_DIR") {
        search_path.push(PathBuf::from(bpf_out_dir));
    } else if let Ok(bpf_out_dir) = std::env::var("SBF_OUT_DIR") {
        search_path.push(PathBuf::from(bpf_out_dir));
    }
    search_path.push(PathBuf::from("tests/fixtures"));
    if let Ok(dir) = std::env::current_dir() {
        search_path.push(dir);
    }
    search_path
}

/// Ripped from solana-program-test after they rugged
/// by requiring &'static str
fn add_program_inner(inner: &mut ProgramTest, program_name: &str, program_id: Pubkey) {
    let add_bpf = |this: &mut ProgramTest, program_file: PathBuf| {
        let data = read_file(&program_file);
        info!(
            "\"{}\" SBF program from {}{}",
            program_name,
            program_file.display(),
            std::fs::metadata(&program_file)
                .map(|metadata| {
                    metadata
                        .modified()
                        .map(|time| {
                            format!(
                                ", modified {}",
                                HumanTime::from(time).to_text_en(Accuracy::Precise, Tense::Past)
                            )
                        })
                        .ok()
                })
                .ok()
                .flatten()
                .unwrap_or_default()
        );

        this.add_account(
            program_id.0,
            solana_sdk::account::Account {
                lamports: Rent::default().minimum_balance(data.len()).max(1),
                data,
                owner: solana_sdk::bpf_loader::id(),
                executable: true,
                rent_epoch: 0,
            },
        );
    };

    let warn_invalid_program_name = || {
        let valid_program_names = default_shared_object_dirs()
            .iter()
            .filter_map(|dir| dir.read_dir().ok())
            .flat_map(|read_dir| {
                read_dir.filter_map(|entry| {
                    let path = entry.ok()?.path();
                    if !path.is_file() {
                        return None;
                    }
                    match path.extension()?.to_str()? {
                        "so" => Some(path.file_stem()?.to_os_string()),
                        _ => None,
                    }
                })
            })
            .collect::<Vec<_>>();

        if valid_program_names.is_empty() {
            // This should be unreachable as `test-bpf` should guarantee at least one shared
            // object exists somewhere.
            warn!("No SBF shared objects found.");
            return;
        }

        warn!(
            "Possible bogus program name. Ensure the program name ({}) \
            matches one of the following recognizable program names:",
            program_name,
        );
        for name in valid_program_names {
            warn!(" - {}", name.to_str().unwrap());
        }
    };

    let program_file = find_file(&format!("{program_name}.so"));
    match program_file {
        // If SBF is preferred (i.e., `test-sbf` is invoked) and a BPF shared object exists,
        // use that as the program data.
        Some(file) => add_bpf(inner, file),

        // Invalid: `test-sbf` invocation with no matching SBF shared object.
        None => {
            warn_invalid_program_name();
            panic!("Program file data not available for {program_name} ({program_id})");
        }
    }
}

fn new_bankrun(
    programs: Vec<(&str, Pubkey)>,
    compute_max_units: Option<u64>,
    transaction_account_lock_limit: Option<usize>,
    accounts: Vec<(Pubkey, Account)>,
) -> ProgramTest {
    let mut pt = ProgramTest::default();
    pt.prefer_bpf(true);
    for prog in programs {
        add_program_inner(&mut pt, prog.0, prog.1);
    }
    if let Some(cmu) = compute_max_units {
        pt.set_compute_max_units(cmu);
    }
    if let Some(lock_lim) = transaction_account_lock_limit {
        pt.set_transaction_account_lock_limit(lock_lim);
    }
    for acc in accounts {
        pt.add_account(acc.0.into(), acc.1.into());
    }
    pt
}

/// Start a bankrun!
///
/// This will spin up a BanksServer and a BanksClient,
/// deploy programs and add accounts as instructed.
///
/// Args:
///     programs (Optional[Sequence[Tuple[str, Pubkey]]]): A sequence of (program_name, program_id) tuples
///         indicating which programs to deploy to the test environment. See the main bankrun docs for more explanation
///         on how to add programs.
///     accounts (Optional[Sequence[Tuple[Pubkey, Account]]]): A sequence of (address, account_object) tuples, indicating
///         what data to write to the given addresses.
///     compute_max_units (Optional[int]): Override the default compute unit limit for a transaction.
///     transaction_account_lock_limit (Optional[int]): Override the default transaction account lock limit.
///
///
/// Returns:
///     ProgramTestContext: a container for stuff you'll need to send transactions and interact with the test environment.
///     
#[pyfunction]
pub fn start<'p>(
    py: Python<'p>,
    programs: Option<Vec<(&str, Pubkey)>>,
    accounts: Option<Vec<(Pubkey, Account)>>,
    compute_max_units: Option<u64>,
    transaction_account_lock_limit: Option<usize>,
) -> PyResult<&'p PyAny> {
    let pt = new_bankrun(
        programs.unwrap_or_default(),
        compute_max_units,
        transaction_account_lock_limit,
        accounts.unwrap_or_default(),
    );
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let inner = pt.start_with_context().await;
        let res: PyResult<PyObject> =
            Python::with_gil(|py| Ok(ProgramTestContext(inner).into_py(py)));
        res
    })
}

/// Start a bankrun in an Anchor workspace, with all the workspace programs deployed.
///
/// This will spin up a BanksServer and a BanksClient,
/// deploy programs and add accounts as instructed.
///
/// Args:
///     path (pathlib.Path): Path to root of the Anchor project.
///     extra_programs (Optional[Sequence[Tuple[str, Pubkey]]]): A sequence of (program_name, program_id) tuples
///         indicating extra programs to deploy alongside the Anchor workspace programs. See the main bankrun docs for more explanation
///         on how to add programs.
///     accounts (Optional[Sequence[Tuple[Pubkey, Account]]]): A sequence of (address, account_object) tuples, indicating
///         what data to write to the given addresses.
///     compute_max_units (Optional[int]): Override the default compute unit limit for a transaction.
///     transaction_account_lock_limit (Optional[int]): Override the default transaction account lock limit.
///
/// Returns:
///     ProgramTestContext: a container for stuff you'll need to send transactions and interact with the test environment.
///     
#[pyfunction]
pub fn start_anchor<'p>(
    py: Python<'p>,
    path: PathBuf,
    extra_programs: Option<Vec<(&str, Pubkey)>>,
    accounts: Option<Vec<(Pubkey, Account)>>,
    compute_max_units: Option<u64>,
    transaction_account_lock_limit: Option<usize>,
) -> PyResult<&'p PyAny> {
    let mut programs = extra_programs.unwrap_or_default();
    let mut anchor_toml_path = path.clone();
    let mut sbf_out_dir = path;
    sbf_out_dir.push("target/deploy");
    anchor_toml_path.push("Anchor.toml");
    let toml_str = std::fs::read_to_string(anchor_toml_path)
        .map_err(|e| PyFileNotFoundError::new_err(e.to_string()))?;
    let parsed_toml = Table::from_str(&toml_str).unwrap();
    let toml_programs_raw = parsed_toml
        .get("programs")
        .and_then(|x| x.get("localnet"))
        .ok_or_else(|| PyValueError::new_err("`programs.localnet` not found in Anchor.toml"))?;
    let toml_programs_parsed = toml_programs_raw
        .as_table()
        .ok_or_else(|| PyValueError::new_err("Failed to parse `programs.localnet` table."))?;
    for (key, val) in toml_programs_parsed {
        let pubkey_with_quotes = val.to_string();
        let pubkey_str = &pubkey_with_quotes[1..pubkey_with_quotes.len() - 1];
        let pk = Pubkey::new_from_str(pubkey_str).map_err(|_| {
            PyValueError::new_err(format!(
                "Invalid pubkey in `programs.localnet` table. {}",
                val
            ))
        })?;
        programs.push((key, pk));
    }
    std::env::set_var("SBF_OUT_DIR", sbf_out_dir);
    let pt = new_bankrun(
        programs,
        compute_max_units,
        transaction_account_lock_limit,
        accounts.unwrap_or_default(),
    );
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let inner = pt.start_with_context().await;
        let res: PyResult<PyObject> =
            Python::with_gil(|py| Ok(ProgramTestContext(inner).into_py(py)));
        res
    })
}

/// The result of calling `bankrun.start()`.
///
/// Contains a BanksClient, a recent blockhash and a funded payer keypair.
#[pyclass(module = "solders.bankrun", subclass)]
#[derive(From, Into)]
pub struct ProgramTestContext(pub ProgramTestContextOriginal);

#[pymethods]
impl ProgramTestContext {
    /// BanksClient: The client for this test.
    #[getter]
    pub fn banks_client(&self) -> BanksClient {
        self.0.banks_client.clone().into()
    }

    /// Hash: The last blockhash registered when the client was initialized.
    #[getter]
    pub fn last_blockhash(&self) -> SolderHash {
        self.0.last_blockhash.into()
    }

    /// Keypair: A funded keypair for sending transactions.
    #[getter]
    pub fn payer(&self) -> Keypair {
        Keypair::from_bytes(self.0.payer.to_bytes()).unwrap()
    }

    /// Manually increment vote credits for the current epoch in the specified vote account to simulate validator voting activity.
    ///
    /// Args:
    ///     vote_account_address (Pubkey): The vote account addess in which to increment credits.
    ///     number_of_credits (int): How many credits to increment by.
    ///
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
    ///
    /// Args:
    ///     address (Pubkey): The address to write to.
    ///     account (Account): The account object to write.
    ///
    pub fn set_account(&mut self, address: &Pubkey, account: Account) {
        self.0.set_account(
            address.as_ref(),
            &AccountSharedData::from(AccountOriginal::from(account)),
        );
    }

    /// Overwrite the clock sysvar.
    ///
    /// Args:
    ///     clock (Clock): The new clock object.
    ///
    pub fn set_clock(&mut self, clock: &Clock) {
        self.0.set_sysvar(&clock.0)
    }

    /// Overwrite the rent sysvar.
    ///
    /// Args:
    ///     rent (Rent): The new rent object.
    ///
    pub fn set_rent(&mut self, rent: &Rent) {
        self.0.set_sysvar(&rent.0)
    }

    /// Force the working bank ahead to a new slot
    ///
    /// Args:
    ///     warp_slot (int): The slot to warp to.
    ///
    pub fn warp_to_slot(&mut self, warp_slot: Slot) -> PyResult<()> {
        self.0
            .warp_to_slot(warp_slot)
            .map_err(|e| to_py_value_err(&e))
    }
}

pub fn create_bankrun_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "bankrun")?;
    m.add("BanksClientError", py.get_type::<BanksClientError>())?;
    m.add_class::<BanksClient>()?;
    m.add_class::<ProgramTestContext>()?;
    m.add_class::<BanksTransactionResultWithMeta>()?;
    m.add_class::<BanksTransactionMeta>()?;
    m.add_function(wrap_pyfunction!(start, m)?)?;
    m.add_function(wrap_pyfunction!(start_anchor, m)?)?;
    Ok(m)
}
