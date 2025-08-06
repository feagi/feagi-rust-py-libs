use std::fmt::{Display, Formatter};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyList};
use numpy::{PyArray1, PyReadonlyArray1};
use ndarray::Array1;
use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
use super::neuron_xyzp::PyNeuronXYZP;


#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "NeuronXYZPArrays")]
pub struct PyNeuronXYZPArrays {
    pub(crate) inner: NeuronXYZPArrays,
}

#[pymethods]
impl PyNeuronXYZPArrays {
    #[new]
    pub fn new() -> PyResult<Self> {
        let inner = NeuronXYZPArrays::new();
        Ok(PyNeuronXYZPArrays {inner})
    }
    
    #[staticmethod]
    pub fn new_from_resolution(resolution: (usize, usize, usize))  -> PyResult<Self> {
        let inner = NeuronXYZPArrays::new_from_resolution(resolution);
        Ok(PyNeuronXYZPArrays {inner})
    }

    #[staticmethod]
    pub fn new_from_numpy(x: PyReadonlyArray1<u32>, y: PyReadonlyArray1<u32>, z: PyReadonlyArray1<u32>, p: PyReadonlyArray1<f32>) -> PyResult<Self> {
        let x_nd = x.as_array().to_owned();
        let y_nd = y.as_array().to_owned();
        let z_nd = z.as_array().to_owned();
        let p_nd = p.as_array().to_owned();
        match NeuronXYZPArrays::new_from_ndarrays(x_nd, y_nd, z_nd, p_nd) {
            Ok(inner) => Ok(PyNeuronXYZPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    //region Array-Like Implementations

    fn __iter__(slf: PyRef<'_, Self>) -> PyNeuronXYZPArraysIterator {
        PyNeuronXYZPArraysIterator {
            inner: slf.inner.copy_as_neuron_xyzp_vec().into_iter(),
        }
    }
    
    #[staticmethod]
    pub fn with_capacity(number_of_neurons_initial: usize) -> PyResult<Self> {
        let inner = NeuronXYZPArrays::with_capacity(number_of_neurons_initial);
        Ok(PyNeuronXYZPArrays{inner})
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

    pub fn push(&mut self, new_neuron: PyNeuronXYZP) {
        self.inner.push(&new_neuron.into())
    }
    
    pub fn get(&mut self, index: usize) -> PyResult<PyNeuronXYZP> { // TODO fix mut
        let result = &self.inner.get(index);
        match result {
            Ok(neuron) => Ok(PyNeuronXYZP{inner: neuron.clone()}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    pub fn pop(&mut self) -> PyResult<PyNeuronXYZP> {
        let option = self.inner.pop();
        match option {
            Some(neuron) => Ok(PyNeuronXYZP{inner: neuron}),
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
            .map(|item| Py::new(py, PyNeuronXYZP{inner: item}).map(|obj| obj.into()))
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

impl From<NeuronXYZPArrays> for PyNeuronXYZPArrays {
    fn from(neurons: NeuronXYZPArrays) -> Self {
        PyNeuronXYZPArrays{inner: neurons}
    }
}

impl From<PyNeuronXYZPArrays> for NeuronXYZPArrays {
    fn from(neurons: PyNeuronXYZPArrays) -> Self {
        neurons.inner
    }
}

impl Display for PyNeuronXYZPArrays {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.inner.to_string();
        write!(f, "{}", s)
    }
}

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
    inner: std::vec::IntoIter<NeuronXYZP>,
}

#[pymethods]
impl PyNeuronXYZPArraysIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyNeuronXYZP> {
        slf.inner.next().map(|neuron| PyNeuronXYZP { inner: neuron })
    }
}