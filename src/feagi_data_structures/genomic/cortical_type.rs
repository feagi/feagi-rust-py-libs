use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::{motor_cortical_units, sensor_cortical_units};
use feagi_data_structures::genomic::{SensoryCorticalUnit, MotorCorticalUnit};
use feagi_data_structures::genomic::cortical_area::{CorticalID, CorticalAreaType, CoreCorticalType};
use feagi_data_structures::genomic::cortical_area::descriptors::{CorticalGroupIndex, CorticalChannelIndex};
use crate::feagi_data_structures::genomic::descriptors::PyCorticalGroupIndex;
use crate::feagi_data_structures::genomic::PyCorticalID;
use crate::py_type_casts;

// creating 2 near identical macros cause screw it
macro_rules! define_input_cortical_types_py {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    snake_case_name: $snake_case_name:expr,
                    accepted_wrapped_io_data_type: $data_type:ident,
                    cortical_id_unit_reference: $base_ascii:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    cortical_area_types: {
                        $(($cortical_area_type_expr:expr, $area_index:expr)),* $(,)?
                    },
                    unit_default_topology: {
                        $($topology_index:expr => { relative_position: $rel_pos:expr, dimensions: $dims:expr }),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) => {

        #[pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[pyo3(name = "SensorCorticalType")]
        pub enum PySensorCorticalType {
            $(
                $cortical_type_key_name
            ),*
        }

        impl From<SensoryCorticalUnit> for PySensorCorticalType {
            fn from(inner: SensoryCorticalUnit) -> Self {
                match inner {
                $(
                     SensoryCorticalUnit::$cortical_type_key_name => Self::$cortical_type_key_name
                ),*
                }
            }
        }

        impl From<PySensorCorticalType> for SensoryCorticalUnit {
            fn from(inner: PySensorCorticalType) -> Self {
                match inner {
                $(
                     PySensorCorticalType::$cortical_type_key_name => SensoryCorticalUnit::$cortical_type_key_name
                ),*
                }
            }
        }

        // TODO: Add as_cortical_id method once we determine how to handle different parameters for different sensor types
    }
}

macro_rules! define_output_cortical_types_py {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    snake_case_name: $snake_case_name:expr,
                    accepted_wrapped_io_data_type: $data_type:ident,
                    cortical_id_unit_reference: $base_ascii:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    cortical_area_types: {
                        $(($cortical_area_type_expr:expr, $area_index:expr)),* $(,)?
                    },
                    unit_default_topology: {
                        $($topology_index:expr => { relative_position: $rel_pos:expr, dimensions: $dims:expr }),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) => {

        #[pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[pyo3(name = "MotorCorticalType")]
        pub enum PyMotorCorticalType {
            $(
                $cortical_type_key_name
            ),*
        }

        impl From<MotorCorticalUnit> for PyMotorCorticalType {
            fn from(inner: MotorCorticalUnit) -> Self {
                match inner {
                $(
                     MotorCorticalUnit::$cortical_type_key_name => Self::$cortical_type_key_name
                ),*
                }
            }
        }

        impl From<PyMotorCorticalType> for MotorCorticalUnit {
            fn from(inner: PyMotorCorticalType) -> Self {
                match inner {
                $(
                     PyMotorCorticalType::$cortical_type_key_name => MotorCorticalUnit::$cortical_type_key_name
                ),*
                }
            }
        }

        // TODO: Add as_cortical_id method once we determine how to handle different parameters for different motor types
    }
}



/*
// PyCorticalType temporarily disabled pending CorticalType enum implementation in beta.56
#[pyclass]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalType")]
pub struct PyCorticalType {
    pub(crate) inner: CorticalType,
}

#[pymethods]
impl PyCorticalType {

    // NOTE: Since Python doesnt support "into()" we have these additional functions for now // TODO inheritance?
    //region Python specific constructors
    #[staticmethod]
    pub fn new_from_core(py_core_cortical_type: PyCoreCorticalType) -> PyResult<Self> {
        let result = CoreCorticalType::try_from(py_core_cortical_type);
        match result {
            Ok(core) => Ok(CorticalType::from(core).into()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_from_sensory(py_sensor_cortical_type: PySensorCorticalType) -> PyResult<Self> {
        let result = SensorCorticalType::try_from(py_sensor_cortical_type);
        match result {
            Ok(sensor) => Ok(CorticalType::from(sensor).into()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_custom() -> PyResult<Self> {
        Ok(PyCorticalType{inner: CorticalType::Custom})
    }

    #[staticmethod]
    pub fn new_memory() -> PyResult<Self> {
        Ok(PyCorticalType{inner: CorticalType::Memory})
    }
    //endregion

    #[staticmethod]
    pub fn get_type_from_bytes(bytes: [u8; CorticalID::CORTICAL_ID_LENGTH]) -> PyResult<Self> { // TODO rename?
        let result = CorticalType::try_get_type_from_bytes(&bytes);
        match result {
            Ok(cortical_type) => Ok(PyCorticalType{inner: cortical_type}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn to_cortical_id<'py>(&self, py: Python<'_>, io_cortical_group_index: PyObject) -> PyResult<PyCorticalID> {
        let io_cortical_grouping_result = PyCorticalGroupIndex::try_get_from_py_object(py, io_cortical_group_index);
        let io_cortical_index = match io_cortical_grouping_result {
            Ok(io_cortical_index) => {io_cortical_index}
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };

        let result = self.inner.to_cortical_id(io_cortical_index);
        match result {
            Ok(cortical_id) => {Ok(cortical_id.into())}
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    // TODO try_get_channel_size_boundaries

    pub fn is_type_core(&self) -> PyResult<bool> {
        Ok(self.is_type_core()?)
    }

    pub fn is_type_sensor(&self) -> PyResult<bool> {
        Ok(self.is_type_sensor()?)
    }

    pub fn is_type_motor(&self) -> PyResult<bool> {
        Ok(self.is_type_motor()?)
    }

    pub fn is_type_custom(&self) -> PyResult<bool> {
        Ok(self.is_type_custom()?)
    }

    pub fn is_type_memory(&self) -> PyResult<bool> {
        Ok(self.is_type_memory()?)
    }
}


py_type_casts!(PyCorticalType, CorticalType);
*/

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

/* Temporarily disabled pending CorticalType enum implementation
impl From<PyCoreCorticalType> for CorticalType {
    fn from(type_: PyCoreCorticalType) -> Self {
        CorticalType::Core(type_.into())
    }
}

impl From<CorticalType> for PyCoreCorticalType {
    fn from(type_: CorticalType) -> Self {
        PyCoreCorticalType::from(type_).into()
    }
}

impl From <CoreCorticalType> for PyCoreCorticalType {
    fn from(type_: CoreCorticalType) -> Self {
        PyCoreCorticalType::from(type_).into()
    }
}
*/


//endregion

//region Sensor Cortical Area Types

sensor_cortical_units!(define_input_cortical_types_py);

//endregion

motor_cortical_units!(define_output_cortical_types_py);