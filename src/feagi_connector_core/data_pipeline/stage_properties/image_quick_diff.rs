use std::ops::RangeInclusive;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::ImageQuickDiffStageProperties;
use feagi_connector_core::data_types::descriptors::ImageFrameProperties;
use feagi_connector_core::data_types::Percentage;
use crate::create_trait_child_with_box_pyclass;
use crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties;
use crate::feagi_connector_core::data_types::PyPercentage;

create_trait_child_with_box_pyclass!(PyPipelineStageProperties, PyImageQuickDiffStageProperties, "ImageQuickDiffStageProperties", PipelineStageProperties);

#[pymethods]
impl PyImageQuickDiffStageProperties {
    #[new]
    pub fn new(per_pixel_min_val: u8, per_pixel_max_val: u8, acceptable_image_activity_min: PyPercentage, acceptable_image_activity_max: PyPercentage, image_properties: PyImageFrameProperties) -> PyResult<(Self, PyPipelineStageProperties)> {
        let per_pixel_allowed_range = per_pixel_min_val..=per_pixel_max_val;
        let acceptable_image_activity: RangeInclusive<Percentage> = acceptable_image_activity_min.into()..=acceptable_image_activity_max.into();
        let image_properties: ImageFrameProperties = image_properties.into();
        let quick_diff_stage_properties = ImageQuickDiffStageProperties::new_box(per_pixel_allowed_range, acceptable_image_activity, image_properties);
        Ok(Self::python_new_child_constructor(quick_diff_stage_properties))
    }
}

impl PyImageQuickDiffStageProperties {
    fn get_parent_box<'a>(self) -> &'a Box<dyn PipelineStageProperties + Send + Sync> {
        todo!()
    }

    fn get_parent_box_mut<'a>(self) -> &'a mut Box<dyn PipelineStageProperties + Send + Sync> {
        todo!()
    }

    fn get_ref<'a>(self) -> &'a ImageQuickDiffStageProperties {
        let parent_box = self.get_parent_box();
        parent_box.as_any()
    }
    

}