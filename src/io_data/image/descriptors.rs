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
    pub fn new(xy_resolution: (usize, usize), color_space: PyColorSpace, color_channel_layout: PyChannelLayout) -> PyResult<Self> {
        let color_space: ColorSpace = color_space.into();
        let color_channel_layout: ChannelLayout = color_channel_layout.into();
        let inner = ImageFrameProperties::new(xy_resolution, color_space, color_channel_layout);
        Ok(Self { inner }) // TODO error check for 0 res!
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
    pub fn expected_channel_layout(&self) -> PyResult<PyChannelLayout> {
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

//region ChannelLayout

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ChannelLayout")]
pub enum PyChannelLayout {
    GrayScale,
    RG,
    RGB,
    RGBA
}

impl From<PyChannelLayout> for ChannelLayout {
    fn from(py_channel_format: PyChannelLayout) -> Self {
        match py_channel_format {
            PyChannelLayout::GrayScale => ChannelLayout::GrayScale,
            PyChannelLayout::RG => ChannelLayout::RG,
            PyChannelLayout::RGB => ChannelLayout::RGB,
            PyChannelLayout::RGBA => ChannelLayout::RGBA,
        }
    }
}

impl From<ChannelLayout> for PyChannelLayout {
    fn from(channel_layout: ChannelLayout) -> Self {
        match channel_layout { 
            ChannelLayout::GrayScale => PyChannelLayout::GrayScale,
            ChannelLayout::RG => PyChannelLayout::RG,
            ChannelLayout::RGB => PyChannelLayout::RGB,
            ChannelLayout::RGBA => PyChannelLayout::RGBA,
        }
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

//region SegmentedFrameCenterProperties

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedFrameCenterProperties")]
pub struct PySegmentedFrameCenterProperties{
    pub inner: SegmentedFrameCenterProperties,
}

#[pymethods]
impl PySegmentedFrameCenterProperties {
    #[new]
    fn new_row_major_where_origin_top_left(center_coordinates_normalized_yx: (f32, f32), center_size_normalized_yx: (f32, f32)) -> PyResult<Self> {
        let result = SegmentedFrameCenterProperties::new_row_major_where_origin_top_left(center_coordinates_normalized_yx, center_size_normalized_yx);
        match result {
            Ok(inner) => Ok(PySegmentedFrameCenterProperties { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string())),
        }
    }

    #[staticmethod]
    fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy: (f32, f32), center_size_normalized_xy: (f32, f32)) -> PyResult<Self> {
        let result = SegmentedFrameCenterProperties::cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy, center_size_normalized_xy);
        match result {
            Ok(inner) => Ok(PySegmentedFrameCenterProperties { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string())),
        }
    }

    #[staticmethod]
    fn create_default_centered() -> Self {
        PySegmentedFrameCenterProperties {
            inner: SegmentedFrameCenterProperties::create_default_centered(),
        }
    }
}

impl From<PySegmentedFrameCenterProperties> for SegmentedFrameCenterProperties {
    fn from(value: PySegmentedFrameCenterProperties) -> Self {
        value.inner
    }
}

impl From<SegmentedFrameCenterProperties> for PySegmentedFrameCenterProperties {
    fn from(value: SegmentedFrameCenterProperties) -> Self {
        PySegmentedFrameCenterProperties {inner: value}
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
