//! Python bindings over feagi-sensorimotor's public percentage coder interface.
//!
//! Python never re-derives the percentage coding variant. Callers pass the unit exactly as the
//! agent registration JSON carries it: the registration section key, the unit key
//! (for example `PositionalServo`) and the unit definition object (its `cortical_unit_index` and
//! `io_configuration_flags`). The binding derives the unit's cortical IDs the same way FEAGI
//! does, reads the IO configuration flag of the chosen cortical area and selects the coding
//! through `PercentageCodingVariant::from_io_flag`. Every failure is a `ValueError` carrying the
//! Rust message; nothing falls back to a default coding.
//!
//! Python access: `from feagi_rust_py_libs import connector_core` (functions are attributes of
//! that submodule, like `connector_core.ConnectorAgent`).
//! @cursor:ffi-safe

use feagi_data_structures::genomic::cortical_area::descriptors::CorticalUnitIndex;
use feagi_data_structures::genomic::cortical_area::io_cortical_area_configuration_flag::IOCorticalAreaConfigurationFlag;
use feagi_data_structures::genomic::{MotorCorticalUnit, SensoryCorticalUnit};
use feagi_sensorimotor::configuration::jsonable::PhysicalRange;
use feagi_sensorimotor::single_voxel_decode::{
    decode_percentage_from_z_indexes, encode_percentage_to_z_indexes, target_to_z_bin,
    PercentageCodingVariant,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;

use super::device_registration_derive::extract_io_config;

/// Registration section that lists motor units (decoder side).
const OUTPUT_SECTION: &str = "output_units_and_decoder_properties";
/// Registration section that lists sensory units (encoder side).
const INPUT_SECTION: &str = "input_units_and_encoder_properties";

/// Resolve the IO configuration flag of one cortical area of a registered unit.
///
/// * `registration_section` - `output_units_and_decoder_properties` or
///   `input_units_and_encoder_properties`.
/// * `unit_key` - the unit key in that section (for example `PositionalServo`).
/// * `unit_definition_json` - the unit definition object (`cortical_unit_index`,
///   `io_configuration_flags`, ...).
/// * `area_index` - which cortical area of the unit (a unit can own several, for example the
///   absolute, incremental and speed areas of a positional servo).
fn resolve_flag(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    area_index: usize,
) -> Result<IOCorticalAreaConfigurationFlag, String> {
    let unit_def: serde_json::Value = serde_json::from_str(unit_definition_json)
        .map_err(|e| format!("unit_definition_json is not valid JSON: {e}"))?;
    let index_raw = unit_def
        .get("cortical_unit_index")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "unit definition missing integer cortical_unit_index".to_string())?;
    let index_u16 = u16::try_from(index_raw)
        .map_err(|_| "cortical_unit_index out of range for u16".to_string())?;
    let unit_index: CorticalUnitIndex = index_u16.into();
    let config = extract_io_config(&unit_def)?;
    let unit_key_json = serde_json::Value::String(unit_key.to_string());

    let cortical_ids = match registration_section {
        OUTPUT_SECTION => serde_json::from_value::<MotorCorticalUnit>(unit_key_json)
            .map_err(|e| format!("unknown motor unit key '{unit_key}': {e}"))?
            .get_cortical_id_vector_from_index_and_serde_io_configuration_flags(unit_index, config)
            .map_err(|e| format!("cannot derive cortical IDs for '{unit_key}': {e}"))?,
        INPUT_SECTION => serde_json::from_value::<SensoryCorticalUnit>(unit_key_json)
            .map_err(|e| format!("unknown sensory unit key '{unit_key}': {e}"))?
            .get_cortical_id_vector_from_index_and_serde_io_configuration_flags(unit_index, config)
            .map_err(|e| format!("cannot derive cortical IDs for '{unit_key}': {e}"))?,
        other => {
            return Err(format!(
            "registration_section must be '{OUTPUT_SECTION}' or '{INPUT_SECTION}', got '{other}'"
        ))
        }
    };
    let cortical_id = cortical_ids.get(area_index).ok_or_else(|| {
        format!(
            "area_index {area_index} out of range: unit '{unit_key}' has {} cortical areas",
            cortical_ids.len()
        )
    })?;
    cortical_id
        .extract_io_data_flag()
        .map_err(|e| format!("cannot read the IO flag of '{unit_key}' area {area_index}: {e}"))
}

/// Name of the percentage coding variant the unit area uses.
fn variant_name(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    area_index: usize,
) -> Result<String, String> {
    let flag = resolve_flag(
        registration_section,
        unit_key,
        unit_definition_json,
        area_index,
    )?;
    let variant = PercentageCodingVariant::from_io_flag(&flag).map_err(|e| e.to_string())?;
    Ok(format!("{variant:?}"))
}

/// Decode one value from its active z indexes (see `feagi_sensorimotor::single_voxel_decode`).
fn decode(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    z_positive: &[u32],
    z_negative: &[u32],
    area_index: usize,
) -> Result<f32, String> {
    let flag = resolve_flag(
        registration_section,
        unit_key,
        unit_definition_json,
        area_index,
    )?;
    decode_percentage_from_z_indexes(&flag, depth, z_positive, z_negative)
        .map_err(|e| e.to_string())
}

/// Encode one value to its active z indexes `(positive, negative)`.
fn encode(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    value: f32,
    area_index: usize,
) -> Result<(Vec<u32>, Vec<u32>), String> {
    let flag = resolve_flag(
        registration_section,
        unit_key,
        unit_definition_json,
        area_index,
    )?;
    let z = encode_percentage_to_z_indexes(&flag, depth, value).map_err(|e| e.to_string())?;
    Ok((z.positive, z.negative))
}

/// The z bin whose decode is nearest to `target`.
fn target_bin(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    target: f64,
    area_index: usize,
) -> Result<u32, String> {
    let flag = resolve_flag(
        registration_section,
        unit_key,
        unit_definition_json,
        area_index,
    )?;
    target_to_z_bin(&flag, depth, target).map_err(|e| e.to_string())
}

/// Validate a `physical_range` object and return `(value_at_0, value_at_1, unit)`.
fn parse_physical_range(physical_range_json: &str) -> Result<(f64, f64, String), String> {
    let range: PhysicalRange = serde_json::from_str(physical_range_json)
        .map_err(|e| format!("invalid physical_range: {e}"))?;
    Ok((
        range.value_at_0(),
        range.value_at_1(),
        range.unit().to_string(),
    ))
}

/// Name of the percentage coding variant of one cortical area of a registered unit.
///
/// Args:
///     registration_section: `output_units_and_decoder_properties` or
///         `input_units_and_encoder_properties`.
///     unit_key: unit key in that section, e.g. `PositionalServo`.
///     unit_definition_json: the unit definition object as a JSON string.
///     area_index: which cortical area of the unit (default 0).
///
/// Raises:
///     ValueError: unknown unit, invalid flags, or a non-percentage area.
#[pyfunction]
#[pyo3(name = "percentage_coding_variant", signature = (registration_section, unit_key, unit_definition_json, area_index=0))]
pub fn py_percentage_coding_variant(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    area_index: usize,
) -> PyResult<String> {
    variant_name(
        registration_section,
        unit_key,
        unit_definition_json,
        area_index,
    )
    .map_err(PyValueError::new_err)
}

/// Decode one percentage value from its active z indexes.
///
/// Returns the value in the variant's native range (0..1 unsigned, -1..1 signed), computed by
/// the same functions as the batch decoders. `z_negative` is the negative lobe of two-lobe
/// variants and must be empty otherwise.
///
/// Raises:
///     ValueError: invalid unit, zero depth, z index out of range, or empty unsigned input.
#[pyfunction]
#[pyo3(name = "decode_percentage_from_z_indexes", signature = (registration_section, unit_key, unit_definition_json, depth, z_positive, z_negative=None, area_index=0))]
pub fn py_decode_percentage_from_z_indexes(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    z_positive: Vec<u32>,
    z_negative: Option<Vec<u32>>,
    area_index: usize,
) -> PyResult<f32> {
    decode(
        registration_section,
        unit_key,
        unit_definition_json,
        depth,
        &z_positive,
        z_negative.as_deref().unwrap_or(&[]),
        area_index,
    )
    .map_err(PyValueError::new_err)
}

/// Encode one percentage value to its active z indexes `(positive, negative)`.
///
/// `negative` is filled only by two-lobe variants. The encoder places a value within one bin
/// of the decoder's reading; use `target_to_z_bin` for the bin that decodes to a wanted value.
///
/// Raises:
///     ValueError: invalid unit, zero depth, or a value outside the variant's range.
#[pyfunction]
#[pyo3(name = "encode_percentage_to_z_indexes", signature = (registration_section, unit_key, unit_definition_json, depth, value, area_index=0))]
pub fn py_encode_percentage_to_z_indexes(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    value: f32,
    area_index: usize,
) -> PyResult<(Vec<u32>, Vec<u32>)> {
    encode(
        registration_section,
        unit_key,
        unit_definition_json,
        depth,
        value,
        area_index,
    )
    .map_err(PyValueError::new_err)
}

/// The z bin whose decode is nearest to `target` (the inverse of the decoder).
///
/// Raises:
///     ValueError: invalid unit, zero depth, target outside the range, or a two-lobe variant
///         (a value there needs a lobe as well as a bin).
#[pyfunction]
#[pyo3(name = "target_to_z_bin", signature = (registration_section, unit_key, unit_definition_json, depth, target, area_index=0))]
pub fn py_target_to_z_bin(
    registration_section: &str,
    unit_key: &str,
    unit_definition_json: &str,
    depth: u32,
    target: f64,
    area_index: usize,
) -> PyResult<u32> {
    target_bin(
        registration_section,
        unit_key,
        unit_definition_json,
        depth,
        target,
        area_index,
    )
    .map_err(PyValueError::new_err)
}

/// Validate a device grouping `physical_range` object (JSON string) with FEAGI's own rules and
/// return `(value_at_0, value_at_1, unit)`.
///
/// Raises:
///     ValueError: non-finite values, equal endpoints, or an empty unit.
#[pyfunction]
#[pyo3(name = "validate_physical_range")]
pub fn py_validate_physical_range(physical_range_json: &str) -> PyResult<(f64, f64, String)> {
    parse_physical_range(physical_range_json).map_err(PyValueError::new_err)
}

/// Add the percentage coding functions to the `connector_core` submodule.
pub fn register_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = parent.getattr("connector_core")?.cast_into::<PyModule>()?;
    module.add_function(wrap_pyfunction!(py_percentage_coding_variant, &module)?)?;
    module.add_function(wrap_pyfunction!(
        py_decode_percentage_from_z_indexes,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        py_encode_percentage_to_z_indexes,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(py_target_to_z_bin, &module)?)?;
    module.add_function(wrap_pyfunction!(py_validate_physical_range, &module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUT: &str = OUTPUT_SECTION;

    /// Positional servo unit, absolute linear, as the registration JSON carries it.
    fn servo_unit(positioning: &str) -> String {
        format!(
            r#"{{"cortical_unit_index": 0, "io_configuration_flags":
               {{"frame_change_handling": "Absolute",
                 "percentage_neuron_positioning": "{positioning}"}}}}"#
        )
    }

    fn rotary_unit() -> String {
        r#"{"cortical_unit_index": 0, "io_configuration_flags":
           {"frame_change_handling": "Absolute", "percentage_neuron_positioning": "Linear"}}"#
            .to_string()
    }

    #[test]
    fn servo_bin_170_at_depth_511_decodes_to_two_thirds_and_target_inverts_it() {
        let unit = servo_unit("Linear");
        let value = decode(OUT, "PositionalServo", &unit, 511, &[170], &[], 0).unwrap();
        assert!((value - (1.0 - 170.0 / 510.0)).abs() < 1e-6, "{value}");
        let bin = target_bin(OUT, "PositionalServo", &unit, 511, value as f64, 0).unwrap();
        assert_eq!(bin, 170);
    }

    #[test]
    fn variant_follows_the_unit_and_its_flags() {
        let servo = variant_name(OUT, "PositionalServo", &servo_unit("Linear"), 0).unwrap();
        assert_eq!(servo, "UnsignedLinear");
        let fractional =
            variant_name(OUT, "PositionalServo", &servo_unit("Fractional"), 0).unwrap();
        assert_eq!(fractional, "UnsignedFractional");
        let rotary = variant_name(OUT, "RotaryMotor", &rotary_unit(), 0).unwrap();
        assert_eq!(rotary, "SignedLinearAlongZ");
    }

    #[test]
    fn signed_along_z_round_trip_goes_through_target_bin() {
        let unit = rotary_unit();
        let bin = target_bin(OUT, "RotaryMotor", &unit, 9, 0.0, 0).unwrap();
        assert_eq!(bin, 4);
        let value = decode(OUT, "RotaryMotor", &unit, 9, &[bin], &[], 0).unwrap();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn encode_returns_positive_and_negative_lobes() {
        let (positive, negative) = encode(OUT, "RotaryMotor", &rotary_unit(), 9, 0.0, 0).unwrap();
        assert_eq!(positive, vec![4]);
        assert!(negative.is_empty());
    }

    #[test]
    fn errors_name_the_problem() {
        let unit = servo_unit("Linear");
        let err = decode(OUT, "NoSuchUnit", &unit, 8, &[0], &[], 0).unwrap_err();
        assert!(err.contains("unknown motor unit key"), "{err}");
        let err = decode("bogus_section", "PositionalServo", &unit, 8, &[0], &[], 0).unwrap_err();
        assert!(err.contains("registration_section"), "{err}");
        let err = decode(OUT, "PositionalServo", &unit, 8, &[0], &[], 7).unwrap_err();
        assert!(err.contains("area_index 7 out of range"), "{err}");
        let err = decode(OUT, "PositionalServo", "{not json", 8, &[0], &[], 0).unwrap_err();
        assert!(err.contains("not valid JSON"), "{err}");
        let err = decode(OUT, "PositionalServo", &unit, 0, &[0], &[], 0).unwrap_err();
        assert!(!err.is_empty());
        let err = decode(OUT, "PositionalServo", &unit, 8, &[8], &[], 0).unwrap_err();
        assert!(err.contains('8'), "{err}");
        let err = target_bin(OUT, "PositionalServo", &unit, 8, 1.5, 0).unwrap_err();
        assert!(!err.is_empty());
        let missing_flags = r#"{"cortical_unit_index": 0, "io_configuration_flags": {}}"#;
        assert!(decode(OUT, "PositionalServo", missing_flags, 8, &[0], &[], 0).is_err());
    }

    #[test]
    fn physical_range_is_validated_by_the_core_rules() {
        let ok = r#"{"value_at_0": -1.0, "value_at_1": 1.0, "unit": "rad"}"#;
        assert_eq!(
            parse_physical_range(ok).unwrap(),
            (-1.0, 1.0, "rad".to_string())
        );
        let equal = r#"{"value_at_0": 1.0, "value_at_1": 1.0, "unit": "rad"}"#;
        assert!(parse_physical_range(equal).is_err());
        let no_unit = r#"{"value_at_0": 0.0, "value_at_1": 1.0, "unit": " "}"#;
        assert!(parse_physical_range(no_unit).is_err());
    }
}
