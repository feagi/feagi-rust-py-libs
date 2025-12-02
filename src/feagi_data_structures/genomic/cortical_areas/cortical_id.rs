use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::CorticalID;
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;

#[pyclass(eq, str)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub(crate) inner: CorticalID,
}

#[pymethods]
impl PyCorticalID {

    #[staticmethod]
    pub fn try_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> PyResult<Self> {
        let cortical_id = CorticalID::try_from_bytes(bytes).map_err(PyFeagiError::from)?;
        Ok(cortical_id.into())
    }


    #[getter]
    pub fn CORTICAL_ID_LENGTH(&self) -> PyResult<usize> {
        Ok(CorticalID::CORTICAL_ID_LENGTH)
    }

    #[getter]
    pub fn NUMBER_OF_BYTES(&self) -> PyResult<usize> {
        Ok(CorticalID::NUMBER_OF_BYTES)
    }
}

project_display!(PyCorticalID);
py_type_casts!(PyCorticalID, CorticalID);
py_object_cast_generic!(PyCorticalID, CorticalID, "Unable to import CorticalID");