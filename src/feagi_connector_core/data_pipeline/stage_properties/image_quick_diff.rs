use std::ops::RangeInclusive;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use pyo3::{pyclass, pymethods, PyResult, PyRef, PyRefMut};
use pyo3::prelude::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::ImageQuickDiffStageProperties;
use feagi_connector_core::data_types::descriptors::ImageFrameProperties;
use feagi_connector_core::data_types::Percentage;
use feagi_data_structures::FeagiDataError;
use crate::create_trait_child_with_box_pyclass;
use crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties;
use crate::feagi_connector_core::data_types::PyPercentage;

create_trait_child_with_box_pyclass!(PyPipelineStageProperties, PyImageQuickDiffStageProperties, "ImageQuickDiffStageProperties", PipelineStageProperties, ImageQuickDiffStageProperties);

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
    
    fn test<'a>(slf: &'a PyRef<'_, Self>) {
        let a = Self::get_parent_box()
    }
    
    /*
    fn get_parent_box<'a>(slf: &'a PyRef<'_, Self>) -> &'a Box<dyn PipelineStageProperties + Send + Sync> {
        let parent: &PyPipelineStageProperties = slf.as_ref();
        &parent.inner
    }

    fn get_parent_box_mut<'a>(slf: &'a mut PyRefMut<'_, Self>) -> &'a mut Box<dyn PipelineStageProperties + Send + Sync> {
        let parent: &mut PyPipelineStageProperties = slf.as_mut();
        &mut parent.inner
    }

    fn get_ref<'a>(slf: &'a PyRef<'_, Self>) -> Result<&'a ImageQuickDiffStageProperties, FeagiDataError> {
        let parent_box = Self::get_parent_box(slf);
        parent_box.as_any().downcast_ref::<ImageQuickDiffStageProperties>()
            
    }

    fn get_ref_mut<'a>(slf: &'a mut PyRefMut<'_, Self>) -> Result<&'a mut ImageQuickDiffStageProperties, FeagiDataError> {
        let parent_box = Self::get_parent_box_mut(slf);
        parent_box.as_any_mut().downcast_mut::<ImageQuickDiffStageProperties>()
            .ok_or_else(|| FeagiDataError::InternalError("Type mismatch: expected ImageQuickDiffStageProperties".into()))
    }
    
     */
}