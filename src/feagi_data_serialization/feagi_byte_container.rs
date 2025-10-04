use feagi_data_serialization::FeagiByteContainer;
use pyo3::{pyclass};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

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
    // get_byte_ref and try_write_data_to_container_and_verify make little sense in python

    pub fn copy_out_as_byte_vector<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.inner.get_byte_ref()))
    }

    //endregion



    //region Get Properties

    #[getter]
    pub fn valid(&self) -> bool {
        self.inner.is_valid()
    }



    //endregion

}