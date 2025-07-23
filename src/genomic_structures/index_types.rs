use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalIOChannelIndex, CorticalGroupingIndex, AgentDeviceIndex};

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
