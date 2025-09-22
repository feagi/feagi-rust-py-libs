use std::fmt::{Display, Formatter};
use feagi_data_structures::data::Percentage;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use crate::{project_display, py_object_cast_generic, py_type_casts};


//region Percentage (0 - 1)

#[pyclass(str)]
#[pyo3(name = "Percentage")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyPercentage {
    inner: Percentage
}

#[pymethods]
impl PyPercentage {
    // Note: Skip any unchecked methods

    //region Constructors

    #[staticmethod]
    pub fn new_from_0_1(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_0_1(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_interp_m1_1(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_interp_m1_1(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_u8_0_255(value: u8) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_u8_0_255(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_0_100(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_0_100(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_linear_interp(value: f32, lower_range: f32, upper_range: f32) -> PyResult<PyPercentage> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err("Lower range cannot be greater than upper range!"));
        }

        Ok(PyPercentage {
            inner: Percentage::new_from_linear_interp(value, &(lower_range..upper_range)).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    //endregion

    //region Update

    pub fn inplace_update_from_0_1(&mut self, value: f32) -> PyResult<()> {
        self.inner.inplace_update_from_0_1(value).map_err(|err| PyValueError::new_err(err.to_string()));
        Ok(())
    }

    pub fn inplace_update_u8_0_255(&mut self, value: u8) -> PyResult<()> {
        self.inner.inplace_update_u8_0_255(value).map_err(|err| PyValueError::new_err(err.to_string()));
        Ok(())
    }

    pub fn inplace_update_0_100(&mut self, value: f32) -> PyResult<()> {
        self.inner.inplace_update_0_100(value).map_err(|err| PyValueError::new_err(err.to_string()));
        Ok(())
    }

    pub fn inplace_update_linear_interp(&mut self, value: f32, lower_range: f32, upper_range: f32) -> PyResult<()> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err("Lower range cannot be greater than upper range!"));
        }
        self.inner.inplace_update_linear_interp(value, &(lower_range..upper_range));
        Ok(())
    }

    //endregion

    //region Properties

    pub fn get_as_0_1(&self) -> f32 {
        self.inner.get_as_0_1()
    }

    pub fn get_as_u8(&self) -> u8 {
        (self.inner.get_as_u8())
    }

    pub fn get_as_0_100(&self) -> f32 {
        self.inner.get_as_0_100()
    }

    //endregion
}

py_type_casts!(PyPercentage, Percentage);
py_object_cast_generic!(PyPercentage, Percentage, "Unable to retrieve Percentage data from given!");
project_display!(PyPercentage);



//endregion

//region SignedPercentage (-1 to 1)

//endregion

//region 2D Percentage Types

//endregion

//region 3D Percentage Types

//endregion

//region 4D Percentage Types

//endregion