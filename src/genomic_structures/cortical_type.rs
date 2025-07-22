// Since Python does not support enums to the same depth as Rust, we are using a intermediate struct to handle conversions

use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, SensorCorticalType, MotorCorticalType, CoreCorticalType};

#[pyclass]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalType")]
pub struct PyCorticalType {
    cortical_type: CorticalType,
}

impl From<CorticalType> for PyCorticalType {
    fn from(c: CorticalType) -> Self {
        PyCorticalType { cortical_type : c }
    }
}

#[pymethods]
impl PyCorticalType {
    #[new]
    pub fn new(core_type: PyCorticalCoreTypeVariant) -> Self {
        PyCorticalType{cortical_type: CorticalType::from(core_type)}
    }
    
    #[staticmethod]
    pub fn new_sensor(sensor_type: PyCorticalSensorTypeVariant) -> Self {
        PyCorticalType{cortical_type: CorticalType::from(sensor_type)}
    }
}

//region enums
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalTypeVariant")]
pub enum PyCorticalTypeVariant {
    Custom,
    Memory,
    Core,
    Sensory,
    Motor,
}

impl From<CorticalType> for PyCorticalTypeVariant {
    fn from(type_: CorticalType) -> Self {
        match type_ { 
            CorticalType::Custom => PyCorticalTypeVariant::Custom,
            CorticalType::Memory => PyCorticalTypeVariant::Memory,
            CorticalType::Core(_) => PyCorticalTypeVariant::Core,
            CorticalType::Sensory(_) => PyCorticalTypeVariant::Sensory,
            CorticalType::Motor(_) => PyCorticalTypeVariant::Motor,
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalSensorTypeVariant")]
pub enum PyCorticalSensorTypeVariant {
    Proximity
}

impl From<SensorCorticalType> for PyCorticalSensorTypeVariant {
    fn from(type_: SensorCorticalType) -> Self {
        // TODO add an actual matching algorithm
        PyCorticalSensorTypeVariant::Proximity
    }
}

impl TryFrom<CorticalType> for PyCorticalSensorTypeVariant {
    type Error = PyErr;
    fn try_from(type_: CorticalType) -> PyResult<Self> {
        match type_ { 
            CorticalType::Sensory(s) => Ok(PyCorticalSensorTypeVariant::Proximity), // TODO temp
            _ => Err(PyValueError::new_err("Cannot cast non-sensor cortical type to sensor type variant!"))
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalSensorTypeVariant")]
pub enum PyCorticalCoreTypeVariant {
    Death,
    Power
}

impl From<PyCorticalCoreTypeVariant> for CorticalType {
    fn from(type_: PyCorticalCoreTypeVariant) -> Self {
        match type_ { 
            PyCorticalCoreTypeVariant::Death => CorticalType::Core(CoreCorticalType::Death),
            PyCorticalCoreTypeVariant::Power => CorticalType::Core(CoreCorticalType::Power),
        }
    }
}

//endregion