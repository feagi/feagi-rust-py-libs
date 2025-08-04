use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
use pyo3::{pyclass, pymethods, Bound, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalIOChannelIndex, CorticalGroupingIndex, AgentDeviceIndex};
use feagi_core_data_structures_and_processing::io_data::IOTypeData;
use pyo3::types::{PyFloat, PyInt};

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "CorticalGroupingIndex")]
pub struct PyCorticalGroupingIndex {
    pub(crate) inner: CorticalGroupingIndex,
}

impl From<PyCorticalGroupingIndex> for CorticalGroupingIndex {
    fn from(p: PyCorticalGroupingIndex) -> Self {
        p.inner
    }
}

#[pymethods]
impl PyCorticalGroupingIndex {
    #[new]
    pub fn new(index: u8) -> PyResult<Self> {
        let result = CorticalGroupingIndex::try_from(index);
        match result {
            Ok(t) => Ok(PyCorticalGroupingIndex {inner: t}),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

// not exposed to python
impl PyCorticalGroupingIndex{
    pub(crate) fn try_from_python<'py>(py: Python<'_>, any: PyObject) -> Result<CorticalGroupingIndex, FeagiDataProcessingError> {
        let bound = any.bind(py);
        match () { 
            _ if bound.is_instance_of::<PyInt>() => {
                let int_val = any.extract::<u8>(py)
                    .map_err(|_| IODataError::InvalidParameters("Failed to extract uint8 value. Is the given value too large or negative?".into()))?;
                Ok(CorticalGroupingIndex::from(int_val))
            }
            _ if bound.is_instance_of::<PyCorticalGroupingIndex>() => {
                let group_index_val = any.extract::<PyCorticalGroupingIndex>(py)
                    .map_err(|_| IODataError::InvalidParameters("Failed to extract uint8 value. Is the given value too large or negative?".into()))?;
                Ok(CorticalGroupingIndex::from(group_index_val))
            }
            // If nothing matches, return error
            _ => Err(IODataError::InvalidParameters("Unable to parse given data as a CorticalGroupingIndex!".into()).into())
        }
    }
}

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "CorticalIOChannelIndex")]
pub struct PyCorticalIOChannelIndex {
    pub(crate) inner: CorticalIOChannelIndex,
}

impl From<PyCorticalIOChannelIndex> for CorticalIOChannelIndex {
    fn from(p: PyCorticalIOChannelIndex) -> Self {
        p.inner
    }
}

#[pymethods]
impl PyCorticalIOChannelIndex {
    #[new]
    pub fn new(index: u32) -> PyResult<Self> {
        let result = CorticalIOChannelIndex::try_from(index);
        match result {
            Ok(t) => Ok(PyCorticalIOChannelIndex {inner: t}),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

// not exposed to python
impl PyCorticalIOChannelIndex{
    pub(crate) fn try_from_python<'py>(py: Python<'_>, any: PyObject) -> Result<CorticalIOChannelIndex, FeagiDataProcessingError> {
        let bound = any.bind(py);
        match () {
            _ if bound.is_instance_of::<PyInt>() => {
                let int_val = any.extract::<u32>(py)
                    .map_err(|_| IODataError::InvalidParameters("Failed to extract uin32 value. Is the given value too large or negative?".into()))?;
                Ok(CorticalIOChannelIndex::from(int_val))
            }
            _ if bound.is_instance_of::<PyCorticalIOChannelIndex>() => {
                let channel_index_val = any.extract::<PyCorticalIOChannelIndex>(py)
                    .map_err(|_| IODataError::InvalidParameters("Failed to extract uint32 value. Is the given value too large?".into()))?;
                Ok(CorticalIOChannelIndex::from(channel_index_val))
            }
            // If nothing matches, return error
            _ => Err(IODataError::InvalidParameters("Unable to parse given data as a CorticalIOChannelIndex!".into()).into())
        }
    }
}
