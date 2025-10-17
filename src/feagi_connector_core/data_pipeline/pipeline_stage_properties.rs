use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::data_pipeline::PipelineStageProperties;
use feagi_connector_core::data_pipeline::stage_properties::{IdentityStageProperties, ImageSegmentorStageProperties};
use feagi_data_structures::FeagiDataError;
use pyo3::exceptions::PyValueError;
use crate::feagi_connector_core::data_pipeline::stage_properties::{PyIdentityStageProperties, PyImageSegmentorStageProperties};
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;

#[pyclass(subclass)]
pub struct
PyPipelineStageProperties {
    inner: Box<dyn PipelineStageProperties>,
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


}

impl From<PyPipelineStageProperties> for Box<dyn PipelineStageProperties> {
    fn from(inner: PyPipelineStageProperties) -> Self {
        inner.inner
    }
}

impl PyPipelineStageProperties {
    // Do not allow instantiation from Python
    pub(crate) fn new(boxed_properties: Box<dyn PipelineStageProperties>) -> Self {
        Self { inner: boxed_properties }
    }

    /// Attempts to convert a boxed rust stage into a python stage properties with the correct inheritance
    pub(crate) fn boxed_to_py(py: Python<'_>, stage_properties: Box<dyn PipelineStageProperties>) -> PyResult<PyObject> {
        if stage_properties.as_any().is::<IdentityStageProperties>() {
            return Ok(Py::new(py, (PyIdentityStageProperties, PyPipelineStageProperties::new(stage_properties)))?.into())
        }
        if stage_properties.as_any().is::<ImageSegmentorStageProperties>() {
            return Ok(Py::new(py, (PyImageSegmentorStageProperties, PyPipelineStageProperties::new(stage_properties)))?.into())
        }

        Err(PyErr::new::<PyValueError, _>("Unsupported stage properties"))?

        // WTF

    }

}

