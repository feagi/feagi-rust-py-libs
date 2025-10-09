use feagi_connector_core::data_pipeline::PipelineStagePropertyIndex;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::IOCache;
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex, NeuronDepth};
use feagi_data_structures::motor_definition;
use crate::feagi_connector_core::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_connector_core::data::PyPercentage4D;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::{extract_pipeline_stage_properties_from_py, PyPipelineStageProperties};
use crate::feagi_connector_core::wrapped_io_data::py_object_to_wrapped_io_data;
use crate::feagi_data_structures::genomic::descriptors::{PyCorticalChannelCount, PyNeuronDepth, PyCorticalChannelIndex, PyCorticalGroupIndex, PyPipelineStagePropertyIndex};
use crate::py_error::PyFeagiError;

macro_rules! motor_registrations {
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
                    wrapped_data_type: $wrapped_data_type:expr,
                    data_type: $data_type:ident,
                }
            ),* $(,)?
        }
    ) => {
        $(
            motor_registrations!(@generate_function
                $cortical_type_key_name,
                $snake_case_identifier,
                $default_coder_type,
                $wrapped_data_type,
                $data_type
            );
        )*
    };

    // Arm for Percentage with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage2D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage2D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage2D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage2D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage2D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage2D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage2D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage2D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage3D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage3D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage3D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage3D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage3D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage3D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage3D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage3D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage4D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage4D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage4D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage4D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage4D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage4D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for Percentage4D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        Percentage4D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage2D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage2D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage2D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage2D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage2D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage2D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage2D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage2D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage3D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage3D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage3D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage3D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage3D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage3D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage3D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage3D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage4D with Absolute Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage4D_Absolute_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage4D with Absolute Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage4D_Absolute_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage4D with Incremental Linear encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage4D_Incremental_Linear,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for SignedPercentage4D with Incremental Fractional encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        SignedPercentage4D_Incremental_Fractional,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for MiscData with Absolute encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        MiscData_Absolute,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for MiscData with Incremental encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        MiscData_Incremental,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for ImageFrame with Absolute encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        ImageFrame_Absolute,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };

    // Arm for ImageFrame with Incremental encoding
    (@generate_function
        $cortical_type_key_name:ident,
        $snake_case_identifier:expr,
        ImageFrame_Incremental,
        $wrapped_data_type:expr,
        $data_type:ident
    ) => {
        ::paste::paste! {
            pub fn [<motor_register_ $snake_case_identifier>](
                &mut self,
                py: Python<'_>,
                group: PyObject,
                number_of_channels: PyObject,
                z_neuron_depth: PyObject
            ) -> PyResult<()>
            {
                let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
                let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
                let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

                self.inner.[<motor_register_ $snake_case_identifier>](group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
                Ok(())
            }
         }
    };
}


#[pyclass]
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


    /*
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
        
        // Extract PyObject to Py<PyPipelineStageProperties>
        let stage_py: Py<PyPipelineStageProperties> = stage.extract(py)?;
        let pipeline_stage_properties = extract_pipeline_stage_properties_from_py(py, stage_py).map_err(PyFeagiError::from)?;

        self.inner.sensor_update_stage_segmented_vision_absolute(cortical_group_index, cortical_channel_index,
                                                                 stage_property_index, pipeline_stage_properties)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }


     */
    /*
    pub fn sensor_get_bytes(&mut self, py: Python<'_>) -> PyResult<Vec<u8>> {
        let bytes = self.inner.sensor_get_bytes().map_err(PyFeagiError::from)?;
        Ok(bytes.to_vec())

    }

    //endregion


    //region Motors

    pub fn motor_send_bytes(&mut self, py: Python<'_>, bytes: Vec<u8>) -> PyResult<()> {
        self.inner.motor_send_bytes(&bytes).map_err(PyFeagiError::from)?;
        Ok(())
    }

     */

    //region Motors

    // Generate all motor registration methods using the macro
    motor_definition!(motor_registrations);

    //region Gaze

    pub fn motor_read_post_processed_gaze_absolute(&mut self, py: Python<'_>, group: PyObject, channel: PyObject) -> PyResult<PyPercentage4D> {
        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let percentage_4d = self.inner.motor_try_read_postprocessed_cached_value_gaze_absolute_linear(cortical_group_index, cortical_channel_index)
            .map_err(PyFeagiError::from)?;
        
        Ok(PyPercentage4D::from(percentage_4d))
    }

    /*
    pub fn motor_add_callback_gaze_absolute(&mut self, py: Python<'_>, group: PyObject, channel: PyObject, callback: PyObject) -> PyResult<PyFeagiSignalIndex> {
        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let cortical_channel_index: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        // Create a closure that calls the Python callback
        let py_callback = callback.clone();
        let rust_callback = move |_data: &()| {
            Python::with_gil(|py| {
                if let Err(e) = py_callback.call0(py) {
                    eprintln!("Error calling Python callback: {:?}", e);
                }
            });
        };

        let signal_index: FeagiSignalIndex = self.inner.motor_add_callback_gaze_absolute(cortical_group_index, cortical_channel_index, rust_callback)
            .map_err(PyFeagiError::from)?;
        
        Ok(PyFeagiSignalIndex::from(signal_index))
    }

     */

    //endregion


}