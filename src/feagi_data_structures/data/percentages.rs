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

#[pyclass(str)]
#[pyo3(name = "SignedPercentage")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySignedPercentage {
    inner: feagi_data_structures::data::SignedPercentage
}

#[pymethods]
impl PySignedPercentage {
    // Note: Skip any unchecked methods

    //region Constructors

    #[staticmethod]
    pub fn new_from_m1_1(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_data_structures::data::SignedPercentage::new_from_m1_1(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_scaled_from_0_1(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_data_structures::data::SignedPercentage::new_scaled_from_0_1(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_m100_100(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_data_structures::data::SignedPercentage::new_from_m100_100(value).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    #[staticmethod]
    pub fn new_from_linear_interp(value: f32, lower_range: f32, upper_range: f32) -> PyResult<PySignedPercentage> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err("Lower range cannot be greater than upper range!"));
        }

        Ok(PySignedPercentage {
            inner: feagi_data_structures::data::SignedPercentage::new_from_linear_interp(value, &(lower_range..upper_range)).map_err(|err| PyValueError::new_err(err.to_string()))?
        })
    }

    //endregion

    //region Update

    pub fn inplace_update_from_m1_1(&mut self, value: f32) -> PyResult<()> {
        self.inner.inplace_update_from_m1_1(value).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_m100_100(&mut self, value: f32) -> PyResult<()> {
        self.inner.inplace_update_m100_100(value).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_linear_interp(&mut self, value: f32, lower_range: f32, upper_range: f32) -> PyResult<()> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err("Lower range cannot be greater than upper range!"));
        }
        self.inner.inplace_update_linear_interp(value, &(lower_range..upper_range)).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    //endregion

    //region Properties

    pub fn get_as_m1_1(&self) -> f32 {
        self.inner.get_as_m1_1()
    }

    pub fn get_as_m100_100(&self) -> f32 {
        self.inner.get_as_m100_100()
    }

    //endregion
}

py_type_casts!(PySignedPercentage, feagi_data_structures::data::SignedPercentage);
py_object_cast_generic!(PySignedPercentage, feagi_data_structures::data::SignedPercentage, "Unable to retrieve SignedPercentage data from given!");
project_display!(PySignedPercentage);

//endregion

//region 2D Percentage Types

#[pyclass(str)]
#[pyo3(name = "Percentage2D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyPercentage2D {
    inner: feagi_data_structures::data::Percentage2D
}

#[pymethods]
impl PyPercentage2D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage) -> Self {
        PyPercentage2D {
            inner: feagi_data_structures::data::Percentage2D::new(a.into(), b.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage2D {
            inner: feagi_data_structures::data::Percentage2D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage2D {
            inner: feagi_data_structures::data::Percentage2D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }
}

py_type_casts!(PyPercentage2D, feagi_data_structures::data::Percentage2D);
py_object_cast_generic!(PyPercentage2D, feagi_data_structures::data::Percentage2D, "Unable to retrieve Percentage2D data from given!");
project_display!(PyPercentage2D);

#[pyclass(str)]
#[pyo3(name = "SignedPercentage2D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySignedPercentage2D {
    inner: feagi_data_structures::data::SignedPercentage2D
}

#[pymethods]
impl PySignedPercentage2D {
    #[new]
    pub fn new(a: PySignedPercentage, b: PySignedPercentage) -> Self {
        PySignedPercentage2D {
            inner: feagi_data_structures::data::SignedPercentage2D::new(a.into(), b.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage2D {
            inner: feagi_data_structures::data::SignedPercentage2D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage2D {
            inner: feagi_data_structures::data::SignedPercentage2D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }
}

py_type_casts!(PySignedPercentage2D, feagi_data_structures::data::SignedPercentage2D);
py_object_cast_generic!(PySignedPercentage2D, feagi_data_structures::data::SignedPercentage2D, "Unable to retrieve SignedPercentage2D data from given!");
project_display!(PySignedPercentage2D);

//endregion

//region 3D Percentage Types

#[pyclass(str)]
#[pyo3(name = "Percentage3D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyPercentage3D {
    inner: feagi_data_structures::data::Percentage3D
}

#[pymethods]
impl PyPercentage3D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage, c: PyPercentage) -> Self {
        PyPercentage3D {
            inner: feagi_data_structures::data::Percentage3D::new(a.into(), b.into(), c.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage3D {
            inner: feagi_data_structures::data::Percentage3D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage3D {
            inner: feagi_data_structures::data::Percentage3D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PyPercentage {
        self.inner.c.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PyPercentage) {
        self.inner.c = value.into();
    }
}

py_type_casts!(PyPercentage3D, feagi_data_structures::data::Percentage3D);
py_object_cast_generic!(PyPercentage3D, feagi_data_structures::data::Percentage3D, "Unable to retrieve Percentage3D data from given!");
project_display!(PyPercentage3D);

#[pyclass(str)]
#[pyo3(name = "SignedPercentage3D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySignedPercentage3D {
    inner: feagi_data_structures::data::SignedPercentage3D
}

#[pymethods]
impl PySignedPercentage3D {
    #[new]
    pub fn new(a: PySignedPercentage, b: PySignedPercentage, c: PySignedPercentage) -> Self {
        PySignedPercentage3D {
            inner: feagi_data_structures::data::SignedPercentage3D::new(a.into(), b.into(), c.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage3D {
            inner: feagi_data_structures::data::SignedPercentage3D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage3D {
            inner: feagi_data_structures::data::SignedPercentage3D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PySignedPercentage {
        self.inner.c.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PySignedPercentage) {
        self.inner.c = value.into();
    }
}

py_type_casts!(PySignedPercentage3D, feagi_data_structures::data::SignedPercentage3D);
py_object_cast_generic!(PySignedPercentage3D, feagi_data_structures::data::SignedPercentage3D, "Unable to retrieve SignedPercentage3D data from given!");
project_display!(PySignedPercentage3D);

//endregion

//region 4D Percentage Types

#[pyclass(str)]
#[pyo3(name = "Percentage4D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyPercentage4D {
    inner: feagi_data_structures::data::Percentage4D
}

#[pymethods]
impl PyPercentage4D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage, c: PyPercentage, d: PyPercentage) -> Self {
        PyPercentage4D {
            inner: feagi_data_structures::data::Percentage4D::new(a.into(), b.into(), c.into(), d.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage4D {
            inner: feagi_data_structures::data::Percentage4D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage4D {
            inner: feagi_data_structures::data::Percentage4D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PyPercentage {
        self.inner.c.into()
    }

    #[getter]
    pub fn d(&self) -> PyPercentage {
        self.inner.d.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PyPercentage) {
        self.inner.c = value.into();
    }

    #[setter]
    pub fn set_d(&mut self, value: PyPercentage) {
        self.inner.d = value.into();
    }
}

py_type_casts!(PyPercentage4D, feagi_data_structures::data::Percentage4D);
py_object_cast_generic!(PyPercentage4D, feagi_data_structures::data::Percentage4D, "Unable to retrieve Percentage4D data from given!");
project_display!(PyPercentage4D);

#[pyclass(str)]
#[pyo3(name = "SignedPercentage4D")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySignedPercentage4D {
    inner: feagi_data_structures::data::SignedPercentage4D
}

#[pymethods]
impl PySignedPercentage4D {
    #[new]
    pub fn new(a: PySignedPercentage, b: PySignedPercentage, c: PySignedPercentage, d: PySignedPercentage) -> Self {
        PySignedPercentage4D {
            inner: feagi_data_structures::data::SignedPercentage4D::new(a.into(), b.into(), c.into(), d.into())
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage4D {
            inner: feagi_data_structures::data::SignedPercentage4D::new_zero()
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage4D {
            inner: feagi_data_structures::data::SignedPercentage4D::new_identical_percentages(percentage.into())
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PySignedPercentage {
        self.inner.c.into()
    }

    #[getter]
    pub fn d(&self) -> PySignedPercentage {
        self.inner.d.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PySignedPercentage) {
        self.inner.c = value.into();
    }

    #[setter]
    pub fn set_d(&mut self, value: PySignedPercentage) {
        self.inner.d = value.into();
    }
}

py_type_casts!(PySignedPercentage4D, feagi_data_structures::data::SignedPercentage4D);
py_object_cast_generic!(PySignedPercentage4D, feagi_data_structures::data::SignedPercentage4D, "Unable to retrieve SignedPercentage4D data from given!");
project_display!(PySignedPercentage4D);

//endregion