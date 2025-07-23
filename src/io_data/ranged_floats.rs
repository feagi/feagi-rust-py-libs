use feagi_core_data_structures_and_processing::io_data::Normalized0To1F32;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;













#[pyclass]
#[pyo3(name = "Normalized0To1F32")]
#[derive(Debug, Clone, Copy)]
pub struct PyNormalized0To1F32 {
    pub(crate) inner: Normalized0To1F32,
}

impl From<PyNormalized0To1F32> for Normalized0To1F32 {
    fn from(inner: PyNormalized0To1F32) -> Self {
        inner.inner
    }
}

impl From<Normalized0To1F32> for PyNormalized0To1F32 {
    fn from(inner: Normalized0To1F32) -> Self {
        PyNormalized0To1F32{ inner }
    }
}


#[pymethods]
impl PyNormalized0To1F32 {
    #[new]
    fn new(float: f32) -> PyResult<Self> {
        let result = Normalized0To1F32::new(float);
        match result {
            Ok(inner) => Ok(PyNormalized0To1F32 { inner: inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    
    #[staticmethod]
    fn new_clamped(float: f32) -> PyResult<Self> {
        let result = Normalized0To1F32::new_with_clamp(float);
        match result {
            Ok(inner) => Ok(PyNormalized0To1F32 { inner: inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}


