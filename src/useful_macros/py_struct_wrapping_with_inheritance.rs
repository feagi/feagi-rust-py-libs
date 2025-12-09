use feagi_data_structures::FeagiDataError;
use pyo3::pymethods;

/// These implementations do not need the parent to store a shared state in a box (child st
//region No Box (Child stores data)
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
    ($parent_pyclass_in_rust:ident, $py_class_name_in_rust:ident, $class_name_in_python_str:expr, $representing_rust_struct:ty) => {

        #[pyo3::pyclass(str, extends=$parent_pyclass_in_rust)]
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
            fn python_new_child_constructor(child_rust_struct: $representing_rust_struct) -> (Self, $parent_pyclass_in_rust) {
                ($py_class_name_in_rust {inner: child_rust_struct}, $parent_pyclass_in_rust::new_blank_parent())
            }

            /// You MUST use for all other constructors except for "new"
            pub fn python_etc_child_constructor<'py>(py: Python<'py>, child_rust_struct: $representing_rust_struct) -> PyResult<Py<Self>> {
                Python::with_gil(|py| {
                    Py::new(py, ($py_class_name_in_rust { inner: child_rust_struct}, $parent_pyclass_in_rust::new_blank_parent()) ) // TODO this is outdated
                })
            }

            /// Use this if you want to export this py-wrapped struct to python with proper inheritance
            fn export_as_python_child<'py>(self, py: Python<'py>) -> PyResult<Py<Self>> {
                Python::with_gil(|py| {
                    Py::new(py, (self, $parent_pyclass_in_rust::new_blank_parent()) ) // TODO this is outdated
                })
            }
        }

    };
}
//endregion

///This implementation assumes the parent is a rust trait, implemented in python as a Box dyn. "clone_box" must be defined!
//region Shared Parent Data
#[macro_export]
macro_rules! create_trait_parent_with_box_pyclass {
    ($parent_class_name_in_python_str:expr, $py_class_parent_name_in_rust:ident, $boxed_rust_type:ident) => {

        #[pyo3::pyclass(str, subclass)]
        #[pyo3(name = $parent_class_name_in_python_str)]
        #[derive(Debug)]
        pub struct $py_class_parent_name_in_rust {
            pub inner: Box<dyn $boxed_rust_type + Send + Sync>,
        }

        // Manual implementation
        impl Clone for $py_class_parent_name_in_rust {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone_box(),
                }
            }
        }

        // Require print support
        impl std::fmt::Display for $py_class_parent_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", $parent_class_name_in_python_str)
            }
        }

        impl $py_class_parent_name_in_rust {
            // Do not allow direct instantiation from Python
            pub(crate) fn new_parent(boxed: Box<dyn $boxed_rust_type + Send + Sync>) -> Self {
                $py_class_parent_name_in_rust {
                    inner: boxed,
                }
            }

            pub(crate) fn py_any_to_box<'py>(_py: Python<'_>, py_any: &pyo3::Bound<'py, pyo3::PyAny>) -> Result<Box<dyn $boxed_rust_type + Send + Sync>, feagi_data_structures::FeagiDataError> {
                if let Ok(reference) = py_any.cast::<( $py_class_parent_name_in_rust )>() {
                    return Ok(reference.borrow().inner.clone_box());
                }
                Err(FeagiDataError::BadParameters(format!("Unable to parse object as any child of {}!", $parent_class_name_in_python_str)))
            }
        }

    };
}

#[macro_export]
macro_rules! create_trait_child_with_box_pyclass {
    ($parent_pyclass_in_rust:ident, $py_class_name_in_rust:ident, $class_name_in_python_str:expr, $boxed_rust_type:ident, $rust_child_concrete_type:ident) => {

        #[pyo3::pyclass(str, extends=$parent_pyclass_in_rust)]
        #[derive(Debug, Clone)]
        #[pyo3(name = $class_name_in_python_str)]
        pub struct $py_class_name_in_rust {}

        // Require print support // TODO improve
        impl std::fmt::Display for $py_class_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $class_name_in_python_str)
            }
        }

        // Into gets weird due to parent owning data. skipping for now

        impl $py_class_name_in_rust {
            /// You MUST use this for the new constructor, and ONLY for that usecase!. Your "new" must use this and return (PyChild, PyParent)
            fn python_new_child_constructor(boxed_data: Box<dyn $boxed_rust_type + Send + Sync>) -> (Self, $parent_pyclass_in_rust) {
                ($py_class_name_in_rust {}, $parent_pyclass_in_rust::new_parent(boxed_data))
            }
        }


        impl $py_class_name_in_rust {

            // To use the following functions, call them from a a pymethods block with input "slf: PyRef<Self>" and in the format of "Self::get_parent_box(&slf)"

            fn get_parent_box<'a>(slf: &'a PyRef<'_, Self>) -> &'a Box<dyn $boxed_rust_type + Send + Sync> {
                let parent: &$parent_pyclass_in_rust = slf.as_ref();
                &parent.inner
            }

            fn get_parent_box_mut<'a>(slf: &'a mut PyRefMut<'_, Self>) -> &'a mut Box<dyn $boxed_rust_type + Send + Sync> {
                let parent: &mut $parent_pyclass_in_rust = slf.as_mut();
                &mut parent.inner
            }

            fn get_ref<'a>(slf: &'a PyRef<'_, Self>) -> Result<&'a $rust_child_concrete_type, feagi_data_structures::FeagiDataError> {
                let parent_box = Self::get_parent_box(slf);
                parent_box.as_any().downcast_ref::<$rust_child_concrete_type>()
                    .ok_or_else(|| feagi_data_structures::FeagiDataError::InternalError("Type mismatch: expected unwrapped $py_class_name_in_rust".into()))
            }

            fn get_ref_mut<'a>(slf: &'a mut PyRefMut<'_, Self>) -> Result<&'a mut $rust_child_concrete_type, feagi_data_structures::FeagiDataError> {
                let parent_box = Self::get_parent_box_mut(slf);
                parent_box.as_any_mut().downcast_mut::<$rust_child_concrete_type>()
                .ok_or_else(|| feagi_data_structures::FeagiDataError::InternalError("Type mismatch: expected unwrapped $py_class_name_in_rust".into()))
            }
        }


    };
}


//endregion
