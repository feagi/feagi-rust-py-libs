use std::ops::{Range, RangeInclusive};
use feagi_connector_core::data_pipeline::stages::ImageFrameQuickDiffStage;
use feagi_data_structures::data::descriptors::{ImageFrameProperties};
use feagi_data_structures::data::Percentage;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::{common_stage_implementations};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameQuickDiffStage")]
#[derive(Clone)]
pub struct PyImageFrameQuickDiffStage;

#[pymethods]
impl PyImageFrameQuickDiffStage {
    #[new]
    pub fn new(image_properties: PyImageFrameProperties, per_pixel_allowed_range: (u8, u8), acceptable_amount_of_activity_percentage_in_image:(f32, f32)) -> PyResult<(PyImageFrameQuickDiffStage, PyPipelineStage)> {
        let per_pixel_allowed_range:RangeInclusive<u8> = per_pixel_allowed_range.0..=per_pixel_allowed_range.1;

        let acceptable_amount_of_activity_in_image = RangeInclusive::new(
            Percentage::new_from_0_1(acceptable_amount_of_activity_percentage_in_image.0).unwrap(),
            Percentage::new_from_0_1(acceptable_amount_of_activity_percentage_in_image.1).unwrap(),
        );
        let image_properties: ImageFrameProperties = image_properties.into();
        let result_stage = ImageFrameQuickDiffStage::new(image_properties, per_pixel_allowed_range, acceptable_amount_of_activity_in_image);
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyImageFrameQuickDiffStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

common_stage_implementations!(PyImageFrameQuickDiffStage, "ImageFrameQuickDiffStage");
