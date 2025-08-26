use pyo3::types::PyInt;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::descriptors::{AgentDeviceIndex, CorticalChannelIndex, CorticalGroupIndex};
use crate::{project_display, py_object_cast_int, py_type_casts};

//region macros

macro_rules! typed_index {
    ($py_struct:ident, $feagi_struct:ident, $number_type:ty, $class_name:expr, $error_msg:expr) => {
        
        
        #[pyclass]
        #[pyo3(name = $class_name)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $py_struct {
            inner: $feagi_struct
        }
        
        #[pymethods]
        impl $py_struct{
            #[new]
            pub fn new(index: $number_type) -> PyResult<Self> {
                Ok(
                    $py_struct { 
                        inner: $feagi_struct::from(index)
                    }
                )
            }
        }
        
        py_type_casts!($py_struct, $feagi_struct);
        py_object_cast_int!($py_struct, $feagi_struct, $number_type, $error_msg);
        project_display!($py_struct);

    };
}

//endregion


//region Indexes

typed_index!(PyCorticalGroupIndex, CorticalGroupIndex, u8, "CorticalGroupIndex", "Unable to retrieve CorticalGroupIndex data from given!");

typed_index!(PyCorticalChannelIndex, CorticalChannelIndex, u32, "CorticalChannelIndex", "Unable to retrieve CorticalChannelIndex data from given!");

typed_index!(PyAgentDeviceIndex, AgentDeviceIndex, u32, "AgentDeviceIndex", "Unable to retrieve AgentDeviceIndex data from given!");


//endregion



