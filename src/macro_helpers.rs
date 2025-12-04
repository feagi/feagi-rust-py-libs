
//region Indexing

#[macro_export]
macro_rules! typed_number {
    ($py_struct:ident, $feagi_struct:ident, $number_type:ty, $class_name:expr, $error_msg:expr) => {


        #[pyclass(str)]
        #[pyo3(name = $class_name)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $py_struct {
            inner: $feagi_struct
        }

        #[pymethods]
        impl $py_struct{
            #[new]
            pub fn new(index: $number_type) -> PyResult<Self> {
                Ok(
                    $py_struct {
                        inner: $feagi_struct::from(index)
                    }
                )
            }
        }

        py_type_casts!($py_struct, $feagi_struct);
        py_object_cast_int!($py_struct, $feagi_struct, $number_type, $error_msg);
        project_display!($py_struct);

    };
}

#[macro_export]
macro_rules! typed_non_zero_number {
    ($py_struct:ident, $feagi_struct:ident, $number_type:ty, $class_name:expr, $error_msg:expr) => {


        #[pyclass(str)]
        #[pyo3(name = $class_name)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $py_struct {
            inner: $feagi_struct
        }

        #[pymethods]
        impl $py_struct{
            #[new]
            pub fn new(index: $number_type) -> PyResult<Self> {
                Ok(
                    $py_struct {
                        inner: $feagi_struct::new(index).map_err(PyFeagiError::from)?
                    }
                )
            }
        }

        py_type_casts!($py_struct, $feagi_struct);
        //py_object_cast_int!($py_struct, $feagi_struct, $number_type, $error_msg);
        py_object_try_cast_int!($py_struct, $feagi_struct, $number_type, $error_msg);
        project_display!($py_struct);

    };
}

//endregion


//region PyClass Helpers

#[macro_export]
/// adds into()
macro_rules! py_type_casts {
    (
        $py_type:ty,
        $feagi_type:ty
    ) => {
        
        impl From<$feagi_type> for $py_type {
            fn from(inner: $feagi_type) -> Self {
                Self { inner }
            }
        }

        impl From<$py_type> for $feagi_type {
            fn from(inner: $py_type) -> Self {
                inner.inner
            }
        }
        
    };
}

#[macro_export]
/// automatically implements display from inner member
macro_rules! project_display { // TODO this should be procedural. Too bad!
    ($py_type:ty) => {
        impl std::fmt::Display for $py_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
            }
        }
    };
}

//endregion

// Wrapped

#[macro_export]
macro_rules! py_object_cast_generic_no_unwrap {
    ($py_type:ty, $error_msg:expr) => {
        impl $py_type{
            pub(crate) fn try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$py_type, FeagiDataError> {
                let bound = any.bind(py);

                match () {
                    _ if bound.is_instance_of::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj)
                    },
                    _ => Err(FeagiDataError::BadParameters($error_msg.into())) // TODO
                }
            }

        }
    }
}

#[macro_export]
/// Simple system to try to match a py object to the given type
macro_rules! py_object_cast_generic {
    ($py_type:ty, $feagi_type:ty, $error_msg:expr) => {
        impl $py_type{
            pub(crate) fn try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type, FeagiDataError> {
                let bound = any.bind(py);
                
                match () {
                    _ if bound.is_instance_of::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    },
                    _ => Err(FeagiDataError::BadParameters($error_msg.into())) // TODO
                }
            }

        }
    };
}

#[macro_export]
/// Simple system to try to match a py object to an int type
macro_rules! py_object_cast_int {
    ($py_type:ty, $feagi_type:ty, $number_type:ty, $error_msg:expr
    ) => {
        impl $py_type{
            pub(crate) fn try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type, FeagiDataError> {
                let bound = any.bind(py);
                
                match () {
                    _ if bound.is_instance_of::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    }
                    
                    _ if bound.is_instance_of::<PyInt>() => {
                        let py_int = any.extract::<$number_type>(py).unwrap();
                        Ok(py_int.into())
                    }
                    
                    _ => Err(FeagiDataError::BadParameters($error_msg.into()))
                    
                }
            }
        }
    };
}

#[macro_export]
/// Simple system to try to match a py object to an int type
macro_rules! py_object_try_cast_int {
    ($py_type:ty, $feagi_type:ty, $number_type:ty, $error_msg:expr
    ) => {
        impl $py_type{
            pub(crate) fn try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type, FeagiDataError> {
                let bound = any.bind(py);

                match () {
                    _ if bound.is_instance_of::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    }

                    _ if bound.is_instance_of::<PyInt>() => {
                        let py_int = any.extract::<$number_type>(py).unwrap();
                        py_int.try_into()
                    }

                    _ => Err(FeagiDataError::BadParameters($error_msg.into()))

                }
            }
        }
    };
}

#[macro_export]
/// Simple system to try to match a py object to a float type
macro_rules! py_object_cast_float {
    (
        $py_type:ty,
        $feagi_type:ty,
        $error_msg:expr
        
    ) => {
        impl $py_type{
            pub(crate) fn try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type, FeagiDataError> {
                let bound = any.bind(py);
                
                match () {
                    _ if bound.is_instance_of::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    }
                    
                    _ if bound.is_instance_of::<PyFloat>() => {
                        let py_f32 = any.extract::<$number_type>(py).unwrap();
                        Ok(py_f32.into())
                    }
                    
                    _ => Err(FeagiDataError::BadParameters($error_msg.into()))
                    
                }
            }
        }
    };
}