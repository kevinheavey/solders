use std::collections::HashMap;

use pyo3::prelude::*;
use solders_rpc_config::create_config_mod;
use solders_rpc_errors::create_errors_mod;
use solders_rpc_filter::create_filter_mod;
use solders_rpc_requests::create_requests_mod;
use solders_rpc_responses::create_responses_mod;

pub(crate) fn create_rpc_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let rpc_mod = PyModule::new(py, "rpc")?;
    let config_mod = create_config_mod(py)?;
    let requests_mod = create_requests_mod(py)?;
    let filter_mod = create_filter_mod(py)?;
    let responses_mod = create_responses_mod(py)?;
    let errors_mod = create_errors_mod(py)?;
    let submodules = [
        config_mod,
        requests_mod,
        filter_mod,
        responses_mod,
        errors_mod,
    ];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("solders.rpc.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        rpc_mod.add_submodule(submod)?;
    }
    Ok(rpc_mod)
}
