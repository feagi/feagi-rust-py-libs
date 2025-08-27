use std::time::Instant;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::neurons::xyzp::CorticalMappedXYZPNeuronData;
use feagi_connector_core::caching::SensorCache;
use feagi_data_structures::data::image_descriptors::ImageFrameProperties;
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex};
use crate::feagi_data_structures::data::image_descriptors::PyImageFrameProperties;
use crate::feagi_data_structures::data::PyImageFrame;
use crate::feagi_data_structures::genomic::descriptors::*;
use crate::py_error::PyFeagiError;

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
    
    
    // TODO macro based
    pub fn register_cortical_group_center_image_camera_input<'py>(&mut self, py: Python<'_>,
                                                                  cortical_group_index: PyObject,
                                                                  number_of_channels: PyObject,
                                                                  allow_stale_data: bool,
                                                                  input_image_properties: PyImageFrameProperties,
                                                                  output_image_properties: PyImageFrameProperties)
                                                                  -> PyResult<()> {
        
        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        
        let temp = self.inner.register_cortical_group_center_image_camera_input(cortical_group_index, 
                                                                     number_of_channels, 
                                                                     allow_stale_data, 
                                                                     input_image_properties.into(), 
                                                                     output_image_properties.into()).map_err(PyFeagiError::from);
        
        match temp {
            Ok(()) => Ok(()),
            Err(e) => {Err(e.into())}
        }
    }
    
    pub fn write_image_for_center_image_camera_input<'py>(&mut self, py: Python<'_>,  cortical_group_index: PyObject, cortical_channel_index: PyObject, new_image: PyImageFrame) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, cortical_channel_index).map_err(PyFeagiError::from)?;
        
        
        let temp = self.inner.write_image_for_center_image_camera_input(
            cortical_group_index, cortical_channel_index, new_image.into());

        match temp {
            Ok(()) => Ok(()),
            Err(e) => {Err(PyValueError::new_err("TODO")) }
        }
    }
    
    pub fn encode_cached_data_into_bytes(&mut self) -> PyResult<()> {
        self.inner.encode_cached_data_into_bytes(Instant::now());
        Ok(())
    }
    
    pub fn retrieve_latest_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.retrieve_latest_bytes().unwrap().to_vec())
    }
    
}