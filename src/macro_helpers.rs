#[macro_export]
/// Handles From for wrapped python type and back, assuming wrapper inner property is called "inner"
macro_rules! py_type_casts { // TODO this should be procedural. Too bad!
    (
        $py_type:ty,
        $feagi_type:ty
    ) => {
        
        impl From<$feagi_type> for $py_type {
            fn from(inner: $feagi_type) -> Self {
                Self { inner }
            }
        }
        
        /*
        impl From<&$feagi_type> for $py_type {
            fn from(inner_ref: &$feagi_type) -> Self {
                Self { inner_ref.clone() }
            }
        }
         */
        
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
    (
        $py_type:ty,
        $feagi_type:ty,
        $number_type:ty,
        $error_msg:expr
        
    ) => {
        impl $py_type:ty{
            pub(crate) try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type:ty, FeagiDataError> {
                let bound = any.bind(py);
                
                match () {
                    _ if bound.is_instance::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    }
                    
                    _ if bound.is_instance::<PyInt>() => {
                        let py_int = any.extract::<PyInt>(py).unwrap() as $number_type;
                        Ok(py_int.into())
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
        impl $py_type:ty{
            pub(crate) try_get_from_py_object<'py>(py: Python<'_>, any: PyObject) -> Result<$feagi_type:ty, FeagiDataError> {
                let bound = any.bind(py);
                
                match () {
                    _ if bound.is_instance::<$py_type>() => {
                        let py_obj = any.extract::<$py_type>(py).unwrap();
                        Ok(py_obj.into())
                    }
                    
                    _ if bound.is_instance::<PyFloat>() => {
                        let py_f32 = any.extract::<PyFloat>(py).unwrap() as f32;
                        Ok(py_f32.into())
                    }
                    
                    _ => Err(FeagiDataError::BadParameters($error_msg.into()))
                    
                }
            }
        }
    };
}