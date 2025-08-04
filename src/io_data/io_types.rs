use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
use pyo3::{pyclass, pymethods, Bound, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyFloat, PyInt};
use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
use pyo3::prelude::PyAnyMethods;
use crate::io_data::image::image_frame::PyImageFrame;
use crate::io_data::image::segmented_vision_frame::PySegmentedImageFrame;

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