use std::any::Any;
use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
use pyo3::{pyclass, pymethods, Bound, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
use pyo3::prelude::PyAnyMethods;
use crate::io_data::ranged_floats::PyNormalized0To1F32;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "IOTypeVariant")]
pub enum PyIOTypeVariant {
    NormalizedM1to1F32,
    Normalized0to1F32,
    BoundedF32,
    ImageFrame,
    SegmentedImageFrame,
}

impl From<IOTypeVariant> for PyIOTypeVariant {
    fn from(io_type: IOTypeVariant) -> Self {
        match io_type {
            IOTypeVariant::NormalizedM1to1F32 => Self::NormalizedM1to1F32,
            IOTypeVariant::Normalized0to1F32 => Self::Normalized0to1F32,
            IOTypeVariant::BoundedF32 => Self::BoundedF32,
            IOTypeVariant::ImageFrame => Self::ImageFrame,
            IOTypeVariant::SegmentedImageFrame => Self::SegmentedImageFrame,
        }
    }
}

impl From<PyIOTypeVariant> for IOTypeVariant {
    fn from(io_type: PyIOTypeVariant) -> Self {
        match io_type {
            PyIOTypeVariant::NormalizedM1to1F32 => Self::NormalizedM1to1F32,
            PyIOTypeVariant::Normalized0to1F32 => Self::Normalized0to1F32,
            PyIOTypeVariant::BoundedF32 => Self::BoundedF32,
            PyIOTypeVariant::ImageFrame => Self::ImageFrame,
            PyIOTypeVariant::SegmentedImageFrame => Self::SegmentedImageFrame,
        }
    }
}


pub(crate) fn try_get_as_io_type_variant<'py>(py: Python<'_>, any: PyObject) -> Result<IOTypeVariant, FeagiDataProcessingError> {
    let normalized_0_1 = py.get_type::<PyNormalized0To1F32>();
    
    // TODO finish
    
    if any.is(&normalized_0_1) {
        return Ok(IOTypeVariant::Normalized0to1F32);
    }
    
    Err(IODataError::InvalidParameters("Unknown Data Type!".into()).into())
}

pub(crate) fn try_wrap_as_io_type_data<'py>(py: Python<'_>, any: PyObject) -> Result<IOTypeData, FeagiDataProcessingError> {
    
    // TODO finish
    // TODO this is likely unoptimized!
    
    if let Ok(normalized_0_1) = any.extract::<PyNormalized0To1F32>(py) {
        return Ok(IOTypeData::Linear0to1NormalizedF32(normalized_0_1.inner))
    }
    
    Err(IODataError::InvalidParameters("Unknown Data Type!".into()).into())
}