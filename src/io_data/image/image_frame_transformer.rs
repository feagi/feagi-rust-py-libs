use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::ImageFrameTransformer;
use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace};
use crate::io_data::image::descriptors::{PyImageFrameProperties, PyColorSpace, PyCornerPoints};

#[pyclass]
#[pyo3(name = "ImageFrameTransformer")]
#[derive(Debug, Clone)]
pub struct PyImageFrameTransformer {
    pub(crate) inner: ImageFrameTransformer,
}

#[pymethods]
impl PyImageFrameTransformer {
    #[new]
    pub fn new(input_image_properties: PyImageFrameProperties) -> Self {
        PyImageFrameTransformer {
            inner: ImageFrameTransformer::new(input_image_properties.into())
        }
    }

    pub fn set_cropping_from(&mut self, upper_left: (usize, usize), lower_right: (usize, usize)) -> PyResult<()> {
        match self.inner.set_cropping_from(upper_left, lower_right) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn set_resizing_to(&mut self, target_resolution: (usize, usize)) -> PyResult<()> {
        match self.inner.set_resizing_to(target_resolution) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn set_conversion_to_color_space(&mut self, target_color_space: PyColorSpace) -> PyResult<()> {
        match self.inner.set_conversion_to_color_space(target_color_space.into()) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn set_brightness_multiplier(&mut self, brightness_factor: f32) -> PyResult<()> {
        match self.inner.set_brightness_multiplier(brightness_factor) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn set_contrast_adjustment(&mut self, contrast_factor: f32) -> PyResult<()> {
        match self.inner.set_contrast_adjustment(contrast_factor) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn set_conversion_to_grayscale(&mut self, convert_to_grayscale: bool) -> PyResult<()> {
        match self.inner.set_conversion_to_grayscale(convert_to_grayscale) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[getter]
    pub fn get_output_image_properties(&self) -> PyImageFrameProperties {
        self.inner.get_output_image_properties().into()
    }
}

impl From<PyImageFrameTransformer> for ImageFrameTransformer {
    fn from(py_transformer: PyImageFrameTransformer) -> Self {
        py_transformer.inner
    }
}

impl From<ImageFrameTransformer> for PyImageFrameTransformer {
    fn from(transformer: ImageFrameTransformer) -> Self {
        PyImageFrameTransformer { inner: transformer }
    }
}
