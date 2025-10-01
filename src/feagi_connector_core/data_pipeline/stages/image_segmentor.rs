use feagi_connector_core::data_pipeline::stages::ImageFrameSegmentatorStage;
use feagi_data_structures::data::descriptors::{ImageFrameProperties, SegmentedImageFrameProperties};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_data_structures::processing::ImageFrameSegmentator;
use crate::{common_stage_implementations};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_data_structures::processing::PyImageFrameSegmentator;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameSegmentatorStage")]
#[derive(Clone)]
pub struct PyImageFrameSegmentatorStage;

#[pymethods]
impl PyImageFrameSegmentatorStage {
    #[new]
    pub fn new(input_image_properties: PyImageFrameProperties, output_image_properties: PySegmentedImageFrameProperties, image_segmentator: PyImageFrameSegmentator) -> PyResult<(PyImageFrameSegmentatorStage, PyPipelineStage)> { // TODO remove excess calls
        let input: ImageFrameProperties = input_image_properties.into();
        let output: SegmentedImageFrameProperties = output_image_properties.into();
        let frame_segmentator: ImageFrameSegmentator = image_segmentator.into();

        let result_stage = ImageFrameSegmentatorStage::new(input, output, frame_segmentator);
        Ok((PyImageFrameSegmentatorStage, PyPipelineStage::new(Box::new(result_stage))))
    }
}

common_stage_implementations!(PyImageFrameSegmentatorStage, "ImageFrameQuickDiffStage");
