
/// Creates a parent class for python, when exposing rust structs that share a trait.
/// Doesn't do much on its own and not intended to be instantiated directly, either in rust or python.
/// Parameter 1: str parent class name as visible in python (ParentClass)
/// Parameter 2: identity name of this struct as visible in rust (PyParentClass)
#[macro_export]
macro_rules! create_trait_parent_pyclass {
    ($parent_class_name_in_python:expr, $py_class_parent_name_in_rust:ident) => {

        #[pyo3::pyclass(str, subclass)]
        #[pyo3(name = $parent_class_name_in_python)]
        #[derive(Debug, Clone)]
        pub struct $py_class_parent_name_in_rust {}

        impl $py_class_parent_name_in_rust {
            pub(crate) fn new_blank_parent() -> Self {
                $py_class_parent_name_in_rust {}
            }
        }

        impl std::fmt::Display for $py_class_parent_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", $parent_class_name_in_python)
            }
        }
    };
}


/// Creates a child class for python, with a parent defined by create_trait_parent_pyclass.
/// Has private method python_new_child_constructor that creates a tuple of (child_py_struct, parent_py_struct)
/// which you must use for your new function. All other constructors are to use python_etc_child_constructor and return
/// PyResult<Py<Self>>
/// Parameter 1: Identity Parent struct name as defined in rust (PyParentClass)
/// Parameter 2: Identity Child struct name (PyChildClass)
/// Parameter 3: str Child struct name (ChildClass)
/// Parameter 4: type representing rust type in inner for this py wrapper (RustStruct)
#[macro_export]
macro_rules! create_trait_child_pyclass {
    ($parent_struct_in_rust:ident, $py_class_name_in_rust:ident, $class_name_in_python_str:expr, $representing_rust_struct:ty) => {

        #[pyo3::pyclass(str, extends=$parent_struct_in_rust)]
        #[derive(Debug, Clone)]
        #[pyo3(name = $class_name_in_python_str)]
        pub struct $py_class_name_in_rust {
            pub inner: $representing_rust_struct
        }

        // Require print support
        impl std::fmt::Display for $py_class_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
            }
        }

        impl From<$representing_rust_struct> for $py_class_name_in_rust {
            fn from(rust_struct: $representing_rust_struct) -> Self {
                $py_class_name_in_rust { inner: rust_struct }
            }
        }

        impl From<$py_class_name_in_rust> for $representing_rust_struct {
            fn from(py: $py_class_name_in_rust) -> Self {
                py.inner
            }
        }

        impl $py_class_name_in_rust {
            /// You MUST use this for the new constructor, and ONLY for that usecase!. Your "new" must use this and return (PyChild, PyParent)
            fn python_new_child_constructor(child_rust_struct: $representing_rust_struct) -> (Self, $parent_struct_in_rust) {
                ($py_class_name_in_rust {inner: child_rust_struct}, $parent_struct_in_rust::new_blank_parent())
            }

            /// You MUST use for all other constructors except for "new"
            pub fn python_etc_child_constructor<'py>(py: Python<'py>, child_rust_struct: $representing_rust_struct) -> PyResult<Py<Self>> {
                Python::with_gil(|py| {
                    Py::new(py, ($py_class_name_in_rust { inner: child_rust_struct}, $parent_struct_in_rust::new_blank_parent()) ) // TODO this is outdated
                })
            }

            /// Use this if you want to export this py-wrapped struct to python with proper inheritance
            fn export_as_python_child<'py>(self, py: Python<'py>) -> PyResult<Py<Self>> {
                Python::with_gil(|py| {
                    Py::new(py, (self, $parent_struct_in_rust::new_blank_parent()) ) // TODO this is outdated
                })
            }
        }

    };
}
