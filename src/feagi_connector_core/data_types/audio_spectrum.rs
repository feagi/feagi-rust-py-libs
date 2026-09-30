use crate::py_error::PyFeagiError;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::{AudioSpectrumFrame, AudioSpectrumProperties};
use pyo3::prelude::*;

create_pyclass!(
    PyAudioSpectrumProperties,
    AudioSpectrumProperties,
    "AudioSpectrumProperties"
);

#[pymethods]
impl PyAudioSpectrumProperties {
    #[staticmethod]
    pub fn new_linear(
        sample_rate_hz: u32,
        window_size: u32,
        hop_size: u32,
        phase_steps: u32,
        magnitude_floor_db: i32,
        magnitude_ceiling_db: i32,
    ) -> PyResult<Self> {
        let inner = AudioSpectrumProperties::new_linear(
            sample_rate_hz,
            window_size,
            hop_size,
            phase_steps,
            magnitude_floor_db,
            magnitude_ceiling_db,
        )
        .map_err(PyFeagiError::from)?;
        Ok(PyAudioSpectrumProperties { inner })
    }

    #[staticmethod]
    pub fn new_logarithmic(
        sample_rate_hz: u32,
        window_size: u32,
        hop_size: u32,
        bin_count: u32,
        phase_steps: u32,
        min_frequency_hz: u32,
        max_frequency_hz: u32,
        magnitude_floor_db: i32,
        magnitude_ceiling_db: i32,
    ) -> PyResult<Self> {
        let inner = AudioSpectrumProperties::new_logarithmic(
            sample_rate_hz,
            window_size,
            hop_size,
            bin_count,
            phase_steps,
            min_frequency_hz,
            max_frequency_hz,
            magnitude_floor_db,
            magnitude_ceiling_db,
        )
        .map_err(PyFeagiError::from)?;
        Ok(PyAudioSpectrumProperties { inner })
    }
    #[getter]
    pub fn bin_count(&self) -> u32 {
        self.inner.bin_count
    }

    #[getter]
    pub fn phase_steps(&self) -> u32 {
        self.inner.phase_steps
    }

    #[getter]
    pub fn sample_rate_hz(&self) -> u32 {
        self.inner.sample_rate_hz
    }

    #[getter]
    pub fn window_size(&self) -> u32 {
        self.inner.window_size
    }

    #[getter]
    pub fn hop_size(&self) -> u32 {
        self.inner.hop_size
    }
}

create_pyclass!(
    PyAudioSpectrumFrame,
    AudioSpectrumFrame,
    "AudioSpectrumFrame"
);

#[pymethods]
impl PyAudioSpectrumFrame {
    #[new]
    pub fn new(properties: PyAudioSpectrumProperties) -> PyResult<Self> {
        let inner = AudioSpectrumFrame::new(&properties.inner).map_err(PyFeagiError::from)?;
        Ok(PyAudioSpectrumFrame { inner })
    }

    pub fn get_properties(&self) -> PyAudioSpectrumProperties {
        (*self.inner.get_properties()).into()
    }

    pub fn get_magnitudes(&self) -> Vec<f32> {
        self.inner.get_magnitudes().to_vec()
    }

    pub fn get_phase_steps(&self) -> Vec<u32> {
        self.inner.get_phase_steps().to_vec()
    }

    pub fn set_column(
        &mut self,
        column: usize,
        magnitude_potential: f32,
        phase_step: u32,
    ) -> PyResult<()> {
        self.inner
            .set_column(column, magnitude_potential, phase_step)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    pub fn silence(&mut self) {
        self.inner.silence();
    }
}
