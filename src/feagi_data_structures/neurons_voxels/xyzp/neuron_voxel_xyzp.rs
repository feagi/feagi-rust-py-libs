use std::fmt::Debug;

use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::neuron_voxels::xyzp::NeuronVoxelXYZP;
use crate::{create_pyclass, __base_py_class_shared};

create_pyclass!(PyNeuronVoxelXYZP, NeuronVoxelXYZP, "PyNeuronVoxelXYZP");

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

/*
impl<'py> IntoPyObject<'py> for PyNeuronVoxelXYZP {
    type Target = PyAny;
    type Output = Bound<'py, PyNeuronVoxelXYZP>;
    type Error = ();

    // almost always PyAny
    fn into_pyobject(self, py: Python<'_>) -> PyResult<&Self::Target> {
        Py::new(py, self)?.into_ref(py).map(|r| r as &PyAny)
    }
}

 */