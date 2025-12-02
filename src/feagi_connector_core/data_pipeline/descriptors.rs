use feagi_connector_core::data_pipeline::PipelineStagePropertyIndex;
use pyo3::types::PyInt;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use crate::{typed_number};
use crate::py_error::PyFeagiError;


typed_number!(PyPipelineStagePropertyIndex, PipelineStagePropertyIndex, u32, "PipelineStagePropertyIndex", "Unable to retrieve PipelineStagePropertyIndex data from given!");