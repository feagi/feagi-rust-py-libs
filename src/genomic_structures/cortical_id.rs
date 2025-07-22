use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::CorticalID;

#[pyclass(eq)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub inner: CorticalID,
}

#[pymethods]
impl PyCorticalID {
    #[new]
    pub fn try_new_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> PyResult<Self> {
        let result = CorticalID::from_bytes(bytes);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }
    
    pub fn try_new_from_string(string: String) -> PyResult<Self> {
        let result = CorticalID::from_string(string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id }),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn new_custom_cortical_area_id(desired_id_string: String) -> PyResult<Self> {
        let result = CorticalID::new_custom_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn new_memory_cortical_area_id(desired_id_string: String) -> PyResult<Self> {
        let result = CorticalID::new_memory_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn new_core_cortical_area_id(desired_id_string: String) -> PyResult<Self> {
        let result = CorticalID::new_core_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }
    
    pub fn new_sensor_cortical_area_id(desired_id_string: String) -> PyResult<Self> {
        let result = CorticalID::new_sensor_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn new_motor_cortical_area_id(desired_id_string: String) -> PyResult<Self> {
        let result = CorticalID::new_motor_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    fn __repr__(&self) -> String {
        format!("Cortical_ID({})", self.as_str())
    }
}