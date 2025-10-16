use crate::{check_for_segment, get_motor_variants, get_sensor_variants, read_source_file, replace_code_segment, save_source_file};

pub fn update_io_cache_source_file(file_path: &str) {

    let checking_flag: &str = "//BUILDRS_ON";
    let motor_start_comment: &str = "//BUILDRS_MOTOR_DEVICE_START";
    let motor_end_comment: &str = "//BUILDRS_MOTOR_DEVICE_END";
    let sensor_start_comment: &str = "//BUILDRS_SENSOR_DEVICE_START";
    let sensor_end_comment: &str = "//BUILDRS_SENSOR_DEVICE_END";

    let source_file_string = read_source_file(file_path);
    check_for_segment(&source_file_string, checking_flag); // Errors out here if not enabled!

    let motor_registration_functions = generate_motor_registration_functions();
    let source_file_string = replace_code_segment(source_file_string, motor_start_comment, motor_end_comment, motor_registration_functions);

    let sensor_registration_functions = generate_sensor_registration_functions();
    let source_file_string = replace_code_segment(source_file_string, sensor_start_comment, sensor_end_comment, sensor_registration_functions);

    save_source_file(source_file_string, file_path);
}


fn generate_motor_registration_functions() -> String {
    let variants = get_motor_variants();
    let mut functions = String::new();

    for variant in &variants {
        functions.push_str("    //region ");
        functions.push_str(&variant.snake_case_identifier);
        functions.push_str("\n");
        functions.push_str(&generate_motor_functions_for_coder_type(
            &variant.snake_case_identifier,
            &variant.default_coder_type,
            &variant.rust_data_type
        ));
        functions.push_str("    //endregion\n\n");
    }

    functions
}

fn generate_sensor_registration_functions() -> String {
    let variants = get_sensor_variants();
    let mut functions = String::new();

    for variant in &variants {
        functions.push_str("    //region ");
        functions.push_str(&variant.snake_case_identifier);
        functions.push_str("\n");
        functions.push_str(&generate_sensor_functions_for_coder_type(
            &variant.snake_case_identifier,
            &variant.default_coder_type,
            &variant.rust_data_type
        ));
        functions.push_str("    //endregion\n\n");
    }

    functions
}

fn generate_motor_functions_for_coder_type(snake_case_identifier: &str, coder_type: &str, rust_data_type: &str) -> String {
    // This function generates registration functions based on the coder type.

    let percentage_functions = format!(
        r#"
    pub fn motor_{snake_case_identifier}_try_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        z_neuron_depth: PyObject
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

        self.inner.motor_{snake_case_identifier}_try_register(group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn motor_{snake_case_identifier}_try_read_preprocessed_cached_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{rust_data_type}>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {rust_data_type} = self.inner.motor_{snake_case_identifier}_try_read_preprocessed_cached_value_(group, channel).map_err(PyFeagiError::from)?; // TODO Typo
        Ok(unwrapped.into())
    }}

    pub fn motor_{snake_case_identifier}_try_read_postprocessed_cached_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{rust_data_type}>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {rust_data_type} = self.inner.motor_{snake_case_identifier}_try_read_postprocessed_cached_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    pub fn motor_{snake_case_identifier}_try_get_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
    ) -> PyResult<PyPipelineStageProperties>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;

        let boxed_stage: Box<dyn PipelineStageProperties + Sync + Send> = self.inner.motor_{snake_case_identifier}_try_get_single_stage_properties(group, channel, stage_index).map_err(PyFeagiError::from)?;
        Ok(boxed_stage.into())
    }}

"#,
    );

    let misc_data_functions = format!(
        r#"
    pub fn motor_{snake_case_identifier}_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        misc_dimensions: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let misc_dimensions: MiscDataDimensions = PyMiscDataDimensions::try_get_from_py_object(py, misc_dimensions).map_err(PyFeagiError::from)?;

        self.inner.motor_{snake_case_identifier}_try_register(group, number_of_channels, misc_dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }}
"#,
    );

    // Match on coder type to generate appropriate function
    // Currently all types use the same template, but each can be customized independently
    match coder_type {
        // Percentage types
        "Percentage_Absolute_Linear" => percentage_functions,
        "Percentage_Absolute_Fractional" => percentage_functions,
        "Percentage_Incremental_Linear" => percentage_functions,
        "Percentage_Incremental_Fractional" => percentage_functions,

        // Percentage2D types
        "Percentage2D_Absolute_Linear" => percentage_functions,
        "Percentage2D_Absolute_Fractional" => percentage_functions,
        "Percentage2D_Incremental_Linear" => percentage_functions,
        "Percentage2D_Incremental_Fractional" => percentage_functions,

        // Percentage3D types
        "Percentage3D_Absolute_Linear" => percentage_functions,
        "Percentage3D_Absolute_Fractional" => percentage_functions,
        "Percentage3D_Incremental_Linear" => percentage_functions,
        "Percentage3D_Incremental_Fractional" => percentage_functions,

        // Percentage4D types
        "Percentage4D_Absolute_Linear" => percentage_functions,
        "Percentage4D_Absolute_Fractional" => percentage_functions,
        "Percentage4D_Incremental_Linear" => percentage_functions,
        "Percentage4D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage types
        "SignedPercentage_Absolute_Linear" => percentage_functions,
        "SignedPercentage_Absolute_Fractional" => percentage_functions,
        "SignedPercentage_Incremental_Linear" => percentage_functions,
        "SignedPercentage_Incremental_Fractional" => percentage_functions,

        // SignedPercentage2D types
        "SignedPercentage2D_Absolute_Linear" => percentage_functions,
        "SignedPercentage2D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage2D_Incremental_Linear" => percentage_functions,
        "SignedPercentage2D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage3D types
        "SignedPercentage3D_Absolute_Linear" => percentage_functions,
        "SignedPercentage3D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage3D_Incremental_Linear" => percentage_functions,
        "SignedPercentage3D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage4D types
        "SignedPercentage4D_Absolute_Linear" => percentage_functions,
        "SignedPercentage4D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage4D_Incremental_Linear" => percentage_functions,
        "SignedPercentage4D_Incremental_Fractional" => percentage_functions,

        // MiscData types
        "MiscData_Absolute" => misc_data_functions,
        "MiscData_Incremental" => misc_data_functions,

        // ImageFrame types
        "ImageFrame_Absolute" => percentage_functions,
        "ImageFrame_Incremental" => percentage_functions,

        // Default case for any future types
        _ => {
            println!("cargo:warning=Unknown coder type '{}', using default template", coder_type);
            percentage_functions
        }
    }
}

fn generate_sensor_functions_for_coder_type(snake_case_identifier: &str, coder_type: &str, rust_data_type: &str) -> String {
    // This function generates registration functions based on the coder type.

    let percentage_functions = format!(
        r#"
    pub fn sensor_register_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        z_neuron_depth: PyObject
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

        self.inner.sensor_register_{}(group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_write_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {} = Py{}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_write_{}(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

"#,
        snake_case_identifier,
        snake_case_identifier,
        snake_case_identifier,
        rust_data_type,
        rust_data_type,
        snake_case_identifier,
    );

    let misc_data_functions = format!(
        r#"
    pub fn sensor_register_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        misc_dimensions: PyMiscDataDimensions,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let misc_dimensions: MiscDataDimensions = misc_dimensions.into();

        self.inner.sensor_register_{}(group, number_of_channels, misc_dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_write_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {} = Py{}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_write_{}(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

"#,
        snake_case_identifier,
        snake_case_identifier,
        snake_case_identifier,
        rust_data_type,
        rust_data_type,
        snake_case_identifier,
    );

    let image_frame_functions = format!(
        r#"
    pub fn sensor_register_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        image_properties: PyImageFrameProperties,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let image_properties: ImageFrameProperties = image_properties.into();

        self.inner.sensor_register_{}(group, number_of_channels, image_properties).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_write_{}(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {} = Py{}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_write_{}(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

"#,
        snake_case_identifier,
        snake_case_identifier,
        snake_case_identifier,
        rust_data_type,
        rust_data_type,
        snake_case_identifier,
    );

    // Match on coder type to generate appropriate function
    match coder_type {
        // Percentage types
        "Percentage_Absolute_Linear" => percentage_functions,
        "Percentage_Absolute_Fractional" => percentage_functions,
        "Percentage_Incremental_Linear" => percentage_functions,
        "Percentage_Incremental_Fractional" => percentage_functions,

        // Percentage2D types
        "Percentage2D_Absolute_Linear" => percentage_functions,
        "Percentage2D_Absolute_Fractional" => percentage_functions,
        "Percentage2D_Incremental_Linear" => percentage_functions,
        "Percentage2D_Incremental_Fractional" => percentage_functions,

        // Percentage3D types
        "Percentage3D_Absolute_Linear" => percentage_functions,
        "Percentage3D_Absolute_Fractional" => percentage_functions,
        "Percentage3D_Incremental_Linear" => percentage_functions,
        "Percentage3D_Incremental_Fractional" => percentage_functions,

        // Percentage4D types
        "Percentage4D_Absolute_Linear" => percentage_functions,
        "Percentage4D_Absolute_Fractional" => percentage_functions,
        "Percentage4D_Incremental_Linear" => percentage_functions,
        "Percentage4D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage types
        "SignedPercentage_Absolute_Linear" => percentage_functions,
        "SignedPercentage_Absolute_Fractional" => percentage_functions,
        "SignedPercentage_Incremental_Linear" => percentage_functions,
        "SignedPercentage_Incremental_Fractional" => percentage_functions,

        // SignedPercentage2D types
        "SignedPercentage2D_Absolute_Linear" => percentage_functions,
        "SignedPercentage2D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage2D_Incremental_Linear" => percentage_functions,
        "SignedPercentage2D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage3D types
        "SignedPercentage3D_Absolute_Linear" => percentage_functions,
        "SignedPercentage3D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage3D_Incremental_Linear" => percentage_functions,
        "SignedPercentage3D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage4D types
        "SignedPercentage4D_Absolute_Linear" => percentage_functions,
        "SignedPercentage4D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage4D_Incremental_Linear" => percentage_functions,
        "SignedPercentage4D_Incremental_Fractional" => percentage_functions,

        // MiscData types
        "MiscData_Absolute" => misc_data_functions,
        "MiscData_Incremental" => misc_data_functions,

        // ImageFrame types
        "ImageFrame_Absolute" => image_frame_functions,
        "ImageFrame_Incremental" => image_frame_functions,

        // Default case for any future types
        _ => {
            println!("cargo:warning=Unknown sensor coder type '{}', using default template", coder_type);
            percentage_functions
        }
    }
}
