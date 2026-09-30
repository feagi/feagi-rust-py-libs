use crate::feagi_connector_core::data_types::{PyAudioSpectrumFrame, PyAudioSpectrumProperties};
use crate::py_error::PyFeagiError;
use feagi_sensorimotor::data_types::processing::{AudioSpectrumAnalyzer, AudioSpectrumSynthesizer};
use pyo3::prelude::*;

#[pyclass(name = "AudioSpectrumAnalyzer")]
pub struct PyAudioSpectrumAnalyzer {
    inner: AudioSpectrumAnalyzer,
}

#[pymethods]
impl PyAudioSpectrumAnalyzer {
    #[new]
    pub fn new(properties: PyAudioSpectrumProperties) -> PyResult<Self> {
        let inner = AudioSpectrumAnalyzer::new(properties.inner).map_err(PyFeagiError::from)?;
        Ok(PyAudioSpectrumAnalyzer { inner })
    }

    pub fn get_properties(&self) -> PyAudioSpectrumProperties {
        (*self.inner.get_properties()).into()
    }

    /// Feed mono PCM in [-1, 1]. Returns one spectrum frame per completed hop.
    pub fn push_samples(&mut self, samples: Vec<f32>) -> PyResult<Vec<PyAudioSpectrumFrame>> {
        let frames = self
            .inner
            .push_samples(&samples)
            .map_err(PyFeagiError::from)?;
        Ok(frames.into_iter().map(PyAudioSpectrumFrame::from).collect())
    }
}

#[pyclass(name = "AudioSpectrumSynthesizer")]
pub struct PyAudioSpectrumSynthesizer {
    inner: AudioSpectrumSynthesizer,
}

#[pymethods]
impl PyAudioSpectrumSynthesizer {
    #[new]
    pub fn new(properties: PyAudioSpectrumProperties) -> PyResult<Self> {
        let inner = AudioSpectrumSynthesizer::new(properties.inner).map_err(PyFeagiError::from)?;
        Ok(PyAudioSpectrumSynthesizer { inner })
    }

    pub fn get_properties(&self) -> PyAudioSpectrumProperties {
        (*self.inner.get_properties()).into()
    }

    pub fn latency_samples(&self) -> usize {
        self.inner.latency_samples()
    }

    /// Consume one spectrum frame and return `hop_size` PCM samples.
    pub fn push_frame(&mut self, frame: &PyAudioSpectrumFrame) -> PyResult<Vec<f32>> {
        Ok(self
            .inner
            .push_frame(&frame.inner)
            .map_err(PyFeagiError::from)?)
    }
}
