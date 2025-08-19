use pyo3::{pyclass, PyObject, Python};
use pyo3::types::{PyFloat};
use pyo3::prelude::PyAnyMethods;
use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant, ImageFrame, SegmentedImageFrame};
use crate::io_data::PyImageFrame;
use crate::io_data::PySegmentedImageFrame;


#[pyclass(eq)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "IOTypeVariant")]
pub struct PyIOTypeVariant {
    inner: IOTypeVariant,
}

impl From<IOTypeVariant> for PyIOTypeVariant {
    fn from(io_type: IOTypeVariant) -> Self {
        PyIOTypeVariant { inner: io_type }
    }
}

impl From<PyIOTypeVariant> for IOTypeVariant {
    fn from(io_type: PyIOTypeVariant) -> Self {
        io_type.inner
    }
}

impl std::fmt::Display for PyIOTypeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.inner.to_string())
    }
}

// NOTE: PyIOTypeData does not exist and should never exist! There is no need to wrap python data!
// Instead, make use of the below functions to handle casting!

pub(crate) fn try_get_as_io_type_variant<'py>(py: Python<'_>, any: PyObject) -> Result<IOTypeVariant, FeagiDataProcessingError> {
    let bound = any.bind(py);
    
    match () {
        _ if bound.is_instance_of::<PyImageFrame>() => {
            let py_image_frame = any.extract::<PyImageFrame>(py).unwrap();
            let image_frame: ImageFrame = py_image_frame.into();
            Ok(IOTypeVariant::ImageFrame(Some(image_frame.get_image_frame_properties())))
        },
        
        _ if bound.is_instance_of::<PySegmentedImageFrame>() => {
            let py_segmented_image_frame = any.extract::<PySegmentedImageFrame>(py).unwrap();
            let segmented_image_frame: SegmentedImageFrame = py_segmented_image_frame.into();
            Ok(IOTypeVariant::SegmentedImageFrame(Some(segmented_image_frame.get_segmented_image_frame_properties())))
        },
        
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
