use pyo3::prelude::*;
use solders_primitives::message::{Message, MessageAddressTableLookup, MessageHeader, MessageV0};

pub(crate) fn create_message_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "message")?;
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<MessageV0>()?;
    m.add_class::<MessageAddressTableLookup>()?;
    Ok(m)
}
