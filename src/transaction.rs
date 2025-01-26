use pyo3::prelude::*;

use solders_traits::{SanitizeError, TransactionError};

use solders_transaction::{Legacy, Transaction, VersionedTransaction};
pub(crate) fn include_transaction(m: &Bound<'_, PyModule>, py: Python) -> PyResult<()> {
    m.add_class::<Transaction>()?;
    m.add_class::<VersionedTransaction>()?;
    m.add_class::<Legacy>()?;
    m.add("SanitizeError", py.get_type::<SanitizeError>())?;
    m.add("TransactionError", py.get_type::<TransactionError>())?;
    Ok(())
}
