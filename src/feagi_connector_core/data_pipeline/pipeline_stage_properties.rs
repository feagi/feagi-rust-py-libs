use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::{ImageQuickDiffStageProperties, ImageFrameSegmentatorStageProperties};
use feagi_data_structures::FeagiDataError;
use pyo3::exceptions::PyValueError;
use crate::{create_trait_parent_pyclass, create_trait_parent_with_box_pyclass};
use crate::feagi_connector_core::data_pipeline::stage_properties::{PyImageQuickDiffStageProperties, PyImageSegmentorStageProperties};
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;


create_trait_parent_with_box_pyclass!("PipelineStageProperties", PyPipelineStageProperties, PipelineStageProperties);

/*
#[pyo3::pyclass(str, subclass)]
#[pyo3(name = "PipelineStageProperties2")]
#[derive(Debug)]
pub struct PyPipelineStageProperties2 {
    pub inner: Box<dyn PipelineStageProperties + Send + Sync>,
}
impl Clone for PyPipelineStageProperties2 {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}
impl PyPipelineStageProperties2 {
    pub(crate) fn new_parent(boxed: Box<dyn PipelineStageProperties + Send + Sync>) -> Self {
        PyPipelineStageProperties2 {
            inner: boxed,
        }
    }

    pub(crate) fn py_any_to_box<'py>(_py: Python<'_>, py_any: &pyo3::Bound<'py, pyo3::PyAny>) -> Result<Box<dyn PipelineStageProperties + Send + Sync>, feagi_data_structures::FeagiDataError> {
        if let Ok(reference) = py_any.cast::<( PyPipelineStageProperties2 )>() {
            return Ok(reference.borrow().inner.clone_box());
        }
        Err(FeagiDataError::BadParameters(format!("Unable to parse object as any child of {}!", "PipelineStageProperties2")))
    }
}
impl std::fmt::Display for PyPipelineStageProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", "PipelineStageProperties2")
    }
}

 */


// TODO we need to update this file!

/*
#[pyclass(subclass)]
#[derive(Debug)]
pub struct
PyPipelineStageProperties {
    inner: Box<dyn PipelineStageProperties + Send + Sync>,
}

 */

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

impl From<PyPipelineStageProperties> for Box<dyn PipelineStageProperties + Send + Sync> {
    fn from(inner: PyPipelineStageProperties) -> Self {
        inner.inner
    }
}

impl PyPipelineStageProperties {
    


}
