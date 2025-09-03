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

macro_rules! define_cortical_group_functions {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    snake_case_identifier: $snake_case_identifier:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:ident,
                }
            ),* $(,)?
        }
    ) => {
        
        
        
        
        
        $(
            // Generate function conditionally based on default_coder_type
            define_cortical_group_functions!(@generate_functions 
                $snake_case_identifier,
                $cortical_type_key_name,
                $default_coder_type
            );
        )*
    };

        // Generate function for F32Normalized0To1_Linear coder type
    (@generate_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, F32Normalized0To1_Linear) => {
        paste::paste! {
            #[doc = "Register cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                number_of_channels: PyObject,
                allow_stale_data: bool,
                neuron_resolution: usize,
                lower_bound: f32,
                upper_bound: f32) -> PyResult<()> {

                let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

                self.inner.[<register_cortical_group_ $snake_case_identifier>](cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound)
                    .map_err(PyFeagiError::from)?;
                Ok(())
            }

            #[doc = "Store data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject,
                new_float: f32
            ) -> PyResult<()> {

                let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                self.inner.[<store_ $snake_case_identifier>](cortical_group, device_channel, new_float)
                    .map_err(PyFeagiError::from)?;
                Ok(())
            }

            #[doc = "Read most recent data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject
                ) -> PyResult<f32> {

                    let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                    let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                    let result = self.inner.[<read_ $snake_case_identifier>](cortical_group, device_channel)
                        .map_err(PyFeagiError::from)?;
                    Ok(result)
            }

            /*
            #[doc = "Set Pipeline Processing Stages of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<set_stages_ $snake_case_identifier>](&mut self,
                cortical_group: CorticalGroupIndex,
                device_channel: CorticalChannelIndex,
                new_stages: Vec<Box<dyn StreamCacheStage + Sync + Send>>) -> Result<(), FeagiDataError> {
                    let sensor_type = SensorCorticalType::$cortical_type_key_name;
                    self.set_processors_for_channel(sensor_type, cortical_group, device_channel, new_stages)
            }
            */

        }
    };

    (@generate_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, F32NormalizedM1To1_SplitSignDivided) => {
        paste::paste! {
            #[doc = "Register cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                number_of_channels: PyObject,
                allow_stale_data: bool,
                neuron_resolution: usize,
                lower_bound: f32,
                upper_bound: f32) -> PyResult<()> {

                let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

                self.inner.[<register_ $snake_case_identifier>](cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound)
                    .map_err(PyFeagiError::from)?;
                Ok(())
            }

            #[doc = "Store data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject,
                new_float: f32
            ) -> PyResult<()> {

                let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                self.inner.[<store_ $snake_case_identifier>](cortical_group, device_channel, new_float)
                    .map_err(PyFeagiError::from)?;
                Ok(())
            }

            #[doc = "Read most recent data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject
                ) -> PyResult<f32> {

                    let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                    let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                    let result = self.inner.[<read_ $snake_case_identifier>](cortical_group, device_channel)
                        .map_err(PyFeagiError::from)?;
                    Ok(result)
            }

            /*
            #[doc = "Set Pipeline Processing Stages of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<set_stages_ $snake_case_identifier>](&mut self,
                cortical_group: CorticalGroupIndex,
                device_channel: CorticalChannelIndex,
                new_stages: Vec<Box<dyn StreamCacheStage + Sync + Send>>) -> Result<(), FeagiDataError> {
                    let sensor_type = SensorCorticalType::$cortical_type_key_name;
                    self.set_processors_for_channel(sensor_type, cortical_group, device_channel, new_stages)
            }
            */

        }
    };

    (@generate_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, ImageFrame) => {
        paste::paste! {
            #[doc = "Register cortical group for " $snake_case_identifier " sensor"]
            pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>,
            cortical_group_index: PyObject, number_of_channels: PyObject, allow_stale_data: bool,
            input_image_properties: PyImageFrameProperties,
            output_image_properties: PyImageFrameProperties) -> PyResult<()> {

                let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

                self.inner.[<register_ $snake_case_identifier>](cortical_group_index,
                                                               number_of_channels,
                                                               allow_stale_data,
                                                               input_image_properties.into(),
                                                               output_image_properties.into())
                    .map_err(PyFeagiError::from)?;
                Ok(())
            }

            #[doc = "Store data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject,
                new_image: PyImageFrame) -> PyResult<()> {

                    let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                    let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                    self.inner.[<store_ $snake_case_identifier>](cortical_group, device_channel, new_image.into())
                        .map_err(PyFeagiError::from)?;
                    Ok(())
            }

            #[doc = "Read most recent data of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject
                ) -> PyResult<PyImageFrame> {

                    let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
                    let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

                    let result = self.inner.[<read_ $snake_case_identifier>](cortical_group, device_channel)
                        .map_err(PyFeagiError::from)?;
                    Ok(result.into())
            }

            /*
            #[doc = "Set Pipeline Processing Stages of cortical group for the " $snake_case_identifier " sensor"]
            pub fn [<set_stages_ $snake_case_identifier>](&mut self, py: Python<'_>,
                cortical_group: PyObject,
                device_channel: PyObject
                new_stages: Vec<Box<dyn StreamCacheStage + Sync + Send>>) -> Result<(), FeagiDataError> {
                    let sensor_type = SensorCorticalType::ImageCameraCenter;
                    self.set_processors_for_channel(sensor_type, cortical_group, device_channel, new_stages)
            }
             */

        }
    };
    // Segmented Image Frame does not get its own!

    // Fallback for other coder types - no function generated
    (@generate_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, $default_coder_type:ident) => {}
        // No functions generated for this type!
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

    //region Specific Sensor Functions

    //region F32Normalized0To1_Linear

    pub fn register_f32_normalized_0_to_1_linear(&mut self, py: Python<'_>,
                                                 sensor_cortical_type: PySensorCorticalType,
                                                 cortical_group: PyObject,
                                                 number_of_channels: PyObject,
                                                 allow_stale_data: bool,
                                                 neuron_resolution: usize,
                                                 lower_bound: f32,
                                                 upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        match sensor_cortical_type {
            SensorCorticalType::Infrared => {
                self.inner.register_cortical_group_infrared(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::ReverseInfrared => {
                self.inner.register_cortical_group_infrared_inverted(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::DigitalGPIOInput => {
                self.inner.register_cortical_group_gpio_digital(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::Proximity => {
                self.inner.register_cortical_group_proximity(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::Battery => {
                self.inner.register_cortical_group_battery_gauge(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn store_f32_normalized_0_to_1_linear(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject,
                                              new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        match sensor_cortical_type {
            SensorCorticalType::Infrared => {
                self.inner.store_infrared(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::ReverseInfrared => {
                self.inner.store_infrared_inverted(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::DigitalGPIOInput => {
                self.inner.store_gpio_digital(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::Proximity => {
                self.inner.store_proximity(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            SensorCorticalType::Battery => {
                self.inner.store_battery_gauge(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn read_f32_normalized_0_to_1_linear(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(match sensor_cortical_type {
            SensorCorticalType::Infrared => {
                self.inner.read_infrared(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            SensorCorticalType::ReverseInfrared => {
                self.inner.read_infrared_inverted(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            SensorCorticalType::DigitalGPIOInput => {
                self.inner.read_gpio_digital(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            SensorCorticalType::Proximity => {
                self.inner.read_proximity(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            SensorCorticalType::Battery => {
                self.inner.read_battery_gauge(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        })
    }


    //endregion

    //region F32NormalizedM1To1_SplitSignDivided

    pub fn register_f32_normalized_m1_to_1_split_sign_divided(&mut self, py: Python<'_>,
                                                 sensor_cortical_type: PySensorCorticalType,
                                                 cortical_group: PyObject,
                                                 number_of_channels: PyObject,
                                                 allow_stale_data: bool,
                                                 neuron_resolution: usize,
                                                 lower_bound: f32,
                                                 upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        match sensor_cortical_type {
            /* // TODO what happened?
            SensorCorticalType::ServoPosition => {
                self.inner.(cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound).map_err(PyFeagiError::from)?;
            }

             */
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn store_f32_normalized_m1_to_1_split_sign_divided(&mut self, py: Python<'_>,
                                              sensor_cortical_type: PySensorCorticalType,
                                              cortical_group: PyObject,
                                              device_channel: PyObject,
                                              new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        match sensor_cortical_type {
            SensorCorticalType::ServoPosition => {
                self.inner.store_servo_position(cortical_group, device_channel, new_float).map_err(PyFeagiError::from)?;
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn read_f32_normalized_m1_to_1_split_sign_divided(&mut self, py: Python<'_>,
                                             sensor_cortical_type: PySensorCorticalType,
                                             cortical_group: PyObject,
                                             device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(match sensor_cortical_type {
            SensorCorticalType::ServoPosition => {
                self.inner.read_servo_position(cortical_group, device_channel).map_err(PyFeagiError::from)?
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        })
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

        match sensor_cortical_type {
            SensorCorticalType::ImageCameraCenter => {
                self.inner.register_image_camera_center(cortical_group, number_of_channels, allow_stale_data, input_image_properties, output_image_properties).map_err(PyFeagiError::from)?;
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn store_image_frame(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType, cortical_group: PyObject, device_channel: PyObject, new_image: PyImageFrame) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let new_image: ImageFrame = new_image.into();

        match sensor_cortical_type {
            SensorCorticalType::ImageCameraCenter => {
                self.inner.store_image_camera_center(cortical_group, device_channel, new_image).map_err(PyFeagiError::from)?;
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        }
        Ok(())
    }

    pub fn read_image_frame(&mut self, py: Python<'_>,
                                                          sensor_cortical_type: PySensorCorticalType,
                                                          cortical_group: PyObject,
                                                          device_channel: PyObject) -> PyResult<(PyImageFrame)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(match sensor_cortical_type {
            SensorCorticalType::ImageCameraCenter => {
                self.inner.read_image_camera_center(cortical_group, device_channel).map_err(PyFeagiError::from)?.into()
            }
            _ => {Err(PyValueError::new_err("SensorCortical type is not supported"))?}
        })
    }

    //endregion

    //region Segmented Image Camera Manual Functions

    pub fn register_image_camera_with_peripheral<'py>(&mut self, py: Python<'_>, cortical_group_index: PyObject,
                                                                         number_of_channels: PyObject,
                                                                         allow_stale_data: bool, input_image_properties: PyImageFrameProperties,
                                                                         output_image_properties: PySegmentedImageFrameProperties,
                                                                         gaze: PyGazeProperties) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let temp =  self.inner.register_image_camera_with_peripheral(
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

        let temp = self.inner.store_segmented_image_camera(new_image.into(), cortical_group_index, cortical_channel_index);

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