use feagi_connector_core::data_pipeline::PipelineStagePropertyIndex;
use pyo3::types::PyInt;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::descriptors::AgentDeviceIndex;
use feagi_data_structures::genomic::cortical_area::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex, NeuronDepth};
use crate::{project_display, py_object_cast_int, py_type_casts, py_object_try_cast_int, typed_number};
use crate::py_error::PyFeagiError;

// TODO GenomeCoordinate

typed_number!(PyAgentDeviceIndex, AgentDeviceIndex, u32, "AgentDeviceIndex", "Unable to retrieve AgentDeviceIndex data from given!");

