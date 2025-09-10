use feagi_data_structures::processing::{ImageFrameSegmentator};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::feagi_data_structures::data::image_descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};

#[pyclass]
#[pyo3(name = "ImageFrameSegmentator")]
#[derive(Clone)]
pub struct PyImageFrameSegmentator {
    inner: ImageFrameSegmentator
}

#[pymethods]
impl PyImageFrameSegmentator {

    // TODO other methods

    #[new]
    pub fn new(input_properties: PyImageFrameProperties, output_properties: PySegmentedImageFrameProperties, initial_gaze: PyGazeProperties) -> PyResult<Self> {
        let result = ImageFrameSegmentator::new(input_properties.into(), output_properties.into(), initial_gaze.into());
        match result {
            Ok(inner) => Ok(PyImageFrameSegmentator { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}