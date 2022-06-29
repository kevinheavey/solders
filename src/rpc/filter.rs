use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_filter::{
    Memcmp as MemcmpOriginal, MemcmpEncodedBytes as MemcmpEncodedBytesOriginal,
    MemcmpEncoding as MemcmpEncodingOriginal, RpcFilterType as RpcFilterTypeOriginal,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromPyObject)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MemcmpEncodedBytes {
    Base58(String),
    Bytes(Vec<u8>),
}

impl From<MemcmpEncodedBytes> for MemcmpEncodedBytesOriginal {
    fn from(m: MemcmpEncodedBytes) -> Self {
        match m {
            MemcmpEncodedBytes::Base58(s) => MemcmpEncodedBytesOriginal::Base58(s),
            MemcmpEncodedBytes::Bytes(v) => MemcmpEncodedBytesOriginal::Bytes(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub enum MemcmpEncoding {
    Binary,
}

impl From<MemcmpEncoding> for MemcmpEncodingOriginal {
    fn from(_: MemcmpEncoding) -> Self {
        MemcmpEncodingOriginal::Binary
    }
}

/// Compares a provided series of bytes with program account data at a particular offset.
///
/// Args:
///     offset (int): Data offset to begin match.
///     bytes (str | Sequnce[int]): Bytes, encoded with specified encoding, or default Binary
///     encoding (Optional[MemcmpEncoding]): Optional encoding specification.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[pyclass]
pub struct Memcmp(MemcmpOriginal);

#[pymethods]
impl Memcmp {
    #[new]
    pub fn new(offset: usize, bytes: MemcmpEncodedBytes, encoding: Option<MemcmpEncoding>) -> Self {
        Self(MemcmpOriginal {
            offset,
            bytes: bytes.into(),
            encoding: encoding.map(|e| e.into()),
        })
    }
}

impl From<Memcmp> for MemcmpOriginal {
    fn from(m: Memcmp) -> Self {
        m.0
    }
}

impl From<MemcmpOriginal> for Memcmp {
    fn from(m: MemcmpOriginal) -> Self {
        Self(m)
    }
}

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
