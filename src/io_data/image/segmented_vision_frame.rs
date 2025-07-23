use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::io_data::descriptors::*;
use feagi_core_data_structures_and_processing::io_data::SegmentedImageFrame;
use feagi_core_data_structures_and_processing::genomic_structures::CorticalID;


use crate::io_data::image::image_frame::PyImageFrame;
use crate::io_data::image::descriptors::*;
use crate::genomic_structures::PyCorticalID;
use crate::neuron_data::xyzp::PyCorticalMappedXYZPNeuronData;


#[pyclass]
#[pyo3(name = "SegmentedImageFrame")]
pub struct PySegmentedImageFrame{
    inner: SegmentedImageFrame,
}

#[pymethods]
impl PySegmentedImageFrame {
    
    //region Common Constructors
    #[new]
    pub fn new(
        segment_resolutions: &PySegmentedVisionTargetResolutions,
        segment_color_channels: PyChannelFormat,
        segment_color_space: PyColorSpace,
        input_frames_source_width_height: (usize, usize)
    ) -> PyResult<Self> {
        match SegmentedImageFrame::new(
            &segment_resolutions.inner,
            &segment_color_channels.into(),
            &segment_color_space.into(),
            input_frames_source_width_height
        ) {
            Ok(inner) => Ok(PySegmentedImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn create_ordered_cortical_ids(camera_index: u8, is_grayscale: bool) -> PyResult<Vec<PyCorticalID>> {
        /*
        match CorticalID::create_ordered_cortical_areas_for_segmented_vision(camera_index, is_grayscale) {
            Ok(cortical_ids) => {
                let py_cortical_ids: Vec<PyCorticalID> = cortical_ids
                    .into_iter()
                    .map(|id| PyCorticalID { inner: id })
                    .collect();
                Ok(py_cortical_ids)
            },
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
        
         */
        
        
         Err(PyErr::new::<PyValueError, _>("Camera does not support neuron data")) // TODO
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
    pub fn color_channels(&self) -> PyChannelFormat {
        match self.inner.get_color_channels() {
            ChannelFormat::GrayScale => PyChannelFormat::GrayScale,
            ChannelFormat::RG => PyChannelFormat::RG,
            ChannelFormat::RGB => PyChannelFormat::RGB,
            ChannelFormat::RGBA => PyChannelFormat::RGBA,
        }
    }
    //endregion
    
    //region Loading in New Data
    pub fn update_segments(
        &mut self,
        source_frame: &PyImageFrame,
        center_properties: &PySegmentedFrameCenterProperties
    ) -> PyResult<()> {
        match self.inner.update_segments(&source_frame.inner, center_properties.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
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