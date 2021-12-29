use pyo3::prelude::*;
use solana_sdk::pubkey::{bytes_are_curve_point, Pubkey};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Check if _bytes s is a valid point on curve or not.
#[pyfunction]
fn is_on_curve(_bytes: &[u8]) -> bool {
    bytes_are_curve_point(_bytes)
}

#[pyclass]
pub struct Vector3 {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
    #[pyo3(get, set)]
    pub z: i32,
}
#[pymethods]
impl Vector3 {
    #[new]
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

#[pyclass]
pub struct PublicKey(Pubkey);

#[pymethods]
impl PublicKey {
    #[new]
    pub fn new(pubkey_vec: &[u8]) -> Self {
        PublicKey(Pubkey::new(pubkey_vec))
    }

    pub fn is_on_curve(&self) -> bool {
        self.0.is_on_curve()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __bytes__(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn solder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_on_curve, m)?)?;
    m.add_class::<Vector3>()?;
    m.add_class::<PublicKey>()?;
    Ok(())
}
