#![allow(clippy::large_enum_variant)]
use pyo3::{prelude::*, PyClass};
use serde::{Deserialize, Serialize};
// use solana_client::nonblocking::rpc_client;
// use solana_client::rpc_response::Response;
// use solana_rpc::rpc;

// note: the `data` field of the error struct is always None

// pub struct GetAccountInfoResp(Response<Option<Account>>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RpcError {
    /// Code
    #[pyo3(get)]
    pub code: i64,
    /// Message
    #[pyo3(get)]
    pub message: String,
}

#[pymethods]
impl RpcError {
    #[new]
    pub fn new(code: i64, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Resp<T: PyClass + IntoPy<PyObject>> {
    Error {
        #[serde(skip)]
        jsonrpc: crate::rpc::requests::V2,
        error: RpcError,
        #[serde(skip)]
        id: u64,
    },
    Result {
        #[serde(skip)]
        jsonrpc: crate::rpc::requests::V2,
        result: T,
        #[serde(skip)]
        id: u64,
    },
}

impl<T: PyClass + IntoPy<PyObject>> IntoPy<PyObject> for Resp<T> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Error { error: e, .. } => e.into_py(py),
            Self::Result { result: r, .. } => r.into_py(py),
        }
    }
}

// The one in solana_client isn't clonable
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct GetBlockCommitmentResp {
    #[pyo3(get)]
    pub commitment: Option<[u64; 32]>,
    #[pyo3(get)]
    pub total_stake: u64,
}

#[pymethods]
impl GetBlockCommitmentResp {
    #[new]
    pub fn new(commitment: Option<[u64; 32]>, total_stake: u64) -> Self {
        Self {
            commitment,
            total_stake,
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    #[staticmethod]
    pub fn from_json(raw: &str) -> Resp<Self> {
        serde_json::from_str(raw).unwrap()
    }
}

#[derive(FromPyObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GetBlockCommitmentWrapper {
    #[serde(skip)]
    Id(u64),
    Error(RpcError),
    Result(GetBlockCommitmentResp),
    #[serde(skip, alias = "jsonrpc")]
    Jsonrpc(String),
}

impl IntoPy<PyObject> for GetBlockCommitmentWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Error(e) => e.into_py(py),
            Self::Result(r) => r.into_py(py),
            _ => panic!("Unreachable"),
        }
    }
}

pub(crate) fn create_responses_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "responses")?;
    m.add_class::<GetBlockCommitmentResp>()?;
    Ok(m)
}
