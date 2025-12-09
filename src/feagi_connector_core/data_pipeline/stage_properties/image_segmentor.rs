use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::stage_properties::ImageFrameSegmentatorStageProperties;
use feagi_connector_core::data_types::descriptors::{ImageFrameProperties, SegmentedImageFrameProperties};
use feagi_connector_core::data_types::GazeProperties;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data_types::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_connector_core::data_types::PyGazeProperties;
use crate::py_error::PyFeagiError;

#[pyclass(extends=PyPipelineStageProperties)]
#[pyo3(name = "ImageSegmentorStageProperties")]
#[derive(Clone)]
pub struct PyImageSegmentorStageProperties;

#[pymethods]
impl PyImageSegmentorStageProperties {
    #[new]
    pub fn new(
        input_image_properties: PyImageFrameProperties,
        output_image_properties: PySegmentedImageFrameProperties,
        initial_gaze: PyGazeProperties
    ) -> PyResult<(Self, PyPipelineStageProperties)> {
        let input_properties: ImageFrameProperties = input_image_properties.into();
        let output_properties: SegmentedImageFrameProperties = output_image_properties.into();
        let gaze: GazeProperties = initial_gaze.into();
        
        let result_properties = ImageFrameSegmentatorStageProperties::new_box(
            input_properties,
            output_properties,
            gaze
        );
        
        Ok((PyImageSegmentorStageProperties, PyPipelineStageProperties::new(result_properties)))
    }
}


