use feagi_data_structures::wrapped_io_data::WrappedIOType;
use pyo3::{pyclass, pymethods, PyResult,};
use pyo3::prelude::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;
use crate::py_error::PyFeagiError;

#[pyclass(str, extends=PyPipelineStageProperties)]
#[pyo3(name = "IdentityStageProperties")]
#[derive(Clone)]
pub struct PyIdentityStageProperties;

#[pymethods]
impl PyIdentityStageProperties {
    #[new]
    pub fn new(wrapped_type: PyWrappedIOType) -> PyResult<Self> {
        let wrapped_type: WrappedIOType = wrapped_type.into();
        let result_stage: IdentityStageProperties = IdentityStageProperties::new(wrapped_type).map_err(PyFeagiError::from)?;
        Ok((PyIdentityStageProperties, PyPipelineStageProperties::new(Box::new(result_stage))))
    }
}