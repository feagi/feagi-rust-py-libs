use feagi_connector_core::data_pipeline::stages::{ImageFrameProcessorStage, ImageFrameSegmentatorStage};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_data_structures::processing::{ImageFrameProcessor, ImageFrameSegmentator};
use pyo3::exceptions::PyValueError;
use crate::{common_stage_implementations};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::image_descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_data_structures::processing::{PyImageFrameProcessor, PyImageFrameSegmentator};

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameProcessorStage")]
#[derive(Clone)]
pub struct PyImageFrameProcessorStage;

#[pymethods]
impl PyImageFrameProcessorStage {
    #[new]
    pub fn new(transformer_definition: PyImageFrameProcessor) -> PyResult<(PyImageFrameProcessorStage, PyPipelineStage)> { // TODO remove excess calls
        let input: ImageFrameProcessor = transformer_definition.into();

        let result_stage = ImageFrameProcessorStage::new(input);
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyImageFrameProcessorStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

common_stage_implementations!(PyImageFrameProcessorStage, "ImageFrameProcessorStage");