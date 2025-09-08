use std::time::Instant;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_connector_core::caching::SensorCache;
use feagi_data_structures::data::image_descriptors::ImageFrameProperties;
use feagi_data_structures::data::ImageFrame;
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex};
use feagi_data_structures::sensor_definition;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::SensorCorticalType;
use crate::feagi_data_structures::data::image_descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_data_structures::data::{PyImageFrame, PySegmentedImageFrame};
use crate::feagi_data_structures::genomic::descriptors::*;
use crate::feagi_data_structures::genomic::{PyCoreCorticalType, PySensorCorticalType};
use crate::py_error::PyFeagiError;

macro_rules! convert_common_parameters {
    (sensor_cortical_type: $sensor_cortical_type:ident, cortical_group: $cortical_group:ident, number_of_channels: $number_of_channels:ident) => {
        let sensor_cortical_type: SensorCorticalType = $sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, $cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, $number_of_channels).map_err(PyFeagiError::from)?;
    };
}

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

    //region Sensor Functions

    //region Generic Types

    //region F32Normalized0To1_Linear

    pub fn register_f32_0_to_1(&mut self, py: Python<'_>,
                                                 sensor_cortical_type: PySensorCorticalType,
                                                 cortical_group: PyObject,
                                                 number_of_channels: PyObject,
                                                 allow_stale_data: bool,
                                                 neuron_resolution: u32,
                                                 lower_bound: f32,
                                                 upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.register_f32_0_to_1(sensor_cortical_type, cortical_group, number_of_channels,
                                       allow_stale_data, neuron_resolution.into(),
                                       lower_bound..upper_bound).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_f32_0_to_1(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject,
                                              new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        self.inner.store_f32_0_to_1(sensor_cortical_type, cortical_group, device_channel,
                                    new_float).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_f32_0_to_1(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(self.inner.read_cache_f32_0_to_1(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?)

    }


    //endregion

    //region F32NormalizedM1To1_SplitSignDivided

    pub fn register_f32_m1_to_1(&mut self, py: Python<'_>,
                                                 sensor_cortical_type: PySensorCorticalType,
                                                 cortical_group: PyObject,
                                                 number_of_channels: PyObject,
                                                 allow_stale_data: bool,
                                                 neuron_resolution: u32,
                                                 lower_bound: f32,
                                                 upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.register_f32_m1_to_1(sensor_cortical_type, cortical_group, number_of_channels,
                                        allow_stale_data, neuron_resolution.into(),
                                        lower_bound..upper_bound).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_f32_m1_to_1(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject,
                                              new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        self.inner.store_f32_m1_to_1(sensor_cortical_type, cortical_group, device_channel,
                                     new_float).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_f32_m1_to_1(&mut self, py: Python<'_>,
                                             sensor_cortical_type: PySensorCorticalType,
                                             cortical_group: PyObject,
                                             device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(self.inner.read_cache_f32_m1_to_1(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?)


    }
    //endregion

    //region ImageFrame

    pub fn register_image_frame(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType,
                                cortical_group: PyObject, number_of_channels: PyObject,
                                allow_stale_data: bool,
                                input_image_properties: PyImageFrameProperties,
                                output_image_properties: PyImageFrameProperties) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        let input_image_properties: ImageFrameProperties = input_image_properties.into();
        let output_image_properties: ImageFrameProperties = output_image_properties.into();
        self.inner.register_image_frame(sensor_cortical_type, cortical_group, number_of_channels,
                                        allow_stale_data, input_image_properties,
                                        output_image_properties).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_image_frame(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType, cortical_group: PyObject, device_channel: PyObject, new_image: PyImageFrame) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        let new_image: ImageFrame = new_image.into();
        self.inner.store_image_frame(sensor_cortical_type, cortical_group, device_channel,
                                     new_image).map_err(PyFeagiError::from)?;

        Ok(())
    }

    pub fn read_image_frame(&mut self, py: Python<'_>,
                                                          sensor_cortical_type: PySensorCorticalType,
                                                          cortical_group: PyObject,
                                                          device_channel: PyObject) -> PyResult<(PyImageFrame)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(self.inner.read_cache_image_frame(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?.into())
    }

    //endregion

    //endregion

    // Manual Functions
    //region Segmented Image Camera Manual Functions

    pub fn register_image_camera_with_peripheral<'py>(&mut self, py: Python<'_>, cortical_group_index: PyObject,
                                                                         number_of_channels: PyObject,
                                                                         allow_stale_data: bool, input_image_properties: PyImageFrameProperties,
                                                                         output_image_properties: PySegmentedImageFrameProperties,
                                                                         gaze: PyGazeProperties) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        let temp =  self.inner.register_segmented_image_frame(
            cortical_group_index, number_of_channels, allow_stale_data,
            input_image_properties.into(), output_image_properties.into(),
            gaze.into()).map_err(PyFeagiError::from);

        match temp {
            Ok(()) => Ok(()),
            Err(e) => { Err(e.into()) }
        }
    }

    pub fn store_image_camera_with_peripheral<'py>(&mut self, py: Python<'_>, cortical_group_index: PyObject, cortical_channel_index: PyObject, new_image: PyImageFrame) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, cortical_channel_index).map_err(PyFeagiError::from)?;

        let temp = self.inner.store_segmented_image_frame(new_image.into(), cortical_group_index, cortical_channel_index);

        match temp {
            Ok(()) => Ok(()),
            Err(e) => {Err(PyValueError::new_err("TODO")) }
        }
    }



    //endregion

    //endregion



    pub fn encode_cached_data_into_bytes(&mut self) -> PyResult<()> {
        self.inner.encode_cached_data_into_bytes(Instant::now());
        Ok(())
    }

    pub fn retrieve_latest_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.retrieve_latest_bytes().unwrap().to_vec())
    }
}