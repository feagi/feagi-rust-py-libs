use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::IOCorticalAreaDataFlag;

#[pyclass(str, eq)]
#[pyo3(name = "IOCorticalAreaDataFlag")]
#[derive(Clone)]
