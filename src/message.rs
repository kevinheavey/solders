use pyo3::prelude::*;
use solders_message::{
    from_bytes_versioned, to_bytes_versioned, Message, MessageAddressTableLookup, MessageHeader,
    MessageV0,
};

pub(crate) fn include_message(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<MessageV0>()?;
    m.add_class::<MessageAddressTableLookup>()?;
    m.add_function(wrap_pyfunction!(to_bytes_versioned, m)?)?;
    m.add_function(wrap_pyfunction!(from_bytes_versioned, m)?)?;
    Ok(())
}
