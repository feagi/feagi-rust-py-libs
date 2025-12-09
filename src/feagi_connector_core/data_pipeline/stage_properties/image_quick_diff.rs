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
use crate::py_error::PyFeagiError;

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
    
    #[getter]
    pub fn get_per_pixel_range(slf: PyRef<Self>) -> PyResult<(u8, u8)> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok((*parent.per_pixel_allowed_range.start(), *parent.per_pixel_allowed_range.end()))
    }

    #[getter]
    pub fn get_acceptable_amount_of_activity_in_image(slf: PyRef<Self>) -> PyResult<(PyPercentage, PyPercentage)> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok((parent.acceptable_amount_of_activity_in_image.start().into(), parent.acceptable_amount_of_activity_in_image.end().into()))
    }

    #[setter]
    pub fn set_per_pixel_range(mut slf: PyRefMut<Self>, range: (u8, u8)) -> PyResult<()> {

        if range.0 >= range.1 {
            Err(FeagiDataError::BadParameters("The first (min) parameter of the pixel range must be smaller than the second (max) parameter!".into())).map_err(PyFeagiError::from)?;
        }
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.per_pixel_allowed_range = RangeInclusive::new(range.0, range.1);

        Ok(())
    }

    #[setter]
    pub fn set_acceptable_amount_of_activity_in_image(mut slf: PyRefMut<Self>, range: (PyPercentage, PyPercentage)) -> PyResult<()> {
        if range.0.get_as_0_1() >= range.1.get_as_0_1() {
            Err(FeagiDataError::BadParameters("The first (min) parameter of the acceptable image activity must be smaller than the second (max) parameter!".into())).map_err(PyFeagiError::from)?;
        }
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.acceptable_amount_of_activity_in_image = RangeInclusive::new(range.0.inner, range.1.inner);
        Ok(())
    }


}