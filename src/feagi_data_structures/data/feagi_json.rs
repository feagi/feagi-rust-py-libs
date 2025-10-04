use pyo3::prelude::*;
use pyo3::types::PyBytes;

use ::feagi_data_structures_core::data::FeagiJSON as CoreFeagiJSON;
use crate::feagi_data_serialization::byte_structure::{FeagiByteStructure, FeagiByteStructureCompatible, FeagiByteStructureType};

#[pyclass]
#[pyo3(name = "FeagiJSON")]
pub struct PyFeagiJSON {
    inner: CoreFeagiJSON,
}

#[pymethods]
impl PyFeagiJSON {
    #[new]
    pub fn new() -> Self {
        Self { inner: CoreFeagiJSON::from_json_value(serde_json::Value::Null) }
    }

    #[staticmethod]
    pub fn from_json_str(s: &str) -> PyResult<Self> {
        let val: serde_json::Value = serde_json::from_str(s)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: CoreFeagiJSON::from_json_value(val) })
    }

    pub fn to_json_str(&self) -> PyResult<String> {
        Ok(self.inner.borrow_json_value().to_string())
    }

    pub fn to_feagi_bytes(&self) -> PyResult<Py<PyBytes>> {
        Python::with_gil(|py| {
            let compatible: &dyn FeagiByteStructureCompatible = &self.inner;
            let mut buf = vec![0u8; compatible.max_number_bytes_needed()];
            let wasted = compatible.overwrite_feagi_byte_structure_slice(buf.as_mut_slice())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            let used = buf.len() - wasted;
            buf.truncate(used);
            Ok(PyBytes::new_bound(py, &buf).unbind())
        })
    }

    #[staticmethod]
    pub fn from_feagi_bytes(b: &[u8]) -> PyResult<Self> {
        let fbs = FeagiByteStructure::create_from_bytes(b.to_vec())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let st = fbs.try_get_structure_type()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        if st != FeagiByteStructureType::JSON {
            return Err(pyo3::exceptions::PyValueError::new_err("Not a JSON byte structure"));
        }
        let json = <CoreFeagiJSON as FeagiByteStructureCompatible>::new_from_feagi_byte_structure(&fbs)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: json })
    }
}


