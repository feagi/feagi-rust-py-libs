use std::fmt::Debug;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::neuron_voxels::xyzp::NeuronVoxelXYZP;

use crate::{create_pyclass_with_clone_equal, __base_py_class_shared, __pyclass_from_py_object};
use crate::{create_pyclass, create_pyclass_with_equal};


/*
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

 */

#[pyclass(str)]
#[pyo3(name = "PyNeuronVoxelXYZP")]
pub struct PyNeuronVoxelXYZP {
    pub inner: NeuronVoxelXYZP,
}

impl std::fmt::Display for PyNeuronVoxelXYZP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}

/*
// Rust -> Python Clone
impl IntoPy<pyo3::PyAny> for PyNeuronVoxelXYZP {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyAny {
        pyo3::Py::new(py, self).unwrap().into_py(py)
    }
}

 */

impl PyNeuronVoxelXYZP {
    /// Create Python wrapped instance of the given Rust structure
    pub(crate) fn from_rust(rust_struct: NeuronVoxelXYZP) -> Self {
        PyNeuronVoxelXYZP {inner: rust_struct}
    }

    /// Try to downcast a &PyAny into PyRef<Self>
    pub fn try_as_py_ref<'py>(obj: pyo3::Bound<'py, PyAny>) -> Result<PyRef<'py, NeuronVoxelXYZP>, FeagiDataError> {
        //let a =   obj.extract::<PyRef<'py, Self>>();
        let result = obj.extract::<PyRef<'py, Self>>();
        match result {
            Ok(py_ref) => Ok(py_ref.inner),
            Err(_) => Err(FeagiDataError::BadParameters(format!("Unable to extract {} from given python variable!", "NeuronVoxelXYZP")))
        }
    }
}




//create_pyclass!(PyNeuronVoxelXYZP, NeuronVoxelXYZP, "PyNeuronVoxelXYZP");