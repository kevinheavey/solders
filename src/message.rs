use pyo3::prelude::*;
use solders_message::{
    from_bytes_versioned, to_bytes_versioned, CompileError, Message, MessageAddressTableLookup,
    MessageHeader, MessageV0, MessageV1, MessageV1Error, TransactionConfig,
};

pub(crate) fn include_message(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<MessageV0>()?;
    m.add_class::<MessageV1>()?;
    m.add_class::<MessageAddressTableLookup>()?;
    m.add_class::<TransactionConfig>()?;
    m.add("CompileError", py.get_type::<CompileError>())?;
    m.add("MessageV1Error", py.get_type::<MessageV1Error>())?;
    m.add_function(wrap_pyfunction!(to_bytes_versioned, m)?)?;
    m.add_function(wrap_pyfunction!(from_bytes_versioned, m)?)?;
    Ok(())
}
