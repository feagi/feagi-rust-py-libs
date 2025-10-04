use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::IOCache;
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex};
use feagi_data_structures::data_pipeline::PipelineStagePropertyIndex;
use crate::feagi_connector_core::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_connector_core::wrapped_io_data::py_object_to_wrapped_io_data;
use crate::feagi_connector_core::data_pipeline::extract_pipeline_stage_properties_from_py;
use crate::feagi_data_structures::genomic::descriptors::{PyCorticalChannelCount, PyCorticalChannelIndex, PyCorticalGroupIndex};
use crate::feagi_data_structures::data_pipeline::PyPipelineStagePropertyIndex;
use crate::py_error::PyFeagiError;

#[pyclass(str)]
#[pyo3(name = "IOCache")]
#[derive()]
pub struct PyIOCache {
    inner: IOCache
}

#[pymethods]
impl PyIOCache {

    #[new]
    pub fn new() -> Self {
        PyIOCache {
            inner: IOCache::new()
        }
    }


    //region Sensors

    pub fn sensor_register_segmented_vision_absolute(&mut self, py: Python<'_>, group: PyObject,
                                                     number_of_channels: PyObject, input_image_properties: PyImageFrameProperties,
                                                     output_segment_properties: PySegmentedImageFrameProperties, gaze: PyGazeProperties) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.sensor_register_segmented_vision_absolute(cortical_group_index,
                                                             number_of_channels, input_image_properties.into(),
                                                             output_segment_properties.into(), gaze.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn sensor_write_segmented_vision_absolute(&mut self, py: Python<'_>, group: PyObject, channel: PyObject, data: PyObject) -> PyResult<()> {
        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let wrapped_io_data = py_object_to_wrapped_io_data(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_write_segmented_vision_absolute(cortical_group_index, cortical_channel_index, &wrapped_io_data)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn sensor_update_stage_segmented_vision_absolute(&mut self, py: Python<'_>, group: PyObject, channel: PyObject, 
                                                         pipeline_stage_property_index: PyObject, stage: PyObject) -> PyResult<()> {
        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_property_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, pipeline_stage_property_index).map_err(PyFeagiError::from)?;
        let pipeline_stage_properties = extract_pipeline_stage_properties_from_py(py, stage).map_err(PyFeagiError::from)?;

        self.inner.sensor_update_stage_segmented_vision_absolute(cortical_group_index, cortical_channel_index, 
                                                                 stage_property_index, pipeline_stage_properties)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    //endregion

    

}