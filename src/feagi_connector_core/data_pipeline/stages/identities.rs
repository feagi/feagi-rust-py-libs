use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_connector_core::data_pipeline::stages::*;
use feagi_data_structures::FeagiDataError;
use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use crate::{project_display, py_type_casts};
use crate::feagi_connector_core::data_pipeline::pipeline_stage::PyPipelineStage;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_pytrait::PipelineStagePyTrait;
use crate::feagi_data_structures::wrapped_io_data::PyWrappedIOType;

#[pyclass(str, extends=PyPipelineStage)]
#[pyo3(name = "IdentityFloatStage")]
#[derive(Clone)]
pub struct PyIdentityFloatStage {
    inner: IdentityFloatStage,
}

#[pymethods]
impl PyIdentityFloatStage {

    #[new]
    pub fn new(initial_value: f32) -> PyResult<(PyIdentityFloatStage, PyPipelineStage)> {
        Ok((
            PyIdentityFloatStage {
                inner: IdentityFloatStage::new(initial_value).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?,
            },
            PyPipelineStage {}
        ))
    }

    //region PipelineStage

    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        Ok(PyWrappedIOType::F32())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        Ok(PyWrappedIOType::F32())
    }

    pub fn get_most_recent_output<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }

    pub fn process_new_input(&self, new_input: PyObject) -> PyResult<(PyObject)> {
        Err(PyErr::new::<PyNotImplementedError, _>("Cannot call parent class!"))
    }


    //endregion


}

 impl PipelineStagePyTrait for PyIdentityFloatStage {
     fn copy_as_box(&self) -> Result<Box<dyn PipelineStage>, FeagiDataError> {
         let stage = self.inner.clone();
        Ok(Box::new(stage))
     }
 }




project_display!(PyIdentityFloatStage);
py_type_casts!(PyIdentityFloatStage, IdentityFloatStage);


