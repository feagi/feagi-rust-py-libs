use feagi_connector_core::data_types::descriptors::{ImageFrameProperties, MiscDataDimensions, SegmentedImageFrameProperties};
use feagi_connector_core::wrapped_io_data::WrappedIOType;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use crate::feagi_connector_core::data_types::descriptors::{PyImageFrameProperties, PySegmentedImageFrameProperties, PyMiscDataDimensions};
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;

#[pyclass(subclass, str)] //TODO subclass?
#[pyo3(name = "WrappedIOType")]
#[derive(Clone)]
pub struct PyWrappedIOType {
    pub inner: WrappedIOType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyWrappedIOType {

    #[staticmethod]
    pub fn Boolean() ->Self {
        PyWrappedIOType { inner: WrappedIOType::Boolean }
    }
    
    #[staticmethod]
    pub fn Percentage() -> Self {
        PyWrappedIOType { inner: WrappedIOType::Percentage }
    }

    #[staticmethod]
    pub fn Percentage_2D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::Percentage_2D }
    }

    #[staticmethod]
    pub fn Percentage_3D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::Percentage_3D }
    }

    #[staticmethod]
    pub fn Percentage_4D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::Percentage_4D }
    }

    #[staticmethod]
    pub fn SignedPercentage() -> Self {
        PyWrappedIOType { inner: WrappedIOType::SignedPercentage }
    }

    #[staticmethod]
    pub fn SignedPercentage_2D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::SignedPercentage_2D }
    }

    #[staticmethod]
    pub fn SignedPercentage_3D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::SignedPercentage_3D }
    }

    #[staticmethod]
    pub fn SignedPercentage_4D() -> Self {
        PyWrappedIOType { inner: WrappedIOType::SignedPercentage_4D }
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

    #[staticmethod]
    pub fn MiscData(optional_misc_dimensions: Option<PyMiscDataDimensions>) -> Self {
        #[inline]
        fn convert(py_misc_dimensions: Option<PyMiscDataDimensions>) -> Option<MiscDataDimensions> {
            match py_misc_dimensions {
                Some(py_misc_dimensions) => Some(py_misc_dimensions.into()),
                None => None
            }
        }
        PyWrappedIOType { inner: WrappedIOType::MiscData(convert(optional_misc_dimensions)) }
    }

    #[staticmethod]
    pub fn GazeProperties() ->Self {
        PyWrappedIOType { inner: WrappedIOType::GazeProperties }
    }

    pub fn is_same_variant(&self, other: &PyWrappedIOType) -> bool {
        WrappedIOType::is_same_variant(&self.inner, &other.inner)
    }
}

project_display!(PyWrappedIOType);
py_type_casts!(PyWrappedIOType, WrappedIOType);
py_object_cast_generic!(PyWrappedIOType, WrappedIOType, "Unable to retrieve WrappedIOType data from given!");














