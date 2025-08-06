use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::{IdentityFloatProcessor, IdentityImageFrameProcessor};
use crate::io_data::PyImageFrame;
use super::super::PyStreamCacheProcessor;

//region Identity Float
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "IdentityFloatProcessor")]
#[derive(Debug, Clone)]
pub struct PyIdentityFloatProcessor {
    pub(crate) inner: IdentityFloatProcessor
}

impl From<PyIdentityFloatProcessor> for IdentityFloatProcessor {
    fn from(py_processor: PyIdentityFloatProcessor) -> Self {
        py_processor.inner
    }
}

#[pymethods]
impl PyIdentityFloatProcessor {
    #[new]
    pub fn new<'py>(py: Python<'py>, initial_value: f32) -> PyResult<Py<Self>> {
        let result = IdentityFloatProcessor::new(initial_value);
        match result {
            Ok(processor) => {
                let child = PyIdentityFloatProcessor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}
//endregion

//region Identity Image Frame
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "IdentityImageFrameProcessor")]
#[derive(Debug, Clone)]
pub struct PyIdentityImageFrameProcessor {
    pub(crate) inner: IdentityImageFrameProcessor
}

impl From<PyIdentityImageFrameProcessor> for IdentityImageFrameProcessor {
    fn from(py_processor: PyIdentityImageFrameProcessor) -> Self {
        py_processor.inner
    }
}

#[pymethods]
impl PyIdentityImageFrameProcessor {
    #[new]
    pub fn new<'py>(py: Python<'py>, initial_image: PyImageFrame) -> PyResult<Py<Self>> {
        let result = IdentityImageFrameProcessor::new(initial_image.into());
        match result {
            Ok(processor) => {
                let child = PyIdentityImageFrameProcessor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}


//endregion


// TODO Segmented Image Frame