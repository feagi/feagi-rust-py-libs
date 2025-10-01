use feagi_data_structures::FeagiDataError;
use feagi_data_structures::processing::{ImageFrameSegmentator};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyValueError};
use pyo3::prelude::*;
use crate::feagi_data_structures::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::{project_display, py_object_cast_generic, py_type_casts};

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

py_type_casts!(PyImageFrameSegmentator, ImageFrameSegmentator);
py_object_cast_generic!(PyImageFrameSegmentator, ImageFrameSegmentator, "Unable to retrieve ImageFrameSegmentator data from given!");
//project_display!(PyImageFrameSegmentator);