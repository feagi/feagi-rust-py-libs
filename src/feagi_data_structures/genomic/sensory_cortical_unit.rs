use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::sensor_cortical_units;
use feagi_data_structures::genomic::SensoryCorticalUnit;


macro_rules! define_sensor_cortical_units_enum {
    (
        SensoryCorticalUnit {
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
        #[pyo3(name = "SensoryCorticalUnit")]
        pub enum PySensoryCorticalUnit {
            $(
                $(#[doc = $doc])?
                $variant_name,
            )*
        }

        impl From<PySensoryCorticalUnit> for SensoryCorticalUnit {
            fn from(py_unit: PySensoryCorticalUnit) -> Self {
                match py_unit {
                    $(
                        PySensoryCorticalUnit::$variant_name => SensoryCorticalUnit::$variant_name,
                    )*
                }
            }
        }

        impl From<SensoryCorticalUnit> for PySensoryCorticalUnit {
            fn from(rust_unit: SensoryCorticalUnit) -> Self {
                match rust_unit {
                    $(
                        SensoryCorticalUnit::$variant_name => PySensoryCorticalUnit::$variant_name,
                    )*
                }
            }
        }

    };
}


// Generate the SensoryCorticalUnit enum and all helper methods from the template
sensor_cortical_units!(define_sensor_cortical_units_enum);
