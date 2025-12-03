use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;


//region Base Classes (No Inheritance)


/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner
#[macro_export]
macro_rules! create_pyclass {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {

        #[pyclass(str)]
        #[pyo3(name = $py_name)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

        __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. Includes try_into from PyAny if the source has Clone
#[macro_export]
macro_rules! create_pyclass_with_clone {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {

        #[pyclass(str)]
        #[pyo3(name = $py_name)]
        #[derive(Clone)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

        __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
        __pyclass_from_py_object!($py_wrapped_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. allows comparison if equal
#[macro_export]
macro_rules! create_pyclass_with_equal {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {
        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(PartialEq)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

         __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. Allows Hashing
#[macro_export]
macro_rules! create_pyclass_with_hash {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {
        #[pyclass(str, hash)]
        #[pyo3(name = $py_name)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

         __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. Includes try_into from PyAny if the source has Clone.
/// Allows comparison if equal
#[macro_export]
macro_rules! create_pyclass_with_clone_equal {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {

        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(Clone, PartialEq)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

        __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
        __pyclass_from_py_object!($py_wrapped_name);
    };
}

//endregion

//region Internal

// NOTE: technically #[macro_export] is required for visibility
/// Shared implementation of base py classes
#[macro_export]
macro_rules! __base_py_class_shared {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {

        // Rust -> Python Clone
        impl IntoPy<pyo3::PyObject> for $py_wrapped_name {
            fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
                pyo3::Py::new(py, self).unwrap().into_py(py)
            }
        }

        impl std::fmt::Display for $py_wrapped_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
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

/// Requires Clone, allows try_into from a PyAny
#[macro_export]
macro_rules! __pyclass_from_py_object {
    ($py_wrapped_name:ident) => {
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
//endregion