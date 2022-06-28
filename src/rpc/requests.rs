use crate::{
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode, CommonMethods, Pubkey,
    PyBytesBincode, PyErrWrapper, PyFromBytesBincode, RichcmpEqualityOnly,
};
use pyo3::{create_exception, exceptions::PyException, prelude::*};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use solders_macros::{common_methods, richcmp_eq_only};

use crate::Signature;

use super::config::{RpcRequestAirdropConfig, RpcSignatureStatusConfig};

create_exception!(
    solders,
    SerdeJSONError,
    PyException,
    "Raised when an error is encountered during JSON (de)serialization."
);

impl From<serde_json::Error> for PyErrWrapper {
    fn from(e: serde_json::Error) -> Self {
        Self(SerdeJSONError::new_err(e.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct RequestBase {
    jsonrpc: String,
    id: u64,
    method: String,
}

impl RequestBase {
    fn new(method: String, id: Option<u64>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id: id.unwrap_or(0),
            method,
        }
    }
}

#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetSignatureStatuses {
    #[serde(flatten)]
    base: RequestBase,
    #[pyo3(get)]
    params: GetSignatureStatusesParams,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl GetSignatureStatuses {
    #[new]
    fn new(
        signatures: Vec<Signature>,
        config: Option<RpcSignatureStatusConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = GetSignatureStatusesParams(signatures, config);
        let method = "getSignatureStatuses".to_owned();
        let base = RequestBase::new(method, id);
        Self { base, params }
    }
}

macro_rules! rpc_impl_display {
    ($ident:ident) => {
        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_json())
            }
        }
    };
}

rpc_impl_display!(GetSignatureStatuses);

impl CommonMethods<'_> for GetSignatureStatuses {}
impl RichcmpEqualityOnly for GetSignatureStatuses {}
pybytes_general_via_bincode!(GetSignatureStatuses);
py_from_bytes_general_via_bincode!(GetSignatureStatuses);

#[serde_as]
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetSignatureStatusesParams(
    #[serde_as(as = "Vec<DisplayFromStr>")] Vec<Signature>,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcSignatureStatusConfig>,
);

#[serde_as]
#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RequestAirdropParams(
    #[serde_as(as = "DisplayFromStr")] Pubkey,
    u64,
    #[serde(skip_serializing_if = "Option::is_none", default)] Option<RpcRequestAirdropConfig>,
);

#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RequestAirdrop {
    #[serde(flatten)]
    base: RequestBase,
    #[pyo3(get)]
    params: RequestAirdropParams,
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RequestAirdrop {
    #[new]
    fn new(
        pubkey: Pubkey,
        lamports: u64,
        config: Option<RpcRequestAirdropConfig>,
        id: Option<u64>,
    ) -> Self {
        let params = RequestAirdropParams(pubkey, lamports, config);
        let method = "requestAirdrop".to_owned();
        let base = RequestBase::new(method, id);
        Self { base, params }
    }
}

rpc_impl_display!(RequestAirdrop);

impl CommonMethods<'_> for RequestAirdrop {}
impl RichcmpEqualityOnly for RequestAirdrop {}
pybytes_general_via_bincode!(RequestAirdrop);
py_from_bytes_general_via_bincode!(RequestAirdrop);

#[derive(FromPyObject, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    GetSignatureStatuses(GetSignatureStatuses),
    RequestAirdrop(RequestAirdrop),
}

impl Body {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Body::GetSignatureStatuses(x) => x.clone().into_py(py),
            Body::RequestAirdrop(x) => x.clone().into_py(py),
        }
    }
}

#[pyfunction]
pub fn batch_to_json(reqs: Vec<Body>) -> String {
    serde_json::to_string(&reqs).unwrap()
}

#[pyfunction]
pub fn batch_from_json(raw: &str) -> PyResult<Vec<PyObject>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let deser: Vec<Body> = serde_json::from_str(raw).unwrap();
    Ok(deser.iter().map(|x| x.to_object(py)).collect())
}

pub fn create_requests_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let requests_mod = PyModule::new(py, "requests")?;
    requests_mod.add_class::<GetSignatureStatuses>()?;
    requests_mod.add_class::<RequestAirdrop>()?;
    let funcs = [
        wrap_pyfunction!(batch_to_json, requests_mod)?,
        wrap_pyfunction!(batch_from_json, requests_mod)?,
    ];
    for func in funcs {
        requests_mod.add_function(func)?;
    }
    Ok(requests_mod)
}
