use feagi_data_structures::FeagiDataError;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::{PyErr, PyResult};
use std::fmt;

/// Custom error type for this crate that wraps FeagiDataError
/// This allows us to implement From traits without violating the orphan rule
#[derive(Debug)]
pub struct PyFeagiError(pub FeagiDataError);

impl fmt::Display for PyFeagiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PyFeagiError {}

impl From<FeagiDataError> for PyFeagiError {
    fn from(error: FeagiDataError) -> Self {
        PyFeagiError(error)
    }
}

impl From<PyFeagiError> for PyErr {
    fn from(error: PyFeagiError) -> PyErr {
        match error.0 {
            FeagiDataError::DeserializationError(msg) => PyValueError::new_err(msg),
            FeagiDataError::SerializationError(msg) => PyValueError::new_err(msg),
            FeagiDataError::BadParameters(msg) => PyValueError::new_err(msg),
            FeagiDataError::InternalError(msg) => PyRuntimeError::new_err(msg),
            FeagiDataError::NotImplemented => PyRuntimeError::new_err("Function not yet implemented! Please reach out on Github!"),
        }
    }
}

/// Type alias for Results using our custom error type
pub type PyFeagiResult<T> = Result<T, PyFeagiError>;