use crate::py_error::PyFeagiError;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_data_structures::FeagiDataError;
use feagi_sensorimotor::data_types::{MidiControl, MidiFrame, MidiProperties};
use pyo3::prelude::*;

create_pyclass!(PyMidiProperties, MidiProperties, "MidiProperties");

#[pymethods]
impl PyMidiProperties {
    /// Note columns are the Notes area width (1 to 128). Expression depth is the
    /// Z resolution of each control level (1 to 1024).
    #[staticmethod]
    pub fn new(note_columns: u32, expression_depth: u32) -> PyResult<Self> {
        let inner =
            MidiProperties::new(note_columns, expression_depth).map_err(PyFeagiError::from)?;
        Ok(PyMidiProperties { inner })
    }

    #[getter]
    pub fn note_columns(&self) -> u32 {
        self.inner.note_columns
    }

    #[getter]
    pub fn expression_depth(&self) -> u32 {
        self.inner.expression_depth
    }
}

create_pyclass!(PyMidiFrame, MidiFrame, "MidiFrame");

#[pymethods]
impl PyMidiFrame {
    #[new]
    pub fn new(properties: PyMidiProperties) -> PyResult<Self> {
        let inner = MidiFrame::new(&properties.inner).map_err(PyFeagiError::from)?;
        Ok(PyMidiFrame { inner })
    }

    pub fn get_properties(&self) -> PyMidiProperties {
        (*self.inner.get_properties()).into()
    }

    /// Strength at a note column on the 0..=1 scale. 0 means not sounding.
    pub fn note_strength(&self, column: usize) -> f32 {
        self.inner.notes.strength(column)
    }

    /// Strengths from the lowest note column to the highest.
    pub fn note_strengths(&self) -> Vec<f32> {
        self.inner.notes.strengths().to_vec()
    }

    /// Raise a note's strength from a membrane potential on FEAGI's 0..=255 scale.
    /// The stored strength is potential / 255, clamped to 0..=1, and a lower
    /// stored strength is left unchanged.
    pub fn raise_note_strength(&mut self, column: usize, potential: f32) -> PyResult<()> {
        if column >= self.inner.notes.note_columns() {
            return Err(PyFeagiError::from(FeagiDataError::BadParameters(format!(
                "MIDI note column {column} is outside 0..{}",
                self.inner.notes.note_columns()
            )))
            .into());
        }
        self.inner.notes.raise_strength(column, potential);
        Ok(())
    }

    /// Level of an Expression control, or None when that control is not driven.
    /// Columns follow MidiControl: 0 pitch bend up, 1 pitch bend down, 2 modulation,
    /// 3 sustain, 4 volume.
    pub fn expression_level(&self, column: usize) -> PyResult<Option<f32>> {
        let control = midi_control(column)?;
        Ok(self.inner.expression.level(control))
    }

    /// Mark an Expression control as driven at `level` (0..=1).
    pub fn set_expression_level(&mut self, column: usize, level: f32) -> PyResult<()> {
        let control = midi_control(column)?;
        self.inner.expression.set_level(control, level);
        Ok(())
    }

    pub fn silence(&mut self) {
        self.inner.silence();
    }
}

fn midi_control(column: usize) -> Result<MidiControl, PyErr> {
    MidiControl::from_column(column).ok_or_else(|| {
        PyFeagiError::from(FeagiDataError::BadParameters(format!(
            "MIDI expression column {column} is outside 0..{}",
            MidiControl::COUNT
        )))
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_sets_a_note_and_an_expression_control() {
        let properties = PyMidiProperties::new(128, 10).unwrap();
        let mut frame = PyMidiFrame::new(properties).unwrap();
        frame.raise_note_strength(60, 255.0).unwrap();
        frame.set_expression_level(2, 0.5).unwrap();
        assert_eq!(frame.note_strength(60), 1.0);
        assert_eq!(frame.expression_level(2).unwrap(), Some(0.5));
        assert!(frame.expression_level(0).unwrap().is_none());
        frame.silence();
        assert_eq!(frame.note_strength(60), 0.0);
        assert!(frame.expression_level(2).unwrap().is_none());
        assert!(frame.raise_note_strength(128, 1.0).is_err());
        assert!(frame.set_expression_level(5, 0.5).is_err());
    }
}
