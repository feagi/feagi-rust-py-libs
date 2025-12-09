use pyo3::{pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::*;

use crate::create_trait_parent_with_box_pyclass;
use crate::feagi_connector_core::data_pipeline::stage_properties::*;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;

create_trait_parent_with_box_pyclass!(
    "PipelineStageProperties",
    PyPipelineStageProperties,
    PipelineStageProperties,
    [
        (PyImagePixelValueCountThresholdStageProperties, ImagePixelValueCountThresholdStageProperties),
        (PyImageQuickDiffStageProperties, ImageQuickDiffStageProperties),
        (PyImageFrameSegmentatorStageProperties, ImageFrameSegmentatorStageProperties),
    ]
);

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
