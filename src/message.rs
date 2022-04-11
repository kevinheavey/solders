use pyo3::prelude::*;
use solana_sdk::message::legacy::Message as MessageOriginal;

#[pyclass]
#[derive(PartialEq, Debug, Clone)]
pub struct Message(pub MessageOriginal);
