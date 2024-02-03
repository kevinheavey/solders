use pyo3::prelude::*;
use solders_address_lookup_table_account::{
    derive_lookup_table_address, AddressLookupTableAccount,
};

pub(crate) fn create_address_lookup_table_account_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "address_lookup_table_account")?;
    m.add_class::<AddressLookupTableAccount>()?;
    m.add_function(wrap_pyfunction!(derive_lookup_table_address, m)?)?;
    Ok(m)
}
