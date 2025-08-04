use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::processors::{LinearAverageRollingWindowProcessor, LinearScaleToM1And1};
use super::super::PyStreamCacheProcessor;

//region linear rolling window
#[pyclass(extends=PyStreamCacheProcessor)]
#[pyo3(name = "LinearAverageRollingWindowProcessor")]
#[derive(Debug, Clone)]
pub struct PyLinearAverageRollingWindowProcessor {
    pub(crate) inner: LinearAverageRollingWindowProcessor
}

impl From<PyLinearAverageRollingWindowProcessor> for LinearAverageRollingWindowProcessor {
    fn from(py: PyLinearAverageRollingWindowProcessor) -> Self {
        py.inner
    }
}

#[pymethods]
impl PyLinearAverageRollingWindowProcessor {
    #[new]
    pub fn new<'py>(py: Python<'py>, window_length: usize, initial_value: f32) -> PyResult<Py<Self>> {
        let result = LinearAverageRollingWindowProcessor::new(window_length, initial_value);
        match result {
            Ok(processor) => {
                let child = PyLinearAverageRollingWindowProcessor {inner: processor};
                let parent = PyStreamCacheProcessor {};
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj)
            }
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}
//endregion