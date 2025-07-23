use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};

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


