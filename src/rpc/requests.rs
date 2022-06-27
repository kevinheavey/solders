use std::fmt::Display;

use crate::{
    py_from_bytes_general_via_bincode, pybytes_general_via_bincode, CommonMethods, Pubkey,
    PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use solders_macros::{common_magic_methods, richcmp_eq_only};

use crate::Signature;

use super::config::{RpcRequestAirdropConfig, RpcSignatureStatusConfig};

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
#[common_magic_methods]
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

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[staticmethod]
    fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap()
    }
}

impl Display for GetSignatureStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl CommonMethods for GetSignatureStatuses {}
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
#[common_magic_methods]
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

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[staticmethod]
    fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap()
    }
}

impl Display for RequestAirdrop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl CommonMethods for RequestAirdrop {}
impl RichcmpEqualityOnly for RequestAirdrop {}
pybytes_general_via_bincode!(RequestAirdrop);
py_from_bytes_general_via_bincode!(RequestAirdrop);

pub fn create_requests_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let requests_mod = PyModule::new(py, "requests")?;
    requests_mod.add_class::<GetSignatureStatuses>()?;
    requests_mod.add_class::<RequestAirdrop>()?;
    Ok(requests_mod)
}
