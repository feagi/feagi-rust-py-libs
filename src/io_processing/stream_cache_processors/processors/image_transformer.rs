use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameTransformerProcessor;
use crate::io_data::PyImageFrameTransformer;
use super::super::PyStreamCacheProcessor;

//region Image Frame Transformer Processor
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "ImageFrameTransformerProcessor")]
#[derive(Debug, Clone)]
pub struct PyImageFrameTransformerProcessor {
    pub(crate) inner: ImageFrameTransformerProcessor
}

impl From<PyImageFrameTransformerProcessor> for ImageFrameTransformerProcessor {
    fn from(py_processor: PyImageFrameTransformerProcessor) -> Self {
        py_processor.inner
    }
}

#[pymethods]
impl PyImageFrameTransformerProcessor {
    #[new]
    pub fn new<'py>(py: Python<'py>, transformer_definition: PyImageFrameTransformer) -> PyResult<Py<Self>> {
        let result = ImageFrameTransformerProcessor::new(transformer_definition.into());
        match result {
            Ok(processor) => {
                let child = PyImageFrameTransformerProcessor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}
//endregion

