use std::fmt::{Display, Formatter};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyList};
use numpy::{PyArray1, PyReadonlyArray1};
use ndarray::Array1;
use feagi_data_structures::neuron_voxels::xyzp::{NeuronVoxelXYZPArrays, NeuronVoxelXYZP};
use crate::{project_display, py_type_casts};
use super::neuron_voxel_xyzp::PyNeuronVoxelXYZP;


#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "NeuronVoxelXYZPArrays")]
pub struct PyNeuronVoxelXYZPArrays {
    pub(crate) inner: NeuronVoxelXYZPArrays,
}

#[pymethods]
impl PyNeuronVoxelXYZPArrays {
    
    //region Unique Constructors
    #[new]
    pub fn new() -> PyResult<Self> {
        let inner = NeuronVoxelXYZPArrays::new();
        Ok(PyNeuronVoxelXYZPArrays {inner})
    }

    #[staticmethod]
    pub fn new_from_numpy(x: PyReadonlyArray1<u32>, y: PyReadonlyArray1<u32>, z: PyReadonlyArray1<u32>, p: PyReadonlyArray1<f32>) -> PyResult<Self> {
        let x_nd = x.as_array().to_owned();
        let y_nd = y.as_array().to_owned();
        let z_nd = z.as_array().to_owned();
        let p_nd = p.as_array().to_owned();
        match NeuronVoxelXYZPArrays::new_from_ndarrays(x_nd, y_nd, z_nd, p_nd) {
            Ok(inner) => Ok(PyNeuronVoxelXYZPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    //endregion

    //region Array-Like Implementations

    fn __iter__(slf: PyRef<'_, Self>) -> PyNeuronXYZPArraysIterator {
        PyNeuronXYZPArraysIterator {
            inner: slf.inner.copy_as_neuron_xyzp_vec().into_iter(),
        }
    }

    #[staticmethod]
    pub fn with_capacity(number_of_neurons_initial: usize) -> PyResult<Self> {
        let inner = NeuronVoxelXYZPArrays::with_capacity(number_of_neurons_initial);
        Ok(PyNeuronVoxelXYZPArrays{inner})
    }

    pub fn capacity(&self) -> PyResult<usize> {
        Ok(self.inner.capacity())
    }

    pub fn spare_capacity(&self) -> PyResult<usize> {
        Ok(self.inner.spare_capacity())
    }

    pub fn len(&self) -> PyResult<usize> {
        Ok(self.inner.len())
    }

    pub fn shrink_to_fit(&mut self)  {
        self.inner.shrink_to_fit()
    }

    pub fn ensure_capacity(&mut self, number_of_neurons_total: usize) {
        self.inner.ensure_capacity(number_of_neurons_total)
    }

    pub fn reserve(&mut self, additional_neuron_count: usize) {
        self.inner.reserve(additional_neuron_count)
    }

    pub fn push(&mut self, new_neuron: PyNeuronVoxelXYZP) {
        self.inner.push(&new_neuron.into())
    }

    pub fn get(&mut self, index: usize) -> PyResult<PyNeuronVoxelXYZP> { // TODO fix mut
        let result = &self.inner.get(index);
        match result {
            Ok(neuron) => Ok(PyNeuronVoxelXYZP{inner: neuron.clone()}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn pop(&mut self) -> PyResult<PyNeuronVoxelXYZP> {
        let option = self.inner.pop();
        match option {
            Some(neuron) => Ok(PyNeuronVoxelXYZP{inner: neuron}),
            None => Err(PyValueError::new_err("Array is Empty!"))
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.inner.is_empty())
    }

    //endregion


    pub fn copy_as_neuron_xyzp_vec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let items = self.inner.copy_as_neuron_xyzp_vec();

        let py_objects: Vec<PyObject> = items
            .into_iter()
            .map(|item| Py::new(py, PyNeuronVoxelXYZP{inner: item}).map(|obj| obj.into()))
            .collect::<PyResult<_>>()?;

        PyList::new(py, py_objects)
    }

    pub fn copy_as_tuple_of_numpy_arrays<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<f32>>)> {
        let nd_arrays_tuple = self.inner.copy_as_tuple_of_nd_arrays();
        Ok((
            PyArray1::from_array(py, &nd_arrays_tuple.0),
            PyArray1::from_array(py, &nd_arrays_tuple.1),
            PyArray1::from_array(py, &nd_arrays_tuple.2),
            PyArray1::from_array(py, &nd_arrays_tuple.3)
        ))
    }

    pub fn get_size_in_number_of_bytes(&self) -> PyResult<usize> {
        Ok(self.inner.get_size_in_number_of_bytes())
    }

    pub fn copy_as_tuple_of_numpy<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<f32>>)> {
        let nd_arrays_tuple = self.inner.copy_as_tuple_of_nd_arrays();
        Ok((
            PyArray1::from_array(py, &nd_arrays_tuple.0),
            PyArray1::from_array(py, &nd_arrays_tuple.1),
            PyArray1::from_array(py, &nd_arrays_tuple.2),
            PyArray1::from_array(py, &nd_arrays_tuple.3)
        ))
    }


}

py_type_casts!(PyNeuronVoxelXYZPArrays, NeuronVoxelXYZPArrays);
project_display!(PyNeuronVoxelXYZPArrays);

pub fn tuple_nd_array_to_tuple_np_array<'py>(input: (Array1<u32>, Array1<u32>, Array1<u32>, Array1<f32>), py: Python<'py>)
                                             -> PyResult<(Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<f32>>)> {

    Ok((
        PyArray1::from_array(py, &input.0),
        PyArray1::from_array(py, &input.1),
        PyArray1::from_array(py, &input.2),
        PyArray1::from_array(py, &input.3)
    ))
}

/// Python iterator for NeuronXYZPArrays.
///
/// This iterator struct enables Pythonic iteration over neuron arrays by implementing
/// the Python iterator protocol through the `__iter__` and `__next__` magic methods.
#[pyclass]
pub struct PyNeuronXYZPArraysIterator {
    inner: std::vec::IntoIter<NeuronVoxelXYZP>,
}

#[pymethods]
impl PyNeuronXYZPArraysIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyNeuronVoxelXYZP> {
        slf.inner.next().map(|neuron| PyNeuronVoxelXYZP { inner: neuron })
    }
}