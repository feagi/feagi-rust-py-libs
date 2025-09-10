use feagi_connector_core::data_pipeline::stages::*;
use pyo3::{pyclass, pymethods, PyResult,};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::{project_display, py_type_casts};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::{PyImageFrame, PySegmentedImageFrame};

//region Identity Float
#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "IdentityFloatStage")]
#[derive(Clone)]
pub struct PyIdentityFloatStage;

#[pymethods]
impl PyIdentityFloatStage {

    #[new]
    pub fn new(initial_value: f32) -> PyResult<(Self, PyPipelineStage)> {
        let result_stage = IdentityFloatStage::new(initial_value);
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyIdentityFloatStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

project_display!(PyIdentityFloatStage);
py_type_casts!(PyIdentityFloatStage, IdentityFloatStage);

//endregion


//region Identity Image Frame
#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "IdentityImageFrameStage")]
#[derive(Clone)]
pub struct PyIdentityImageFrameStage;

#[pymethods]
impl PyIdentityImageFrameStage {

    #[new]
    pub fn new(initial_image: PyImageFrame) -> PyResult<(Self, PyPipelineStage)> {
        let result_stage = IdentityImageFrameStage::new(initial_image.into());
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyIdentityImageFrameStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

project_display!(PyIdentityImageFrameStage);
py_type_casts!(PyIdentityImageFrameStage, IdentityImageFrameStage);

//endregion


//region Identity Segmented Image Frame
#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "IdentitySegmentedImageFrameStage")]
#[derive(Clone)]
pub struct PyIdentitySegmentedImageFrameStage;

#[pymethods]
impl PyIdentitySegmentedImageFrameStage {

    #[new]
    pub fn new(initial_images: PySegmentedImageFrame) -> PyResult<(Self, PyPipelineStage)> {
        let result_stage = IdentitySegmentedImageFrameStage::new(initial_images.into());
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyIdentitySegmentedImageFrameStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

project_display!(PyIdentitySegmentedImageFrameStage);
py_type_casts!(PyIdentitySegmentedImageFrameStage, IdentitySegmentedImageFrameStage);

//endregion