use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::IdentityLinearFloatCacheProcessor;
use crate::io_data::try_wrap_as_io_type_data;
use super::PyStreamCacheProcessor;

#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "IdentityLinearFloatCacheProcessor")]
#[derive(Clone)]
pub struct PyIdentityLinearFloatCacheProcessor {
    pub(crate) inner: IdentityLinearFloatCacheProcessor
}

#[pymethods]
impl PyIdentityLinearFloatCacheProcessor {
    #[new]
    pub fn new<'py>(py: Python<'py>, initial_value: PyObject) -> PyResult<Py<Self>> {
        let result = try_wrap_as_io_type_data(py, initial_value);
        match result {
            Ok(inner) => {
                let child = PyIdentityLinearFloatCacheProcessor {inner: IdentityLinearFloatCacheProcessor::new(inner).unwrap()};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}