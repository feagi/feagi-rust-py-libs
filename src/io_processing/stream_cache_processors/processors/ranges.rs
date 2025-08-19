use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::{LinearScaleTo0And1Processor, LinearScaleToM1And1};
use super::super::PyStreamCacheProcessor;

//region 0 - 1
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "LinearScaleTo0And1Processor")]
#[derive(Debug, Clone)]
pub struct PyLinearScaleTo0And1Processor {
    pub(crate) inner: LinearScaleTo0And1Processor
}

impl From<PyLinearScaleTo0And1Processor> for LinearScaleTo0And1Processor {
    fn from(py_linear_scale_to_0_and_py: PyLinearScaleTo0And1Processor) -> Self {
        py_linear_scale_to_0_and_py.inner
    }
}

#[pymethods]
impl PyLinearScaleTo0And1Processor {
    #[new]
    pub fn new<'py>(py: Python<'py>, lower_bound: f32, upper_bound: f32, initial_value: f32) -> PyResult<Py<Self>> {
        let result = LinearScaleTo0And1Processor::new(lower_bound, upper_bound, initial_value);
        match result {
            Ok(processor) => {
                let child = PyLinearScaleTo0And1Processor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}
//endregion

//region -1 - 1
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "LinearScaleToM1And1Processor")]
#[derive(Debug, Clone)]
pub struct PyLinearScaleToM1And1 {
    pub(crate) inner: LinearScaleToM1And1
}

impl From<PyLinearScaleToM1And1> for LinearScaleToM1And1 {
    fn from(py_linear_scale_to_0_and_py: PyLinearScaleToM1And1) -> Self {
        py_linear_scale_to_0_and_py.inner
    }
}

#[pymethods]
impl PyLinearScaleToM1And1 {
    #[new]
    pub fn new<'py>(py: Python<'py>, lower_bound: f32, upper_bound: f32, initial_value: f32) -> PyResult<Py<Self>> {
        let result = LinearScaleToM1And1::new(lower_bound, upper_bound, initial_value);
        match result {
            Ok(processor) => {
                let child = PyLinearScaleToM1And1 {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}

//endregion