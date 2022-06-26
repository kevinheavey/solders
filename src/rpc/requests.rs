use jsonrpc_core::{
    types::{Id, MethodCall, Params},
    Version,
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Signature;

use super::config::RpcSignatureStatusConfig;

macro_rules! rpc_params {
	($($param:expr),*) => {
		{
			let mut __params = vec![];
			$(
				__params.push(serde_json::to_value($param).unwrap());
			)*
			Params::Array(__params)
		}
	};
	() => {
		Params::None
	}
}

fn method_call(id: Option<u64>, method: String, params: Params) -> MethodCall {
    MethodCall {
        jsonrpc: Some(Version::V2),
        id: Id::Num(id.unwrap_or(0)),
        method,
        params,
    }
}

#[pyclass(module = "solders.rpc.requests")]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetSignatureStatuses(MethodCall);

#[pymethods]
impl GetSignatureStatuses {
    #[new]
    fn new(signatures: Vec<Signature>, config: RpcSignatureStatusConfig, id: Option<u64>) -> Self {
        Self(method_call(
            id,
            "getSignatureStatuses".to_owned(),
            rpc_params![
                signatures.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                config
            ],
        ))
    }

    /// Convert to JSON
    ///
    /// Example:
    ///     >>> from solders.rpc.requests import GetSignatureStatuses
    ///     >>> from solders.signature import Signature
    ///     >>> from solders.rpc.config import RpcSignatureStatusConfig
    ///     >>> req = GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True))
    ///     >>> req.to_json()
    ///     '{"jsonrpc":"2.0","method":"getSignatureStatuses","params":[["1111111111111111111111111111111111111111111111111111111111111111"],{"searchTransactionHistory":true}],"id":0}'
    ///
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[staticmethod]
    fn from_json(raw: &str) -> Self {
        let underlying: MethodCall = serde_json::from_str(raw).unwrap();
        Self(underlying)
    }
}

pub fn create_requests_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let requests_mod = PyModule::new(py, "requests")?;
    requests_mod.add_class::<GetSignatureStatuses>()?;
    Ok(requests_mod)
}
