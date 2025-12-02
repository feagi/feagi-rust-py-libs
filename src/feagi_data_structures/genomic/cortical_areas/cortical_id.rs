use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::CorticalID;
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;

#[pyclass(eq, str)]
#[derive(PartialEq, Clone, Hash)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub(crate) inner: CorticalID,
}

#[pymethods]
impl PyCorticalID {

    //region Constructors

    /// Create a CorticalID from raw bytes.
    /// 
    /// Args:
    ///     bytes: A bytes object of exactly 8 bytes representing the cortical ID.
    /// 
    /// Returns:
    ///     CorticalID: The constructed cortical ID.
    /// 
    /// Raises:
    ///     ValueError: If the bytes are invalid or wrong length.
    #[staticmethod]
    pub fn try_from_bytes(bytes: [u8; CorticalID::CORTICAL_ID_LENGTH]) -> PyResult<Self> {
        let cortical_id = CorticalID::try_from_bytes(&bytes).map_err(PyFeagiError::from)?;
        Ok(cortical_id.into())
    }

    /// Create a CorticalID from a 64-bit unsigned integer.
    /// 
    /// Args:
    ///     value: A 64-bit unsigned integer representing the cortical ID.
    /// 
    /// Returns:
    ///     CorticalID: The constructed cortical ID.
    /// 
    /// Raises:
    ///     ValueError: If the integer represents an invalid cortical ID.
    #[staticmethod]
    pub fn try_from_u64(value: u64) -> PyResult<Self> {
        let cortical_id = CorticalID::try_from_u64(value).map_err(PyFeagiError::from)?;
        Ok(cortical_id.into())
    }

    /// Create a CorticalID from a base64-encoded string.
    /// 
    /// Args:
    ///     base64_str: A base64-encoded string representing the cortical ID.
    /// 
    /// Returns:
    ///     CorticalID: The constructed cortical ID.
    /// 
    /// Raises:
    ///     ValueError: If the string is not valid base64 or represents an invalid cortical ID.
    #[staticmethod]
    pub fn try_from_base_64(base64_str: &str) -> PyResult<Self> {
        let cortical_id = CorticalID::try_from_base_64(base64_str).map_err(PyFeagiError::from)?;
        Ok(cortical_id.into())
    }

    //endregion

    //region Export Methods

    /// Get the cortical ID as raw bytes.
    /// 
    /// Returns:
    ///     bytes: The 8-byte representation of the cortical ID.
    pub fn as_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.as_bytes())
    }

    /// Get the cortical ID as a 64-bit unsigned integer.
    /// 
    /// Returns:
    ///     int: The cortical ID as a 64-bit unsigned integer.
    pub fn as_u64(&self) -> u64 {
        self.inner.as_u64()
    }

    /// Get the cortical ID as a base64-encoded string.
    /// 
    /// Returns:
    ///     str: The base64-encoded representation of the cortical ID.
    pub fn as_base_64(&self) -> String {
        self.inner.as_base_64()
    }

    //endregion

    //region Constants (as class attributes)

    /// The length of the cortical ID in bytes (8 bytes).
    #[classattr]
    pub const CORTICAL_ID_LENGTH: usize = CorticalID::CORTICAL_ID_LENGTH;

    /// The number of bytes in the cortical ID (same as CORTICAL_ID_LENGTH).
    #[classattr]
    pub const NUMBER_OF_BYTES: usize = CorticalID::NUMBER_OF_BYTES;

    //endregion
}

project_display!(PyCorticalID);
py_type_casts!(PyCorticalID, CorticalID);
py_object_cast_generic!(PyCorticalID, CorticalID, "Unable to import CorticalID");