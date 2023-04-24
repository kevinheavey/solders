use pyo3::{prelude::*, types::PyTuple, PyTypeInfo};
use solders_message::{
    from_bytes_versioned, to_bytes_versioned, Message, MessageAddressTableLookup, MessageHeader,
    MessageV0,
};

pub(crate) fn create_message_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "message")?;
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<MessageV0>()?;
    m.add_class::<MessageAddressTableLookup>()?;
    m.add_function(wrap_pyfunction!(to_bytes_versioned, m)?)?;
    m.add_function(wrap_pyfunction!(from_bytes_versioned, m)?)?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    m.add(
        "VersionedMessage",
        union.get_item(PyTuple::new(
            py,
            vec![MessageV0::type_object(py), Message::type_object(py)],
        ))?,
    )?;
    Ok(m)
}
