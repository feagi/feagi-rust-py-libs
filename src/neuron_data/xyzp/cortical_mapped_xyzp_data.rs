use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use numpy::PyArray1;
use feagi_core_data_structures_and_processing::neuron_data::xyzp::{CorticalMappedXYZPNeuronData};
use feagi_core_data_structures_and_processing::io_processing::byte_structures::{FeagiByteStructureCompatible};
use crate::io_processing::byte_structures::{PyFeagiByteStructure, PyFeagiByteStructureCompatible, PyFeagiByteStructureType};
use crate::genomic_structures::{PyCorticalID};
use super::neuron_xyzp_arrays::{PyNeuronXYZPArrays, tuple_nd_array_to_tuple_np_array};

#[pyclass(extends=PyFeagiByteStructureCompatible)]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedXYZPNeuronData")]
pub struct PyCorticalMappedXYZPNeuronData { // HashMap<CorticalID, NeuronYXCPArrays>
    pub inner: CorticalMappedXYZPNeuronData
}

impl PyCorticalMappedXYZPNeuronData {
    /// Create the object with proper inheritance in python
    pub(crate) fn instantiate_inherited_cortical_data(py: Python, cortical_mapped_data: CorticalMappedXYZPNeuronData) -> PyResult<PyObject> where Self: Sized {
        let child = PyCorticalMappedXYZPNeuronData { inner: cortical_mapped_data };
        let parent = PyFeagiByteStructureCompatible::new();
        let py_obj = Py::new(py, (child, parent))?;
        Ok(py_obj.into())
    }
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronData {
    
    //region Definitions for base class

    #[getter]
    pub fn byte_structure_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::NeuronCategoricalXYZP
    }
    
    #[getter]
    pub fn byte_structure_version(&self) -> u8 { self.inner.get_version() } // This is a overridden placeholder
    
    // overwrite_feagi_byte_structure_slice skipped
    
    #[getter]
    pub fn max_number_bytes_needed(&self) -> usize {
        self.inner.max_number_bytes_needed()
    }

    #[staticmethod]
    pub fn new_from_feagi_byte_structure<'py>(py: Python<'py>, byte_structure: PyFeagiByteStructure) -> PyResult<PyObject> where Self: Sized {
        let result = CorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(&byte_structure.inner);
        match result {
            Ok(inner) => {
                Self::instantiate_inherited_cortical_data(py, inner)
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        let result = self.inner.as_new_feagi_byte_structure();
        match result {
            Ok(result) => Ok(PyFeagiByteStructure { inner: result }),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }
    
    //endregion
    
    
    #[new]
    pub fn new() -> PyCorticalMappedXYZPNeuronData {
        PyCorticalMappedXYZPNeuronData {inner: CorticalMappedXYZPNeuronData::new()}
    }

    //region HashMap like implementation
    
    #[staticmethod]
    pub fn new_with_capacity(py: Python, capacity: usize) -> PyResult<PyObject> {
        Self::instantiate_inherited_cortical_data(py, CorticalMappedXYZPNeuronData::new_with_capacity(capacity))
    }
    
    
    
    pub fn len(&self) -> PyResult<usize> {
        Ok(self.inner.len())
    }
    
    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.inner.is_empty())
    }
    
    pub fn capacity(&self) -> PyResult<usize> {
        Ok(self.inner.capacity())
    }
    
    pub fn reserve(&mut self, additional_capacity: usize) {
        self.inner.reserve(additional_capacity);
    }
    
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }
    
    pub fn get_neurons_of(&self, cortical_id: PyCorticalID) -> PyResult<Option<PyNeuronXYZPArrays>> {
        let result = self.inner.get_neurons_of(&cortical_id.inner);
        Ok(result.map(|arrays| PyNeuronXYZPArrays { inner: arrays.clone() }))
    }
    
    pub fn contains_cortical_id(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains_cortical_id(&cortical_id.inner))
    }
    
    pub fn remove(&mut self, cortical_id: PyCorticalID) -> PyResult<Option<PyNeuronXYZPArrays>> {
        let result = self.inner.remove(cortical_id.inner);
        Ok(result.map(|arrays| PyNeuronXYZPArrays { inner: arrays }))
    }
    
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    //endregion

    pub fn insert(&mut self, cortical_id: PyCorticalID, data: PyNeuronXYZPArrays) -> PyResult<Option<PyNeuronXYZPArrays>> {
        let result = self.inner.insert(cortical_id.inner, data.inner);
        Ok(result.map(|old_data| PyNeuronXYZPArrays { inner: old_data }))
    }

    pub fn contains(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains_cortical_id(&cortical_id.inner))
    }
    
    fn __iter__(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataFullIter> {
        let items: Vec<(PyCorticalID, PyNeuronXYZPArrays)> = self
            .inner
            .mappings
            .iter()
            .map(|(k, v)| (PyCorticalID { inner: k.clone() }, PyNeuronXYZPArrays { inner: v.clone() }))
            .collect();
        Ok(PyCorticalMappedXYZPNeuronDataFullIter { items, index: 0 })
    }

    fn iter_full(&self, py: Python<'_>) -> PyResult<PyCorticalMappedXYZPNeuronDataEasyIter> {
        let mut items: Vec<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))> = Vec::new();
        
        for (k, v) in self.inner.mappings.iter() {
            let cortical_id_str = k.to_string();
            let nd_arrays = v.copy_as_tuple_of_nd_arrays();
            let bound_arrays = tuple_nd_array_to_tuple_np_array(nd_arrays, py)?;
            let np_arrays = (
                bound_arrays.0.unbind(),
                bound_arrays.1.unbind(),
                bound_arrays.2.unbind(),
                bound_arrays.3.unbind(),
            );
            items.push((cortical_id_str, np_arrays));
        }
        
        Ok(PyCorticalMappedXYZPNeuronDataEasyIter { items, index: 0 })
    }
    
    /// Returns an iterator over just the cortical IDs (keys).
    ///
    /// # Examples
    /// ```python
    /// data = CorticalMappedXYZPNeuronData.new()
    /// for cortical_id in data.keys():
    ///     print(f"Found cortical area: {cortical_id}")
    /// ```
    fn keys(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataKeysIter> {
        let items: Vec<PyCorticalID> = self
            .inner
            .mappings
            .keys()
            .map(|k| PyCorticalID { inner: k.clone() })
            .collect();
        Ok(PyCorticalMappedXYZPNeuronDataKeysIter { items, index: 0 })
    }
    
    /// Returns an iterator over just the neuron arrays (values).
    ///
    /// # Examples
    /// ```python
    /// data = CorticalMappedXYZPNeuronData.new()
    /// for neuron_arrays in data.values():
    ///     print(f"Found {len(neuron_arrays)} neurons")
    /// ```
    fn values(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataValuesIter> {
        let items: Vec<PyNeuronXYZPArrays> = self
            .inner
            .mappings
            .values()
            .map(|v| PyNeuronXYZPArrays { inner: v.clone() })
            .collect();
        Ok(PyCorticalMappedXYZPNeuronDataValuesIter { items, index: 0 })
    }
}




impl From<PyCorticalMappedXYZPNeuronData> for CorticalMappedXYZPNeuronData {
    fn from(inner: PyCorticalMappedXYZPNeuronData) -> Self {
        inner.inner
    }
}


#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataFullIter {
    items: Vec<(PyCorticalID, PyNeuronXYZPArrays)>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataFullIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(PyCorticalID, PyNeuronXYZPArrays)> {
        if self.index >= self.items.len() {
            None
        } else {
            let pair = self.items[self.index].clone();
            self.index += 1;
            Some(pair)
        }
    }
}

#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataEasyIter {
    items: Vec<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataEasyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))> {
        if self.index >= self.items.len() {
            None
        } else {
            let pair = self.items.swap_remove(self.index);
            Some(pair)
        }
    }
}

#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataKeysIter {
    items: Vec<PyCorticalID>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataKeysIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyCorticalID> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataValuesIter {
    items: Vec<PyNeuronXYZPArrays>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataValuesIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyNeuronXYZPArrays> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}
    
