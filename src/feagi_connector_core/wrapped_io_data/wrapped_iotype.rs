use crate::feagi_connector_core::data_types::descriptors::{
    PyImageFrameProperties, PyMiscDataDimensions, PySegmentedImageFrameProperties,
};
use crate::feagi_connector_core::data_types::PyAudioSpectrumProperties;
use crate::feagi_connector_core::data_types::PyPoseEstimationProperties;
use crate::{__base_py_class_shared, wrap_flat_enum};
use feagi_sensorimotor::data_types::descriptors::{
    ImageFrameProperties, MiscDataDimensions, PoseEstimationProperties,
    SegmentedImageFrameProperties,
};
use feagi_sensorimotor::wrapped_io_data::WrappedIOType;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, PyResult};

wrap_flat_enum!(PyWrappedIOType, WrappedIOType, "WrappedIOType");

#[pymethods]
#[allow(non_snake_case)]
impl PyWrappedIOType {
    #[staticmethod]
    pub fn Boolean() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::Boolean,
        }
    }

    #[staticmethod]
    pub fn Percentage() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::Percentage,
        }
    }

    #[staticmethod]
    pub fn Percentage_2D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::Percentage_2D,
        }
    }

    #[staticmethod]
    pub fn Percentage_3D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::Percentage_3D,
        }
    }

    #[staticmethod]
    pub fn Percentage_4D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::Percentage_4D,
        }
    }

    #[staticmethod]
    pub fn SignedPercentage() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::SignedPercentage,
        }
    }

    #[staticmethod]
    pub fn SignedPercentage_2D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::SignedPercentage_2D,
        }
    }

    #[staticmethod]
    pub fn SignedPercentage_3D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::SignedPercentage_3D,
        }
    }

    #[staticmethod]
    pub fn SignedPercentage_4D() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::SignedPercentage_4D,
        }
    }

    #[staticmethod]
    pub fn ImageFrame(optional_image_properties: Option<PyImageFrameProperties>) -> Self {
        #[inline]
        fn convert(
            py_image_frame_properties: Option<PyImageFrameProperties>,
        ) -> Option<ImageFrameProperties> {
            match py_image_frame_properties {
                Some(py_image_frame_properties) => Some(py_image_frame_properties.into()),
                None => None,
            }
        }

        PyWrappedIOType {
            inner: WrappedIOType::ImageFrame(convert(optional_image_properties)),
        }
    }

    #[staticmethod]
    pub fn SegmentedImageFrame(
        optional_image_properties: Option<PySegmentedImageFrameProperties>,
    ) -> Self {
        #[inline]
        fn convert(
            py_image_frame_properties: Option<PySegmentedImageFrameProperties>,
        ) -> Option<SegmentedImageFrameProperties> {
            match py_image_frame_properties {
                Some(py_image_frame_properties) => Some(py_image_frame_properties.into()),
                None => None,
            }
        }
        PyWrappedIOType {
            inner: WrappedIOType::SegmentedImageFrame(convert(optional_image_properties)),
        }
    }

    /// Composite Raw IMU sensor descriptor (accelerometer + gyroscope + magnetometer
    /// readings, each a 3-axis signed percentage). Spans 3 sub-cortical-areas.
    #[staticmethod]
    pub fn RawIMU() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::RawIMU,
        }
    }

    #[staticmethod]
    pub fn MiscData(optional_misc_dimensions: Option<PyMiscDataDimensions>) -> Self {
        #[inline]
        fn convert(py_misc_dimensions: Option<PyMiscDataDimensions>) -> Option<MiscDataDimensions> {
            match py_misc_dimensions {
                Some(py_misc_dimensions) => Some(py_misc_dimensions.into()),
                None => None,
            }
        }
        PyWrappedIOType {
            inner: WrappedIOType::MiscData(convert(optional_misc_dimensions)),
        }
    }

    #[staticmethod]
    pub fn GazeProperties() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::GazeProperties,
        }
    }

    #[staticmethod]
    pub fn ImageFilteringSettings() -> Self {
        PyWrappedIOType {
            inner: WrappedIOType::ImageFilteringSettings,
        }
    }

    #[staticmethod]
    pub fn AudioSpectrumFrame(
        optional_audio_properties: Option<PyAudioSpectrumProperties>,
    ) -> Self {
        let audio_properties = optional_audio_properties.map(|p| p.inner);
        PyWrappedIOType {
            inner: WrappedIOType::AudioSpectrumFrame(audio_properties),
        }
    }

    #[staticmethod]
    pub fn PoseEstimationData(
        optional_pose_properties: Option<PyPoseEstimationProperties>,
    ) -> Self {
        let pose_properties: Option<PoseEstimationProperties> =
            optional_pose_properties.map(|properties| properties.inner);
        PyWrappedIOType {
            inner: WrappedIOType::PoseEstimationData(pose_properties),
        }
    }

    pub fn is_same_variant(&self, other: &PyWrappedIOType) -> bool {
        WrappedIOType::is_same_variant(&self.inner, &other.inner)
    }
}
