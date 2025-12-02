use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_serialization::FeagiByteStructureType;
use crate::{project_display, py_object_cast_generic, py_type_casts};

#[pyclass(str, eq)]
#[pyo3(name = "FeagiByteStructureType")]
#[derive(Clone)]
pub struct PyFeagiByteStructureType {
    pub inner: FeagiByteStructureType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyFeagiByteStructureType {
    #[staticmethod]
    pub fn JSON() -> Self {
        PyFeagiByteStructureType {inner: FeagiByteStructureType::JSON}
    }

    #[staticmethod]
    pub fn NeuronCategoricalXYZP() -> Self {
        PyFeagiByteStructureType {inner: FeagiByteStructureType::NeuronCategoricalXYZP}
    }
    
}

project_display!(PyFeagiByteStructureType);
py_type_casts!(PyFeagiByteStructureType, FeagiByteStructureType);
py_object_cast_generic!(PyFeagiByteStructureType, FeagiByteStructureType, "Unable to retrieve FeagiByteStructureType data from given!");