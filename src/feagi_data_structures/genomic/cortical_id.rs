use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::{SensoryCorticalUnit, MotorCorticalUnit};
use feagi_data_structures::genomic::cortical_area::{CorticalID, CoreCorticalType};
use crate::feagi_data_structures::genomic::cortical_type::{PyCoreCorticalType, /* PyCorticalType, */ PySensorCorticalType};
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::feagi_data_structures::genomic::descriptors::PyCorticalGroupIndex;
use crate::feagi_data_structures::genomic::PyMotorCorticalType;
use crate::py_error::PyFeagiError;

#[pyclass(eq, str)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub(crate) inner: CorticalID,
}

#[pymethods]
impl PyCorticalID {

    /* Temporarily disabled pending beta.56 API migration
    #[staticmethod]
    pub fn new_custom_cortical_area_id(desired_id_string: String)  -> PyResult<Self> {
        let cortical_id = CorticalID::new_custom_cortical_area_id(desired_id_string).map_err(PyFeagiError::from)?;
        Ok(cortical_id.into())
    }

    #[staticmethod]
    pub fn new_memory_cortical_area_id(desired_id_string: String)  -> PyResult<Self> {
        let result = CorticalID::new_memory_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID {inner: cortical_id}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_core_cortical_area_id(core_cortical_type: PyCoreCorticalType)  -> PyResult<Self> {
        let core_type: CoreCorticalType = core_cortical_type.into();
        let result = CorticalID::new_core_cortical_area_id(core_type);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID {inner: cortical_id}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_sensor_cortical_area_id<'py>(py: Python<'_>, sensor_cortical_type: PySensorCorticalType, input_index: PyObject)  -> PyResult<Self> {

        let input_index_result = PyCorticalGroupIndex::try_get_from_py_object(py, input_index);
        let input_index = match input_index_result {
            Ok(input_index) => input_index,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };

        let sensor_type: SensorCorticalType = sensor_cortical_type.into();
        let result = CorticalID::new_sensor_cortical_area_id(sensor_type, input_index);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID {inner: cortical_id}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_motor_cortical_area_id<'py>(py: Python<'_>, motor_cortical_type: PyMotorCorticalType, input_index: PyObject)  -> PyResult<Self> {
        let input_index_result = PyCorticalGroupIndex::try_get_from_py_object(py, input_index);
        let input_index = match input_index_result {
            Ok(input_index) => input_index,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };

        let motor_type: MotorCorticalType = motor_cortical_type.into();
        let result = CorticalID::new_motor_cortical_area_id(motor_type, input_index);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID {inner: cortical_id}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }


    #[staticmethod]
    pub fn try_new_from_bytes(bytes: [u8; CorticalID::CORTICAL_ID_LENGTH]) -> PyResult<Self> {
        let result = CorticalID::from_bytes(&bytes);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    #[staticmethod]
    pub fn try_new_from_string(string: String) -> PyResult<Self> {
        let result = CorticalID::from_string(string.into());
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id }),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    #[staticmethod]
    pub fn try_from_cortical_type<'py>(py: Python<'_>, cortical_type: PyCorticalType, io_cortical_index: PyObject) -> PyResult<Self> {

        let io_cortical_index_result = PyCorticalGroupIndex::try_get_from_py_object(py, io_cortical_index);
        let io_cortical_index = match io_cortical_index_result {
            Ok(io_cortical_index) => io_cortical_index,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };

        let result = CorticalID::try_from_cortical_type(&cortical_type.into(), io_cortical_index);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id }),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }
    */

    pub fn as_bytes(&self) -> [u8; CorticalID::CORTICAL_ID_LENGTH] {
        self.inner.as_bytes().clone()
    }

    /* Temporarily disabled pending beta.56 API migration
    pub fn as_ascii_string(&self) -> String {
        self.inner.as_ascii_string()
    }

    pub fn get_cortical_type(&self) -> PyCorticalType {
        self.inner.get_cortical_type().into()
    }
    */

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