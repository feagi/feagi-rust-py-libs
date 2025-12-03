use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::prelude::*;
use numpy::PyArray1;
use feagi_data_structures::neuron_voxels::xyzp::{CorticalMappedXYZPNeuronVoxels};
use feagi_data_serialization::FeagiSerializable;
use crate::feagi_data_serialization::{PyFeagiSerializable, PyFeagiByteStructureType};
use crate::feagi_data_structures::genomic::cortical_area::PyCorticalID;
use super::neuron_voxel_xyzp_arrays::{PyNeuronVoxelXYZPArrays, tuple_nd_array_to_tuple_np_array};

#[pyclass(str, extends=PyFeagiSerializable)]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedXYZPNeuronVoxels")]
pub struct PyCorticalMappedXYZPNeuronVoxels { // HashMap<CorticalID, NeuronYXCPArrays>
    pub inner: CorticalMappedXYZPNeuronVoxels
}

impl PyCorticalMappedXYZPNeuronVoxels {
    /// Create the object with proper inheritance in python
    pub(crate) fn instantiate_inherited_cortical_data(py: Python, cortical_mapped_data: CorticalMappedXYZPNeuronVoxels) -> PyResult<PyObject> where Self: Sized {
        let child = PyCorticalMappedXYZPNeuronVoxels { inner: cortical_mapped_data };
        let parent = PyFeagiSerializable::new();
        let py_obj = Py::new(py, (child, parent))?;
        Ok(py_obj.into())
    }
}

// TODO split this up as per implementation

#[pymethods]
impl PyCorticalMappedXYZPNeuronVoxels {

    //region Definitions for base class

    #[getter]
    pub fn byte_structure_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::NeuronCategoricalXYZP()
    }

    #[getter]
    pub fn byte_structure_version(&self) -> u8 { self.inner.get_version() } // This is a overridden placeholder

    // overwrite_feagi_byte_structure_slice skipped

    #[getter]
    pub fn max_number_bytes_needed(&self) -> usize {
        self.inner.get_number_of_bytes_needed()
    }

    //endregion


    #[new]
    pub fn new() -> (PyCorticalMappedXYZPNeuronVoxels, PyFeagiSerializable) {
        (
            PyCorticalMappedXYZPNeuronVoxels {
                inner: CorticalMappedXYZPNeuronVoxels::new()
            },
            PyFeagiSerializable::new()
        )
    }

    //region HashMap like implementation

    #[staticmethod]
    pub fn new_with_capacity(py: Python, capacity: usize) -> PyResult<PyObject> {
        Self::instantiate_inherited_cortical_data(py, CorticalMappedXYZPNeuronVoxels::new_with_capacity(capacity))
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

    pub fn get_neurons_of(&self, cortical_id: PyCorticalID) -> PyResult<Option<PyNeuronVoxelXYZPArrays>> {
        let result = self.inner.get_neurons_of(&cortical_id.inner);
        Ok(result.map(|arrays| PyNeuronVoxelXYZPArrays { inner: arrays.clone() }))
    }

    pub fn contains_cortical_id(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains_cortical_id(&cortical_id.inner))
    }

    pub fn remove(&mut self, cortical_id: PyCorticalID) -> PyResult<Option<PyNeuronVoxelXYZPArrays>> {
        let result = self.inner.remove(cortical_id.inner);
        Ok(result.map(|arrays| PyNeuronVoxelXYZPArrays { inner: arrays }))
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    //endregion

    pub fn insert(&mut self, cortical_id: PyCorticalID, data: PyNeuronVoxelXYZPArrays) -> PyResult<Option<PyNeuronVoxelXYZPArrays>> {
        let result = self.inner.insert(cortical_id.inner, data.inner);
        Ok(result.map(|old_data| PyNeuronVoxelXYZPArrays { inner: old_data }))
    }

    pub fn contains(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains_cortical_id(&cortical_id.inner))
    }

    fn __iter__(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataFullIter> {
        let items: Vec<(PyCorticalID, PyNeuronVoxelXYZPArrays)> = self
            .inner
            .mappings
            .iter()
            .map(|(k, v)| (PyCorticalID { inner: k.clone() }, PyNeuronVoxelXYZPArrays { inner: v.clone() }))
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
    ///     print(f"Found {len(neuron_arrays)} neurons_voxels")
    /// ```
    fn values(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataValuesIter> {
        let items: Vec<PyNeuronVoxelXYZPArrays> = self
            .inner
            .mappings
            .values()
            .map(|v| PyNeuronVoxelXYZPArrays { inner: v.clone() })
            .collect();
        Ok(PyCorticalMappedXYZPNeuronDataValuesIter { items, index: 0 })
    }
}

impl PyCorticalMappedXYZPNeuronVoxels {
    pub(crate) fn get_mut(&mut self) -> &mut CorticalMappedXYZPNeuronVoxels {
        &mut self.inner
    }
}

impl From<CorticalMappedXYZPNeuronVoxels> for PyCorticalMappedXYZPNeuronVoxels {
    fn from(inner: CorticalMappedXYZPNeuronVoxels) -> Self {
        PyCorticalMappedXYZPNeuronVoxels { inner }
    }
}

impl From<PyCorticalMappedXYZPNeuronVoxels> for CorticalMappedXYZPNeuronVoxels {
    fn from(inner: PyCorticalMappedXYZPNeuronVoxels) -> Self {
        inner.inner
    }
}

impl std::fmt::Display for PyCorticalMappedXYZPNeuronVoxels {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}


//region Iterators
#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataFullIter {
    items: Vec<(PyCorticalID, PyNeuronVoxelXYZPArrays)>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataFullIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(PyCorticalID, PyNeuronVoxelXYZPArrays)> {
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
    items: Vec<PyNeuronVoxelXYZPArrays>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataValuesIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyNeuronVoxelXYZPArrays> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

//endregion