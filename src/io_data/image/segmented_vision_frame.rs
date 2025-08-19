use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::image_descriptors::*;
use feagi_core_data_structures_and_processing::io_data::SegmentedImageFrame;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalID};
use crate::io_data::image::image_frame::PyImageFrame;
use crate::io_data::image::descriptors::*;
use crate::genomic_structures::{PyCorticalGroupingIndex, PyCorticalID, PyCorticalType};
use crate::neuron_data::xyzp::PyCorticalMappedXYZPNeuronData;


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
        segment_resolutions: &PySegmentedFrameTargetResolutions,
        segment_color_space: PyColorSpace,
        center_color_channels: PyColorChannelLayout,
        peripheral_color_channels: PyColorChannelLayout,
    ) -> PyResult<Self> {
        match SegmentedImageFrame::new(
            &segment_resolutions.inner,
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
    pub fn create_ordered_cortical_ids_for_segmented_vision(camera_index: PyCorticalGroupingIndex) -> PyResult<[PyCorticalID; 9]> {
        let camera_index: CorticalGroupingIndex = camera_index.into();
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
    
    
    
    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        match self.inner.get_color_space() {
            ColorSpace::Linear => PyColorSpace::Linear,
            ColorSpace::Gamma => PyColorSpace::Gamma,
        }
    }

    #[getter]
    pub fn center_channel_layout(&self) -> PyColorChannelLayout {
        match self.inner.get_center_channel_layout() {
            ColorChannelLayout::GrayScale => PyColorChannelLayout::GrayScale,
            ColorChannelLayout::RG => PyColorChannelLayout::RG,
            ColorChannelLayout::RGB => PyColorChannelLayout::RGB,
            ColorChannelLayout::RGBA => PyColorChannelLayout::RGBA,
        }
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

impl From<SegmentedImageFrame> for PySegmentedImageFrame {
    fn from(inner:SegmentedImageFrame) -> Self {
        PySegmentedImageFrame{inner}
    }
}

impl From<PySegmentedImageFrame> for SegmentedImageFrame {
    fn from(inner:PySegmentedImageFrame) -> Self {
        inner.inner
    }
}