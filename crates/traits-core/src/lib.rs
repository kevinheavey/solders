use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    pyclass::CompareOp,
    types::PyBytes,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
};

pub fn richcmp_type_error(op: &str) -> PyErr {
    let msg = format!("{op} not supported.");
    PyTypeError::new_err(msg)
}

fn calculate_hash<T>(t: &T) -> u64
where
    T: Hash + ?Sized,
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait RichcmpEqualityOnly: PartialEq {
    fn richcmp(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self == other),
            CompareOp::Ne => Ok(self != other),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}
pub trait RichcmpFull: PartialEq + PartialOrd {
    fn richcmp(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Lt => self < other,
            CompareOp::Gt => self > other,
            CompareOp::Le => self <= other,
            CompareOp::Ge => self >= other,
        }
    }
}

#[macro_export]
macro_rules! impl_display {
    ($ident:ident) => {
        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

pub trait PyHash: Hash {
    fn pyhash(&self) -> u64 {
        calculate_hash(self)
    }
}

pub trait PyBytesSlice: AsRef<[u8]> {
    fn pybytes_slice<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, AsRef::<[u8]>::as_ref(self))
    }
}

#[macro_export]
macro_rules! pybytes_general_for_pybytes_slice {
    ($ident:ident) => {
        impl $crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                $crate::PyBytesSlice::pybytes_slice(self, py)
            }
        }
    };
}

#[macro_export]
macro_rules! pybytes_general_for_pybytes_bincode {
    ($ident:ident) => {
        impl $crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                $crate::PyBytesBincode::pybytes_bincode(self, py)
            }
        }
    };
}

#[macro_export]
macro_rules! pybytes_general_for_pybytes_cbor {
    ($ident:ident) => {
        impl $crate::PyBytesGeneral for $ident {
            fn pybytes_general<'a>(
                &self,
                py: pyo3::prelude::Python<'a>,
            ) -> &'a pyo3::types::PyBytes {
                $crate::PyBytesCbor::pybytes_cbor(self, py)
            }
        }
    };
}

pub trait PyBytesBincode: Serialize {
    fn pybytes_bincode<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &bincode::serialize(self).unwrap())
    }
}

pub trait PyBytesCbor: Serialize + std::marker::Sized {
    fn pybytes_cbor<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, &serde_cbor::to_vec(self).unwrap())
    }
}

pub trait PyBytesGeneral {
    fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes;
}

#[macro_export]
macro_rules! pybytes_general_via_slice {
    ($ident:ident) => {
        impl $crate::PyBytesSlice for $ident {}
        $crate::pybytes_general_for_pybytes_slice!($ident);
    };
}

#[macro_export]
macro_rules! pybytes_general_via_bincode {
    ($ident:ident) => {
        impl $crate::PyBytesBincode for $ident {}
        $crate::pybytes_general_for_pybytes_bincode!($ident);
    };
}

#[macro_export]
macro_rules! pybytes_general_via_cbor {
    ($ident:ident) => {
        impl $crate::PyBytesCbor for $ident {}
        $crate::pybytes_general_for_pybytes_cbor!($ident);
    };
}

#[macro_export]
macro_rules! py_from_bytes_general_for_py_from_bytes_bincode {
    ($ident:ident) => {
        impl $crate::PyFromBytesGeneral for $ident {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                <Self as $crate::PyFromBytesBincode>::py_from_bytes_bincode(raw)
            }
        }
    };
}

#[macro_export]
macro_rules! py_from_bytes_general_for_py_from_bytes_cbor {
    ($ident:ident) => {
        impl $crate::PyFromBytesGeneral for $ident {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                <Self as $crate::PyFromBytesCbor>::py_from_bytes_cbor(raw)
            }
        }
    };
}

#[macro_export]
macro_rules! py_from_bytes_general_via_bincode {
    ($ident:ident) => {
        impl $crate::PyFromBytesBincode<'_> for $ident {}
        $crate::py_from_bytes_general_for_py_from_bytes_bincode!($ident);
    };
}

pub fn to_py_value_err(err: &impl ToString) -> PyErr {
    PyValueError::new_err(err.to_string())
}

pub fn handle_py_value_err<T: Into<P>, E: ToString, P>(res: Result<T, E>) -> PyResult<P> {
    res.map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
}

pub trait PyFromBytesBincode<'b>: Deserialize<'b> {
    fn py_from_bytes_bincode(raw: &'b [u8]) -> PyResult<Self> {
        let deser = bincode::deserialize::<Self>(raw);
        handle_py_value_err(deser)
    }
}

#[macro_export]
macro_rules! py_from_bytes_general_via_cbor {
    ($ident:ident) => {
        impl $crate::PyFromBytesCbor<'_> for $ident {}
        $crate::py_from_bytes_general_for_py_from_bytes_cbor!($ident);
    };
}

pub trait PyFromBytesCbor<'b>: Deserialize<'b> {
    fn py_from_bytes_cbor(raw: &'b [u8]) -> PyResult<Self> {
        let deser = serde_cbor::from_slice::<Self>(raw);
        handle_py_value_err(deser)
    }
}

pub trait PyFromBytesGeneral: Sized {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self>;
}

pub trait CommonMethodsCore:
    fmt::Display + fmt::Debug + PyBytesGeneral + PyFromBytesGeneral + IntoPy<PyObject> + Clone
{
    fn pybytes<'b>(&self, py: Python<'b>) -> &'b PyBytes {
        PyBytesGeneral::pybytes_general(self, py)
    }

    fn pystr(&self) -> String {
        self.to_string()
    }
    fn pyrepr(&self) -> String {
        format!("{self:#?}")
    }

    fn py_from_bytes(raw: &[u8]) -> PyResult<Self> {
        <Self as PyFromBytesGeneral>::py_from_bytes_general(raw)
    }

    fn pyreduce(&self) -> PyResult<(PyObject, PyObject)> {
        let cloned = self.clone();
        Python::with_gil(|py| {
            let constructor = cloned.into_py(py).getattr(py, "from_bytes")?;
            Ok((
                constructor,
                (PyBytesGeneral::pybytes_general(self, py).to_object(py),).to_object(py),
            ))
        })
    }
}

pub trait CommonMethods<'a>: CommonMethodsCore + Serialize + Deserialize<'a> {
    fn py_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn py_from_json(raw: &'a str) -> PyResult<Self> {
        serde_json::from_str(raw).map_err(|e| to_py_value_err(&e))
    }
}

#[macro_export]
macro_rules! common_methods_default {
    ($ty:ty) => {
        impl $crate::CommonMethodsCore for $ty {}
        impl $crate::CommonMethods<'_> for $ty {}
    };
}

#[macro_export]
macro_rules! transaction_status_boilerplate {
    ($name:ident) => {
        impl $crate::RichcmpEqualityOnly for $name {}
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        $crate::pybytes_general_via_bincode!($name);
        $crate::py_from_bytes_general_via_bincode!($name);
        $crate::common_methods_default!($name);
    };
}
