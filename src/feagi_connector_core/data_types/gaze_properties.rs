use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, PyResult};
use feagi_data_structures::FeagiDataError;
use feagi_connector_core::data_types::GazeProperties;
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::feagi_connector_core::data_types::{PyPercentage, PyPercentage2D};

#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "GazeProperties")]
pub struct PyGazeProperties{
    pub inner: GazeProperties,
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

py_type_casts!(PyGazeProperties, GazeProperties);
py_object_cast_generic!(PyGazeProperties, GazeProperties, "Unable to retrieve GazeProperties data from given!");
project_display!(PyGazeProperties);

