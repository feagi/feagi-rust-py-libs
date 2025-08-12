use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::image_descriptors::*;



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

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedVisionTargetResolutions")]
pub struct PySegmentedVisionTargetResolutions{
    pub inner: SegmentedFrameTargetResolutions,
}

#[pymethods]
impl PySegmentedVisionTargetResolutions {
    #[new]
    pub fn new(        lower_left: (usize, usize),
                       middle_left: (usize, usize),
                       upper_left: (usize, usize),
                       upper_middle: (usize, usize),
                       upper_right: (usize, usize),
                       middle_right: (usize, usize),
                       lower_right: (usize, usize),
                       lower_middle: (usize, usize),
                       center: (usize, usize)
    ) -> PyResult<Self> {
        let result = SegmentedFrameTargetResolutions::new(lower_left, middle_left, upper_left, upper_middle, upper_right, middle_right, lower_right, lower_middle, center);
        match result {
            Ok(inner) => Ok(PySegmentedVisionTargetResolutions { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[staticmethod]
    fn create_with_same_sized_peripheral(center_width_height: (usize, usize), peripheral_width_height: (usize, usize)) -> PyResult<Self> {
        let result = SegmentedFrameTargetResolutions::create_with_same_sized_peripheral(center_width_height, peripheral_width_height);
        match result {
            Ok(inner) => Ok(PySegmentedVisionTargetResolutions { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

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