
use pyo3::{pyclass};
use pyo3::prelude::*;
use feagi_data_serialization::FeagiByteStructureType;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "FeagiByteStructureType")]
pub enum PyFeagiByteStructureType{
    JSON = 1,
    NeuronCategoricalXYZP = 11,
}

impl PyFeagiByteStructureType{
    pub fn from_base(e: FeagiByteStructureType) -> Self{
        match e {
            FeagiByteStructureType::JSON => PyFeagiByteStructureType::JSON,
            FeagiByteStructureType::NeuronCategoricalXYZP => PyFeagiByteStructureType::NeuronCategoricalXYZP,
        }
    }

    pub fn to_base(e: PyFeagiByteStructureType) -> FeagiByteStructureType{
        match e {
            PyFeagiByteStructureType::JSON => FeagiByteStructureType::JSON,
            PyFeagiByteStructureType::NeuronCategoricalXYZP => FeagiByteStructureType::NeuronCategoricalXYZP,
        }
    }
}