use pyo3::prelude::*;
use solders_rpc_config::include_config;
use solders_rpc_errors::include_errors;
use solders_rpc_filter::include_filter;
use solders_rpc_requests::include_requests;
use solders_rpc_responses::include_responses;

pub(crate) fn include_rpc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    include_config(m)?;
    include_requests(m)?;
    include_filter(m)?;
    include_responses(m)?;
    include_errors(m)?;
    Ok(())
}
