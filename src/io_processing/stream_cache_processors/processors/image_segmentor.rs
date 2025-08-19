use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameSegmentatorProcessor;
use crate::io_data::image::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::io_data::image::PyImageFrameSegmentator;
use super::super::PyStreamCacheProcessor;

//region Image Frame Segmentator Processor
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "ImageFrameSegmentatorProcessor")]
#[derive(Debug, Clone)]
pub struct PyImageFrameSegmentatorProcessor {
    pub(crate) inner: ImageFrameSegmentatorProcessor
}

impl From<PyImageFrameSegmentatorProcessor> for ImageFrameSegmentatorProcessor {
    fn from(py_processor: PyImageFrameSegmentatorProcessor) -> Self {
        py_processor.inner
    }
}

#[pymethods]
impl PyImageFrameSegmentatorProcessor {
    #[new]
    pub fn new<'py>(
        py: Python<'py>, 
        input_image_properties: PyImageFrameProperties,
        output_image_properties: PySegmentedImageFrameProperties,
        image_segmentator: PyImageFrameSegmentator
    ) -> PyResult<Py<Self>> {
        let processor = ImageFrameSegmentatorProcessor::new(
            input_image_properties.into(),
            output_image_properties.into(),
            image_segmentator.into()
        );
        
        let child = PyImageFrameSegmentatorProcessor {inner: processor};
        let parent = PyStreamCacheProcessor {};
        let py_obj = Py::new(py, (child, parent))?;
        Ok(py_obj)
    }
}
//endregion
