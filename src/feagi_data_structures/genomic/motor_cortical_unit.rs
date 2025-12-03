use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::motor_cortical_units;
use feagi_data_structures::genomic::MotorCorticalUnit;

macro_rules! define_motor_cortical_units_enum {
    (
        MotorCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $variant_name:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_name: $snake_case_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:expr,
                    cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    cortical_area_properties: {
                        $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) => {

        #[pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[pyo3(name = "MotorCorticalUnit")]
        pub enum PyMotorCorticalUnit {
            $(
                $(#[doc = $doc])?
                $variant_name,
            )*
        }

        impl From<PyMotorCorticalUnit> for MotorCorticalUnit {
            fn from(inner: PyMotorCorticalUnit) -> Self {
                $(
                    PyMotorCorticalUnit::$variant_name => MotorCorticalUnit::$variant_name,
                )*
            }
        }

        impl From<MotorCorticalUnit> for PyMotorCorticalUnit {
            fn from(inner: MotorCorticalUnit) -> Self {
                $(
                    MotorCorticalUnit::$variant_name => PyMotorCorticalUnit::$variant_name,
                )*
            }
        }

    };
}


// Generate the MotorCorticalUnit enum and all helper methods from the template
motor_cortical_units!(define_motor_cortical_units_enum);
