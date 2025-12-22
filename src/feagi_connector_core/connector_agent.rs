
use std::sync::MutexGuard;
use std::time::Instant;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::types::{PyByteArray, PyBytes};
use pyo3::prelude::*;
use feagi_data_structures::{motor_cortical_units, sensor_cortical_units, FeagiDataError};
use feagi_data_structures::genomic::cortical_area::descriptors::*;
use feagi_data_structures::genomic::cortical_area::io_cortical_area_data_type::FrameChangeHandling;
use feagi_data_structures::genomic::cortical_area::io_cortical_area_data_type::PercentageNeuronPositioning;
use feagi_sensorimotor::caching::{MotorDeviceCache, SensorDeviceCache};
use feagi_sensorimotor::ConnectorAgent;
use feagi_sensorimotor::data_pipeline::{PipelineStageProperties, PipelineStagePropertyIndex};
use feagi_sensorimotor::data_types::*;
use feagi_sensorimotor::data_types::descriptors::*;
use feagi_sensorimotor::wrapped_io_data::WrappedIOData;
use crate::feagi_connector_core::data_types::descriptors::PyMiscDataDimensions;
use crate::{create_pyclass_no_clone, __base_py_class_shared};
use crate::py_error::PyFeagiError;
use crate::feagi_connector_core::data_types::descriptors::*;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data_types::*;
use crate::feagi_connector_core::wrapped_io_data::{py_any_to_wrapped_io_data, wrapped_io_data_to_py_object};
use crate::feagi_data_serialization::PyFeagiByteContainer;
use crate::feagi_data_structures::genomic::cortical_area::*;

type Pybool = bool; // ALL HAIL THE LOAD BEARING BOOLEAN

macro_rules! sensor_unit_functions {
    (
        SensoryCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_name: $snake_case_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:ident,
                    cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    cortical_area_properties: {
                        $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) =>
    {
        $(
            sensor_unit_functions!(@generate_functions
            $cortical_type_key_name,
            $snake_case_name,
            $accepted_wrapped_io_data_type
            );
        )*
    };


    //region Similar Functions
    // Helper macro to generate stage and other similar functions
    // "it's time for me to live up to my family name and face full life consequences"
    (@generate_similar_functions
        $cortical_type_key_name:ident,
        $snake_case_name:expr,
        $wrapped_data_type:ident
    ) => {
        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {

                pub fn [<sensor_ $snake_case_name _write>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    data: &Bound<'_, PyAny>,
                ) -> PyResult<()> {


                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let data: WrappedIOData = py_any_to_wrapped_io_data(py, data).map_err(PyFeagiError::from)?;

                    self.get_sensor_cache().[<$snake_case_name _write>](group, channel_index, data).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $snake_case_name _read_postprocessed_cache_value>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_sensor_cache().[<$snake_case_name _read_postprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<sensor_ $snake_case_name _get_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32
                ) -> PyResult<Py<PyPipelineStageProperties>>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();

                    let boxed_stage = self.get_sensor_cache().[<$snake_case_name _get_single_stage_properties>](group, channel_index, pipeline_stage_property_index).map_err(PyFeagiError::from)?;
                    let py_stage = PyPipelineStageProperties::from_box_to_parent_typed(py, boxed_stage)?;
                    Ok(py_stage)
                }

                pub fn [<sensor_ $snake_case_name _get_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<Vec<pyo3::Py<PyPipelineStageProperties>>>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let boxed_stages = self.get_sensor_cache().[<$snake_case_name _get_all_stage_properties>](group, channel_index).map_err(PyFeagiError::from)?;
                    PyPipelineStageProperties::from_vec_box_to_vec_parent_typed(py, boxed_stages)
                }


                pub fn [<sensor_ $snake_case_name _update_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties> // TODO move to bound
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_sensor_cache().[<$snake_case_name _update_single_stage_properties>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $snake_case_name _update_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_sensor_cache().[<$snake_case_name _update_all_stage_properties>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;

                    Ok(())
                }


                pub fn [<sensor_ $snake_case_name _replace_single_stage>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties> // TODO move to bound
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_sensor_cache().[<$snake_case_name _replace_single_stage>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }


                pub fn [<sensor_ $snake_case_name _replace_all_stages>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_sensor_cache().[<$snake_case_name _replace_all_stages>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }


                pub fn [<sensor_ $snake_case_name _removing_all_stages>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    self.get_sensor_cache().[<$snake_case_name _removing_all_stages>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(())
                }

             }
        }
    };
    //endregion


    // Arm for WrappedIOType::Boolean
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        Boolean
    ) => {


        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }

        }
        // NOTE: Used the type Pybool at the to work. Fucking Cursed.
        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, bool);
    };

    // Arm for WrappedIOType::Percentage
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        Percentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, Percentage);
    };

    // Arm for WrappedIOType::Percentage_3D
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        Percentage_3D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, Percentage3D);
    };

    // Arm for WrappedIOType::SignedPercentage_4D
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        SignedPercentage_4D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, SignedPercentage4D);
    };

    // Arm for WrappedIOType::SegmentedImageFrame
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        SegmentedImageFrame
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: &pyo3::Bound<PyFrameChangeHandling>,
                    input_image_properties: &pyo3::Bound<PyImageFrameProperties>,
                    segmented_image_properties: &pyo3::Bound<PySegmentedImageFrameProperties>,
                    initial_gaze: &pyo3::Bound<PyGazeProperties>,
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = PyFrameChangeHandling::from_bound(frame_change_handling);
                    let input_image_properties: ImageFrameProperties = PyImageFrameProperties::copy_out_from_bound(input_image_properties);
                    let segmented_image_properties: SegmentedImageFrameProperties = PySegmentedImageFrameProperties::copy_out_from_bound(segmented_image_properties);
                    let initial_gaze: GazeProperties = PyGazeProperties::copy_out_from_bound(initial_gaze);

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, input_image_properties, segmented_image_properties, initial_gaze).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }


        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, SegmentedImageFrame);
    };

    // Arm for WrappedIOType::MiscData
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        MiscData
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    misc_data_dimensions: PyMiscDataDimensions,
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let misc_data_dimensions: MiscDataDimensions = misc_data_dimensions.into();

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, misc_data_dimensions).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, MiscData);
    };


    // Arm for WrappedIOType::ImageFrame
    (@generate_functions
        $sensory_unit:ident,
        $snake_case_name:expr,
        ImageFrame
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    image_properties: PyImageFrameProperties
                    ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let image_properties: ImageFrameProperties = image_properties.into();

                    self.get_sensor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, image_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, $snake_case_name, ImageFrame);
    };
}

macro_rules! motor_unit_functions {
    (
        MotorCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_name: $snake_case_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:ident,
                    cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    cortical_area_properties: {
                        $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) =>
    {
        $(
            motor_unit_functions!(@generate_functions
            $cortical_type_key_name,
            $snake_case_name,
            $accepted_wrapped_io_data_type
            );
        )*
    };

    //region Similar Functions
    // Helper macro to generate stage and other similar functions
    (@generate_similar_functions
        $cortical_type_key_name:ident,
        $snake_case_name:expr,
        $wrapped_data_type:ident
    ) => {
        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {

                pub fn [<motor_ $snake_case_name _read_preprocessed_cache_value>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_motor_cache().[<$snake_case_name _read_preprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<motor_ $snake_case_name _read_postprocessed_cache_value>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_motor_cache().[<$snake_case_name _read_postprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<motor_ $snake_case_name _get_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32
                ) -> PyResult<Py<PyPipelineStageProperties>>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();

                    let boxed_stage = self.get_motor_cache().[<$snake_case_name _get_single_stage_properties>](group, channel_index, pipeline_stage_property_index).map_err(PyFeagiError::from)?;
                    let py_stage = PyPipelineStageProperties::from_box_to_parent_typed(py, boxed_stage)?;
                    Ok(py_stage)
                }

                pub fn [<motor_ $snake_case_name _get_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<Vec<pyo3::Py<PyPipelineStageProperties>>>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let boxed_stages = self.get_motor_cache().[<$snake_case_name _get_all_stage_properties>](group, channel_index).map_err(PyFeagiError::from)?;
                    PyPipelineStageProperties::from_vec_box_to_vec_parent_typed(py, boxed_stages)
                }

                pub fn [<motor_ $snake_case_name _update_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_motor_cache().[<$snake_case_name _update_single_stage_properties>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $snake_case_name _update_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_motor_cache().[<$snake_case_name _update_all_stage_properties>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;

                    Ok(())
                }

                pub fn [<motor_ $snake_case_name _replace_single_stage>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_motor_cache().[<$snake_case_name _replace_single_stage>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $snake_case_name _replace_all_stages>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_motor_cache().[<$snake_case_name _replace_all_stages>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $snake_case_name _removing_all_stages>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    self.get_motor_cache().[<$snake_case_name _removing_all_stages>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(())
                }

            }
        }
    };
    //endregion

    // Arm for WrappedIOType::GazeProperties
    (@generate_functions
        $motor_unit:ident,
        $snake_case_name:expr,
        GazeProperties
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    eccentricity_z_neuron_resolution: u32,
                    modulation_z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let eccentricity_z_neuron_resolution: NeuronDepth = eccentricity_z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let modulation_z_neuron_resolution: NeuronDepth = modulation_z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, eccentricity_z_neuron_resolution, modulation_z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, $snake_case_name, GazeProperties);

    };

    // Arm for WrappedIOType::Percentage
    (@generate_functions
        $motor_unit:ident,
        $snake_case_name:expr,
        Percentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, $snake_case_name, Percentage);

    };

    // Arm for WrappedIOType::Percentage3D
    (@generate_functions
        $motor_unit:ident,
        $snake_case_name:expr,
        Percentage_3D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, $snake_case_name, Percentage3D);

    };

    // Arm for WrappedIOType::SignedPercentage
    (@generate_functions
        $motor_unit:ident,
        $snake_case_name:expr,
        SignedPercentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, $snake_case_name, SignedPercentage);
    };

    // Arm for WrappedIOType::MiscData
    (@generate_functions
        $motor_unit:ident,
        $snake_case_name:expr,
        MiscData
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $snake_case_name _register>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: &pyo3::Bound<PyFrameChangeHandling>,
                    misc_data_dimensions: &pyo3::Bound<PyMiscDataDimensions>,
                    ) -> PyResult<()>
                {

                    let group: CorticalGroupIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = PyFrameChangeHandling::from_bound(frame_change_handling);
                    let misc_data_dimensions: MiscDataDimensions = PyMiscDataDimensions::copy_out_from_bound(misc_data_dimensions);

                    self.get_motor_cache().[<$snake_case_name _register>](group, number_channels, frame_change_handling, misc_data_dimensions).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }
        motor_unit_functions!(@generate_similar_functions $motor_unit, $snake_case_name, MiscData);
    };

}

create_pyclass_no_clone!(PyConnectorAgent, ConnectorAgent, "ConnectorAgent");

impl PyConnectorAgent {
    fn get_sensor_cache(&self) -> MutexGuard<SensorDeviceCache> {
        self.inner.get_sensor_cache()
    }

    fn get_motor_cache(&self) -> MutexGuard<MotorDeviceCache> {
        self.inner.get_motor_cache()

    }
}


/// Initialize Rust tracing logging (call once from Python)
#[pyfunction]
pub fn init_rust_logging() {
    use std::sync::OnceLock;
    static INIT: OnceLock<()> = OnceLock::new();

    INIT.get_or_init(|| {
        use tracing_subscriber::{fmt, EnvFilter};

        // Default to INFO level if RUST_LOG not set
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .init();
    });
}

#[pymethods]
impl PyConnectorAgent {
    #[new]
    pub fn new() -> Self {
        PyConnectorAgent {
            inner: ConnectorAgent::new(),
        }
    }

    /// Export all registered device capabilities as JSON string in new format
    /// 
    /// Returns a JSON string containing all registered sensors and motors with their
    /// configurations including pipeline stages and friendly names.
    /// 
    /// # Returns
    /// JSON string in format: {"capabilities": {"input": {...}, "output": {...}}}
    pub fn export_capabilities_json(&self, _py: Python<'_>) -> PyResult<String> {
        let json_value = self.inner.export_device_registrations_as_config_json()
            .map_err(PyFeagiError::from)?;
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| PyFeagiError::from(FeagiDataError::SerializationError(e.to_string())))
            .map_err(Into::into)
    }
    
    /// Import capabilities from JSON string (devices must be registered first!)
    /// 
    /// Parses JSON and updates pipeline stages and friendly names for already-registered devices.
    /// Devices must be registered first using the appropriate register functions (e.g., sensor_simple_vision_register).
    /// 
    /// # Arguments
    /// * `json_str` - JSON string in new capabilities format
    /// 
    /// # Raises
    /// * `FeagiError` - If JSON is malformed or references unregistered devices
    pub fn import_capabilities_json(&mut self, json_str: &str, py: Python<'_>) -> PyResult<()> {
        py.allow_threads(|| {
            self.inner.import_device_registrations_from_config_json(json_str)
                .map_err(PyFeagiError::from)?;
            Ok(())
        })
    }

    /// Encode all cached sensor data to bytes
    /// 
    /// Encodes all sensor data that has been written to cache into neuron voxel format
    /// and then serializes to FeagiByteContainer. This should be called after writing
    /// sensor data and before sending to FEAGI.
    pub fn sensors_encode_cached_sensor_data_to_bytes(&mut self) -> PyResult<()> {
        use std::time::Instant;

        let mut sensor_cache = self.get_sensor_cache();
        
        // Get current time for burst
        let time_of_burst = Instant::now();
        
        // Encode all sensors to neurons
        sensor_cache.encode_all_sensors_to_neurons(time_of_burst)
            .map_err(PyFeagiError::from)?;
        
        // Encode neurons to bytes
        sensor_cache.encode_neurons_to_bytes()
            .map_err(PyFeagiError::from)?;
        
        Ok(())
    }

    pub fn sensors_read_bytes(&mut self) -> PyResult<Vec<u8>> {
        let mut sensor_cache = self.get_sensor_cache();
        let byte_container = sensor_cache.get_feagi_byte_container();;
        let bytes = byte_container.get_byte_ref().to_vec();
        Ok(bytes)
    }

    /// Can take in a BytesArray (faster) or Bytes. Loads into rust memory and ensures the structure is sound.
    pub fn motors_load_in_bytes_and_verify(&mut self, py: Python<'_>, obj: &Bound<PyAny>) -> PyResult<()> {
        if let Ok(bytes) = Bound::cast::<PyByteArray>(obj) {
            let byte_data = bytes.to_vec();
            let mut motor_cache = self.get_motor_cache();
            let mut byte_container = motor_cache.get_feagi_byte_container_mut();;
            byte_container.try_write_data_by_ownership_to_container_and_verify(byte_data).map_err(PyFeagiError::from)?;
        }
        else if let Ok(bytes) = Bound::cast::<PyBytes>(obj) {
            let byte_data = bytes.extract::<&[u8]>()?;
            let mut motor_cache = self.get_motor_cache();
            let mut byte_container = motor_cache.get_feagi_byte_container_mut();;
            byte_container.try_write_data_by_copy_and_verify(byte_data).map_err(PyFeagiError::from)?;
        }
        Err(FeagiDataError::BadParameters("Expected preferably a ByteArray or Bytes!".into())).map_err(PyFeagiError::from)?
    }

    pub fn motors_decode_cached_byte_data_to_motor(&mut self) -> PyResult<()> {
        let mut motor_cache = self.get_motor_cache();
        motor_cache.try_decode_bytes_to_neural_data().map_err(PyFeagiError::from)?;
        motor_cache.try_decode_neural_data_into_cache(Instant::now()).map_err(PyFeagiError::from)?;
        Ok(())
    }


    // While technically possible, we are going to discourage grabbing the FeagiByteContainer directly and
    // instead push to use the above methods to access the byte data, as they make use of
    // internal optimizations
    /*
/// Get the encoded sensor byte container
///
/// Returns the FeagiByteContainer after encoding. Call sensors_encode_cached_data_to_bytes()
/// first to encode the data.
pub fn sensor_get_byte_container(&self) -> PyResult<PyFeagiByteContainer> {
    use crate::feagi_data_serialization::PyFeagiByteContainer;

    let sensor_cache = self.get_sensor_cache();
    let byte_container = sensor_cache.get_feagi_byte_container();

    // Convert to PyFeagiByteContainer (clone the inner FeagiByteContainer)
    // PyFeagiByteContainer has pub(crate) inner field, so we can create it directly
    Ok(PyFeagiByteContainer {
        inner: byte_container.clone()
    })

        /// Get the encoded motor byte container
///
/// Returns the FeagiByteContainer after encoding. Call motors_encode_cached_data_to_bytes()
/// first to encode the data.
pub fn motor_get_byte_container(&self) -> PyResult<PyFeagiByteContainer> {
    use crate::feagi_data_serialization::PyFeagiByteContainer;

    let motor_cache = self.get_motor_cache();
    let byte_container = motor_cache.get_feagi_byte_container();

    // Convert to PyFeagiByteContainer
    Ok(PyFeagiByteContainer {
        inner: byte_container.clone()
    })
}
}

 */

}

sensor_cortical_units!(sensor_unit_functions);

motor_cortical_units!(motor_unit_functions);

