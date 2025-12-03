use pyo3::types::PyInt;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::descriptors::{CorticalChannelCount, CorticalChannelDimensions, CorticalChannelIndex, CorticalGroupIndex, CorticalUnitIndex, NeuronDepth};
use feagi_data_structures::genomic::descriptors::AgentDeviceIndex;
use crate::py_error::PyFeagiError;
use crate::{typed_non_zero_number, typed_number};
//region Cortical Indexing

typed_number!(PyCorticalChannelIndex, CorticalChannelIndex, u32, "CorticalChannelIndex", "Unable to retrieve CorticalChannelIndex data from given!");

typed_number!(PyCorticalGroupIndex, CorticalGroupIndex, u8, "CorticalGroupIndex", "Unable to retrieve CorticalGroupIndex data from given!");

typed_number!(PyCorticalUnitIndex, CorticalUnitIndex, u8, "CorticalUnitIndex", "Unable to retrieve CorticalUnitIndex data from given!");

//endregion

//region Channels

typed_non_zero_number!(PyCorticalChannelCount, CorticalChannelCount, u32, "CorticalChannelCount", "Unable to retrieve CorticalChannelCount data from given!");

typed_non_zero_number!(PyNeuronDepth, NeuronDepth, u32, "NeuronDepth", "Unable to retrieve NeuronDepth data from given!");

typed_non_zero_number!(PyCorticalChannelDimensions, CorticalChannelDimensions, u32, "CorticalChannelDimensions", "Unable to retrieve CorticalChannelDimensions data from given!");

//endregion

//region Spatial

// TODO NeuronVoxelCoordinate CorticalAreaDimensions

//endregion
