//! Rust PYO3 Compiled Python Module
//! All docs pertaining to python exposed modules must 
//! be reflected to the 'feagi_data_processing.pyi.template' file!

mod miscellaneous_types;
mod neuron_data;
mod io_processing;
mod genomic_structures;
mod io_data;

use pyo3::prelude::*;

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


/// Core Module, accessible to users
#[pymodule]
fn feagi_data_processing(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {

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
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyImageFrameProperties);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyCornerPoints);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyColorSpace);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyChannelLayout);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PySegmentedFrameCenterProperties);
    add_python_class!(py, m, "io_data.image_descriptors", io_data::image_descriptors::PySegmentedFrameTargetResolutions);

    add_python_class!(py, m, "io_processing.bytes", io_processing::byte_structures::PyFeagiByteStructure);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearAverageRollingWindowProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyIdentityFloatProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyIdentityImageFrameProcessor);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearScaleTo0And1);
    add_python_class!(py, m, "io_processing.processors", io_processing::processors::PyLinearScaleToM1And1);
    add_python_class!(py, m, "io_processing.cache", io_processing::PySensorCache);

    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyCorticalMappedXYZPNeuronData);
    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyNeuronXYZPArrays);
    add_python_class!(py, m, "neuron_data.xyzp", neuron_data::xyzp::PyNeuronXYZP);
    
    // add_python_class!(py, m, "brain_input.vision", brain_input::vision::quick_image_diff::PyQuickImageDiff);
    
    Ok(())
}
