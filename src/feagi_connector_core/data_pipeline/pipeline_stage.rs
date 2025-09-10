use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::{PyNotImplementedError};
use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_data_structures::FeagiDataError;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_pytrait::PipelineStagePyTrait;
use crate::feagi_data_structures::wrapped_io_data::PyWrappedIOType;

#[pyclass(subclass)]
#[derive(Clone)]
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

impl PipelineStagePyTrait for PyPipelineStage {
    fn copy_as_box(&self) -> Result<Box<dyn PipelineStage>, FeagiDataError> {
        Err(FeagiDataError::InternalError("Cannot call parent class!".to_string()))
    }
}