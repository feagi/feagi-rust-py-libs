use feagi_data_serialization::FeagiByteContainer;
use pyo3::{pyclass};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use crate::feagi_data_serialization::PyFeagiSerializable;
use crate::py_error::PyFeagiError;

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "FeagiByteContainer")]
pub struct PyFeagiByteContainer {
    pub(crate) inner: FeagiByteContainer,
}

#[pymethods]
impl PyFeagiByteContainer {

    //region Constructors

    #[new]
    pub fn new() -> Self {
        PyFeagiByteContainer {
            inner: FeagiByteContainer::new_empty()
        }
    }

    //endregion

    //region Direct Data Access
    // some differences here since references dont apply in python

    pub fn copy_out_as_byte_vector<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.inner.get_byte_ref()))
    }

    pub fn load_bytes_and_verify<'py>(&mut self, py: Python<'py>, bytes: Bound<'py, PyBytes>) -> PyResult<()> {
        let byte_arr: Vec<u8> = bytes.as_bytes().to_vec();
        self.inner.try_write_data_to_container_and_verify(
            &mut | current_bytes| {
                current_bytes.clear();
                current_bytes.extend_from_slice(&byte_arr);
                Ok(())
            }
        ).map_err(PyFeagiError::from)?;
        Ok(())
    }

    //endregion



    //region Get Properties

    #[getter]
    pub fn valid(&self) -> bool {
        self.inner.is_valid()
    }

    #[getter]
    pub fn number_contained_structures(&self) -> PyResult<usize> {
        Ok(self.inner.try_get_number_contained_structures().map_err(PyFeagiError::from)?)
    }

    #[getter]
    pub fn number_of_bytes_used(&self) -> usize {
        self.inner.get_number_of_bytes_used()
    }

    #[getter]
    pub fn number_of_bytes_allocated(&self) -> usize {
        self.inner.get_number_of_bytes_allocated()
    }

    #[getter]
    pub fn increment_counter(&self) -> PyResult<u16> {
        Ok(self.inner.get_increment_counter().map_err(PyFeagiError::from)?)
    }


    //endregion



    //region Extracting Struct Data

    pub fn try_create_new_struct_from_index(&self, index: u8) -> PyResult<PyFeagiSerializable> {
        let deserialized = self.inner.try_create_new_struct_from_index(index).map_err(PyFeagiError::from)?;

    }

    //endregion
}