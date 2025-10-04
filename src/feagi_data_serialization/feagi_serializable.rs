use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;

#[pyclass(subclass)]
#[pyo3(name = "FeagiSerializable")]
pub struct PyFeagiSerializable {}

#[pymethods]
impl PyFeagiSerializable {
    
}

impl PyFeagiSerializable {
    pub(crate) fn new() -> Self {
        PyFeagiSerializable {}
    }
}