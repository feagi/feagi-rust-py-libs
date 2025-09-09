use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::{PyNotImplementedError};
use feagi_connector_core::data_pipeline::PipelineStage;
use crate::feagi_data_structures::wrapped_io_data::PyWrappedIOType;

#[pyclass(subclass)]
#[pyo3(name = "PipelineStage")]
pub struct PyPipelineStage {}

#[pymethods]
impl PyPipelineStage {

    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }

    pub fn get_most_recent_output(&self) -> PyResult<PyObject> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }

    pub fn process_new_input(&self, new_input: PyObject) -> PyResult<(PyObject)> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }

    // NOTE: Do not implement clone_box

    #[new]
    pub(crate) fn new() -> Self {
        PyPipelineStage {}
    }
}