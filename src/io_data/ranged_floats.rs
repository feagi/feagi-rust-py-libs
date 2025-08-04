use feagi_core_data_structures_and_processing::io_data::NormalizedM1To1F32;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;



#[pyclass]
#[pyo3(name = "NormalizedM1To1F32")]
#[derive(Debug, Clone, Copy)]
pub struct PyNormalizedM1To1F32 {
    pub(crate) inner: NormalizedM1To1F32,
}

impl From<PyNormalizedM1To1F32> for NormalizedM1To1F32 {
    fn from(inner: PyNormalizedM1To1F32) -> Self {
        inner.inner
    }
}

impl From<NormalizedM1To1F32> for PyNormalizedM1To1F32 {
    fn from(inner: NormalizedM1To1F32) -> Self {
        PyNormalizedM1To1F32{ inner }
    }
}


#[pymethods]
impl PyNormalizedM1To1F32 {
    #[new]
    fn new(float: f32) -> PyResult<Self> {
        let result = NormalizedM1To1F32::new(float);
        match result {
            Ok(inner) => Ok(PyNormalizedM1To1F32 { inner: inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    
    #[staticmethod]
    fn new_clamped(float: f32) -> PyResult<Self> {
        let result = NormalizedM1To1F32::new_with_clamp(float);
        match result {
            Ok(inner) => Ok(PyNormalizedM1To1F32 { inner: inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}


