use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, CoreCorticalType, SensorCorticalType};
use feagi_core_data_structures_and_processing::sensor_definition;

// creating 2 near identical macros cause screw it
macro_rules! define_input_cortical_types_py { 
    (
        $cortical_io_type_enum_name:ident {
            $(
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:expr,
                }
            ),* $(,)?
        }
    ) => {
        
        #[pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[pyo3(name = "SensorCorticalType")]
        pub enum PyCorticalSensorTypeVariant {
            $(
                $cortical_type_key_name
            ),*
        }
        
        impl From<SensorCorticalType> for PyCorticalSensorTypeVariant {
            fn from(inner: SensorCorticalType) -> Self {
                match inner {
                $(
                     SensorCorticalType::$cortical_type_key_name => Self::$cortical_type_key_name,
                ),*
                }
            }
        }
        
        impl From<PyCorticalSensorTypeVariant> for SensorCorticalType {
            fn from(inner: PyCorticalSensorTypeVariant) -> Self {
                match inner {
                $(
                     PyCorticalSensorTypeVariant::$cortical_type_key_name => SensorCorticalType::$cortical_type_key_name,
                ),*
                }
            }
        }
        
        // TODO expose to_cortical_id, get_type_from_bytes, get_channel_dimension_range
    }
}


#[pyclass]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalType")]
pub struct PyCorticalType {
    pub(crate) inner: CorticalType,
}

impl From<CorticalType> for PyCorticalType {
    fn from(c: CorticalType) -> Self {
        PyCorticalType { inner : c }
    }
}

impl From<PyCorticalType> for CorticalType {
    fn from(p: PyCorticalType) -> Self {
        p.inner
    }
}

#[pymethods]
impl PyCorticalType {
    #[new]
    pub fn new(core_type: PyCoreCorticalType) -> Self {
        PyCorticalType{inner: CorticalType::from(core_type)}
    }
    
    #[staticmethod]
    pub fn new_core(core_type: PyCoreCorticalType) -> Self {
        PyCorticalType{inner: CorticalType::from(core_type)}
    }
    
    #[staticmethod]
    pub fn new_sensor(sensor_type: PyCorticalSensorTypeVariant) -> Self {
        let sensor_type: SensorCorticalType = sensor_type.into();
        PyCorticalType{inner: CorticalType::from(sensor_type)}
    }
    
    // TODO motor

    #[staticmethod]
    pub fn new_custom() -> Self {
        PyCorticalType {inner: CorticalType::Custom}
    }
    
    #[staticmethod]
    pub fn new_memory() -> Self {
        PyCorticalType {inner: CorticalType::Memory}
    }
    
    pub fn get_type_variant(&self) -> PyCorticalTypeVariant{
        match self.inner {
            CorticalType::Custom => PyCorticalTypeVariant::Custom,
            CorticalType::Memory => PyCorticalTypeVariant::Memory,
            CorticalType::Core(_) => PyCorticalTypeVariant::Core,
            CorticalType::Sensory(_) => PyCorticalTypeVariant::Sensory,
            CorticalType::Motor(_) => PyCorticalTypeVariant::Motor,
        }
    }
}



/// Only used to get as an enum what type a PyCorticalType is
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "CorticalTypeVariant")]
pub enum PyCorticalTypeVariant {
    Custom,
    Memory,
    Core,
    Sensory,
    Motor,
}

//region Core
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CoreCorticalType")]
pub enum PyCoreCorticalType {
    Death,
    Power
}

impl From<PyCoreCorticalType> for CoreCorticalType {
    fn from(type_: PyCoreCorticalType) -> Self {
        match type_ {
            PyCoreCorticalType::Death => CoreCorticalType::Death,
            PyCoreCorticalType::Power => CoreCorticalType::Power
        }
    }
}

impl From<PyCoreCorticalType> for CorticalType {
    fn from(type_: PyCoreCorticalType) -> Self {
        CorticalType::Core(type_.into())
    }
}

//endregion

//region Sensor Cortical Area Types

sensor_definition!(define_input_cortical_types_py);

//endregion