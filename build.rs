use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=feagi_data_processing.pyi.template");
    
    let template_path = "feagi_data_processing.pyi.template";
    let output_path = "feagi_data_processing.pyi";
    
    // Read the template file
    let template_content = fs::read_to_string(template_path)
        .expect("Failed to read feagi_data_processing.pyi.template");
    
    // Generate the SensorCorticalType class definition using the macro
    let sensor_class_def = generate_sensor_cortical_type_class();
    
    // Insert the SensorCorticalType into the template
    let final_content = insert_sensor_cortical_type(template_content, sensor_class_def);
    
    // Write the final .pyi file
    fs::write(output_path, final_content)
        .expect("Failed to write feagi_data_processing.pyi");
    
    println!("Generated feagi_data_processing.pyi with SensorCorticalType");
}

use feagi_data_structures::sensor_definition;

// TODO: Rename to feagi_data_libraries?
// TODO add macro(s) / funcs for going from PyObject to index types?
// TODO: confirm func for building inheritance?


// Macro to collect sensor variant information
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
                }
            ),*
        ]
    };
}

#[derive(Debug)]
struct SensorVariant {
    name: String,
    doc: Option<String>,
    #[allow(dead_code)]
    friendly_name: String,
}

fn get_sensor_variants() -> Vec<SensorVariant> {
    sensor_definition!(collect_sensor_variants)
}

fn generate_sensor_cortical_type_class() -> String {
    let variants = get_sensor_variants();
    let mut class_def = String::new();
    
    class_def.push_str("    class SensorCorticalType:\n");
    class_def.push_str("        \"\"\"Enum representing different types of sensor cortical areas.\n");
    class_def.push_str("        \n");
    class_def.push_str("        This enum defines all the available sensor types that can be used\n");
    class_def.push_str("        in FEAGI for processing input data from various sensors and devices.\n");
    class_def.push_str("        Each sensor type has specific characteristics and use cases.\n");
    class_def.push_str("        \"\"\"\n");
    class_def.push_str("        \n");
    
    // Add each variant as a class attribute with docstrings
    for variant in &variants {
        class_def.push_str(&format!("        {}: 'SensorCorticalType'\n", variant.name));
        if let Some(doc) = &variant.doc {
            class_def.push_str(&format!("        \"\"\"{}.\"\"\"\n", doc));
        }
        class_def.push_str("        \n");
    }
    
    class_def
}

fn insert_sensor_cortical_type(template: String, sensor_class_def: String) -> String {
    // Find the insertion point - after CoreCorticalType but before CorticalGroupingIndex
    let insertion_marker = "    class CoreCorticalType:\n        \"\"\"Enum representing core cortical area types.\"\"\"\n        Death: 'CoreCorticalType'\n        Power: 'CoreCorticalType'\n    \n";
    
    if let Some(pos) = template.find(insertion_marker) {
        let end_pos = pos + insertion_marker.len();
        let mut result = String::new();
        result.push_str(&template[..end_pos]);
        result.push_str(&sensor_class_def);
        result.push_str(&template[end_pos..]);
        result
    } else {
        // Fallback: try to find a different marker
        let fallback_marker = "    class CorticalGroupingIndex:";
        if let Some(pos) = template.find(fallback_marker) {
            let mut result = String::new();
            result.push_str(&template[..pos]);
            result.push_str(&sensor_class_def);
            result.push_str("    ");
            result.push_str(&template[pos..]);
            result
        } else {
            // If we can't find the insertion point, just append at the end of the genome module
            let genome_end_marker = "\n# IO Data module";
            if let Some(pos) = template.find(genome_end_marker) {
                let mut result = String::new();
                result.push_str(&template[..pos]);
                result.push_str("\n");
                result.push_str(&sensor_class_def);
                result.push_str(&template[pos..]);
                result
            } else {
                // Last resort: just return template with sensor class appended
                let mut result = template;
                result.push_str("\n");
                result.push_str(&sensor_class_def);
                result
            }
        }
    }
}
