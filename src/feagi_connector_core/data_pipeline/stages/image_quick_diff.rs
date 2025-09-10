use feagi_connector_core::data_pipeline::stages::ImageFrameQuickDiffStage;
use feagi_data_structures::data::image_descriptors::ImageFrameProperties;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::{project_display, py_type_casts};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameQuickDiffStage")]
#[derive(Clone)]
pub struct PyImageFrameQuickDiffStage;

#[pymethods]
impl PyImageFrameQuickDiffStage {
    #[new]
    pub fn new(image_properties: PyImageFrameProperties, threshold: u8) -> PyResult<(PyImageFrameQuickDiffStage, PyPipelineStage)> {
        let image_properties: ImageFrameProperties = image_properties.into();
        let result_stage = ImageFrameQuickDiffStage::new(image_properties, threshold);
        if result_stage.is_err() {
            return Err(PyValueError::new_err(format!("{:?}", result_stage.err().unwrap())))
        }
        Ok((PyImageFrameQuickDiffStage, PyPipelineStage::new(Box::new(result_stage.unwrap()))))
    }
}

project_display!(PyImageFrameQuickDiffStage);
py_type_casts!(PyImageFrameQuickDiffStage, ImageFrameQuickDiffStage);