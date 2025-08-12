use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::{CoreCorticalType, CorticalID, SensorCorticalType};
use crate::genomic_structures::{PyCoreCorticalType, PyCorticalGroupingIndex, PyCorticalType, PySensorCorticalType};

#[pyclass(eq, str)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub(crate) inner: CorticalID,
}

impl std::fmt::Display for PyCorticalID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CorticalID({})", self.inner.to_string())
    }
}

#[pymethods]
impl PyCorticalID {
    
    #[staticmethod]
    pub fn new_custom_cortical_area_id(desired_id_string: String)  -> PyResult<Self> {
        let result = CorticalID::new_custom_cortical_area_id(desired_id_string);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID {inner: cortical_id}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
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
        
        let input_index_result = PyCorticalGroupingIndex::try_from_python(py, input_index);
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
    pub fn create_ordered_cortical_areas_for_segmented_vision<'py>(py: Python<'_>, camera_index: PyObject) -> PyResult<[Self; 9]> {

        let camera_index_result = PyCorticalGroupingIndex::try_from_python(py, camera_index);
        let camera_index = match camera_index_result {
            Ok(camera_index) => camera_index,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };
        
        let result = CorticalID::create_ordered_cortical_areas_for_segmented_vision(camera_index);
        Ok([
            result[0].into(),
            result[1].into(),
            result[2].into(),
            result[3].into(),
            result[4].into(),
            result[5].into(),
            result[6].into(),
            result[7].into(),
            result[8].into(),
        ])
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

        let io_cortical_index_result = PyCorticalGroupingIndex::try_from_python(py, io_cortical_index);
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
    
    pub fn as_bytes(&self) -> [u8; CorticalID::CORTICAL_ID_LENGTH] {
        self.inner.as_bytes().clone()
    }
    
    pub fn as_ascii_string(&self) -> String {
        self.inner.as_ascii_string()
    }
    
    pub fn get_cortical_type(&self) -> PyCorticalType {
        self.inner.get_cortical_type().into()
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

impl From<CorticalID> for PyCorticalID {
    fn from(inner: CorticalID) -> Self {
        PyCorticalID{inner: inner}
    }
}