use feagi_core_data_structures_and_processing::io_processing::StreamCacheProcessor;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::io_processing::processors::{PyIdentityFloatProcessor, PyIdentityImageFrameProcessor, PyLinearAverageRollingWindowProcessor, PyLinearScaleTo0And1Processor, PyLinearScaleToM1And1};
// This is a trait in the core lib, exposed here as a base class

#[pyclass(subclass)]
#[pyo3(name = "StreamCacheProcessor")]
pub struct PyStreamCacheProcessor {
}

#[pymethods]
impl PyStreamCacheProcessor {
}

// TODO move away from ifs
// Helper function to extract the inner StreamCacheProcessor from Python wrapper
pub(crate) fn extract_stream_cache_processor<'py>(
    py: Python<'py>,
    py_processor: Py<PyStreamCacheProcessor>
) -> PyResult<Box<dyn StreamCacheProcessor + Sync + Send>> {
    let bound = py_processor.bind(py);

    // Try to downcast to each concrete processor type and extract the inner implementation

    // Try PyIdentityFloatProcessor
    if let Ok(identity_float) = bound.downcast::<PyIdentityFloatProcessor>() {
        let processor = identity_float.borrow().inner.clone();
        return Ok(Box::new(processor));
    }

    // Try PyIdentityImageFrameProcessor
    if let Ok(identity_image) = bound.downcast::<PyIdentityImageFrameProcessor>() {
        let processor = identity_image.borrow().inner.clone();
        return Ok(Box::new(processor));
    }

    // Try PyLinearAverageRollingWindowProcessor
    if let Ok(rolling_window) = bound.downcast::<PyLinearAverageRollingWindowProcessor>() {
        let processor = rolling_window.borrow().inner.clone();
        return Ok(Box::new(processor));
    }

    // Try PyLinearScaleTo0And1
    if let Ok(scale_0_1) = bound.downcast::<PyLinearScaleTo0And1Processor>() {
        let processor = scale_0_1.borrow().inner.clone();
        return Ok(Box::new(processor));
    }

    // Try PyLinearScaleToM1And1
    if let Ok(scale_m1_1) = bound.downcast::<PyLinearScaleToM1And1>() {
        let processor = scale_m1_1.borrow().inner.clone();
        return Ok(Box::new(processor));
    }

    // If none of the downcasts succeeded, return an error
    Err(PyValueError::new_err("Unknown StreamCacheProcessor type"))
}