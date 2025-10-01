use feagi_data_structures::data::descriptors::MiscDataDimensions;
use feagi_data_structures::data::{ImageFrame, MiscData};
use numpy::{PyArray3, PyReadonlyArray3};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use pyo3::exceptions::PyValueError;
use crate::{project_display, py_object_cast_generic, py_type_casts};

#[pyclass(str)]
#[pyo3(name = "MiscData")]
#[derive(Clone)]
pub struct PyMiscData {
    pub inner: MiscData,
}

#[pymethods]
impl PyMiscData {

    #[new]
    pub fn new(x: u32, y: u32, z: u32) -> PyResult<PyMiscData> {
        let resolution = MiscDataDimensions::new(x, y, z).unwrap_or_else(|e| panic!("{}", e));
        Ok(PyMiscData { inner: MiscData::new(&resolution).unwrap()})
    }
    
    #[staticmethod]
    pub fn new_from_array(input: PyReadonlyArray3<f32>, py: Python) -> PyResult<PyMiscData> {
        let array = input.as_array().to_owned();
        match MiscData::new_with_data(array) {
            Ok(inner) => Ok(PyMiscData { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn copy_to_numpy_array<'py>(&self, py: Python) -> PyResult<Py<PyArray3<f32>>> {
        Ok(Py::from(PyArray3::from_array(py, &self.inner.get_internal_data())))
    }

    pub fn blank_data(&mut self) -> PyResult<()> {
        self.inner.blank_data();
        Ok(())
    }

}

py_type_casts!(PyMiscData, MiscData);
py_object_cast_generic!(PyMiscData, MiscData, "Unable to retrieve MiscData data from given!");
project_display!(PyMiscData);


