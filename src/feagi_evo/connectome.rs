use feagi_services::brain_artifact::{
    supported_brain_artifact_versions, SupportedArtifactVersions,
};
use feagi_services::connectome::ConnectomeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};

pyo3::create_exception!(
    feagi_rust_py_libs.connectome,
    UnsupportedArtifactVersionError,
    PyValueError,
    "Raised when a connectome artifact was written by a newer FEAGI build than this one reads.\n\n\
     Subclasses `ValueError`, so callers that catch `ValueError` keep working. The remedy is \
     upgrading the reader, not repairing the file. `str()` is the full message \
     (\"Connectome validation failed: ...\").\n\n\
     Instances raised by this module carry the attributes `component` (str: \"contract\", \
     \"container\", \"connectome_schema\" or \"genome_schema\"), `artifact_version` (int, \
     version found in the artifact) and `supported_version` (int, highest version this build \
     reads for that component). PyO3 cannot subclass a native exception under abi3, so the \
     attributes are set on the instance when it is raised."
);

/// Convert a core connectome error to the Python exception documented for this module.
///
/// `UnsupportedArtifactVersion` becomes `UnsupportedArtifactVersionError` with its
/// `component`, `artifact_version` and `supported_version` attributes set; every other error is
/// a plain `ValueError` with the same "Connectome validation failed: ..." message.
fn connectome_error_to_pyerr(error: ConnectomeError) -> PyErr {
    let message = format!("Connectome validation failed: {error}");
    match error {
        ConnectomeError::UnsupportedArtifactVersion {
            component,
            artifact_version,
            supported_version,
        } => Python::attach(|py| {
            let err = PyErr::new::<UnsupportedArtifactVersionError, _>(message);
            let instance = err.value(py);
            let attributes = (|| -> PyResult<()> {
                instance.setattr("component", component.as_str())?;
                instance.setattr("artifact_version", artifact_version)?;
                instance.setattr("supported_version", supported_version)?;
                Ok(())
            })();
            match attributes {
                Ok(()) => err,
                Err(attribute_error) => attribute_error,
            }
        }),
        _ => PyValueError::new_err(message),
    }
}

/// Validate and migrate a connectome artifact using FEAGI core.
///
/// Returns immutable migrated bytes and a JSON compatibility report. Invalid,
/// corrupt, future-version, or semantically inconsistent artifacts raise
/// `ValueError` and produce no output artifact. An artifact newer than this build
/// supports raises `UnsupportedArtifactVersionError` (a `ValueError` subclass).
#[pyfunction]
#[pyo3(name = "validate_and_migrate_connectome")]
pub fn py_validate_and_migrate_connectome(connectome_bytes: &[u8]) -> PyResult<(Vec<u8>, String)> {
    let result =
        feagi_services::brain_artifact::validate_and_migrate_brain_artifact(connectome_bytes)
            .map_err(connectome_error_to_pyerr)?;
    let report_json = serde_json::to_string(&result.report).map_err(|error| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to serialize connectome report: {error}"
        ))
    })?;
    Ok((result.artifact_bytes, report_json))
}

/// Highest artifact version this build reads, per component.
///
/// Returns a dict with int values under the keys `contract`, `container`,
/// `connectome_schema`, and `genome_schema`.
#[pyfunction]
#[pyo3(name = "supported_artifact_versions")]
pub fn py_supported_artifact_versions(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    supported_versions_to_dict(py, supported_brain_artifact_versions())
}

/// Build the Python dict for a set of supported artifact versions.
fn supported_versions_to_dict(
    py: Python<'_>,
    versions: SupportedArtifactVersions,
) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("contract", versions.contract)?;
    dict.set_item("container", versions.container)?;
    dict.set_item("connectome_schema", versions.connectome_schema)?;
    dict.set_item("genome_schema", versions.genome_schema)?;
    Ok(dict)
}

/// Register the authoritative connectome compatibility module with Python.
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let connectome_module = PyModule::new(py, "connectome")?;
    connectome_module.add_function(wrap_pyfunction!(
        py_validate_and_migrate_connectome,
        &connectome_module
    )?)?;
    connectome_module.add_function(wrap_pyfunction!(
        py_supported_artifact_versions,
        &connectome_module
    )?)?;
    connectome_module.add(
        "UnsupportedArtifactVersionError",
        py.get_type::<UnsupportedArtifactVersionError>(),
    )?;
    parent_module.add_submodule(&connectome_module)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use feagi_services::brain_artifact::ArtifactVersionComponent;
    use pyo3::types::PyAnyMethods;

    /// Run `body` with an initialized interpreter and the module registered on a scratch parent.
    fn with_module<R>(body: impl FnOnce(Python<'_>, &Bound<'_, PyModule>) -> R) -> R {
        Python::initialize();
        Python::attach(|py| {
            let parent = PyModule::new(py, "parent").expect("parent module");
            register_module(py, &parent).expect("register connectome");
            let module = parent.getattr("connectome").expect("connectome submodule");
            body(py, module.cast::<PyModule>().expect("module"))
        })
    }

    #[test]
    fn unsupported_version_raises_subclass_with_attributes() {
        with_module(|py, module| {
            let error = connectome_error_to_pyerr(ConnectomeError::UnsupportedArtifactVersion {
                component: ArtifactVersionComponent::GenomeSchema,
                artifact_version: 9,
                supported_version: 3,
            });
            let class = module
                .getattr("UnsupportedArtifactVersionError")
                .expect("class registered");
            assert!(error.matches(py, &class).expect("matches"));
            assert!(error.is_instance_of::<PyValueError>(py));
            let value = error.value(py);
            assert_eq!(
                value
                    .getattr("component")
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
                "genome_schema"
            );
            assert_eq!(
                value
                    .getattr("artifact_version")
                    .unwrap()
                    .extract::<u32>()
                    .unwrap(),
                9
            );
            assert_eq!(
                value
                    .getattr("supported_version")
                    .unwrap()
                    .extract::<u32>()
                    .unwrap(),
                3
            );
            let text = value.str().unwrap().to_string();
            assert!(text.starts_with("Connectome validation failed: "), "{text}");
            assert!(
                text.ends_with("Brain artifact genome_schema v9 is newer than supported v3"),
                "{text}"
            );
        });
    }

    #[test]
    fn every_component_maps_to_its_stable_name() {
        with_module(|py, _| {
            for (component, name) in [
                (ArtifactVersionComponent::Contract, "contract"),
                (ArtifactVersionComponent::Container, "container"),
                (
                    ArtifactVersionComponent::ConnectomeSchema,
                    "connectome_schema",
                ),
                (ArtifactVersionComponent::GenomeSchema, "genome_schema"),
            ] {
                let error =
                    connectome_error_to_pyerr(ConnectomeError::UnsupportedArtifactVersion {
                        component,
                        artifact_version: 2,
                        supported_version: 1,
                    });
                let got: String = error
                    .value(py)
                    .getattr("component")
                    .unwrap()
                    .extract()
                    .unwrap();
                assert_eq!(got, name);
            }
        });
    }

    #[test]
    fn other_errors_stay_plain_value_error_with_same_message() {
        with_module(|py, module| {
            let error = connectome_error_to_pyerr(ConnectomeError::ChecksumMismatch);
            let class = module.getattr("UnsupportedArtifactVersionError").unwrap();
            assert!(error.is_instance_of::<PyValueError>(py));
            assert!(!error.matches(py, &class).unwrap());
            assert!(error.get_type(py).is(py.get_type::<PyValueError>()));
            assert_eq!(
                error.value(py).str().unwrap().to_string(),
                "Connectome validation failed: Checksum mismatch: file may be corrupted"
            );
        });
    }

    #[test]
    fn invalid_bytes_raise_plain_value_error_through_the_pyfunction() {
        with_module(|py, _| {
            let error = py_validate_and_migrate_connectome(b"not a connectome")
                .expect_err("garbage must be rejected");
            assert!(error.get_type(py).is(py.get_type::<PyValueError>()));
            assert!(error
                .value(py)
                .str()
                .unwrap()
                .to_string()
                .starts_with("Connectome validation failed: "));
        });
    }

    #[test]
    fn supported_artifact_versions_matches_core() {
        with_module(|py, module| {
            let func = module.getattr("supported_artifact_versions").unwrap();
            let result = func.call0().unwrap();
            let dict = result.cast::<PyDict>().unwrap();
            let expected = supported_brain_artifact_versions();
            assert_eq!(dict.len(), 4);
            for (key, value) in [
                ("contract", expected.contract),
                ("container", expected.container),
                ("connectome_schema", expected.connectome_schema),
                ("genome_schema", expected.genome_schema),
            ] {
                let item = dict.get_item(key).unwrap().expect(key);
                assert!(item.is_instance_of::<pyo3::types::PyInt>());
                assert_eq!(item.extract::<u32>().unwrap(), value);
            }
            let _ = py;
        });
    }
}
