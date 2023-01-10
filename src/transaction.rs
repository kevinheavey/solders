use pyo3::{
    prelude::*,
    types::{PyInt, PyTuple},
    PyTypeInfo,
};

use solders_traits::{SanitizeError, TransactionError};

use solders_primitives::{
    keypair::Keypair,
    null_signer::NullSigner,
    presigner::Presigner,
    transaction::{Legacy, Transaction, VersionedTransaction},
};

pub(crate) fn create_transaction_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "transaction")?;
    m.add_class::<Transaction>()?;
    m.add_class::<VersionedTransaction>()?;
    m.add_class::<Legacy>()?;
    m.add("SanitizeError", py.get_type::<SanitizeError>())?;
    m.add("TransactionError", py.get_type::<TransactionError>())?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    m.add(
        "TransactionVersion",
        union.get_item(PyTuple::new(
            py,
            vec![Legacy::type_object(py), PyInt::type_object(py)],
        ))?,
    )?;
    m.add(
        "Signer",
        union.get_item(PyTuple::new(
            py,
            vec![
                Keypair::type_object(py),
                Presigner::type_object(py),
                NullSigner::type_object(py),
            ],
        ))?,
    )?;
    Ok(m)
}
