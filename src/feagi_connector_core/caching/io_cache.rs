use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_connector_core::IOCache;
use feagi_data_structures::genomic::descriptors::{CorticalChannelCount, CorticalGroupIndex};
use crate::feagi_connector_core::data::descriptors::{PyGazeProperties, PyImageFrameProperties, PySegmentedImageFrameProperties};
use crate::feagi_data_structures::genomic::descriptors::{PyCorticalChannelCount, PyCorticalGroupIndex};
use crate::py_error::PyFeagiError;

#[pyclass(str)]
#[pyo3(name = "IOCache")]
#[derive()]
pub struct PyIOCache {
    inner: IOCache
}

#[pymethods]
impl PyIOCache {

    #[new]
    pub fn new() -> Self {
        PyIOCache {
            inner: IOCache::new()
        }
    }


    //region Sensors

    pub fn sensor_register_segmented_vision_absolute(&mut self, py: Python<'_>, group: PyObject,
                                                     number_of_channels: PyObject, input_image_properties: PyImageFrameProperties,
                                                     output_segment_properties: PySegmentedImageFrameProperties, gaze: PyGazeProperties) -> PyResult<()> {

        let cortical_group_index: CorticalGroupIndex = PyCorticalGroupIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;

        self.inner.sensor_register_segmented_vision_absolute(cortical_group_index,
                                                             number_of_channels, input_image_properties.into(),
                                                             output_segment_properties.into(), gaze.into()).map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn sensor_write_segmented_vision_absolute(&mut self, py: Python<'_>, group: PyObject, )


    //endregion


}