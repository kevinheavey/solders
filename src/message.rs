use pyo3::prelude::*;
use solders_message::{
    from_bytes_versioned, include_v1_constants, to_bytes_versioned, CompileError, Message,
    MessageAddressTableLookup, MessageError, MessageHeader, MessageV0, MessageV1,
    TransactionConfig,
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
    m.add("MessageError", py.get_type::<MessageError>())?;
    include_v1_constants(m)?;
    m.add_function(wrap_pyfunction!(to_bytes_versioned, m)?)?;
    m.add_function(wrap_pyfunction!(from_bytes_versioned, m)?)?;
    Ok(())
}
