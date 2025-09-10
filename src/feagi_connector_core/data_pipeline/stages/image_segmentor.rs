use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use feagi_data_structures::data::image_descriptors::ImageFrameProperties;
use crate::{common_stage_implementations};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameSegmentatorStage")]
#[derive(Clone)]
pub struct PyImageFrameSegmentatorStage;

#[pymethods]
impl PyImageFrameSegmentatorStage {
    #[new]
    pub fn new(image_segmentor: PyImageSeg) -> PyResult<(PyImageFrameSegmentatorStage, PyPipelineStage)> {
        let image_properties: ImageFrameProperties = image_properties.into();
        let result_stage = ImageFrameQuickDiffStage::new(image_properties, threshold);
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyImageFrameSegmentatorStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

common_stage_implementations!(PyImageFrameSegmentatorStage, "ImageFrameQuickDiffStage");
