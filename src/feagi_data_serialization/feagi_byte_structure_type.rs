use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use feagi_serialization::FeagiByteStructureType;
use crate::{wrap_flat_enum, __base_py_class_shared};

wrap_flat_enum!(PyFeagiByteStructureType, FeagiByteStructureType, "FeagiByteStructureType");


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
