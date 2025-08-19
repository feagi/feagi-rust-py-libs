use std::time::Instant;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalIOChannelIndex};
use feagi_core_data_structures_and_processing::io_data::ImageFrame;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::SensorCache;
use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
use crate::genomic_structures::{PyCorticalGroupingIndex, PyCorticalIOChannelIndex};
use crate::io_data::image_descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::io_data::PyImageFrame;
use crate::neuron_data::xyzp::PyCorticalMappedXYZPNeuronData;

#[pyclass]
#[pyo3(name = "SensorCache")]
pub struct PySensorCache {
    inner: SensorCache,
}

#[pymethods]
impl PySensorCache {
    
    #[new]
    pub fn new() -> Self {
        PySensorCache {
            inner: SensorCache::new(),
        }
    }
    
    //region Registration
    
    //region Macros
    
    pub fn register_cortical_group_for_proximity<'py>(&mut self, py: Python<'_>, cortical_group: PyObject,
                                                      number_of_channels: usize, allow_stale_data: bool, 
                                                      neuron_resolution: usize, lower_bound: f32, 
                                                      upper_bound:f32) -> PyResult<()> {
        

        let cortical_grouping_index: CorticalGroupingIndex = match PyCorticalGroupingIndex::try_from_python(py, cortical_group) {
            Ok(c) => c,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };
        
        let result = self.inner.register_cortical_group_for_proximity(cortical_grouping_index, 
                                                                      number_of_channels, allow_stale_data, neuron_resolution, 
                                                                      lower_bound, upper_bound);
        match result { 
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
        
    }
    
    //endregion
    
    //region Custom Calls
    
    pub fn register_cortical_group_for_image_camera<'py>(&mut self, py: Python<'_>, cortical_group: PyObject, 
                                                         number_of_channels: usize, allow_stale_data: bool, 
                                                         input_image_properties: PyImageFrameProperties, 
                                                         output_image_properties: PyImageFrameProperties) -> PyResult<()> {

        let cortical_grouping_index: CorticalGroupingIndex = match PyCorticalGroupingIndex::try_from_python(py, cortical_group) {
            Ok(c) => c,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };
        let result = self.inner.register_cortical_group_for_image_camera(cortical_grouping_index, 
                                                                         number_of_channels, allow_stale_data, 
                                                                         input_image_properties.into(), 
                                                                         output_image_properties.into());
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
        
    }





    pub fn register_cortical_group_for_image_camera_with_peripheral<'py>(&mut self, py: Python<'_>, cortical_group: PyObject, 
                                                         number_of_channels: usize, allow_stale_data: bool, 
                                                         input_image_properties: PyImageFrameProperties, 
                                                         output_image_properties: PySegmentedImageFrameProperties, 
                                                         segmentation_center_properties: PyGazeProperties) -> PyResult<()> {

        let cortical_grouping_index: CorticalGroupingIndex = match PyCorticalGroupingIndex::try_from_python(py, cortical_group) {
            Ok(c) => c,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };
        let result = self.inner.register_cortical_group_for_image_camera_with_peripheral(cortical_grouping_index, 
                                                                                         number_of_channels, allow_stale_data, 
                                                                                         input_image_properties.into(), 
                                                                                         output_image_properties.into(),
                                                                                         segmentation_center_properties.into());
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
        
    }
        
    //endregion
    
    //endregion
    
    //region Send Data
    
    //region macro
    
    pub fn send_data_for_proximity<'py>(&mut self, py: Python<'_>, new_value: f32, cortical_grouping_index: PyObject, device_channel: PyObject) -> PyResult<()> {

        let cortical_grouping_index: CorticalGroupingIndex = match PyCorticalGroupingIndex::try_from_python(py, cortical_grouping_index) {
            Ok(c) => c,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };

        let device_channel: CorticalIOChannelIndex = match PyCorticalIOChannelIndex::try_from_python(py, device_channel) {
            Ok(c) => c,
            Err(e) => return Err(PyValueError::new_err(e.to_string()))
        };
        
        match self.inner.send_data_for_proximity(new_value, cortical_grouping_index, device_channel) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    //endregion
    
    //region Custom Calls

    pub fn send_data_for_image_camera<'py>(&mut self, py: Python<'_>, new_value: PyImageFrame, cortical_grouping_index: PyCorticalGroupingIndex, device_channel: PyCorticalIOChannelIndex) -> PyResult<()> {
        let new_value: ImageFrame = new_value.into();
        let cortical_grouping_index: CorticalGroupingIndex = cortical_grouping_index.into();
        let device_channel: CorticalIOChannelIndex = device_channel.into();
        match self.inner.send_data_for_image_camera(new_value, cortical_grouping_index, device_channel) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn send_data_for_segmented_image_camera<'py>(&mut self, py: Python<'_>, new_value: PyImageFrame, cortical_grouping_index: PyCorticalGroupingIndex, device_channel: PyCorticalIOChannelIndex) -> PyResult<()> {
        let new_value: ImageFrame = new_value.into();
        let cortical_grouping_index: CorticalGroupingIndex = cortical_grouping_index.into();
        let device_channel: CorticalIOChannelIndex = device_channel.into();
        match self.inner.send_data_for_segmented_image_camera(new_value, cortical_grouping_index, device_channel) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    //endregion
    
    //endregion

    pub fn encode_to_neurons<'py>(&mut self, py: Python<'py>, mut writing_target: PyCorticalMappedXYZPNeuronData) -> PyResult<()> {
        // TODO pass in instant? Review how to handle this
        let mut mapped_data: &mut CorticalMappedXYZPNeuronData = writing_target.get_mut();
        let result = self.inner.encode_to_neurons(Instant::now(), &mut mapped_data);
        match result {
            Ok(()) => {
                Ok(())
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
}