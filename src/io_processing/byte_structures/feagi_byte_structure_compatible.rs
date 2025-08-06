use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::byte_structures::FeagiByteStructureType;
use crate::io_processing::byte_structures::{PyFeagiByteStructure, PyFeagiByteStructureType};

#[pyclass(subclass)]
#[pyo3(name = "FeagiByteStructureCompatible")]
pub struct PyFeagiByteStructureCompatible {}

#[pymethods]
impl PyFeagiByteStructureCompatible {

    #[new]
    pub fn new() -> Self {
        PyFeagiByteStructureCompatible {}
    }

    #[getter]
    pub fn struct_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::from_base(FeagiByteStructureType::JSON) // This is a overridden placeholder
    }

    pub fn version(&self) -> u8 { 0 } // This is an overridden placeholder

    #[staticmethod]
    pub fn new_from_feagi_byte_structure(_byte_structure: PyFeagiByteStructure) -> PyResult<Self> where Self: Sized {
        Err(PyValueError::new_err("Not properly overridden PyFeagiByteStructureCompatible abstract member!"))
    }

    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        Err(PyValueError::new_err("Not properly overridden PyFeagiByteStructureCompatible abstract member!"))
    }
}