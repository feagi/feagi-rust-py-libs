use std::ops::RangeInclusive;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use pyo3::{pyclass, pymethods, PyResult, PyRef, PyRefMut};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::stage_properties::ImagePixelValueCountThresholdStageProperties;
use feagi_connector_core::data_types::descriptors::ImageFrameProperties;
use feagi_connector_core::data_types::Percentage;
use crate::create_trait_child_with_box_pyclass;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties;
use crate::feagi_connector_core::data_types::PyPercentage;
use crate::py_error::PyFeagiError;

create_trait_child_with_box_pyclass!(PyPipelineStageProperties, PyImagePixelValueCountThresholdStageProperties, "ImagePixelValueCountThresholdStageProperties", PipelineStageProperties, ImagePixelValueCountThresholdStageProperties);

#[pymethods]
impl PyImagePixelValueCountThresholdStageProperties {
    #[new]
    pub fn new(
        input_definition: PyImageFrameProperties,
        pixel_range_min: u8,
        pixel_range_max: u8,
        acceptable_activity_min: PyPercentage,
        acceptable_activity_max: PyPercentage
    ) -> PyResult<(Self, PyPipelineStageProperties)> {
        let input_props: ImageFrameProperties = input_definition.into();
        let inclusive_pixel_range = pixel_range_min..=pixel_range_max;
        let acceptable_activity: RangeInclusive<Percentage> = acceptable_activity_min.into()..=acceptable_activity_max.into();
        
        let result_properties = ImagePixelValueCountThresholdStageProperties::new_box(
            input_props,
            inclusive_pixel_range,
            acceptable_activity
        );
        Ok(Self::python_new_child_constructor(result_properties))
    }

    #[getter]
    pub fn get_inclusive_pixel_range(slf: PyRef<Self>) -> PyResult<(u8, u8)> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok((*parent.inclusive_pixel_range.start(), *parent.inclusive_pixel_range.end()))
    }

    #[getter]
    pub fn get_acceptable_amount_of_activity_in_image(slf: PyRef<Self>) -> PyResult<(PyPercentage, PyPercentage)> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok((
            parent.acceptable_amount_of_activity_in_image.start().into(),
            parent.acceptable_amount_of_activity_in_image.end().into()
        ))
    }

    #[setter]
    pub fn set_inclusive_pixel_range(mut slf: PyRefMut<Self>, range: (u8, u8)) -> PyResult<()> {
        if range.0 >= range.1 {
            Err(feagi_data_structures::FeagiDataError::BadParameters(
                "The first (min) parameter of the pixel range must be smaller than the second (max) parameter!".into()
            )).map_err(PyFeagiError::from)?;
        }
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.inclusive_pixel_range = RangeInclusive::new(range.0, range.1);
        Ok(())
    }

    #[setter]
    pub fn set_acceptable_amount_of_activity_in_image(mut slf: PyRefMut<Self>, range: (PyPercentage, PyPercentage)) -> PyResult<()> {
        if range.0.get_as_0_1() >= range.1.get_as_0_1() {
            Err(feagi_data_structures::FeagiDataError::BadParameters(
                "The first (min) parameter of the acceptable image activity must be smaller than the second (max) parameter!".into()
            )).map_err(PyFeagiError::from)?;
        }
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.acceptable_amount_of_activity_in_image = RangeInclusive::new(range.0.inner, range.1.inner);
        Ok(())
    }
}

