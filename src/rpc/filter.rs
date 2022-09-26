use crate::rpc::tmp_filter::{
    Memcmp as MemcmpOriginal, MemcmpEncodedBytes as MemcmpEncodedBytesOriginal,
    MemcmpEncoding as MemcmpEncodingOriginal, RpcFilterType as RpcFilterTypeOriginal,
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use derive_more::{From, Into};
use solders_macros::{common_methods, enum_original_mapping, richcmp_eq_only};

use crate::{
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode, CommonMethods,
    PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MemcmpEncodedBytes {
    Base58(String),
    Bytes(Vec<u8>),
}

impl IntoPy<PyObject> for MemcmpEncodedBytes {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Base58(s) => s.into_py(py),
            Self::Bytes(v) => v.into_py(py),
        }
    }
}

impl From<MemcmpEncodedBytes> for MemcmpEncodedBytesOriginal {
    fn from(m: MemcmpEncodedBytes) -> Self {
        match m {
            MemcmpEncodedBytes::Base58(s) => MemcmpEncodedBytesOriginal::Base58(s),
            MemcmpEncodedBytes::Bytes(v) => MemcmpEncodedBytesOriginal::Bytes(v),
        }
    }
}

impl From<MemcmpEncodedBytesOriginal> for MemcmpEncodedBytes {
    fn from(m: MemcmpEncodedBytesOriginal) -> Self {
        match m {
            MemcmpEncodedBytesOriginal::Base58(s) => Self::Base58(s),
            MemcmpEncodedBytesOriginal::Bytes(v) => Self::Bytes(v),
            _ => panic!("Unexpected variant: {:?}", m),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[enum_original_mapping(MemcmpEncodingOriginal)]
#[pyclass(module = "solders.rpc.filter")]
pub enum MemcmpEncoding {
    Binary,
}

/// Compares a provided series of bytes with program account data at a particular offset.
///
/// Args:
///     offset (int): Data offset to begin match.
///     bytes_ (str | Sequnce[int]): Bytes, encoded with specified encoding, or default Binary
///     encoding (Optional[MemcmpEncoding]): Optional encoding specification.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, From, Into)]
#[pyclass(module = "solders.rpc.filter", subclass)]
pub struct Memcmp(MemcmpOriginal);

pybytes_general_via_bincode!(Memcmp);
py_from_bytes_general_via_bincode!(Memcmp);
impl_display!(Memcmp);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Memcmp {
    #[new]
    pub fn new(
        offset: usize,
        bytes_: MemcmpEncodedBytes,
        encoding: Option<MemcmpEncoding>,
    ) -> Self {
        Self(MemcmpOriginal {
            offset,
            bytes: bytes_.into(),
            encoding: encoding.map(|e| e.into()),
        })
    }

    #[getter]
    pub fn offset(&self) -> usize {
        self.0.offset
    }

    #[getter]
    pub fn bytes_(&self, py: Python) -> PyObject {
        MemcmpEncodedBytes::from(self.0.bytes.clone()).into_py(py)
    }

    #[getter]
    pub fn encoding(&self) -> Option<MemcmpEncoding> {
        self.0.encoding.clone().map(MemcmpEncoding::from)
    }
}

impl RichcmpEqualityOnly for Memcmp {}
impl CommonMethods<'_> for Memcmp {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase")]
pub enum RpcFilterType {
    DataSize(u64),
    Memcmp(Memcmp),
}

impl IntoPy<PyObject> for RpcFilterType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            RpcFilterType::DataSize(num) => num.into_py(py),
            RpcFilterType::Memcmp(mem) => mem.into_py(py),
        }
    }
}

impl From<RpcFilterType> for RpcFilterTypeOriginal {
    fn from(r: RpcFilterType) -> Self {
        match r {
            RpcFilterType::DataSize(num) => RpcFilterTypeOriginal::DataSize(num),
            RpcFilterType::Memcmp(mem) => RpcFilterTypeOriginal::Memcmp(mem.into()),
        }
    }
}

impl From<RpcFilterTypeOriginal> for RpcFilterType {
    fn from(r: RpcFilterTypeOriginal) -> Self {
        match r {
            RpcFilterTypeOriginal::DataSize(num) => RpcFilterType::DataSize(num),
            RpcFilterTypeOriginal::Memcmp(mem) => RpcFilterType::Memcmp(mem.into()),
        }
    }
}

pub(crate) fn create_filter_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "filter")?;
    m.add_class::<MemcmpEncoding>()?;
    m.add_class::<Memcmp>()?;
    Ok(m)
}
