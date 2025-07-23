use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::{SingleChannelDimensions};

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SingleChannelDimensions")]
pub struct PySingleChannelDimensions {
    pub(crate) inner: SingleChannelDimensions
}

#[pymethods]
impl PySingleChannelDimensions {
    #[new]
    pub fn new(x: u32, y: u32, z: u32) -> PyResult<Self> {
        let result = SingleChannelDimensions::new(x, y, z);
        match result {
            Ok(s) => Ok(PySingleChannelDimensions {inner: s}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}