use pyo3::{prelude::*, types::PyTuple, PyTypeInfo};
use solders_primitives::message::{Message, MessageAddressTableLookup, MessageHeader, MessageV0};

pub(crate) fn create_message_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "message")?;
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<MessageV0>()?;
    m.add_class::<MessageAddressTableLookup>()?;
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
