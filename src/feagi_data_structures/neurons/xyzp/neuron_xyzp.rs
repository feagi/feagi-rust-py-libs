use std::fmt::Debug;
use pyo3::{pyclass, pymethods};
use feagi_data_structures::neurons::xyzp::NeuronXYZP;
use crate::{project_display, py_type_casts};

//region NeuronXYZP Implementation
#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "NeuronXYZP")]
pub struct PyNeuronXYZP {
    pub inner: NeuronXYZP,
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

fn test(a: PyNeuronXYZP) {
    let a = a.inner;
}

py_type_casts!(PyNeuronXYZP, NeuronXYZP);
project_display!(PyNeuronXYZP);
