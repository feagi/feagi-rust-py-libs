use feagi_data_structures::data::image_descriptors::{ImageFrameProperties, SegmentedImageFrameProperties};
use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::prelude::*;
use feagi_data_structures::wrapped_io_data::{WrappedIOType, WrappedIOData};
use crate::feagi_data_structures::data::image_descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::{project_display, py_object_cast_generic, py_type_casts};

#[pyclass(subclass, str)]
#[pyo3(name = "WrappedIOType")]
#[derive(Clone)]
pub struct PyWrappedIOType {
    pub inner: WrappedIOType,
}

#[pymethods]
impl PyWrappedIOType {

    #[staticmethod]
    pub fn F32() -> Self {
        PyWrappedIOType { inner: WrappedIOType::F32 }
    }

    #[staticmethod]
    pub fn F32Normalized0To1() -> Self {
        PyWrappedIOType { inner: WrappedIOType::F32Normalized0To1 }
    }

    #[staticmethod]
    pub fn F32NormalizedM1To1() -> Self {
        PyWrappedIOType { inner: WrappedIOType::F32NormalizedM1To1 }
    }

    #[staticmethod]
    pub fn ImageFrame(optional_image_properties: Option<PyImageFrameProperties>) -> Self {

        #[inline]
        fn convert(py_image_frame_properties: Option<PyImageFrameProperties>) -> Option<ImageFrameProperties> {
            match py_image_frame_properties {
                Some(py_image_frame_properties) => Some(py_image_frame_properties.into()),
                None => None
            }
        }

        PyWrappedIOType { inner: WrappedIOType::ImageFrame(convert(optional_image_properties)) }
    }

    #[staticmethod]
    pub fn SegmentedImageFrame(optional_image_properties: Option<PySegmentedImageFrameProperties>) -> Self {
        #[inline]
        fn convert(py_image_frame_properties: Option<PySegmentedImageFrameProperties>) -> Option<SegmentedImageFrameProperties> {
            match py_image_frame_properties {
                Some(py_image_frame_properties) => Some(py_image_frame_properties.into()),
                None => None
            }
        }
        PyWrappedIOType { inner: WrappedIOType::SegmentedImageFrame(convert(optional_image_properties)) }

    }
}

project_display!(PyWrappedIOType);
py_type_casts!(PyWrappedIOType, WrappedIOType);














