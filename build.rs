mod rust_build_scripts;
use std::fs;
use feagi_data_structures::sensor_definition;
use feagi_data_structures::motor_definition;

fn main() {
    println!("cargo:rerun-if-changed=feagi_data_processing.pyi.template");
    println!("cargo:rerun-if-changed=src/feagi_connector_core/caching/io_cache.rs");
    
    let template_path = "feagi_data_processing.pyi.template";
    let pyi_output_path = "feagi_data_processing.pyi";

    let io_cache_path = "src/feagi_connector_core/caching/io_cache.rs";



    // Update IOCache stuff
    rust_build_scripts::io_cache_template_writer::update_io_cache_source_file(io_cache_path);
}


// TODO: Rename to feagi_data_libraries?
// TODO add macro(s) / funcs for going from PyObject to index types?
// TODO: confirm func for building inheritance?

fn read_source_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));
    content
}

fn save_source_file(data: String, file_path: &str) {
    // Write the updated content back to the file
    fs::write(file_path, &data)
        .unwrap_or_else(|_| panic!("Failed to write {}", file_path));
}

fn check_for_segment(source_string: &String, checking: &str) {
    _ = source_string.find(checking)
        .unwrap_or_else(|| panic!("Could not find '{}' requirement in source file!", checking));
}
fn replace_code_segment(source_string: String, start_marker: &str, end_marker: &str, replacing_string: String) -> String {
    // Read the file
    let content = source_string;

    // Find the positions of the markers
    let start_pos = content.find(start_marker)
        .unwrap_or_else(|| panic!("Could not find {} marker in source file!", start_marker));
    let end_pos = content.find(end_marker)
        .unwrap_or_else(|| panic!("Could not find {} marker in source file!", end_marker));

    // Ensure the markers are in the correct order
    if start_pos >= end_pos {
        panic!("Markers are in the wrong order in source file!");
    }

    // Calculate the position after the start marker (including the newline)
    let replace_start = start_pos + start_marker.len();

    // Find the newline after the start marker
    let replace_start = if content[replace_start..].starts_with('\r') {
        replace_start + 2 // Skip \r\n
    } else if content[replace_start..].starts_with('\n') {
        replace_start + 1 // Skip \n
    } else {
        replace_start
    };

    // Find the position before the end marker (including any leading whitespace on that line)
    let replace_end = content[..end_pos].rfind('\n').map(|pos| pos + 1).unwrap_or(end_pos);

    // Build the new content
    let mut new_content = String::new();
    new_content.push_str(&content[..replace_start]);
    new_content.push_str(&replacing_string);
    new_content.push_str(&content[replace_end..]);

    new_content
}



//region Collect Sensor / Motor context

// Macro to collect motor variant information
macro_rules! collect_motor_variants {
    (
        MotorCorticalType {
            $(
                #[doc = $doc:expr]
                $variant:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_identifier: $snake_case_identifier:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:ident,
                    wrapped_data_type: $wrapped_data_type:expr,
                    data_type: $data_type:ident,
                }$(,)?
            )*
        }
    ) => {
        vec![
            $(
                MotorVariant {
                    name: stringify!($variant).to_string(),
                    doc: Some($doc.to_string()),
                    friendly_name: $friendly_name.to_string(),
                    snake_case_identifier: $snake_case_identifier.to_string(),
                    default_coder_type: stringify!($default_coder_type).to_string(),
                    rust_data_type: stringify!($data_type).to_string()
                }
            ),*
        ]
    };
}

macro_rules! collect_sensor_variants {
    (
        SensorCorticalType {
            $(
                #[doc = $doc:expr]
                $variant:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_identifier: $snake_case_identifier:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:ident,
                    wrapped_data_type: $wrapped_data_type:expr,
                    data_type: $data_type:ident,
                }$(,)?
            )*
        }
    ) => {
        vec![
            $(
                SensorVariant {
                    name: stringify!($variant).to_string(),
                    doc: Some($doc.to_string()),
                    friendly_name: $friendly_name.to_string(),
                    snake_case_identifier: $snake_case_identifier.to_string(),
                    default_coder_type: stringify!($default_coder_type).to_string(),
                    rust_data_type: stringify!($data_type).to_string()
                }
            ),*
        ]
    };
}

#[derive(Debug)]
struct MotorVariant {
    name: String,
    doc: Option<String>,
    friendly_name: String,
    snake_case_identifier: String,
    default_coder_type: String,
    rust_data_type: String,
}

#[derive(Debug)]
struct SensorVariant {
    name: String,
    doc: Option<String>,
    friendly_name: String,
    snake_case_identifier: String,
    default_coder_type: String,
    rust_data_type: String,
}

fn get_sensor_variants() -> Vec<SensorVariant> {
    sensor_definition!(collect_sensor_variants)
}
fn get_motor_variants() -> Vec<MotorVariant> {motor_definition!(collect_motor_variants)}

//endregion