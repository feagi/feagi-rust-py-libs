use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};
use feagi_connector_core::data_types::GazeProperties;
use crate::{create_pyclass, __base_py_class_shared};
use crate::feagi_connector_core::data_types::{PyPercentage, PyPercentage2D};

create_pyclass!(PyGazeProperties, GazeProperties, "GazeProperties");

// Additional From implementation to ensure compatibility with connector_agent macros
impl From<&GazeProperties> for PyGazeProperties {
    fn from(inner: &GazeProperties) -> Self {
        PyGazeProperties { inner: inner.clone() }
    }
}

#[pymethods]
impl PyGazeProperties {

    #[new]
    fn new(eccentricity_center_xy: PyPercentage2D, modularity_size: PyPercentage) -> PyResult<Self> {
        let inner = GazeProperties::new(eccentricity_center_xy.into(), modularity_size.into());
        Ok(PyGazeProperties { inner })
    }

    #[staticmethod]
    fn create_default_centered() -> Self {
        PyGazeProperties {
            inner: GazeProperties::create_default_centered(),
        }
    }
}
