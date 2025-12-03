use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::{CorticalAreaType, CoreCorticalType, CustomCorticalType, MemoryCorticalType};
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::feagi_data_structures::genomic::cortical_area::PyCorticalID;

#[pyclass(str, eq)]
#[pyo3(name = "CorticalAreaType")]
#[derive(Clone)]
pub struct PyCorticalAreaType {
    pub inner: CorticalAreaType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyCorticalAreaType {
    #[staticmethod]
    pub fn Core(core_cortical_type: PyCoreCorticalType) -> Self {
        PyCorticalAreaType {inner: CorticalAreaType::Core(core_cortical_type.into())}
    }

    #[staticmethod]
    pub fn Custom(custom_cortical_type: PyCustomCorticalType) -> Self {
        PyCorticalAreaType {inner: CorticalAreaType::Custom(custom_cortical_type.into())}
    }

    #[staticmethod]
    pub fn Memory(memory_cortical_type: PyMemoryCorticalType) -> Self {
        PyCorticalAreaType {inner: CorticalAreaType::Memory(memory_cortical_type.into())}
    }

}

project_display!(PyCorticalAreaType);
py_type_casts!(PyCorticalAreaType, CorticalAreaType);
py_object_cast_generic!(PyCorticalAreaType, CorticalAreaType, "Unable to retrieve CorticalAreaType data from given!");

//region Core
#[pyclass(str, eq)]
#[pyo3(name = "CoreCorticalType")]
#[derive(Clone)]
pub struct PyCoreCorticalType {
    pub inner: CoreCorticalType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyCoreCorticalType {
    #[staticmethod]
    pub fn Death() -> Self {
        PyCoreCorticalType {inner: CoreCorticalType::Death}
    }

    #[staticmethod]
    pub fn Power() -> Self {
        PyCoreCorticalType {inner: CoreCorticalType::Power}
    }

    pub fn to_cortical_id(&self) -> PyCorticalID {
        self.inner.to_cortical_id().into()
    }
}

project_display!(PyCoreCorticalType);
py_type_casts!(PyCoreCorticalType, CoreCorticalType);
py_object_cast_generic!(PyCoreCorticalType, CoreCorticalType, "Unable to retrieve CoreCorticalType data from given!");

//endregion

//region Custom
#[pyclass(str, eq)]
#[pyo3(name = "CustomCorticalType")]
#[derive(Clone)]
pub struct PyCustomCorticalType {
    pub inner: CustomCorticalType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyCustomCorticalType {
    #[staticmethod]
    pub fn LeakyIntegrateFire() -> Self {
        PyCustomCorticalType {inner: CustomCorticalType::LeakyIntegrateFire}
    }
}

project_display!(PyCustomCorticalType);
py_type_casts!(PyCustomCorticalType, CustomCorticalType);
py_object_cast_generic!(PyCustomCorticalType, CustomCorticalType, "Unable to retrieve CustomCorticalType data from given!");

//endregion

//region Memory
#[pyclass(str, eq)]
#[pyo3(name = "MemoryCorticalType")]
#[derive(Clone)]
pub struct PyMemoryCorticalType {
    pub inner: MemoryCorticalType,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyMemoryCorticalType {
    #[staticmethod]
    pub fn Memory() -> Self {
        PyMemoryCorticalType {inner: MemoryCorticalType::Memory}
    }
}

project_display!(PyMemoryCorticalType);
py_type_casts!(PyMemoryCorticalType, MemoryCorticalType);
py_object_cast_generic!(PyMemoryCorticalType, MemoryCorticalType, "Unable to retrieve MemoryCorticalType data from given!");

//endregion




