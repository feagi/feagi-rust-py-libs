use feagi_core_data_structures_and_processing::genomic_structures::CorticalGroupingIndex;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_processing::SensorCache;
use crate::genomic_structures::PyCorticalGroupingIndex;
use crate::io_data::image_descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};

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
    
    
}