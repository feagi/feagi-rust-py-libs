use crate::py_error::PyFeagiError;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::descriptors::PoseEstimationProperties;
use feagi_sensorimotor::data_types::{JointPosition, PoseEstimationData};
use pyo3::prelude::*;

create_pyclass!(
    PyPoseEstimationProperties,
    PoseEstimationProperties,
    "PoseEstimationProperties"
);

#[pymethods]
impl PyPoseEstimationProperties {
    #[new]
    pub fn new(width: u32, height: u32, depth: u32) -> PyResult<Self> {
        let inner =
            PoseEstimationProperties::new(width, height, depth).map_err(PyFeagiError::from)?;
        Ok(PyPoseEstimationProperties { inner })
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.height
    }

    #[getter]
    pub fn depth(&self) -> u32 {
        self.inner.depth
    }
}

create_pyclass!(
    PyPoseEstimationData,
    PoseEstimationData,
    "PoseEstimationData"
);

#[pymethods]
impl PyPoseEstimationData {
    #[new]
    pub fn new(properties: PyPoseEstimationProperties) -> PyResult<Self> {
        let inner = PoseEstimationData::new(&properties.inner).map_err(PyFeagiError::from)?;
        Ok(PyPoseEstimationData { inner })
    }

    pub fn get_properties(&self) -> PyPoseEstimationProperties {
        (*self.inner.get_properties()).into()
    }

    /// Joints as `(x, y, confidence)` in [0, 1], or `None` when that joint was not detected.
    pub fn get_joints(&self) -> Vec<Option<(f32, f32, f32)>> {
        self.inner
            .get_joints()
            .iter()
            .map(|joint| joint.map(|position| (position.x, position.y, position.confidence)))
            .collect()
    }

    pub fn set_joint(
        &mut self,
        joint_index: usize,
        position: Option<(f32, f32, f32)>,
    ) -> PyResult<()> {
        let joint = position.map(|(x, y, confidence)| JointPosition { x, y, confidence });
        self.inner.set_joint(joint_index, joint);
        Ok(())
    }

    pub fn clear_all_joints(&mut self) {
        self.inner.clear_all_joints();
    }

    pub fn detected_joint_count(&self) -> usize {
        self.inner.detected_joint_count()
    }
}
