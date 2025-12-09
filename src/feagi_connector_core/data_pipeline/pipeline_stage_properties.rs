use pyo3::{pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::*;
use feagi_data_structures::FeagiDataError;

use crate::{create_trait_parent_with_box_pyclass};
use crate::feagi_connector_core::data_pipeline::stage_properties::*;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;
use crate::py_error::PyFeagiError;

create_trait_parent_with_box_pyclass!("PipelineStageProperties", PyPipelineStageProperties, PipelineStageProperties);


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
    /// Converts a boxed trait object to the correct Python child class.
    /// This is needed when returning stage properties from Rust to Python,
    /// ensuring the correct subclass type is returned rather than just the parent.
    pub(crate) fn from_box_to_correct_child(py: Python<'_>, boxed: Box<dyn PipelineStageProperties + Send + Sync>) -> PyResult<Py<PyAny>> {
        // Try ImagePixelValueCountThresholdStageProperties
        if boxed.as_any().downcast_ref::<ImagePixelValueCountThresholdStageProperties>().is_some() {
            let (child, parent) = PyImagePixelValueCountThresholdStageProperties::python_new_child_constructor(boxed);
            return Py::new(py, (child, parent)).map(|obj| obj.into_any());
        }
        
        // Try ImageQuickDiffStageProperties
        if boxed.as_any().downcast_ref::<ImageQuickDiffStageProperties>().is_some() {
            let (child, parent) = PyImageQuickDiffStageProperties::python_new_child_constructor(boxed);
            return Py::new(py, (child, parent)).map(|obj| obj.into_any());
        }
        
        // Try ImageFrameSegmentatorStageProperties
        if boxed.as_any().downcast_ref::<ImageFrameSegmentatorStageProperties>().is_some() {
            let (child, parent) = PyImageFrameSegmentatorStageProperties::python_new_child_constructor(boxed);
            return Py::new(py, (child, parent)).map(|obj| obj.into_any());
        }
        
        // No matching type found
        Err(FeagiDataError::InternalError(
            "Missing Definition for PyPipelineStageProperties - unknown concrete type!".into()
        )).map_err(PyFeagiError::from)?
    }
}
