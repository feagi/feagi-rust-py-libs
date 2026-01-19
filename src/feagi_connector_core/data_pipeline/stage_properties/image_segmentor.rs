use feagi_sensorimotor::data_pipeline::PipelineStageProperties;
use pyo3::{pymethods, PyResult, PyRef, PyRefMut};
use pyo3::prelude::*;
use feagi_sensorimotor::data_pipeline::stage_properties::ImageFrameSegmentatorStageProperties;
use feagi_sensorimotor::data_types::descriptors::{ImageFrameProperties, SegmentedImageFrameProperties};
use feagi_sensorimotor::data_types::GazeProperties;
use crate::create_trait_child_with_box_pyclass;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data_types::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_connector_core::data_types::PyGazeProperties;
use crate::py_error::PyFeagiError;

create_trait_child_with_box_pyclass!(PyPipelineStageProperties, PyImageFrameSegmentatorStageProperties, "ImageFrameSegmentatorStageProperties", PipelineStageProperties, ImageFrameSegmentatorStageProperties);

#[pymethods]
impl PyImageFrameSegmentatorStageProperties {
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

        Ok(Self::python_new_child_constructor(result_properties))
    }

    /*

    #[getter]
    pub fn get_input_image_properties(slf: PyRef<Self>) -> PyResult<PyImageFrameProperties> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok(parent.input_image_properties.into())
    }

    #[getter]
    pub fn get_output_image_properties(slf: PyRef<Self>) -> PyResult<PySegmentedImageFrameProperties> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok(parent.output_image_properties.into())
    }

     */    #[getter]
    pub fn get_segmentation_gaze(slf: PyRef<Self>) -> PyResult<PyGazeProperties> {
        let parent = Self::get_ref(&slf).map_err(PyFeagiError::from)?;
        Ok(parent.segmentation_gaze.into())
    }    #[setter]
    pub fn set_segmentation_gaze(mut slf: PyRefMut<Self>, gaze: PyGazeProperties) -> PyResult<()> {
        let parent = Self::get_ref_mut(&mut slf).map_err(PyFeagiError::from)?;
        parent.segmentation_gaze = gaze.into();
        Ok(())
    }
}