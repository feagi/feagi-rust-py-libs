use std::time::Instant;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::{SensorCache, StreamCacheProcessor};
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
use crate::genomic_structures::{PyCorticalGroupingIndex, PyCorticalIOChannelIndex, PySensorCorticalType, PyCorticalType, PySingleChannelDimensions};
use crate::io_data::{try_get_as_io_type_variant, try_wrap_as_io_type_data};
use crate::io_processing::byte_structures::PyFeagiByteStructureCompatible;
use crate::io_processing::PyStreamCacheProcessor;
use crate::io_processing::stream_cache_processors::extract_stream_cache_processor;
use crate::io_processing::processors::{
    PyIdentityFloatProcessor, PyIdentityImageFrameProcessor,
    PyLinearAverageRollingWindowProcessor, PyLinearScaleTo0And1, PyLinearScaleToM1And1
};
use crate::neuron_data::xyzp::PyCorticalMappedXYZPNeuronData;

#[pyclass]
#[pyo3(name = "SensorCache")]
pub struct PySensorCache {
    pub(crate) inner: SensorCache,
}

#[pymethods]
impl PySensorCache {
    #[new]
    pub fn new() -> Self {
        PySensorCache {inner: SensorCache::new()}
    }
    
    pub fn register_single_cortical_area<'py>(&mut self, py: Python<'_>, cortical_sensor_type: PySensorCorticalType, cortical_grouping_index: PyObject, number_supported_channels: u32, channel_dimensions: PySingleChannelDimensions) -> PyResult<()> {
        
        let cortical_group_index_result = PyCorticalGroupingIndex::try_from_python(py, cortical_grouping_index);
        if cortical_group_index_result.is_err() {
            return Err(PyValueError::new_err(cortical_group_index_result.unwrap_err().to_string()));
        }
        let cortical_grouping_index: CorticalGroupingIndex = cortical_group_index_result.unwrap();
        
        let channel_dimensions: SingleChannelDimensions = channel_dimensions.into(); // TODO can we make this automatic?
        
        
        let result = self.inner.register_single_cortical_area(cortical_sensor_type.into(), cortical_grouping_index, number_supported_channels, channel_dimensions);
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    
    pub fn register_single_channel<'py>(&mut self, py: Python<'_>, cortical_sensor_type: PySensorCorticalType, cortical_grouping_index: PyObject,
                            channel: PyObject, sensory_processors: Vec<Py<PyStreamCacheProcessor>>, should_sensor_allow_sending_stale_data: bool) -> PyResult<()> {

        let cortical_group_index_result = PyCorticalGroupingIndex::try_from_python(py, cortical_grouping_index);
        if cortical_group_index_result.is_err() {
            return Err(PyValueError::new_err(cortical_group_index_result.unwrap_err().to_string()));
        }
        let cortical_grouping_index: CorticalGroupingIndex = cortical_group_index_result.unwrap();

        let cortical_channel_index_result = PyCorticalIOChannelIndex::try_from_python(py, channel);
        if cortical_channel_index_result.is_err() {
            return Err(PyValueError::new_err(cortical_channel_index_result.unwrap_err().to_string()));
        }
        let cortical_channel_index: CorticalIOChannelIndex = cortical_channel_index_result.unwrap();
        
        let mut sensory_processors_unwrapped: Vec<Box<dyn StreamCacheProcessor + Sync + Send>> = Vec::with_capacity(sensory_processors.len());
        for py_processor_parent_class in sensory_processors {
            let processor_box = extract_stream_cache_processor(py, py_processor_parent_class)?;
            sensory_processors_unwrapped.push(processor_box);
        }
        
        let result = self.inner.register_single_channel(cortical_sensor_type.into(), cortical_grouping_index, cortical_channel_index, sensory_processors_unwrapped, should_sensor_allow_sending_stale_data);
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
        
    }
    
    // TODO register_agent_device_index
    
    pub fn update_value_by_channel<'py>(&mut self, py: Python<'_>, value: PyObject, cortical_sensor_type: PySensorCorticalType, cortical_grouping_index: PyObject, channel: PyObject) -> PyResult<()> {
        
        
        let cortical_group_index_result = PyCorticalGroupingIndex::try_from_python(py, cortical_grouping_index);
        if cortical_group_index_result.is_err() {
            return Err(PyValueError::new_err(cortical_group_index_result.unwrap_err().to_string()));
        }
        let cortical_grouping_index: CorticalGroupingIndex = cortical_group_index_result.unwrap();

        let cortical_channel_index_result = PyCorticalIOChannelIndex::try_from_python(py, channel);
        if cortical_channel_index_result.is_err() {
            return Err(PyValueError::new_err(cortical_channel_index_result.unwrap_err().to_string()));
        }
        let cortical_channel_index: CorticalIOChannelIndex = cortical_channel_index_result.unwrap();
        
        
        let input_data = try_wrap_as_io_type_data(py, value);
        match input_data {
            Err(err) => {Err(PyValueError::new_err(err.to_string()))}
            Ok(input_data) => {
                let cortical_grouping_index = cortical_grouping_index;
                let channel = cortical_channel_index;
                self.inner.update_value_by_channel(input_data, cortical_sensor_type.into(), cortical_grouping_index, channel);
                Ok(())
            }
        }
    }
    
    //  TODO get way to query current channel data?

    pub fn encode_to_neurons<'py>(&mut self, py: Python<'py>) -> PyResult<PyObject> {
        // TODO pass in instant? Review how to handle this
        let mut mapped_data: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new();
        let result = self.inner.encode_to_neurons(Instant::now(), &mut mapped_data);
        match result {
            Ok(()) => {
                let child = PyCorticalMappedXYZPNeuronData::from(mapped_data);
                let parent = PyFeagiByteStructureCompatible::new();
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj.into())
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

}
