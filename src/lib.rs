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
mod feagi_agent_sdk;
mod feagi_evo;

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
    
    // Genomic
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCorticalID);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCorticalType);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyCoreCorticalType);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PySensorCorticalType);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyMotorCorticalType);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyCorticalChannelCount);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyCorticalChannelIndex);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyCorticalGroupIndex);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyAgentDeviceIndex);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyPipelineStagePropertyIndex);

    // Neurons
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyCorticalMappedXYZPNeuronVoxels);
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZPArrays);
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZP);


    //endregion

    //region Feagi Data Serialization Crate
    
    // Byte Structure API (for FEAGI Core)
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiByteStructureType);
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiSerializable);
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiByteContainer);
    
    //endregion
    
    //region Feagi Connector Core Crate
    
    // Cache
    add_python_class!(py, m, "connector_core.caching", feagi_connector_core::caching::PyIOCache);

    // Data
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyImageFrame);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PySegmentedImageFrame);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyMiscData);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyPercentage);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PySignedPercentage);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyPercentage2D);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PySignedPercentage2D);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyPercentage3D);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PySignedPercentage3D);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PyPercentage4D);
    add_python_class!(py, m, "connector_core.data", feagi_connector_core::data::PySignedPercentage4D);
    
    // Data Descriptors
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyImageXYPoint);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyImageXYResolution);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyImageXYZDimensions);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PySegmentedXYImageResolutions);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyColorSpace);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyColorChannelLayout);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyImageFrameProperties);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PySegmentedImageFrameProperties);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyCornerPoints);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyGazeProperties);
    add_python_class!(py, m, "connector_core.data.descriptors", feagi_connector_core::data::descriptors::PyMiscDataDimensions);
    
    // Wrapped IO Data
    add_python_class!(py, m, "connector_core.wrapped_io_data", feagi_connector_core::wrapped_io_data::PyWrappedIOType);

    // Data Pipeline Stage Properties
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyIdentityStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyImageSegmentorStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyImageQuickDiffStageProperties);


    // Data Pipeline Stages
    /*
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameQuickDiffStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentityFloatStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentityImageFrameStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyIdentitySegmentedImageFrameStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameProcessorStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImageFrameSegmentatorStage);
    add_python_class!(py, m, "connector_core.data_pipeline.stages", feagi_connector_core::data_pipeline::stages::PyImagePixelValueCountThresholdStage);

     */

    //endregion
    
    //region FEAGI Agent SDK
    
    // Register the agent SDK module
    feagi_agent_sdk::register_module(py, m)?;
    
    //endregion
    
    //region FEAGI Evo (Genome Validation)
    
    // Register the genome validation module
    feagi_evo::validator::register_module(py, m)?;
    
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
