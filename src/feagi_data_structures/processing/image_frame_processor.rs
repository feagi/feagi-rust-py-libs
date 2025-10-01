use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::processing::ImageFrameProcessor;
use crate::feagi_data_structures::data::descriptors::PyImageFrameProperties;
use crate::{project_display, py_object_cast_generic, py_type_casts};

#[pyclass(str)]
#[pyo3(name = "ImageFrameProcessor")]
#[derive(Clone)]
pub struct PyImageFrameProcessor {
    inner: ImageFrameProcessor
}

#[pymethods]
impl PyImageFrameProcessor {

    // TODO other methods

    #[new]
    pub fn new(input_image_properties: PyImageFrameProperties) -> PyResult<Self> {
        Ok(ImageFrameProcessor::new(input_image_properties.into()).into())
    }


    #[staticmethod]
    pub fn new_from_input_output_properties(input: PyImageFrameProperties, output: PyImageFrameProperties) -> PyResult<Self> {
        let result = ImageFrameProcessor::new_from_input_output_properties(&input.into(), &output.into());
        match result {
            Ok(inner) => Ok(PyImageFrameProcessor { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    //region Set Settings

    pub fn set_brightness_offset(&mut self, brightness_offset: i32) -> PyResult<()> {
        self.inner.set_brightness_offset(brightness_offset);
        Ok(())
    }

    pub fn set_contrast_change(&mut self, contrast_change: f32) -> PyResult<()> {
        self.inner.set_contrast_change(contrast_change);
        Ok(())
    }


    //endregion


}

py_type_casts!(PyImageFrameProcessor, ImageFrameProcessor);
py_object_cast_generic!(PyImageFrameProcessor, ImageFrameProcessor, "Unable to retrieve ImageFrameProcessor data from given!");
project_display!(PyImageFrameProcessor);
