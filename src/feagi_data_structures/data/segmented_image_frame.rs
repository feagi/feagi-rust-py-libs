use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::data::{ImageFrame, SegmentedImageFrame};
use feagi_data_structures::genomic::descriptors::CorticalGroupIndex;
use crate::feagi_data_structures::data::image_descriptors::*;
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::feagi_data_structures::genomic::descriptors::PyCorticalGroupIndex;
use crate::feagi_data_structures::genomic::{PyCorticalID, PyCorticalType};

#[pyclass]
#[pyo3(name = "SegmentedImageFrame")]
#[derive(Clone)]
pub struct PySegmentedImageFrame{
    pub(crate) inner: SegmentedImageFrame,
}

#[pymethods]
impl PySegmentedImageFrame {

    //region Common Constructors
    #[new]
    pub fn new(
        segment_resolutions: PySegmentedXYImageResolutions,
        segment_color_space: PyColorSpace,
        center_color_channels: PyColorChannelLayout,
        peripheral_color_channels: PyColorChannelLayout,
    ) -> PyResult<Self> {
        match SegmentedImageFrame::new(
            &segment_resolutions.into(),
            &segment_color_space.into(),
            &center_color_channels.into(),
            &peripheral_color_channels.into()
        ) {
            Ok(inner) => Ok(PySegmentedImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_segmented_image_frame_properties(properties: PySegmentedImageFrameProperties) -> PyResult<Self> {
        let result = SegmentedImageFrame::from_segmented_image_frame_properties(&properties.into());
        match result {
            Ok(segmented) => Ok(PySegmentedImageFrame {inner: segmented}),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    //endregion

    //region Static Methods

    #[staticmethod]
    pub fn create_ordered_cortical_ids_for_segmented_vision(camera_index: PyCorticalGroupIndex) -> PyResult<[PyCorticalID; 9]> {
        let camera_index: CorticalGroupIndex = camera_index.into();
        let ids = SegmentedImageFrame::create_ordered_cortical_ids_for_segmented_vision(camera_index);
        Ok([
            ids[0].into(),
            ids[1].into(),
            ids[2].into(),
            ids[3].into(),
            ids[4].into(),
            ids[5].into(),
            ids[6].into(),
            ids[7].into(),
            ids[8].into(),
        ])
    }

    #[staticmethod]
    pub fn create_ordered_cortical_types_for_segmented_vision() -> PyResult<[PyCorticalType; 9]> {
        let cortical_types = SegmentedImageFrame::create_ordered_cortical_types_for_segmented_vision();
        Ok([
            cortical_types[0].into(),
            cortical_types[1].into(),
            cortical_types[2].into(),
            cortical_types[3].into(),
            cortical_types[4].into(),
            cortical_types[5].into(),
            cortical_types[6].into(),
            cortical_types[7].into(),
            cortical_types[8].into(),
        ])
    }

    //endregion

    //region Get Properties

    pub fn get_segmented_image_frame_properties(&self) -> PySegmentedImageFrameProperties {
        self.inner.get_segmented_image_frame_properties().into()
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    #[getter]
    pub fn center_channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_center_channel_layout().into()
    }

    #[getter]
    pub fn peripheral_channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_peripheral_channel_layout().into()
    }
    #[getter]
    pub fn segmented_frame_target_resolutions(&self) -> PySegmentedXYImageResolutions {
        self.inner.get_segmented_frame_target_resolutions().into()
    }
    //endregion


    //region Neuron Export
    pub fn export_as_new_cortical_mapped_neuron_data<'py>(&mut self, py: Python<'py>, camera_index: u8) -> PyResult<PyObject> {

        /*
        match self.inner.export_as_new_cortical_mapped_neuron_data(camera_index) {
            Ok(neuron_data) => {
                let child = PyCorticalMappedXYZPNeuronData { inner: neuron_data };
                let parent = crate::byte_structures::PyFeagiByteStructureCompatible::new();
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj.into())
            },
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
        
         */
        Err(PyErr::new::<PyValueError, _>("Camera does not support neuron data")) // TODO
    }

    // NOTE: inplace_export_cortical_mapped_neuron_data is not exposed to python since inplace operations make no sense

    //endregion
}