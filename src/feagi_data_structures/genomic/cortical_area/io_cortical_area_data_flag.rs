use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::cortical_area::{IOCorticalAreaDataFlag};
use feagi_data_structures::genomic::cortical_area::io_cortical_area_data_type::{FrameChangeHandling, PercentageNeuronPositioning};
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;


//region IOCorticalAreaDataFlag

/// Enum representing the data type configuration for IO cortical areas.
/// 
/// This determines how data is encoded/decoded for brain input/output regions.
#[pyclass(str, eq)]
#[pyo3(name = "IOCorticalAreaDataFlag")]
#[derive(Clone, PartialEq)]
pub struct PyIOCorticalAreaDataFlag {
    pub inner: IOCorticalAreaDataFlag,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyIOCorticalAreaDataFlag {

    //region Constructors

    /// Create a Boolean data flag (for on/off signals).
    #[staticmethod]
    pub fn Boolean() -> Self {
        PyIOCorticalAreaDataFlag { inner: IOCorticalAreaDataFlag::Boolean }
    }

    /// Create a Percentage data flag (single 0-1 value).
    #[staticmethod]
    pub fn Percentage(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::Percentage(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a Percentage2D data flag (2D percentage values).
    #[staticmethod]
    pub fn Percentage2D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::Percentage2D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a Percentage3D data flag (3D percentage values).
    #[staticmethod]
    pub fn Percentage3D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::Percentage3D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a Percentage4D data flag (4D percentage values).
    #[staticmethod]
    pub fn Percentage4D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::Percentage4D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a SignedPercentage data flag (single -1 to 1 value).
    #[staticmethod]
    pub fn SignedPercentage(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::SignedPercentage(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a SignedPercentage2D data flag (2D signed percentage values).
    #[staticmethod]
    pub fn SignedPercentage2D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::SignedPercentage2D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a SignedPercentage3D data flag (3D signed percentage values).
    #[staticmethod]
    pub fn SignedPercentage3D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::SignedPercentage3D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a SignedPercentage4D data flag (4D signed percentage values, e.g., quaternion).
    #[staticmethod]
    pub fn SignedPercentage4D(frame_change_handling: PyFrameChangeHandling, percentage_neuron_positioning: PyPercentageNeuronPositioning) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::SignedPercentage4D(frame_change_handling.inner, percentage_neuron_positioning.inner) 
        }
    }

    /// Create a CartesianPlane data flag (for 2D image/grid data).
    #[staticmethod]
    pub fn CartesianPlane(frame_change_handling: PyFrameChangeHandling) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::CartesianPlane(frame_change_handling.inner) 
        }
    }

    /// Create a Misc data flag (for miscellaneous/custom data).
    #[staticmethod]
    pub fn Misc(frame_change_handling: PyFrameChangeHandling) -> Self {
        PyIOCorticalAreaDataFlag { 
            inner: IOCorticalAreaDataFlag::Misc(frame_change_handling.inner) 
        }
    }

    //endregion

    //region Conversion Methods

    /// Convert to a 16-bit data type configuration flag.
    /// 
    /// Returns:
    ///     int: The packed configuration flag as a 16-bit unsigned integer.
    pub fn to_data_type_configuration_flag(&self) -> u16 {
        self.inner.to_data_type_configuration_flag()
    }

    /// Create from a 16-bit data type configuration flag.
    /// 
    /// Args:
    ///     flag: The 16-bit configuration flag.
    /// 
    /// Returns:
    ///     IOCorticalAreaDataFlag: The decoded data flag.
    /// 
    /// Raises:
    ///     ValueError: If the flag represents an invalid configuration.
    #[staticmethod]
    pub fn try_from_data_type_configuration_flag(flag: u16) -> PyResult<Self> {
        let data_flag = IOCorticalAreaDataFlag::try_from_data_type_configuration_flag(flag)
            .map_err(PyFeagiError::from)?;
        Ok(PyIOCorticalAreaDataFlag { inner: data_flag })
    }

    //endregion
}

project_display!(PyIOCorticalAreaDataFlag);
py_type_casts!(PyIOCorticalAreaDataFlag, IOCorticalAreaDataFlag);
py_object_cast_generic!(PyIOCorticalAreaDataFlag, IOCorticalAreaDataFlag, "Unable to retrieve IOCorticalAreaDataFlag data from given!");

//endregion

//region PercentageNeuronPositioning

/// Enum representing how percentage values are mapped to neuron positions.
///
/// Variants:
///     Linear: Linear mapping of percentage to neuron position
///     Fractional: Fractional/exponential mapping (default)
#[pyclass(str, eq)]
#[pyo3(name = "PercentageNeuronPositioning")]
#[derive(Clone, PartialEq)]
pub struct PyPercentageNeuronPositioning {
    pub inner: PercentageNeuronPositioning,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyPercentageNeuronPositioning {
    /// Create a Linear neuron positioning mode.
    #[staticmethod]
    pub fn Linear() -> Self {
        PyPercentageNeuronPositioning { inner: PercentageNeuronPositioning::Linear }
    }

    /// Create a Fractional neuron positioning mode (default).
    #[staticmethod]
    pub fn Fractional() -> Self {
        PyPercentageNeuronPositioning { inner: PercentageNeuronPositioning::Fractional }
    }
}

project_display!(PyPercentageNeuronPositioning);
py_type_casts!(PyPercentageNeuronPositioning, PercentageNeuronPositioning);
py_object_cast_generic!(PyPercentageNeuronPositioning, PercentageNeuronPositioning, "Unable to retrieve PercentageNeuronPositioning data from given!");

//endregion

//region FrameChangeHandling

/// Enum representing how frame changes are handled.
///
/// Variants:
///     Absolute: Values are treated as absolute positions (default)
///     Incremental: Values are treated as incremental changes
#[pyclass(str, eq)]
#[pyo3(name = "FrameChangeHandling")]
#[derive(Clone, PartialEq)]
pub struct PyFrameChangeHandling {
    pub inner: FrameChangeHandling,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyFrameChangeHandling {
    /// Create an Absolute frame change handling mode (default).
    #[staticmethod]
    pub fn Absolute() -> Self {
        PyFrameChangeHandling { inner: FrameChangeHandling::Absolute }
    }

    /// Create an Incremental frame change handling mode.
    #[staticmethod]
    pub fn Incremental() -> Self {
        PyFrameChangeHandling { inner: FrameChangeHandling::Incremental }
    }
}

project_display!(PyFrameChangeHandling);
py_type_casts!(PyFrameChangeHandling, FrameChangeHandling);
py_object_cast_generic!(PyFrameChangeHandling, FrameChangeHandling, "Unable to retrieve FrameChangeHandling data from given!");

//endregion
