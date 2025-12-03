use std::ops::RangeInclusive;
use pyo3::{pyclass, pymethods, PyResult,};
use pyo3::prelude::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::ImageQuickDiffStageProperties;
use feagi_connector_core::data_types::descriptors::ImageFrameProperties;
use feagi_connector_core::data_types::Percentage;
use crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties;
use crate::feagi_connector_core::data_types::PyPercentage;

#[pyclass(extends=PyPipelineStageProperties)]
#[pyo3(name = "ImageQuickDiffStageProperties")]
#[derive(Debug, Clone)]
pub struct PyImageQuickDiffStageProperties;

#[pymethods]
impl PyImageQuickDiffStageProperties {
    #[new]
    pub fn new(per_pixel_min_val: u8, per_pixel_max_val: u8, acceptable_image_activity_min: PyPercentage, acceptable_image_activity_max: PyPercentage, image_properties: PyImageFrameProperties) -> PyResult<(Self, PyPipelineStageProperties)> {

        let per_pixel_allowed_range = per_pixel_min_val..=per_pixel_max_val;
        let acceptable_image_activity: RangeInclusive<Percentage> = acceptable_image_activity_min.into()..=acceptable_image_activity_max.into();
        let image_properties: ImageFrameProperties = image_properties.into();
        let quick_diff_stage_properties = ImageQuickDiffStageProperties::new_box(per_pixel_allowed_range, acceptable_image_activity, image_properties);
        Ok((PyImageQuickDiffStageProperties, PyPipelineStageProperties::new(quick_diff_stage_properties)))
    }
}