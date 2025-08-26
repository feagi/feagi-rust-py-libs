use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::ImageFrameSegmentator;
use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, SegmentedImageFrameProperties, GazeProperties};
use crate::io_data::image::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties, PyGazeProperties};

#[pyclass]
#[pyo3(name = "ImageFrameSegmentator")]
#[derive(Debug, Clone)]
pub struct PyImageFrameSegmentator {
    pub(crate) inner: ImageFrameSegmentator,
}

#[pymethods]
impl PyImageFrameSegmentator {
    #[new]
    pub fn new(
        input_properties: PyImageFrameProperties,
        output_properties: PySegmentedImageFrameProperties,
        initial_gaze: PyGazeProperties
    ) -> PyResult<Self> {
        match ImageFrameSegmentator::new(
            input_properties.into(),
            output_properties.into(),
            initial_gaze.into()
        ) {
            Ok(inner) => Ok(PyImageFrameSegmentator { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn update_gaze(&mut self, gaze: PyGazeProperties) -> PyResult<()> {
        match self.inner.update_gaze(&gaze.into()) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}

impl From<PyImageFrameSegmentator> for ImageFrameSegmentator {
    fn from(py_segmentator: PyImageFrameSegmentator) -> Self {
        py_segmentator.inner
    }
}

impl From<ImageFrameSegmentator> for PyImageFrameSegmentator {
    fn from(segmentator: ImageFrameSegmentator) -> Self {
        PyImageFrameSegmentator { inner: segmentator }
    }
}

