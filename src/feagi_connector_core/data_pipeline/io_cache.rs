use std::time::Instant;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_connector_core::caching::IOCache;
use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_data_structures::data::descriptors::{ImageFrameProperties, MiscDataDimensions};
use feagi_data_structures::data::{ImageFrame, MiscData, Percentage4D};
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex};
use feagi_data_structures::genomic::{MotorCorticalType, SensorCorticalType};
use pyo3::types::PyBytes;
use crate::feagi_connector_core::data_pipeline::{extract_pipeline_stage_from_py, PyPipelineStage};
use crate::feagi_data_structures::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_data_structures::data::{PyImageFrame, PyMiscData, PySegmentedImageFrame};
use crate::feagi_data_structures::genomic::descriptors::{PyCorticalChannelCount, PyCorticalChannelIndex, PyCorticalGroupIndex};
use crate::feagi_data_structures::genomic::{PyMotorCorticalType, PySensorCorticalType};
use crate::py_error::PyFeagiError;

macro_rules! convert_common_sensor_parameters {
    (sensor_cortical_type: $sensor_cortical_type:ident, cortical_group: $cortical_group:ident, number_of_channels: $number_of_channels:ident) => {
        let sensor_cortical_type: SensorCorticalType = $sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, $cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, $number_of_channels).map_err(PyFeagiError::from)?;
    };
}


#[pyclass]
#[pyo3(name="IOCache")]
pub struct PyIOCache {
    inner: IOCache
}

#[pymethods]
impl PyIOCache {
    #[new]
    pub fn new() -> Self {
        PyIOCache {
            inner: IOCache::new(),
        }
    }

    //region Sensor Interfaces


    pub fn sensor_encode_cached_data_into_bytes(&mut self) -> PyResult<()> {
        self.inner.sensor_encode_cached_data_into_bytes(Instant::now());
        Ok(())
    }

    pub fn retrieve_latest_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.sensor_retrieve_latest_bytes().unwrap().to_vec())
    }



    //region Common

    //region Percentage

    pub fn register_percentage_sensor(&mut self, py: Python<'_>,
                               sensor_cortical_type: PySensorCorticalType,
                               cortical_group: PyObject,
                               number_of_channels: PyObject,
                               neuron_resolution: u32,
                               lower_bound: f32,
                               upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.register_percentage_sensor(sensor_cortical_type, cortical_group, number_of_channels,
                                       neuron_resolution.into(),
                                       lower_bound..upper_bound).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_percentage_sensor(&mut self, py: Python<'_>,
                            sensor_cortical_type: PySensorCorticalType,
                            cortical_group: PyObject,
                            device_channel: PyObject,
                            new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        self.inner.store_percentage_sensor(sensor_cortical_type, cortical_group, device_channel,
                                    new_float).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_cache_percentage_sensor(&mut self, py: Python<'_>,
                                 sensor_cortical_type: PySensorCorticalType,
                                 cortical_group: PyObject,
                                 device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(self.inner.read_cache_percentage_sensor(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?)

    }

    pub fn set_pipeline_stage_percentage_sensor(&mut self, py: Python<'_>,sensor_cortical_type: PySensorCorticalType,
                                         cortical_group: PyObject, device_channel: PyObject, new_stage: Py<PyPipelineStage>, stage_index: u32) -> PyResult<()> {
        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;

        self.inner.set_pipeline_stage_percentage_sensor(sensor_cortical_type, cortical_group, device_channel, stage, stage_index.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    //endregion

    //region SignedPercentage

    pub fn register_signed_percentage_sensor(&mut self, py: Python<'_>,
                                sensor_cortical_type: PySensorCorticalType,
                                cortical_group: PyObject,
                                number_of_channels: PyObject,
                                neuron_resolution: u32,
                                lower_bound: f32,
                                upper_bound: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.register_signed_percentage_sensor(sensor_cortical_type, cortical_group, number_of_channels,
                                        neuron_resolution.into(),
                                        lower_bound..upper_bound).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_signed_percentage_sensor(&mut self, py: Python<'_>,
                             sensor_cortical_type: PySensorCorticalType,
                             cortical_group: PyObject,
                             device_channel: PyObject,
                             new_float: f32) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        self.inner.store_signed_percentage_sensor(sensor_cortical_type, cortical_group, device_channel,
                                     new_float).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_signed_percentage_sensor(&mut self, py: Python<'_>,
                            sensor_cortical_type: PySensorCorticalType,
                            cortical_group: PyObject,
                            device_channel: PyObject) -> PyResult<(f32)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        Ok(self.inner.read_cache_signed_percentage_sensor(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?)


    }

    pub fn set_pipeline_stage_signed_percentage_sensor(&mut self, py: Python<'_>,sensor_cortical_type: PySensorCorticalType,
                                          cortical_group: PyObject, device_channel: PyObject, new_stage: Py<PyPipelineStage>, stage_index: u32) -> PyResult<()> {
        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;

        self.inner.set_pipeline_stage_signed_percentage_sensor(sensor_cortical_type, cortical_group, device_channel, stage, stage_index.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }


    //endregion

    //region ImageFrame

    pub fn register_image_frame_sensor(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType,
                                cortical_group: PyObject, number_of_channels: PyObject,
                                input_image_properties: PyImageFrameProperties,
                                output_image_properties: PyImageFrameProperties) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        let input_image_properties: ImageFrameProperties = input_image_properties.into();
        let output_image_properties: ImageFrameProperties = output_image_properties.into();
        self.inner.register_image_frame_sensor(sensor_cortical_type, cortical_group, number_of_channels,
                                        input_image_properties,
                                        output_image_properties).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_image_frame_sensor(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType, cortical_group: PyObject, device_channel: PyObject, new_image: PyImageFrame) -> PyResult<()> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        let new_image: ImageFrame = new_image.into();
        self.inner.store_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel,
                                     new_image).map_err(PyFeagiError::from)?;

        Ok(())
    }

    pub fn read_image_frame_sensor(&mut self, py: Python<'_>,
                            sensor_cortical_type: PySensorCorticalType,
                            cortical_group: PyObject,
                            device_channel: PyObject) -> PyResult<(PyImageFrame)> {

        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        //Ok(self.inner.read_cache_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?.into())
        Ok(self.inner.read_cache_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel).unwrap().into())
    }

    pub fn set_pipeline_stage_image_frame_sensor(&mut self, py: Python<'_>,sensor_cortical_type: PySensorCorticalType,
                                          cortical_group: PyObject, device_channel: PyObject, new_stage: Py<PyPipelineStage>, stage_index: u32) -> PyResult<()> {
        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;

        self.inner.set_pipeline_stage_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel, stage, stage_index.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    /*

    pub fn clone_pipeline_stage_image_frame(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType,
                                            cortical_group: PyObject, device_channel: PyObject, stage_index: u32) -> PyResult<PyImageFrameQuickDiffStage> {
        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let result = self.inner.clone_pipeline_stage_image_frame(sensor_cortical_type, cortical_group, device_channel, stage_index.into());
        match result {
            Ok(result) => Ok({
                let box_result = result;
                let unboxed: ImageFrameQuickDiffStage = box_result.as
                let py_wrapped: PyImageFrameQuickDiffStage = unboxed.into();
                return Ok(py_wrapped);
            }),
        }
    }

     */

    //endregion

    //region MiscData

    pub fn register_misc_data_sensor(&mut self, py: Python<'_>,
                                       cortical_group: PyObject, number_of_channels: PyObject,
                                       dimensions: (u32, u32, u32)) -> PyResult<()> {

        let dimensions = MiscDataDimensions::new(dimensions.0, dimensions.1, dimensions.2).map_err(PyFeagiError::from)?;
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.register_misc_data_sensor(cortical_group, number_of_channels,
                                             dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn store_misc_data_sensor(&mut self, py: Python<'_>, cortical_group: PyObject, device_channel: PyObject, new_data: PyMiscData) -> PyResult<()> {

        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        let new_data: MiscData = new_data.into();
        self.inner.store_misc_data_sensor(cortical_group, device_channel,
                                            new_data).map_err(PyFeagiError::from)?;

        Ok(())
    }

    pub fn read_cache_misc_data_sensor(&mut self, py: Python<'_>,
                                   cortical_group: PyObject,
                                   device_channel: PyObject) -> PyResult<(PyMiscData)> {

        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        //Ok(self.inner.read_cache_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?.into())
        Ok(self.inner.read_cache_misc_data_sensor(cortical_group, device_channel).unwrap().into())
    }

    pub fn set_pipeline_stage_misc_data_sensor(&mut self, py: Python<'_>, cortical_group: PyObject,
                                               device_channel: PyObject, new_stage: Py<PyPipelineStage>, stage_index: u32) -> PyResult<()> {
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;

        self.inner.set_pipeline_stage_misc_data_sensor(cortical_group, device_channel, stage, stage_index.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    /*

    pub fn clone_pipeline_stage_image_frame(&mut self, py: Python<'_>, sensor_cortical_type: PySensorCorticalType,
                                            cortical_group: PyObject, device_channel: PyObject, stage_index: u32) -> PyResult<PyImageFrameQuickDiffStage> {
        let sensor_cortical_type: SensorCorticalType = sensor_cortical_type.into();
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let result = self.inner.clone_pipeline_stage_image_frame(sensor_cortical_type, cortical_group, device_channel, stage_index.into());
        match result {
            Ok(result) => Ok({
                let box_result = result;
                let unboxed: ImageFrameQuickDiffStage = box_result.as
                let py_wrapped: PyImageFrameQuickDiffStage = unboxed.into();
                return Ok(py_wrapped);
            }),
        }
    }

     */

    //endregion

    //endregion

    //region Unique

    //region Segmented Image Camera

    pub fn register_image_camera_with_peripheral<'py>(&mut self, py: Python<'_>, cortical_group_index: PyObject,
                                                      number_of_channels: PyObject,
                                                      input_image_properties: PyImageFrameProperties,
                                                      output_image_properties: PySegmentedImageFrameProperties,
                                                      gaze: PyGazeProperties) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group_index).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        let temp =  self.inner.register_segmented_image_frame_sensor(
            cortical_group_index, number_of_channels,
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

        let temp = self.inner.store_segmented_image_frame_sensor(new_image.into(), cortical_group_index, cortical_channel_index);

        match temp {
            Ok(()) => Ok(()),
            Err(e) => {Err(PyValueError::new_err("TODO")) }
        }
    }


    pub fn read_cache_image_camera_with_peripheral(&mut self, py: Python<'_>,
                                                   cortical_group: PyObject,
                                                   device_channel: PyObject) -> PyResult<(PySegmentedImageFrame)> {

        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        let result = self.inner.read_cache_segmented_image_frame_sensor(cortical_group, device_channel).map_err(PyFeagiError::from);
        match result {
            Ok(frame) => Ok((frame.into())),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn set_pipeline_stage_image_camera_with_peripheral(&mut self, py: Python<'_>,
                                                           cortical_group: PyObject, device_channel: PyObject, new_stage: Py<PyPipelineStage>, stage_index: u32) -> PyResult<()> {

        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;

        self.inner.set_pipeline_stage_segmented_image_frame_sensor(cortical_group, device_channel, stage, stage_index.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn set_pipeline_stages_image_camera_with_peripheral(&mut self, py: Python<'_>,
                                                            cortical_group: PyObject, device_channel: PyObject, new_stages: Vec<Py<PyPipelineStage>>) -> PyResult<()> {
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStage + Send + Sync>> = Vec::with_capacity(new_stages.len());
        for new_stage in new_stages {
            let extracted_stage = extract_pipeline_stage_from_py(py, new_stage).map_err(PyFeagiError::from)?;
            stages.push(extracted_stage);
        };
        self.inner.set_pipeline_stages_segmented_image_frame_sensor(cortical_group, device_channel, stages).map_err(PyFeagiError::from)?;
        Ok(())

    }

    //endregion

    //endregion

    //endregion

    //region Motor Interfaces

    pub fn manually_write_byte_data_to_motor(&mut self, py: Python<'_>, bytes: Vec<u8>) -> PyResult<()> {
        // TODO this is awful, we need to relook at interface for FBS
        let byte_struct = self.inner.get_motor_byte_structure_mut();
        let byte_vec = byte_struct.borrow_data_as_mut_vec();
        byte_vec.resize(bytes.len(), 0);
        byte_vec.copy_from_slice(bytes.as_slice());
        self.inner.process_motor_byte_structure_data().map_err(PyFeagiError::from)?;
        Ok(())
    }

    //region Common

    //region MiscData

    pub fn register_misc_data_motor(&mut self, py: Python<'_>, cortical_group: PyObject, number_of_channels: PyObject,
                                    dimensions: (u32, u32, u32) ) -> PyResult<()> {

        let dimensions = MiscDataDimensions::new(dimensions.0, dimensions.1, dimensions.2).map_err(PyFeagiError::from)?;
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        self.inner.register_misc_data_motor(cortical_group, number_of_channels, dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_cache_misc_data_motor(&mut self, py: Python<'_>, cortical_group: PyObject,
                                      device_channel: PyObject)
                                      -> PyResult<(PyMiscData)> {

        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;

        //Ok(self.inner.read_cache_image_frame_sensor(sensor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?.into())
        Ok(self.inner.read_cache_misc_data_motor(cortical_group, device_channel).unwrap().into())
    }

    // TODO other methods?

    //endregion

    //region Percentage4D

    pub fn register_percentage_4d_motor(&mut self, py: Python<'_>, motor_cortical_type: PyMotorCorticalType, cortical_group: PyObject, number_of_channels: PyObject,
                                    z_depth: u32 ) -> PyResult<()> {

        let motor_cortical_type: MotorCorticalType = MotorCorticalType::from(motor_cortical_type);
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        self.inner.register_percentage_4d_data_motor(motor_cortical_type, cortical_group, number_of_channels, z_depth).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn read_cache_percentage_4d_motor(&mut self, py: Python<'_>, motor_cortical_type: PyMotorCorticalType, cortical_group: PyObject,
                                      device_channel: PyObject)
                                      -> PyResult<((f32, f32, f32, f32))> {
        let motor_cortical_type: MotorCorticalType = MotorCorticalType::from(motor_cortical_type);
        let cortical_group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, cortical_group).map_err(PyFeagiError::from)?;
        let device_channel = PyCorticalChannelIndex::try_get_from_py_object(py, device_channel).map_err(PyFeagiError::from)?;
        let result: Percentage4D = self.inner.read_cache_percentage_4d_data_motor(motor_cortical_type, cortical_group, device_channel).map_err(PyFeagiError::from)?.into();

        //Ok((result.a.get_as_0_1(), result.b.get_as_0_1(), result.c.get_as_0_1(), result.d.get_as_0_1()))
        Ok((1.0, 1.0, 1.0, 1.0))
    }

    //endregion

    //endregion

    //endregion




}