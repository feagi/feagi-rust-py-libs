use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use feagi_data_structures::FeagiDataError;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;
use crate::py_object_cast_generic_no_unwrap;

#[pyclass(subclass)]
pub struct PyPipelineStageProperties {
    inner: Box<dyn PipelineStageProperties>,
}

#[pymethods]
impl PyPipelineStageProperties {
    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_input_data_type();
        Ok(result.into())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_output_data_type();
        Ok(result.into())
    }
}



impl PyPipelineStageProperties {
    pub(crate) fn new(inner: Box<dyn PipelineStageProperties>) -> Self {
        PyPipelineStageProperties { inner}
    }
}


impl From<Box<dyn PipelineStageProperties + Sync + Send>> for PyPipelineStageProperties {
    fn from(inner: Box<dyn PipelineStageProperties + Sync + Send>) -> Self {
        PyPipelineStageProperties { inner }
    }
}


pub fn extract_pipeline_stage_properties_from_py(py: Python, py_stage: Py<PyPipelineStageProperties>) -> Result<Box<dyn PipelineStageProperties>, FeagiDataError> {
    let stage_ref = py_stage.borrow(py);
    Ok(stage_ref.inner.clone_box())
}

pub fn wrap_pipeline_stage_for_py(py: Python, stage: Box<dyn PipelineStageProperties>) -> PyResult<Py<PyPipelineStageProperties>> {
    Py::new(py, PyPipelineStageProperties::new(stage))
}