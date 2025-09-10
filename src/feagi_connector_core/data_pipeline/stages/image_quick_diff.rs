use std::ptr::read;
use std::time::Instant;
use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_connector_core::data_pipeline::stages::ImageFrameQuickDiffStage;
use feagi_data_structures::data::ImageFrame;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::wrapped_io_data::WrappedIOData;
use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use crate::{project_display, py_type_casts};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_pytrait::PipelineStagePyTrait;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;
use crate::feagi_data_structures::data::PyImageFrame;
use crate::feagi_data_structures::wrapped_io_data::PyWrappedIOType;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "ImageFrameQuickDiffStage")]
#[derive(Clone)]
pub struct PyImageFrameQuickDiffStage {
    inner: ImageFrameQuickDiffStage,
}

#[pymethods]
impl PyImageFrameQuickDiffStage {

    #[new]
    pub fn new(image_properties: PyImageFrameProperties, threshold: u8) -> PyResult<(PyImageFrameQuickDiffStage, PyPipelineStage)> {
        Ok((
            PyImageFrameQuickDiffStage {
                inner: ImageFrameQuickDiffStage::new(image_properties.into(), threshold).unwrap()
            },
            PyPipelineStage {}
        ))
    }

    //region PipelineStage

    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        Ok(self.inner.get_input_data_type().into())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        Ok(self.inner.get_output_data_type().into())
    }

    pub fn get_most_recent_output<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        let image: ImageFrame = self.inner.get_most_recent_output().clone().try_into().unwrap();
        let py_image: PyImageFrame = image.into();
        Ok(py_image.into_py(py))
    }

    pub fn process_new_input<'py>(&mut self, py: Python<'py>, new_input: PyObject) -> PyResult<(PyObject)> {
        let image: ImageFrame = PyImageFrame::try_get_from_py_object(py, new_input).unwrap(); // TODO NO
        let result = self.inner.process_new_input(&WrappedIOData::ImageFrame(image), Instant::now()).map_err(|e| PyOSError::new_err(format!("{:?}", e)))?;
        let result_copied = result.clone();
        let image_out: ImageFrame = result_copied.try_into().unwrap();
        let py_image_out: PyImageFrame = image_out.into();
        Ok(py_image_out.into_py(py))
    }


    //endregion

}

impl PipelineStagePyTrait for PyImageFrameQuickDiffStage {
    fn copy_as_box(&self) -> Result<Box<dyn PipelineStage>, FeagiDataError> {
        let stage = self.inner.clone();
        Ok(Box::new(stage))
    }
}

project_display!(PyImageFrameQuickDiffStage);
py_type_casts!(PyImageFrameQuickDiffStage, ImageFrameQuickDiffStage);