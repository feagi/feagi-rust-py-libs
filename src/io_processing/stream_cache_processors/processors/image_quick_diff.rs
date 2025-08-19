use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameQuickDiffProcessor;
use crate::io_data::image_descriptors::PyImageFrameProperties;
use super::super::PyStreamCacheProcessor;

//region Image Frame Quick Diff Processor
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "ImageFrameQuickDiffProcessor")]
#[derive(Debug, Clone)]
pub struct PyImageFrameQuickDiffProcessor {
    pub(crate) inner: ImageFrameQuickDiffProcessor
}

impl From<PyImageFrameQuickDiffProcessor> for ImageFrameQuickDiffProcessor {
    fn from(py_processor: PyImageFrameQuickDiffProcessor) -> Self {
        py_processor.inner
    }
}

#[pymethods]
impl PyImageFrameQuickDiffProcessor {
    #[new]
    pub fn new<'py>(
        py: Python<'py>, 
        image_properties: PyImageFrameProperties,
        threshold: f32
    ) -> PyResult<Py<Self>> {
        let result = ImageFrameQuickDiffProcessor::new(image_properties.into(), threshold);
        match result {
            Ok(processor) => {
                let child = PyImageFrameQuickDiffProcessor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}
//endregion
