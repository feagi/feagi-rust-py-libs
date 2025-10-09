use feagi_data_serialization::{FeagiByteContainer, FeagiByteStructureType};
use feagi_data_structures::neuron_voxels::xyzp::CorticalMappedXYZPNeuronVoxels;
use feagi_data_structures::FeagiJSON;
use pyo3::{pyclass, PyObject};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyValueError;
use crate::feagi_data_serialization::PyFeagiSerializable;
use crate::feagi_data_structures::neurons_voxels::xyzp::{PyCorticalMappedXYZPNeuronVoxels};
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

    /// Creates and returns a new Python wrapper for the serialized structure at the given index.
    /// 
    /// This extracts the structure data from the container and wraps it in the appropriate
    /// Python class with proper inheritance from `FeagiSerializable`.
    /// 
    /// # Arguments
    ///
    /// * `py` - Python interpreter token
    /// * `index` - The index of the structure in the container (0-based)
    /// 
    /// # Returns
    /// A `PyObject` representing the deserialized structure with proper Python type
    /// 
    /// # Errors
    /// Returns an error if:
    /// - The index is out of bounds
    /// - The structure type is not yet supported in Python bindings
    /// - Deserialization fails
    pub fn try_create_new_struct_from_index(&self, py: Python, index: u8) -> PyResult<PyObject> {
        // First verify the index is valid and get the structure type
        let struct_types = self.inner.get_contained_struct_types();
        if index as usize >= struct_types.len() {
            return Err(PyValueError::new_err(format!("Index {} out of bounds, container only has {} structures", index, struct_types.len())));
        }
        
        let struct_type = struct_types[index as usize];
        
        // Match on type and create appropriate Python wrapper with inheritance
        match struct_type {
            FeagiByteStructureType::NeuronCategoricalXYZP => {
                // Create a new empty instance
                let mut rust_obj = CorticalMappedXYZPNeuronVoxels::new();
                // Update it with data from the container
                self.inner.try_update_struct_from_index(index, &mut rust_obj).map_err(PyFeagiError::from)?;
                // Wrap in Python object with proper inheritance
                PyCorticalMappedXYZPNeuronVoxels::instantiate_inherited_cortical_data(py, rust_obj)
            }
            FeagiByteStructureType::JSON => {
                // Create a new empty instance
                let mut rust_obj = FeagiJSON::new_empty();
                // Update it with data from the container
                self.inner.try_update_struct_from_index(index, &mut rust_obj).map_err(PyFeagiError::from)?;
                // For now, return as string or dict - JSON doesn't have a Python wrapper yet
                // TODO: Create proper PyFeagiJSON wrapper when needed
                Err(PyValueError::new_err("JSON structure type not yet implemented in Python bindings"))
            }
        }
    }

    //endregion
}