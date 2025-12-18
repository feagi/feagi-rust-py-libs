use pyo3::{pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;

/// PyO3 wrapper for PipelineStageProperties enum
/// 
/// Note: PipelineStageProperties changed from trait-based to enum-based in json_config merge.
/// Individual stage property wrappers are temporarily disabled until they can be properly
/// reimplemented to work with the new enum structure.
#[pyclass(name = "PipelineStageProperties")]
#[derive(Clone)]
pub struct PyPipelineStageProperties {
    pub(crate) inner: PipelineStageProperties,
}

#[pymethods]
impl PyPipelineStageProperties {
    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_input_data_type();
        Ok(result.into())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_output_data_type();
        Ok(result.into())
    }
    
    pub fn variant_name(&self) -> PyResult<String> {
        Ok(self.inner.variant_name().to_string())
    }
}

impl From<PipelineStageProperties> for PyPipelineStageProperties {
    fn from(inner: PipelineStageProperties) -> Self {
        Self { inner }
    }
}

impl From<PyPipelineStageProperties> for PipelineStageProperties {
    fn from(val: PyPipelineStageProperties) -> Self {
        val.inner
    }
}

impl PyPipelineStageProperties {
    /// Convert a single Python PyPipelineStageProperties to Rust PipelineStageProperties
    pub fn from_py_to_box(py: Python<'_>, py_stage: &Py<PyPipelineStageProperties>) -> pyo3::PyResult<PipelineStageProperties> {
        let stage = py_stage.borrow(py);
        Ok(stage.inner.clone())
    }
    
    /// Convert a vector of Python PyPipelineStageProperties to Rust PipelineStageProperties
    pub fn from_vec_py_to_vec(py_stages: Vec<Py<PyPipelineStageProperties>>) -> pyo3::PyResult<Vec<PipelineStageProperties>> {
        Python::with_gil(|py| {
            py_stages.into_iter()
                .map(|py_stage| {
                    let stage = py_stage.borrow(py);
                    Ok(stage.inner.clone())
                })
                .collect()
        })
    }
    
    /// Convert Rust PipelineStageProperties enum to Python wrapper (for compatibility with old API)
    pub fn from_box_to_parent_typed(py: Python<'_>, stage: PipelineStageProperties) -> PyResult<Py<PyPipelineStageProperties>> {
        Py::new(py, PyPipelineStageProperties { inner: stage })
    }
    
    /// Convert vector of Rust PipelineStageProperties to vector of Python wrappers (for compatibility with old API)
    pub fn from_vec_box_to_vec_parent_typed(py: Python<'_>, stages: Vec<PipelineStageProperties>) -> PyResult<Vec<Py<PyPipelineStageProperties>>> {
        stages.into_iter()
            .map(|stage| Py::new(py, PyPipelineStageProperties { inner: stage }))
            .collect()
    }
}


