use feagi_data_structures::processing::ImageFrameProcessor;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;

#[pyclass]
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