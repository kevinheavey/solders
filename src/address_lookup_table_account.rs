use pyo3::prelude::*;
use solders_primitives::address_lookup_table_account::AddressLookupTableAccount;

pub(crate) fn create_address_lookup_table_account_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "address_lookup_table_account")?;
    m.add_class::<AddressLookupTableAccount>()?;
    Ok(m)
}
