use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;


//region Simple PyStruct Classes (No Inheritance)
/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner
macro_rules! create_base_pyclass {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr, $params:ident) => {

        __create_base_class_shared!($py_wrapped_name, $rust, $py_name, $params);

        // NOTE: Requires Clone on the base struct
        // Python -> Rust Clone
        impl<'py> pyo3::FromPyObject<'py> for $py_wrapped_name {
            fn extract(obj: &'py pyo3::PyAny) -> pyo3::PyResult<Self> {
                let py_ref: pyo3::PyRef<$py_wrapped_name> = obj.extract()?;
                Ok(Self {
                    inner: py_ref.inner.clone(),
                })
            }
        }

    };
}


/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner
macro_rules! __create_base_class_shared {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr, $params:ident) => {

        __define_base_pyclass_header!($py_name, $params)
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

        // Rust -> Python Clone
        impl IntoPy<pyo3::PyObject> for $py_wrapped_name {
            fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
                pyo3::Py::new(py, self).unwrap().into_py(py)
            }
        }

        impl $py_wrapped_name {

            /// Create Python wrapped instance of the given Rust structure
            pub(crate) fn from_rust(rust_struct: $rust:ty) -> Self {
                $py_wrapped_name {inner: rust_struct}
            }

            /// Try to downcast a &PyAny into PyRef<Self>
            pub fn try_as_pyref<'py>(obj: &'py PyAny) -> PyResult<PyRef<'py, Self>> {
                obj.extract::<PyRef<'py, Self>>()
            }
        }
    };
}

macro_rules! __define_base_pyclass_header {
    ($py_name:expr, ()) => {
        #[pyclass(str)]
        #[pyo3(name = $py_name)]
    };


    ($py_name:expr, (equal)) => {
        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(PartialEq)]
    };
    ($py_name:expr, (hash)) => {
        #[pyclass(str, hash)]
        #[pyo3(name = $py_name)]
    };
    ($py_name:expr, (clone)) => {
        #[pyclass(str, hash)]
        #[pyo3(name = $py_name)]
        #[derive(Clone)]
    };


    ($py_name:expr, (equal, hash)) => {
        #[pyclass(str, eq, hash)]
        #[pyo3(name = $py_name)]
        #[derive(PartialEq)]
    };
    ($py_name:expr, (equal, clone)) => {
        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(PartialEq, Clone)]
    };
    ($py_name:expr, (hash, clone)) => {
        #[pyclass(str, hash)]
        #[pyo3(name = $py_name)]
        #[derive(Clone)]
    };


    ($py_name:expr, (equal, hash, clone)) => {
        #[pyclass(str, eq, hash)]
        #[pyo3(name = $py_name)]
        #[derive(PartialEq, Clone)]
    };

}

//endregion

