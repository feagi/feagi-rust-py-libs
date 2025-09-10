use std::time::Instant;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_data_structures::wrapped_io_data::WrappedIOData;
use pyo3::exceptions::PyValueError;
use crate::feagi_data_structures::wrapped_io_data::{py_object_to_wrapped_io_data, wrapped_io_data_to_py_object, PyWrappedIOType};


#[pyclass(subclass)]
pub struct PyPipelineStage {
    inner: Box<dyn PipelineStage>,
}

#[pymethods]
impl PyPipelineStage {
    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_input_data_type();
        Ok(result.into())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_output_data_type();
        Ok(result.into())
    }

    pub fn get_most_recent_output(&self) -> PyResult<PyObject> {
        let wrapped_type = self.inner.get_most_recent_output().clone();
        wrapped_io_data_to_py_object(wrapped_type)
    }

    pub fn process_new_input<'py>(&mut self, py: Python<'_>, py_wrapped: PyObject) ->PyResult<PyObject> {
        let wrapped_result = py_object_to_wrapped_io_data(py, py_wrapped);
        if wrapped_result.is_err() {
            return Err(PyErr::new("ERROR"))
        }
        let result = self.inner.process_new_input(&wrapped_result.unwrap(), Instant::now());
        match result {
            Ok(wrapped_io_data) => {
                wrapped_io_data_to_py_object(wrapped_io_data.clone())
            },
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
}

impl PyPipelineStage {

    pub(crate) fn new(inner: Box<dyn PipelineStage>) -> Self {
        PyPipelineStage { inner }
    }
}
