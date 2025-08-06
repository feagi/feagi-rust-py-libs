use std::fmt::Debug;
use pyo3::{pyclass, pymethods};
use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;

//region NeuronXYZP Implementation
#[pyclass]
#[derive(Clone)]
#[pyo3(name = "NeuronXYZP")]
pub struct PyNeuronXYZP {
    pub inner: NeuronXYZP,
}

impl std::fmt::Display for PyNeuronXYZP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

#[pymethods]
impl PyNeuronXYZP {
    #[new]
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        PyNeuronXYZP {inner: NeuronXYZP::new(x, y, z, p)}
    }

    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        self.inner.as_tuple()
    }
}
//endregion

//region NeuronXYZP Conversions
impl From<NeuronXYZP> for PyNeuronXYZP {
    fn from(neuron: NeuronXYZP) -> Self {
        PyNeuronXYZP{inner: neuron}
    }
}

impl From<PyNeuronXYZP> for NeuronXYZP {
    fn from(neuron: PyNeuronXYZP) -> Self {
        neuron.inner
    }
}

impl From<&NeuronXYZP> for PyNeuronXYZP {
    fn from(neuron: &NeuronXYZP) -> Self {
        PyNeuronXYZP{inner: neuron.clone()}
    }
}

impl From<&PyNeuronXYZP> for NeuronXYZP {
    fn from(neuron: &PyNeuronXYZP) -> Self {
        neuron.clone().inner
    }
}
//endregion