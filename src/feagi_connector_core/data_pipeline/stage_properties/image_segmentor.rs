use feagi_connector_core::data_pipeline::stage_properties::ImageSegmentorStageProperties;
use feagi_connector_core::data_types::descriptors::{GazeProperties, ImageFrameProperties, SegmentedImageFrameProperties};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
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
        
        let result_properties = ImageSegmentorStageProperties::new_box(
            input_properties,
            output_properties,
            gaze
        ).map_err(PyFeagiError::from)?;
        
        Ok((PyImageSegmentorStageProperties, PyPipelineStageProperties::new(result_properties)))
    }
}


