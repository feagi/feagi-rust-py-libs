//! Rust PYO3 Compiled Python Module
//! All docs pertaining to python exposed modules must 
//! be reflected to the 'feagi_data_processing.pyi.template' file!

//mod miscellaneous_types;
//mod neuron_data;
//mod io_processing;
//mod genomic_structures;
//mod io_data;
mod feagi_data_structures;
mod py_error;
mod macro_helpers;
mod feagi_data_serialization;
mod feagi_connector_core;

use pyo3::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn check_submodule_exists(parent: &Bound<'_, PyModule>, submodule_name: &str) -> bool {
    match parent.getattr(submodule_name) {
        Ok(attr) => attr.is_instance_of::<PyModule>(),
        Err(_) => false,
    }
}

macro_rules! add_python_class {
    ($python:expr, $root_python_module:expr, $class_path:expr, $class:ty) => {
        {

            let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
            let mut current_module = $root_python_module.clone();

            for path_step in path {
                if !check_submodule_exists(&current_module, &path_step) {
                    // we need to add a submodule
                    let child_module = PyModule::new_bound($python, &path_step)?;
                    current_module.add_submodule(&child_module)?;
                    current_module = child_module;
                }
                else {
                    // child module already exists. Switch to it
                    let child_module = current_module.getattr(&path_step)?;
                    current_module = child_module.downcast::<PyModule>()?.clone();
                }
            }

            current_module.add_class::<$class>()?;
        }
    };
}

macro_rules! add_python_function {
    ($python:expr, $root_python_module:expr, $class_path:expr, $function:ty) => {
        {

            let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
            let mut current_module = $root_python_module.clone();

            for path_step in path {
                if !check_submodule_exists(&current_module, &path_step) {
                    // we need to add a submodule
                    let child_module = PyModule::new_bound($python, &path_step)?;
                    current_module.add_submodule(&child_module)?;
                    current_module = child_module;
                }
                else {
                    // child module already exists. Switch to it
                    let child_module = current_module.getattr(&path_step)?;
                    current_module = child_module.downcast::<PyModule>()?.clone();
                }
            }

            current_module.add_function::<$function>()?;
        }
    };
}

// TODO the above macros can be consolidated

/// Procedural macro to generate Python wrapper methods for sensor functions
/// This macro dynamically reads from the sensor_definition! to maintain single source of truth
#[proc_macro]
pub fn generate_sensor_python_methods(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream to extract sensor definitions
    // The input should be the sensor_definition! macro invocation
    
    let expanded = quote! {
        // We'll use the sensor_definition! macro to generate wrapper methods
        // by creating a custom macro that generates Python wrappers
        
        macro_rules! generate_python_wrappers_inner {
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
                    generate_python_wrappers_inner!(@generate_wrapper_functions
                        $snake_case_identifier,
                        $cortical_type_key_name,
                        $default_coder_type
                    );
                )*
            };
            
            // Generate wrapper functions for F32Normalized0To1_Linear coder type
            (@generate_wrapper_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, F32Normalized0To1_Linear) => {
                paste::paste! {
                    pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, number_of_channels: PyObject, allow_stale_data: bool,
                        neuron_resolution: usize, lower_bound: f32, upper_bound: f32) -> PyResult<()> {
                        PySensorCache::[<register_ $snake_case_identifier>](self, py, cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound)
                    }
                    
                    pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject, new_float: f32) -> PyResult<()> {
                        PySensorCache::[<store_ $snake_case_identifier>](self, py, cortical_group, device_channel, new_float)
                    }
                    
                    pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject) -> PyResult<f32> {
                        PySensorCache::[<read_ $snake_case_identifier>](self, py, cortical_group, device_channel)
                    }
                }
            };

            // Generate wrapper functions for F32NormalizedM1To1_SplitSignDivided coder type
            (@generate_wrapper_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, F32NormalizedM1To1_SplitSignDivided) => {
                paste::paste! {
                    pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, number_of_channels: PyObject, allow_stale_data: bool,
                        neuron_resolution: usize, lower_bound: f32, upper_bound: f32) -> PyResult<()> {
                        PySensorCache::[<register_ $snake_case_identifier>](self, py, cortical_group, number_of_channels, allow_stale_data, neuron_resolution, lower_bound, upper_bound)
                    }
                    
                    pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject, new_float: f32) -> PyResult<()> {
                        PySensorCache::[<store_ $snake_case_identifier>](self, py, cortical_group, device_channel, new_float)
                    }
                    
                    pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject) -> PyResult<f32> {
                        PySensorCache::[<read_ $snake_case_identifier>](self, py, cortical_group, device_channel)
                    }
                }
            };

            // Generate wrapper functions for ImageFrame coder type
            (@generate_wrapper_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, ImageFrame) => {
                paste::paste! {
                    pub fn [<register_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group_index: PyObject, number_of_channels: PyObject, allow_stale_data: bool, 
                        input_image_properties: PyImageFrameProperties, 
                        output_image_properties: PyImageFrameProperties) -> PyResult<()> {
                        PySensorCache::[<register_ $snake_case_identifier>](self, py, cortical_group_index, number_of_channels, allow_stale_data, input_image_properties, output_image_properties)
                    }
                    
                    pub fn [<store_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject, new_image: PyImageFrame) -> PyResult<()> {
                        PySensorCache::[<store_ $snake_case_identifier>](self, py, cortical_group, device_channel, new_image)
                    }
                    
                    pub fn [<read_ $snake_case_identifier>](&mut self, py: Python<'_>, 
                        cortical_group: PyObject, device_channel: PyObject) -> PyResult<PyImageFrame> {
                        PySensorCache::[<read_ $snake_case_identifier>](self, py, cortical_group, device_channel)
                    }
                }
            };
            
            // Fallback for other coder types - no function generated
            (@generate_wrapper_functions $snake_case_identifier:expr, $cortical_type_key_name:ident, $default_coder_type:ident) => {
                // No wrapper functions generated for this type!
            };
        }
        
        // Generate the actual wrapper functions using the sensor definitions
        sensor_definition!(generate_python_wrappers_inner);
    };

    TokenStream::from(expanded)
}

/// Core Module, accessible to users
#[pymodule]
fn feagi_rust_py_libs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    
    //region Feagi Data Structures Crate
    
    // Data
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyImageXYPoint);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyImageXYResolution);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PySegmentedXYImageResolutions);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyColorSpace);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyColorChannelLayout);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyImageFrameProperties);
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PySegmentedImageFrameProperties);
    //add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::P); // TODO PyCornerPoints!
    add_python_class!(py, m, "data_structures.data.image_descriptors", feagi_data_structures::data::image_descriptors::PyGazeProperties);
    add_python_class!(py, m, "data_structures.data", feagi_data_structures::data::PyImageFrame);
    add_python_class!(py, m, "data_structures.data", feagi_data_structures::data::PySegmentedImageFrame);
    
    // Genomic
    
    // Processing

    //endregion

    //region Feagi Data Serialization Crate
    
    // Byte Structure
    
    //endregion
    
    //region Feagi Connector Core Crate
    
    // Sensory
    add_python_class!(py, m, "connector_core", feagi_connector_core::sensory::PySensorCache);
    
    //endregion
    
    
    /*
    add_python_class!(py, m, "genome", genomic_structures::PyCorticalID);
    add_python_class!(py, m, "genome", genomic_structures::PyCorticalType);
    add_python_class!(py, m, "genome", genomic_structures::PyCorticalTypeVariant);
    add_python_class!(py, m, "genome", genomic_structures::PySensorCorticalType);
    add_python_class!(py, m, "genome", genomic_structures::PyCoreCorticalType);
    add_python_class!(py, m, "genome", genomic_structures::PyCorticalGroupingIndex);
    add_python_class!(py, m, "genome", genomic_structures::PyCorticalIOChannelIndex);
    add_python_class!(py, m, "genome", genomic_structures::PySingleChannelDimensions);

    add_python_class!(py, m, "io_data", io_data::PyIOTypeVariant);
    add_python_class!(py, m, "io_data", io_data::PyImageFrame);
    add_python_class!(py, m, "io_data", io_data::PySegmentedImageFrame);
    add_python_class!(py, m, "io_data", io_data::PyImageFrameTransformer);
    add_python_class!(py, m, "io_data", io_data::PyImageFrameSegmentator);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyImageFrameProperties);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PySegmentedImageFrameProperties);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyCornerPoints);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyColorSpace);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyColorChannelLayout);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyGazeProperties);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PySegmentedFrameTargetResolutions);
    
    add_python_class!(py, m, "io_processing.bytes", io_processing::byte_structures::PyFeagiByteStructure);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearAverageRollingWindowProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyIdentityFloatProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyIdentityImageFrameProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearScaleTo0And1Processor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearScaleToM1And1);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyImageFrameTransformerProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyImageFrameSegmentatorProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyImageFrameQuickDiffProcessor);
    add_python_class!(py, m, "io_processing.cache", io_processing::PySensorCache);

    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyCorticalMappedXYZPNeuronData);
    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyNeuronXYZPArrays);
    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyNeuronXYZP);
    
    // add_python_class!(py, m, "brain_input.vision", brain_input::vision::quick_image_diff::PyQuickImageDiff);


     */

    Ok(())
}
