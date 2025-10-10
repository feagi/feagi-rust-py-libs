use std::fmt::Debug;
use pyo3::{pyclass, pymethods};
use feagi_data_structures::neuron_voxels::xyzp::NeuronVoxelXYZP;
use crate::{project_display, py_type_casts};

//region NeuronVoxelXYZP Implementation
#[pyclass(str)]
#[derive(Clone)]
#[pyo3(name = "NeuronVoxelXYZP")]
pub struct PyNeuronVoxelXYZP {
    pub inner: NeuronVoxelXYZP,
}

#[pymethods]
impl PyNeuronVoxelXYZP {
    #[new]
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        PyNeuronVoxelXYZP {inner: NeuronVoxelXYZP::new(x, y, z, p)}
    }

    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        self.inner.as_tuple()
    }
}
//endregion

py_type_casts!(PyNeuronVoxelXYZP, NeuronVoxelXYZP);
project_display!(PyNeuronVoxelXYZP);
