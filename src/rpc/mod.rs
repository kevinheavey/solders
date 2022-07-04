use std::collections::HashMap;

use self::{config::create_config_mod, filter::create_filter_mod, requests::create_requests_mod};
use pyo3::prelude::*;

pub mod config;
pub mod filter;
pub mod requests;
mod tmp_config;
mod tmp_filter;

pub fn create_rpc_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let rpc_mod = PyModule::new(py, "rpc")?;
    let config_mod = create_config_mod(py)?;
    let requests_mod = create_requests_mod(py)?;
    let filter_mod = create_filter_mod(py)?;
    let submodules = [config_mod, requests_mod, filter_mod];
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
