use std::fmt::{Display, Formatter};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::data::descriptors::*;
use feagi_data_structures::FeagiDataError;
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;

//region Image XY

#[pyclass(str)]
#[pyo3(name = "ImageXYPoint")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyImageXYPoint {
    inner: ImageXYPoint
}

#[pymethods]
impl PyImageXYPoint {
    #[new]
    pub fn new(x: u32, y: u32) -> PyResult<Self> {
        Ok(PyImageXYPoint {
            inner: ImageXYPoint::new(x, y)
        })
    }

    #[getter]
    pub fn x(&self) -> u32 {self.inner.x}

    #[getter]
    pub fn y(&self) -> u32 { self.inner.y }
}

impl TryFrom<(u32, u32)> for PyImageXYPoint {
    type Error = PyErr;
    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        PyImageXYPoint::new(value.0, value.1)
    }
}


py_type_casts!(PyImageXYPoint, ImageXYPoint);
py_object_cast_generic!(PyImageXYPoint, ImageXYPoint, "Unable to retrieve ImageXYPoint data from given!");
project_display!(PyImageXYPoint);



#[pyclass(str)] // TODO this should be u32!
#[pyo3(name = "ImageXYResolution")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyImageXYResolution {
    inner: ImageXYResolution
}

#[pymethods]
impl PyImageXYResolution {
    #[new]
    pub fn new(width: usize, height: usize) -> PyResult<Self> {
        Ok(PyImageXYResolution {
            inner: ImageXYResolution::new(width as u32, height as u32).map_err(PyFeagiError::from)?
        })
    }

    #[getter]
    pub fn width(&self) -> usize {
        self.inner.width as usize
    }

    #[getter]
    pub fn height(&self) -> usize {
        self.inner.height as usize
    }
}

impl TryFrom<(usize, usize)> for PyImageXYResolution {
    type Error = PyErr;
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        PyImageXYResolution::new(value.0, value.1)
    }
}

py_type_casts!(PyImageXYResolution, ImageXYResolution);
py_object_cast_generic!(PyImageXYResolution, ImageXYResolution, "Unable to retrieve ImageXYResolution data from given!");
project_display!(PyImageXYResolution);

//endregion

//region Segmented Image XY Resolutions

#[pyclass(str)]
#[pyo3(name = "SegmentedXYImageResolutions")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PySegmentedXYImageResolutions {
    inner: SegmentedXYImageResolutions
}

#[pymethods]
impl PySegmentedXYImageResolutions {
    #[new]
    pub fn new(
        lower_left: PyImageXYResolution,
        lower_middle: PyImageXYResolution,
        lower_right: PyImageXYResolution,
        middle_left: PyImageXYResolution,
        center: PyImageXYResolution,
        middle_right: PyImageXYResolution,
        upper_left: PyImageXYResolution,
        upper_middle: PyImageXYResolution,
        upper_right: PyImageXYResolution,
    ) -> Self {
        let inner = SegmentedXYImageResolutions::new(
            lower_left.into(),
            lower_middle.into(),
            lower_right.into(),
            middle_left.into(),
            center.into(),
            middle_right.into(),
            upper_left.into(),
            upper_middle.into(),
            upper_right.into(),
        );
        PySegmentedXYImageResolutions {
            inner
        }
    }

    #[staticmethod]
    pub fn create_with_same_sized_peripheral(center_resolution: PyImageXYResolution, peripheral_resolutions: PyImageXYResolution) -> PySegmentedXYImageResolutions {
        SegmentedXYImageResolutions::create_with_same_sized_peripheral(center_resolution.into(), peripheral_resolutions.into()).into()
    }

    //TODO
    /*
    pub fn as_ordered_array(&self) -> Vec<PyImageXYResolution> {
        let refs = self.inner.as_ordered_array();
        vec![refs.into()]
    }
    
     */
}

py_type_casts!(PySegmentedXYImageResolutions, SegmentedXYImageResolutions);
py_object_cast_generic!(PySegmentedXYImageResolutions, SegmentedXYImageResolutions, "Unable to retrieve SegmentedXYImageResolutions data from given!");
project_display!(PySegmentedXYImageResolutions);

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

//region Image Frame Properties

#[pyclass(str)]
#[pyo3(name = "ImageFrameProperties")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyImageFrameProperties {
    inner: ImageFrameProperties
}

#[pymethods]
impl PyImageFrameProperties {
    #[new] // TODO accept PyObject for xy_resolution
    pub fn new(xy_resolution: PyImageXYResolution, color_space: PyColorSpace, color_channel_layout: PyColorChannelLayout) -> PyResult<Self> {
        let color_space: ColorSpace = color_space.into();
        let color_channel_layout: ColorChannelLayout = color_channel_layout.into();
        let inner = ImageFrameProperties::new(xy_resolution.into(), color_space, color_channel_layout)
            .map_err(PyFeagiError::from)?;
        Ok(PyImageFrameProperties { inner })
    }

    #[getter]
    pub fn xy_resolution(&self) -> PyResult<PyImageXYResolution> {
        Ok(self.inner.get_image_resolution().into())
    }

    #[getter]
    pub fn color_space(&self) -> PyResult<PyColorSpace> {
        Ok(self.inner.get_color_space().into())
    }

    #[getter]
    pub fn channel_layout(&self) -> PyResult<PyColorChannelLayout> {
        Ok(self.inner.get_color_channel_layout().into())
    }
}

py_type_casts!(PyImageFrameProperties, ImageFrameProperties);
py_object_cast_generic!(PyImageFrameProperties, ImageFrameProperties, "Unable to retrieve ImageFrameProperties data from given!");
project_display!(PyImageFrameProperties);

//endregion

//region Segmented Image Frame Properties

#[pyclass(str)]
#[pyo3(name = "SegmentedImageFrameProperties")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PySegmentedImageFrameProperties {
    inner: SegmentedImageFrameProperties
}

#[pymethods]
impl PySegmentedImageFrameProperties {
    #[new]
    pub fn new(
        segment_xy_resolutions: PySegmentedXYImageResolutions,
        center_color_channels: PyColorChannelLayout,
        peripheral_color_channels: PyColorChannelLayout,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        Ok(SegmentedImageFrameProperties::new(
            &segment_xy_resolutions.into(),
            &center_color_channels.into(),
            &peripheral_color_channels.into(),
            &color_space.into(),
        ).into())
    }

    #[getter]
    pub fn resolutions(&self) -> PySegmentedXYImageResolutions {
        self.inner.get_resolutions().clone().into()
    }

    #[getter]
    pub fn center_color_channel(&self) -> PyColorChannelLayout {
        self.inner.get_center_color_channel().into()
    }


    #[getter]
    pub fn peripheral_color_channels(&self) -> PyColorChannelLayout {
        self.inner.get_peripheral_color_channels().into()
    }
    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    // TODO verify_segmented_image_frame_matches_properties?
}

py_type_casts!(PySegmentedImageFrameProperties, SegmentedImageFrameProperties);
py_object_cast_generic!(PySegmentedImageFrameProperties, SegmentedImageFrameProperties, "Unable to retrieve SegmentedImageFrameProperties data from given!");
project_display!(PySegmentedImageFrameProperties);

//endregion

//region Corner Points

//TODO PyCornerPoints

//endregion

//region Gaze Properties

#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "GazeProperties")]
pub struct PyGazeProperties{
    pub inner: GazeProperties,
}

#[pymethods]
impl PyGazeProperties {

    #[new]
    fn new(eccentricity_center_xy: (f32, f32), modularity_size_xy: (f32, f32)) -> PyResult<Self> {

        let eccentricity_center_xy = (GazeEccentricity::new_from_0_1_unchecked(eccentricity_center_xy.0), GazeEccentricity::new_from_0_1_unchecked(eccentricity_center_xy.1));
        let modularity_size_xy = (GazeModulation::new_from_0_1_unchecked(modularity_size_xy.0) , GazeModulation::new_from_0_1_unchecked(modularity_size_xy.1));

        let inner = GazeProperties::new(eccentricity_center_xy, modularity_size_xy);
        Ok(PyGazeProperties { inner })
    }


    #[staticmethod]
    fn create_default_centered() -> Self {
        PyGazeProperties {
            inner: GazeProperties::create_default_centered(),
        }
    }

}

py_type_casts!(PyGazeProperties, GazeProperties);
py_object_cast_generic!(PyGazeProperties, GazeProperties, "Unable to retrieve GazeProperties data from given!");
project_display!(PyGazeProperties);

//endregion