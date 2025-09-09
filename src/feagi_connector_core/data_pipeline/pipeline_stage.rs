use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_connector_core::data_pipeline::PipelineStage;

#[pyclass(subclass)]
#[pyo3(name = "PipelineStage")]
pub struct PyPipelineStage {}

#[pymethods]
impl PyPipelineStage {

    pub fn get_input_data_type(&self) -> PyWrappedIOType {
        
    }
    
    
}