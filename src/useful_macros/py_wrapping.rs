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
        #[derive(Debug)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

        __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. allows comparison if equal
#[macro_export]
macro_rules! create_pyclass_with_equal {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {
        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone, PartialEq)]
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
        #[derive(Debug, Clone)]
        pub struct $py_wrapped_name {
            pub inner: $rust,
        }

         __base_py_class_shared!($py_wrapped_name, $rust, $py_name);
    };
}

//endregion

//region Internal

// NOTE: technically #[macro_export] is required for visibility
/// Shared implementation of base py classes
#[macro_export]
macro_rules! __base_py_class_shared {
    ($py_wrapped_name:ident, $rust:ty, $py_name:expr) => {

        // Require print support
        impl std::fmt::Display for $py_wrapped_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
            }
        }


        impl $py_wrapped_name {
            /// Create Python wrapped instance of the given Rust structure
            pub(crate) fn new_from_rust(rust_struct: $rust) -> Self {
                $py_wrapped_name {inner: rust_struct}
            }

            /// Static wrapping an existing rust non-py wrapped instance directly to PyAny
            pub fn wrap_to_bound_any(py: Python<'_>, rust_struct: $rust) -> PyResult<Bound<'_, PyAny>> {
                Bound::new(py, Self {inner: rust_struct} ).map(|b| b.into_any())
            }

            /// Wraps self into into a Bound<PyAny>
            pub fn wrap_self_into_bound_any(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
                Bound::new(py, self).map(|b| b.into_any())
            }

            /// Attempt to downcast as a reference to the python wrapper.
            /// Use &ref.inner to borrow the rust data.
            /// Note that doing if-elif-else chains will have to be the main way to get out a type from multiple.
            pub fn try_extract_from_bound_any<'py>(obj: pyo3::Bound<'py, pyo3::PyAny>) -> Result<pyo3::PyRef<'py, Self>, feagi_data_structures::FeagiDataError> {
                use pyo3::prelude::PyAnyMethods;
                use feagi_data_structures::FeagiDataError;
                obj.cast::<Self>()
                    .map_err(|_| FeagiDataError::BadParameters(format!("Expected {} but got {:?}", $py_name, obj.get_type())))?
                    .try_borrow()
                    .map_err(|e| FeagiDataError::BadParameters(format!("Failed to borrow: {}", e)))
            }

        }
    };
}

//endregion