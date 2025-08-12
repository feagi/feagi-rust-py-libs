use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
use pyo3::{pyclass, PyObject, Python};
use pyo3::types::{PyFloat};
use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
use feagi_core_data_structures_and_processing::io_data::image_descriptors::ImageFrameProperties;
use pyo3::prelude::PyAnyMethods;
use crate::io_data::PyImageFrame;
use crate::io_data::PySegmentedImageFrame;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "IOTypeVariant")]
pub enum PyIOTypeVariant {
    F32,
    F32Normalized0To1,
    F32NormalizedM1To1,
    ImageFrame,
    SegmentedImageFrame,
}

impl From<IOTypeVariant> for PyIOTypeVariant {
    fn from(io_type: IOTypeVariant) -> Self {
        match io_type {
            IOTypeVariant::F32 => Self::F32,
            IOTypeVariant::F32Normalized0To1 => Self::F32Normalized0To1,
            IOTypeVariant::F32NormalizedM1To1 => Self::F32NormalizedM1To1,
            IOTypeVariant::ImageFrame => Self::ImageFrame,
            IOTypeVariant::SegmentedImageFrame => Self::SegmentedImageFrame,
        }
    }
}

impl From<PyIOTypeVariant> for IOTypeVariant {
    fn from(io_type: PyIOTypeVariant) -> Self {
        match io_type {
            PyIOTypeVariant::F32 => Self::F32,
            PyIOTypeVariant::F32Normalized0To1 => Self::F32Normalized0To1,
            PyIOTypeVariant::F32NormalizedM1To1 => Self::F32NormalizedM1To1,
            PyIOTypeVariant::ImageFrame => Self::ImageFrame,
            PyIOTypeVariant::SegmentedImageFrame => Self::SegmentedImageFrame,
        }
    }
}

// NOTE: PyIOTypeData does not exist and should never exist! There is no need to wrap python data!
// Instead, make use of the below functions to handle casting!

pub(crate) fn try_get_as_io_type_variant<'py>(py: Python<'_>, any: PyObject) -> Result<IOTypeVariant, FeagiDataProcessingError> {
    let bound = any.bind(py);
    
    match () {
        _ if bound.is_instance_of::<PyImageFrame>() => Ok(IOTypeVariant::ImageFrame),
        
        _ if bound.is_instance_of::<PySegmentedImageFrame>() => Ok(IOTypeVariant::SegmentedImageFrame),
        
        _ if bound.is_instance_of::<PyFloat>() => Ok(IOTypeVariant::F32),
        
        // NOTE: specifically not treating ints as floats
        
        _ => Err(IODataError::InvalidParameters("Unknown Data Type!".into()).into())
    }
}

pub(crate) fn try_wrap_as_io_type_data<'py>(py: Python<'_>, any: PyObject) -> Result<IOTypeData, FeagiDataProcessingError> {
    let bound = any.bind(py);
    
    // Use type introspection for efficiency, then extract data when type matches
    match () {
        
        _ if bound.is_instance_of::<PyImageFrame>() => {
            let image_frame = any.extract::<PyImageFrame>(py)
                .map_err(|_| IODataError::InvalidParameters("Failed to extract PyImageFrame".into()))?;
            Ok(IOTypeData::ImageFrame(image_frame.inner))
        },
        
        _ if bound.is_instance_of::<PySegmentedImageFrame>() => {
            let segmented_frame = any.extract::<PySegmentedImageFrame>(py)
                .map_err(|_| IODataError::InvalidParameters("Failed to extract PySegmentedImageFrame".into()))?;
            Ok(IOTypeData::SegmentedImageFrame(segmented_frame.inner))
        },
        
        // Handle basic Python float last
        _ if bound.is_instance_of::<PyFloat>() => {
            let float_val = any.extract::<f32>(py)
                .map_err(|_| IODataError::InvalidParameters("Failed to extract float value".into()))?;
            Ok(IOTypeData::F32(float_val))
        },

        // If nothing matches, return error
        _ => Err(IODataError::InvalidParameters("Unknown Data Type!".into()).into())
    }
}
