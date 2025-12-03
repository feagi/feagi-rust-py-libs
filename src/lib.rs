//! Rust PYO3 Compiled Python Module
//! All docs pertaining to python exposed modules must 
//! be reflected to the 'feagi_data_processing.pyi.template' file!


mod feagi_data_structures;
mod py_error;
mod macro_helpers;
mod feagi_data_serialization;
mod feagi_connector_core;
//mod feagi_agent_sdk;
//mod feagi_evo;

use pyo3::prelude::*;

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
    
    //region Feagi Data Structures

    // Genomic
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyCorticalID);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyCustomCorticalType);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyMemoryCorticalType);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyCorticalAreaType);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyCoreCorticalType);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyCorticalChannelCount);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyCorticalGroupIndex);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyCorticalChannelIndex);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyCorticalChannelDimensions);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyCorticalUnitIndex);
    add_python_class!(py, m, "data_structures.genomic.cortical_area.descriptors", feagi_data_structures::genomic::cortical_area::descriptors::PyNeuronDepth);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyFrameChangeHandling);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyIOCorticalAreaDataFlag);
    add_python_class!(py, m, "data_structures.genomic.cortical_area", feagi_data_structures::genomic::cortical_area::PyPercentageNeuronPositioning);
    add_python_class!(py, m, "data_structures.genomic.descriptors", feagi_data_structures::genomic::descriptors::PyAgentDeviceIndex);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PyMotorCorticalUnit);
    add_python_class!(py, m, "data_structures.genomic", feagi_data_structures::genomic::PySensoryCorticalUnit);

    // Neurons Voxels
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyCorticalMappedXYZPNeuronVoxels);
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZPArrays);
    add_python_class!(py, m, "data_structures.neurons_voxels.xyzp", feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZP);

    // Processing
    // TODO should even have this module?

    //endregion

    
    
    //region Feagi Data Serialization
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiByteStructureType);
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiSerializable);
    add_python_class!(py, m, "data_serialization", feagi_data_serialization::PyFeagiByteContainer);
    
    //endregion
    
    
    
    //region Feagi Connector Core
    
    // Cache - MotorDeviceCache (new clean API)
    add_python_class!(py, m, "connector_core.caching", feagi_connector_core::caching::PyMotorDeviceCache);
    
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyImageFrame);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PySegmentedImageFrame);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyMiscData);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyGazeProperties);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyPercentage);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PySignedPercentage);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyPercentage2D);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PySignedPercentage2D);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyPercentage3D);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PySignedPercentage3D);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PyPercentage4D);
    add_python_class!(py, m, "connector_core.data_types", feagi_connector_core::data_types::PySignedPercentage4D);
    
    
    // Data Descriptors
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyImageXYPoint);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyImageXYResolution);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyImageXYZDimensions);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PySegmentedXYImageResolutions);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyColorSpace);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyColorChannelLayout);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyImageFrameProperties);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PySegmentedImageFrameProperties);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyCornerPoints);
    add_python_class!(py, m, "connector_core.data_types.descriptors", feagi_connector_core::data_types::descriptors::PyMiscDataDimensions);
    
    //Wrapped IO Data
    add_python_class!(py, m, "connector_core.wrapped_io_data", feagi_connector_core::wrapped_io_data::PyWrappedIOType);

   // Data Pipeline Stage Properties
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyIdentityStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyImageSegmentorStageProperties);
    add_python_class!(py, m, "connector_core.data_pipeline.stage_properties", feagi_connector_core::data_pipeline::stage_properties::PyImageQuickDiffStageProperties);
    
    //endregion
    
    //region FEAGI Agent SDK
    
    // Register the agent SDK module
    //feagi_agent_sdk::register_module(py, m)?;
    
    //endregion
    
    //region FEAGI Evo (Genome Validation) - Temporarily disabled pending beta.56 migration
    
    // Register the genome validation module
    // feagi_evo::validator::register_module(py, m)?;
    
    //endregion
    
    Ok(())
}
