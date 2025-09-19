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
    
    // Wrapped IO Data
    add_python_class!(py, m, "data_structures.wrapped_io_data", feagi_data_structures::wrapped_io_data::PyWrappedIOType);
    
    // Genomic
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCorticalID);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCorticalType);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCoreCorticalType);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PySensorCorticalType);

    // neurons
    add_python_class!(py, m, "data_structures.neurons.xyzp", feagi_data_structures::neurons::xyzp::PyCorticalMappedXYZPNeuronData);
    add_python_class!(py, m, "data_structures.neurons.xyzp", feagi_data_structures::neurons::xyzp::PyNeuronXYZPArrays);
    add_python_class!(py, m, "data_structures.neurons.xyzp", feagi_data_structures::neurons::xyzp::PyNeuronXYZP);
    
    // Processing
    add_python_class!(py, m, "data_structures.processing", feagi_data_structures::processing::PyImageFrameProcessor);
    add_python_class!(py, m, "data_structures.processing", feagi_data_structures::processing::PyImageFrameSegmentator);


    //endregion

    //region Feagi Data Serialization Crate
    
    // Byte Structure
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiByteStructure);
    
    //endregion
    
    //region Feagi Connector Core Crate
    
    // Cache
    add_python_class!(py, m, "connector_core", feagi_connector_core::data_pipeline::PyIOCache);

    // Data Pipeline
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameQuickDiffStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentityFloatStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentityImageFrameStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentitySegmentedImageFrameStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameProcessorStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameSegmentatorStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImagePixelValueCountThresholdStage);

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
