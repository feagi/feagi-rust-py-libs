use std::fmt::Formatter;
use numpy::{PyArray3, PyReadonlyArray3};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::data::ImageFrame;
use crate::feagi_data_structures::data::image_descriptors::*;
use crate::{project_display, py_object_cast_generic, py_type_casts};

#[pyclass(str)]
#[pyo3(name = "ImageFrame")]
#[derive(Clone)]
pub struct PyImageFrame {
    pub inner: ImageFrame,
}

#[pymethods]
impl PyImageFrame {
    
    #[staticmethod]
    pub fn INTERNAL_MEMORY_LAYOUT() -> PyMemoryOrderLayout { ImageFrame::INTERNAL_MEMORY_LAYOUT.into() }
    
    //region common contructors
    #[new]
    pub fn new(channel_format: PyColorChannelLayout, color_space: PyColorSpace, xy_resolution: PyImageXYResolution) -> PyResult<Self> {
        let result = ImageFrame::new(&channel_format.into(), &color_space.into(), &xy_resolution.into());
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(format!("{}", err))),

        }
    }

    #[staticmethod]
    pub fn from_array(input: PyReadonlyArray3<f32>, color_space: PyColorSpace, source_memory_order: PyMemoryOrderLayout, py: Python) -> PyResult<PyImageFrame> {
        let array = input.as_array().to_owned();
        match ImageFrame::from_array(array, &color_space.into(), &source_memory_order.into()) {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_image_frame_properties(image_frame_properties: PyImageFrameProperties) -> PyResult<PyImageFrame> {
        let result = ImageFrame::from_image_frame_properties(&image_frame_properties.into());
        match result {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    //endregion

    //region get properties

    #[staticmethod]
    pub fn do_resolutions_channel_depth_and_color_spaces_match(a: &PyImageFrame, b: &PyImageFrame) -> bool {
        ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&a.inner, &b.inner)
    }

    #[staticmethod]
    pub fn is_array_valid_for_image_frame(array: PyReadonlyArray3<f32>, py: Python) -> bool {
        let array_view = array.as_array().to_owned();
        ImageFrame::is_array_valid_for_image_frame(&array_view.to_owned())
    }

    pub fn get_image_frame_properties(&self) -> PyImageFrameProperties {
        self.inner.get_image_frame_properties().into()
    }
    
    #[getter]
    pub fn channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_channel_layout().clone().into()
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    #[getter]
    pub fn color_channel_count(&self) -> usize {
        self.inner.get_color_channel_count()
    }

    // NOTE: get_pixels_view skipped, equivalent is copy_to_numpy_array

    pub fn copy_to_numpy_array(&self, py: Python) -> PyResult<Py<PyArray3<f32>>> {
        Ok(Py::from(PyArray3::from_array(py, &self.inner.get_pixels_view())))
    }
    
    #[getter]
    pub fn get_number_elements(&self) -> usize {
        self.inner.get_number_elements()
    }

    //endregion

    //region Image Processing

    //region In-Place

    pub fn change_brightness(&mut self, brightness_factor: f32) -> PyResult<()> {
        match self.inner.change_brightness(brightness_factor) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn change_contrast(&mut self, contrast_factor: f32) -> PyResult<()> {
        match self.inner.change_contrast(contrast_factor) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    //endregion

    //region Out-Place

    pub fn resize_nearest_neighbor(&mut self, target_width_height: (usize, usize)) -> PyResult<()> {
        let result = self.resize_nearest_neighbor(target_width_height);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    //endregion

    //endregion

    //region Neuron Export

    // NOTE: write_thresholded_xyzp_neuron_arrays is not exposed as it makes no sense for python

    //endregion
}

py_type_casts!(PyImageFrame, ImageFrame);
py_object_cast_generic!(PyImageFrame, ImageFrame, "Unable to retrieve ImageFrame data from given!");
project_display!(PyImageFrame);