use pyo3::types::PyInt;
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::descriptors::AgentDeviceIndex;
use crate::{project_display, py_object_cast_int, py_type_casts, typed_number};

// TODO GenomeCoordinate

typed_number!(PyAgentDeviceIndex, AgentDeviceIndex, u32, "AgentDeviceIndex", "Unable to retrieve AgentDeviceIndex data from given!");

