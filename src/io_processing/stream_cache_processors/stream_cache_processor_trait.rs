use pyo3::prelude::*;

// This is a trait in the core lib, exposed here as a base class

#[pyclass(subclass)]
#[pyo3(name = "StreamCacheProcessor")]
pub struct PyStreamCacheProcessor {
}

#[pymethods]
impl PyStreamCacheProcessor {
}
