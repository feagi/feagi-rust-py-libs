use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::image_descriptors::*;

//region ImageFrameProperties

#[pyclass]
#[pyo3(name = "ImageFrameProperties")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyImageFrameProperties {
    inner: ImageFrameProperties
}

#[pymethods]
impl PyImageFrameProperties {
    #[new]
    pub fn new(xy_resolution: (usize, usize), color_space: PyColorSpace, color_channel_layout: PyColorChannelLayout) -> PyResult<Self> {
        let color_space: ColorSpace = color_space.into();
        let color_channel_layout: ColorChannelLayout = color_channel_layout.into();
        let result = ImageFrameProperties::new(xy_resolution, color_space, color_channel_layout);
        match result {
            Ok(inner) => Ok(PyImageFrameProperties { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[getter]
    pub fn expected_xy_resolution(&self) -> PyResult<(usize, usize)> {
        Ok(self.inner.get_expected_xy_resolution())
    }

    #[getter]
    pub fn expected_color_space(&self) -> PyResult<PyColorSpace> {
        Ok(self.inner.get_expected_color_space().into())
    }

    #[getter]
    pub fn expected_channel_layout(&self) -> PyResult<PyColorChannelLayout> {
        Ok(self.inner.get_expected_color_channel_layout().into())
    }
}

impl From<ImageFrameProperties> for PyImageFrameProperties {
    fn from(frame_properties: ImageFrameProperties) -> Self {
        PyImageFrameProperties{inner: frame_properties}
    }
}

impl From<PyImageFrameProperties> for ImageFrameProperties {
    fn from(frame_properties: PyImageFrameProperties) -> Self {
        frame_properties.inner
    }
}

//endregion

//region Segmented Image Frame Properties
#[pyclass]
#[pyo3(name = "SegmentedImageFrameProperties")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PySegmentedImageFrameProperties {
    inner: SegmentedImageFrameProperties
}

// TODO implement Display
#[pymethods]
impl PySegmentedImageFrameProperties {

    #[new]
    pub fn new(segment_xy_resolutions: PySegmentedFrameTargetResolutions, center_color_channels: PyColorChannelLayout,
               peripheral_color_channels: PyColorChannelLayout, color_space: PyColorSpace) -> PyResult<Self> {
        let segment_xy_resolutions: SegmentedFrameTargetResolutions = segment_xy_resolutions.into();
        let center_color_channels: ColorChannelLayout = center_color_channels.into();
        let peripheral_color_channels: ColorChannelLayout = peripheral_color_channels.into();
        let color_space: ColorSpace = color_space.into();
        Ok(PySegmentedImageFrameProperties{
            inner: SegmentedImageFrameProperties::new(
                &segment_xy_resolutions,
                &center_color_channels,
                &peripheral_color_channels,
                &color_space
            )
        })
    }

    #[getter]
    pub fn expected_resolutions(&self) -> PyResult<PySegmentedFrameTargetResolutions> {
        Ok(self.inner.get_expected_resolutions().clone().into())
    }

    #[getter]
    pub fn center_color_channel(&self) -> PyResult<PyColorChannelLayout> {
        Ok(self.inner.get_center_color_channel().clone().into())
    }

    #[getter]
    pub fn peripheral_color_channel(&self) -> PyResult<PyColorChannelLayout> {
        Ok(self.inner.get_center_color_channel().clone().into())
    }

    #[getter]
    pub fn color_space(&self) -> PyResult<PyColorSpace> {
        Ok(self.inner.get_color_space().clone().into())
    }

}

impl From<SegmentedImageFrameProperties> for PySegmentedImageFrameProperties {
    fn from(frame_properties: SegmentedImageFrameProperties) -> Self {
        PySegmentedImageFrameProperties{inner: frame_properties}
    }
}

impl From<PySegmentedImageFrameProperties> for SegmentedImageFrameProperties {
    fn from(frame_properties: PySegmentedImageFrameProperties) -> Self {
        frame_properties.inner
    }
}

//endregion

//region SegmentedFrameTargetResolutions

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedFrameTargetResolutions")]
pub struct PySegmentedFrameTargetResolutions {
    pub inner: SegmentedFrameTargetResolutions,
}

#[pymethods]
impl PySegmentedFrameTargetResolutions {
    #[getter]
    fn lower_left(&self) -> (usize, usize) {
        self.inner.lower_left
    }

    #[getter]
    fn middle_left(&self) -> (usize, usize) {
        self.inner.middle_left
    }

    #[getter]
    fn upper_left(&self) -> (usize, usize) {
        self.inner.upper_left
    }

    #[getter]
    fn upper_middle(&self) -> (usize, usize) {
        self.inner.upper_middle
    }

    #[getter]
    fn upper_right(&self) -> (usize, usize) {
        self.inner.upper_right
    }

    #[getter]
    fn middle_right(&self) -> (usize, usize) {
        self.inner.middle_right
    }

    #[getter]
    fn lower_right(&self) -> (usize, usize) {
        self.inner.lower_right
    }

    #[getter]
    fn lower_middle(&self) -> (usize, usize) {
        self.inner.lower_middle
    }

    #[getter]
    fn center(&self) -> (usize, usize) {
        self.inner.center
    }
}

impl From<PySegmentedFrameTargetResolutions> for SegmentedFrameTargetResolutions {
    fn from(value: PySegmentedFrameTargetResolutions) -> Self {
        value.inner
    }
}

impl From<SegmentedFrameTargetResolutions> for PySegmentedFrameTargetResolutions {
    fn from(value: SegmentedFrameTargetResolutions) -> Self {
        PySegmentedFrameTargetResolutions{inner: value}
    }
}

//endregion

//region CornerPoints

#[pyclass]
#[pyo3(name = "CornerPoints")]
#[derive(Clone)]
pub struct PyCornerPoints {
    pub inner: CornerPoints,
}

#[pymethods]
impl PyCornerPoints {
    #[new]
    fn new_from_row_major_where_origin_top_left(lower_left: (usize, usize), upper_right: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new_from_row_major(lower_left, upper_right);
        match result {
            Ok(inner) => Ok(PyCornerPoints { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[staticmethod]
    fn new_from_cartesian_where_origin_bottom_left(lower_left: (usize, usize), upper_right: (usize, usize), total_resolution_width_height: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new_from_cartesian(lower_left, upper_right, total_resolution_width_height);
        match result {
            Ok(inner) => Ok(PyCornerPoints { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    fn does_fit_in_frame_of_width_height(&self, source_total_resolution: (usize, usize)) -> bool {
        return self.inner.does_fit_in_frame_of_width_height(source_total_resolution);
    }

    fn enclosed_area_width_height(&self) -> (usize, usize) {
        return self.inner.enclosed_area_width_height();
    }

    #[getter]
    fn lower_right_row_major(&self) -> (usize, usize) {
        return self.inner.lower_right_row_major();
    }

    #[getter]
    fn upper_left_row_major(&self) -> (usize, usize) {
        return self.inner.upper_left_row_major();
    }

    #[getter]
    fn lower_left_row_major(&self) -> (usize, usize) {
        return self.inner.lower_left_row_major();
    }

    #[getter]
    fn upper_right_row_major(&self) -> (usize, usize) {
        return self.inner.upper_right_row_major();
    }
}

impl From<PyCornerPoints> for CornerPoints {
    fn from(points: PyCornerPoints) -> CornerPoints {
        points.inner
    }
}

impl From<CornerPoints> for PyCornerPoints {
    fn from(points: CornerPoints) -> PyCornerPoints {
        PyCornerPoints { inner: points }
    }
}

//endregion

//region Gaze Properties

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "GazeProperties")]
pub struct PyGazeProperties{
    pub inner: GazeProperties,
}

#[pymethods]
impl PyGazeProperties {
    #[new]
    fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy: (f32, f32), center_size_normalized_xy: (f32, f32)) -> PyResult<Self> {
        let result = GazeProperties::cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy, center_size_normalized_xy);
        match result {
            Ok(inner) => Ok(PyGazeProperties { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string())),
        }
    }

    #[staticmethod]
    fn create_default_centered() -> Self {
        PyGazeProperties {
            inner: GazeProperties::create_default_centered(),
        }
    }
}

impl From<PyGazeProperties> for GazeProperties {
    fn from(value: PyGazeProperties) -> Self {
        value.inner
    }
}

impl From<GazeProperties> for PyGazeProperties {
    fn from(value: GazeProperties) -> Self {
        PyGazeProperties {inner: value}
    }
}

//endregion

//region Enums

//region ColorSpace

// Add ColorSpace enum for Python
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ColorSpace")]
pub enum PyColorSpace {
    Linear,
    Gamma,
}

impl From<PyColorSpace> for ColorSpace {
    fn from(py_color_space: PyColorSpace) -> Self {
        match py_color_space {
            PyColorSpace::Linear => ColorSpace::Linear,
            PyColorSpace::Gamma => ColorSpace::Gamma,
        }
    }
}

impl From<ColorSpace> for PyColorSpace {
    fn from(color_space: ColorSpace) -> Self {
        match color_space {
            ColorSpace::Linear => PyColorSpace::Linear,
            ColorSpace::Gamma => PyColorSpace::Gamma,
        }
    }
}

//endregion

//region ColorChannelLayout

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ColorChannelLayout")]
pub enum PyColorChannelLayout {
    GrayScale,
    RG,
    RGB,
    RGBA
}

impl From<PyColorChannelLayout> for ColorChannelLayout {
    fn from(py_channel_format: PyColorChannelLayout) -> Self {
        match py_channel_format {
            PyColorChannelLayout::GrayScale => ColorChannelLayout::GrayScale,
            PyColorChannelLayout::RG => ColorChannelLayout::RG,
            PyColorChannelLayout::RGB => ColorChannelLayout::RGB,
            PyColorChannelLayout::RGBA => ColorChannelLayout::RGBA,
        }
    }
}

impl From<ColorChannelLayout> for PyColorChannelLayout {
    fn from(channel_layout: ColorChannelLayout) -> Self {
        match channel_layout {
            ColorChannelLayout::GrayScale => PyColorChannelLayout::GrayScale,
            ColorChannelLayout::RG => PyColorChannelLayout::RG,
            ColorChannelLayout::RGB => PyColorChannelLayout::RGB,
            ColorChannelLayout::RGBA => PyColorChannelLayout::RGBA,
        }
    }
}

impl From<&ColorChannelLayout> for PyColorChannelLayout {
    fn from(channel_layout: &ColorChannelLayout) -> Self {
        channel_layout.clone().into()
    }
}

//endregion

//region MemoryOrderLayout

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "MemoryOrderLayout")]
pub enum PyMemoryOrderLayout {
    HeightsWidthsChannels, // default, also called row major
    ChannelsHeightsWidths, // common in machine learning
    WidthsHeightsChannels, // cartesian, the best one
    HeightsChannelsWidths,
    ChannelsWidthsHeights,
    WidthsChannelsHeights,
}

impl From<PyMemoryOrderLayout> for MemoryOrderLayout {
    fn from(py_memory_layout: PyMemoryOrderLayout) -> Self {
        match py_memory_layout {
            PyMemoryOrderLayout::HeightsWidthsChannels => MemoryOrderLayout::HeightsWidthsChannels,
            PyMemoryOrderLayout::ChannelsHeightsWidths => MemoryOrderLayout::ChannelsHeightsWidths,
            PyMemoryOrderLayout::WidthsHeightsChannels => MemoryOrderLayout::WidthsHeightsChannels,
            PyMemoryOrderLayout::HeightsChannelsWidths => MemoryOrderLayout::HeightsChannelsWidths,
            PyMemoryOrderLayout::ChannelsWidthsHeights => MemoryOrderLayout::ChannelsWidthsHeights,
            PyMemoryOrderLayout::WidthsChannelsHeights => MemoryOrderLayout::WidthsChannelsHeights,
        }
    }
}

impl From<MemoryOrderLayout> for PyMemoryOrderLayout {
    fn from(memory_order_layout: MemoryOrderLayout) -> Self {
        match memory_order_layout {
            MemoryOrderLayout::HeightsWidthsChannels => PyMemoryOrderLayout::HeightsWidthsChannels,
            MemoryOrderLayout::HeightsChannelsWidths => PyMemoryOrderLayout::HeightsChannelsWidths,
            MemoryOrderLayout::ChannelsHeightsWidths => PyMemoryOrderLayout::ChannelsHeightsWidths,
            MemoryOrderLayout::ChannelsWidthsHeights => PyMemoryOrderLayout::ChannelsWidthsHeights,
            MemoryOrderLayout::WidthsChannelsHeights => PyMemoryOrderLayout::WidthsChannelsHeights,
            MemoryOrderLayout::WidthsHeightsChannels => PyMemoryOrderLayout::WidthsChannelsHeights,
        }
    }
}

//endregion

//endregion
