use feagi_connector_core::data_pipeline::PipelineStagePropertyIndex;
use pyo3::types::PyInt;
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use crate::{typed_number, project_display, py_object_cast_int, py_type_casts};


typed_number!(PyPipelineStagePropertyIndex, PipelineStagePropertyIndex, u32, "PipelineStagePropertyIndex", "Unable to retrieve PipelineStagePropertyIndex data from given!");