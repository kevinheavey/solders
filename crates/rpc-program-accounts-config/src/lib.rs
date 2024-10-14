use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

use solders_rpc_account_info_config::RpcAccountInfoConfig;
use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_rpc_filter::RpcFilterType;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getProgramAccounts``.
    ///
    /// Args:
    ///     account_config (RpcAccountInfoConfig): Account info config.
    ///     filters (Optional[Sequence[int | Memcmp]]): Filter results using various filter objects; account must meet all filter criteria to be included in results.
    ///     with_context (Optional[bool]): Wrap the result in an RpcResponse JSON object.
    ///
    => RpcProgramAccountsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcProgramAccountsConfig {
    #[new]
    pub fn new(
        account_config: RpcAccountInfoConfig,
        filters: Option<Vec<RpcFilterType>>,
        with_context: Option<bool>,
        sort_results: Option<bool>,
    ) -> Self {
        Self(rpc_config::RpcProgramAccountsConfig {
            filters: filters.map(|v| v.into_iter().map(|f| f.into()).collect()),
            account_config: account_config.into(),
            with_context,
            sort_results,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcEpochConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn account_config(&self) -> RpcAccountInfoConfig {
        self.0.account_config.clone().into()
    }

    #[getter]
    pub fn filters(&self) -> Option<Vec<PyObject>> {
        let cloned = self.0.filters.clone();
        Python::with_gil(|py| {
            cloned.map(|v| {
                v.into_iter()
                    .map(|f| RpcFilterType::from(f).into_py(py))
                    .collect()
            })
        })
    }

    #[getter]
    pub fn with_context(&self) -> Option<bool> {
        self.0.with_context
    }
}
