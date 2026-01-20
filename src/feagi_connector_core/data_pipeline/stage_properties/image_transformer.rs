use feagi_sensorimotor::data_pipeline::PipelineStageProperties;
use pyo3::{pyclass, pymethods, PyResult, PyRef, PyRefMut};
use pyo3::prelude::*;
use feagi_sensorimotor::data_pipeline::stage_properties::ImageFrameProcessorStageProperties;
use feagi_sensorimotor::data_types::ImageFrameProcessor;
use crate::create_trait_child_with_box_pyclass;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_data_structures::processing::PyImageFrameProcessor;
use crate::py_error::PyFeagiError;

// TODO: we need to separate out the transformer definition before exposing this stage!

create_trait_child_with_box_pyclass!(PyPipelineStageProperties, PyImageFrameProcessorStageProperties, "ImageFrameProcessorStageProperties", PipelineStageProperties, ImageFrameProcessorStageProperties);

#[pymethods]
impl PyImageFrameProcessorStageProperties {
    #[new]
    pub fn new(transformer_definition: PyImageFrameProcessor) -> PyResult<(Self, PyPipelineStageProperties)> {
        let transformer: ImageFrameProcessor = transformer_definition.into();
        let result_properties = ImageFrameProcessorStageProperties::new_box(transformer);
        Ok(Self::python_new_child_constructor(result_properties))
    }

    #[getter]
    pub fn get_transformer_definition(slf: PyRef<Self>) -> PyResult<PyImageFrameProcessor> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok(parent.transformer_definition.clone().into())
    }

    #[setter]
    pub fn set_transformer_definition(mut slf: PyRefMut<Self>, transformer: PyImageFrameProcessor) -> PyResult<()> {
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.transformer_definition = transformer.into();
        Ok(())
    }
}

