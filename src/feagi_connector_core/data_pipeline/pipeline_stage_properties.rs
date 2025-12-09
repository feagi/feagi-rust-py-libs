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
