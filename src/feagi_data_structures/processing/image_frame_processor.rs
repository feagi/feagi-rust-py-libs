use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::processing::ImageFrameProcessor;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;
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

    #[staticmethod]
    pub fn new_from_input_output_properties(input: PyImageFrameProperties, output: PyImageFrameProperties) -> PyResult<Self> {
        let result = ImageFrameProcessor::new_from_input_output_properties(&input.into(), &output.into());
        match result {
            Ok(inner) => Ok(PyImageFrameProcessor { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}

py_type_casts!(PyImageFrameProcessor, ImageFrameProcessor);
py_object_cast_generic!(PyImageFrameProcessor, ImageFrameProcessor, "Unable to retrieve ImageFrameProcessor data from given!");
project_display!(PyImageFrameProcessor);
