use std::time::Instant;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::{SensorCache, StreamCacheProcessor};
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
use crate::genomic_structures::{PyCorticalGroupingIndex, PyCorticalIOChannelIndex, PyCorticalType, PySingleChannelDimensions};
use crate::io_data::{try_get_as_io_type_variant, try_wrap_as_io_type_data};
use crate::io_processing::stream_cache_processors::{PyIdentityLinearFloatCacheProcessor, PyStreamCacheProcessor};
use crate::io_processing::byte_structures::PyFeagiByteStructureCompatible;
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
    
    pub fn register_single_cortical_area(&mut self, cortical_type: PyCorticalType, cortical_grouping_index: PyCorticalGroupingIndex, number_supported_channels: u32, channel_dimensions: PySingleChannelDimensions) -> PyResult<()> {
        let cortical_type: CorticalType = cortical_type.into();
        let cortical_grouping_index: CorticalGroupingIndex = cortical_grouping_index.into();
        let channel_dimensions: SingleChannelDimensions = channel_dimensions.into();
        
        let result = self.inner.register_single_cortical_area(cortical_type, cortical_grouping_index, number_supported_channels, channel_dimensions);
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    
    pub fn register_channel(&mut self, cortical_type: PyCorticalType, cortical_grouping_index: PyCorticalGroupingIndex,
                            channel: PyCorticalIOChannelIndex, sensory_processor: PyIdentityLinearFloatCacheProcessor, should_sensor_allow_sending_stale_data: bool) -> PyResult<()> {

        let cortical_type: CorticalType = cortical_type.into();
        let cortical_grouping_index: CorticalGroupingIndex = cortical_grouping_index.into();
        let channel: CorticalIOChannelIndex = channel.into();
        
        //let cache_processor: Box<dyn StreamCacheProcessor> = sensory_processor.into();
        let sensory_processor = Box::new(sensory_processor.inner);
        
        let result = self.inner.register_channel(cortical_type, cortical_grouping_index, channel, sensory_processor, should_sensor_allow_sending_stale_data);
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
        
    }
    
    
    pub fn update_value_by_channel<'py>(&mut self, py: Python<'_>, value: PyObject, cortical_type: PyCorticalType, cortical_grouping_index: PyCorticalGroupingIndex, channel: PyCorticalIOChannelIndex) -> PyResult<()> {
        
        let input_data = try_wrap_as_io_type_data(py, value);
        match input_data {
            Err(err) => {Err(PyValueError::new_err(err.to_string()))}
            Ok(input_data) => {
                let cortical_type = cortical_type.inner;
                let cortical_grouping_index = cortical_grouping_index.inner;
                let channel = channel.inner;
                self.inner.update_value_by_channel(input_data, cortical_type, cortical_grouping_index, channel);
                Ok(())
            }
        }
        
    }

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