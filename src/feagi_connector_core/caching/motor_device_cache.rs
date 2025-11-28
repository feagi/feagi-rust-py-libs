use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use feagi_connector_core::motor_device_cache::MotorDeviceCache;
use feagi_data_structures::genomic::MotorCorticalUnit;
use feagi_data_structures::genomic::cortical_area::descriptors::{CorticalChannelCount, CorticalChannelIndex, CorticalGroupIndex, NeuronDepth};
use feagi_data_structures::genomic::cortical_area::io_cortical_area_data_type::{FrameChangeHandling, PercentageNeuronPositioning};
use feagi_data_structures::FeagiSignalIndex;
use feagi_connector_core::wrapped_io_data::WrappedIOData;
use crate::py_error::PyFeagiError;
use crate::feagi_data_structures::genomic::cortical_type::PyMotorCorticalType;

/// Python wrapper for MotorDeviceCache
/// Provides a clean, generic API for registering motors and receiving callbacks
/// 
/// Note: Wrapped in Arc<Mutex<>> because MotorDeviceCache contains FnMut callbacks
/// which are !Sync. This is safe because Python's GIL ensures single-threaded access.
#[pyclass(name = "MotorDeviceCache")]
pub struct PyMotorDeviceCache {
    /// Inner cache wrapped in Arc<Mutex> for thread safety
    inner: Arc<Mutex<MotorDeviceCache>>,
    /// Store Python callbacks by signal index to prevent garbage collection
    callbacks: Arc<Mutex<HashMap<u64, Arc<PyObject>>>>,
}

#[pymethods]
impl PyMotorDeviceCache {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MotorDeviceCache::new())),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a motor device
    /// 
    /// # Arguments
    /// * `motor_unit` - The motor type (e.g., MotorCorticalType.RotaryMotor)
    /// * `group` - Cortical group index (0-255)
    /// * `channels` - Number of motor channels
    /// * `z_resolution` - Neuron depth resolution (typically 100)
    /// * `frame_change_handling` - How to handle frame changes (0=Absolute, 1=Incremental, default: 0)
    /// * `percentage_positioning` - How neurons map to percentages (0=Linear, 1=Fractional, default: 0)
    /// 
    /// # Example
    /// ```python
    /// from feagi_rust_py_libs.connector_core.caching import MotorDeviceCache
    /// from feagi_rust_py_libs.data_structures.genomic import MotorCorticalType
    /// 
    /// cache = MotorDeviceCache()
    /// cache.register(
    ///     motor_unit=MotorCorticalType.RotaryMotor,
    ///     group=0,
    ///     channels=2,
    ///     z_resolution=100
    /// )
    /// ```
    pub fn register(
        &mut self,
        motor_unit: PyMotorCorticalType,
        group: u8,
        channels: u32,
        z_resolution: u32,
        frame_change_handling: Option<u8>,  // 0=Cumulative, 1=Replace
        percentage_positioning: Option<u8>, // 0=Linear, 1=Fractional
    ) -> PyResult<()> {
        let motor_type: MotorCorticalUnit = motor_unit.into();
        let group_idx = CorticalGroupIndex::from(group);
        let channel_count = CorticalChannelCount::new(channels).map_err(PyFeagiError::from)?;
        let depth = NeuronDepth::new(z_resolution).map_err(PyFeagiError::from)?;
        
        let frame_handling = match frame_change_handling.unwrap_or(0) {
            0 => FrameChangeHandling::Absolute,
            1 => FrameChangeHandling::Incremental,
            _ => return Err(PyFeagiError::from_string("Invalid frame_change_handling value").into()),
        };
        
        let positioning = match percentage_positioning.unwrap_or(0) {
            0 => PercentageNeuronPositioning::Linear,
            1 => PercentageNeuronPositioning::Fractional,
            _ => return Err(PyFeagiError::from_string("Invalid percentage_positioning value").into()),
        };
        
        // Lock the inner cache
        let mut cache = self.inner.lock().unwrap();
        
        // Call the appropriate registration method based on motor type
        match motor_type {
            MotorCorticalUnit::RotaryMotor => {
                cache.rotary_motor_register(
                    group_idx,
                    channel_count,
                    frame_handling,
                    depth,
                    positioning,
                ).map_err(PyFeagiError::from)?;
            }
            MotorCorticalUnit::PositionalServo => {
                cache.positional_servo_register(
                    group_idx,
                    channel_count,
                    frame_handling,
                    depth,
                    positioning,
                ).map_err(PyFeagiError::from)?;
            }
            MotorCorticalUnit::Shock => {
                cache.gaze_control_register(
                    group_idx,
                    channel_count,
                    frame_handling,
                    depth,
                    positioning,
                ).map_err(PyFeagiError::from)?;
            }
            MotorCorticalUnit::MiscData => {
                // TODO: MiscData requires MiscDataDimensions parameter
                return Err(PyFeagiError::from_string(
                    "MiscData motor type not yet supported in minimal Python bindings"
                ).into());
            }
            _ => {
                return Err(PyFeagiError::from_string(&format!(
                    "Motor type {:?} not yet supported in Python bindings",
                    motor_type
                )).into());
            }
        }
        
        Ok(())
    }

    /// Register a callback to be notified when FEAGI sends motor commands
    /// 
    /// # Arguments
    /// * `motor_unit` - The motor type
    /// * `group` - Cortical group index
    /// * `channel` - Motor channel index
    /// * `callback` - Python function to call when motor data arrives
    ///                Signature: callback(value: float) -> None
    /// 
    /// # Returns
    /// Signal index that can be used to unregister the callback
    /// 
    /// # Example
    /// ```python
    /// def on_left_wheel(value):
    ///     print(f"Left wheel: {value}%")
    ///     robot.set_left_speed(value)
    /// 
    /// signal_id = cache.register_callback(
    ///     motor_unit=MotorCorticalType.RotaryMotor,
    ///     group=0,
    ///     channel=0,
    ///     callback=on_left_wheel
    /// )
    /// ```
    pub fn register_callback(
        &mut self,
        py: Python,
        motor_unit: PyMotorCorticalType,
        group: u8,
        channel: u32,
        callback: PyObject,
    ) -> PyResult<u32> {
        let motor_type: MotorCorticalUnit = motor_unit.into();
        let group_idx = CorticalGroupIndex::from(group);
        let channel_idx = CorticalChannelIndex::from(channel);
        
        // Wrap callback in Arc for thread-safe sharing
        let callback_arc = Arc::new(callback.clone_ref(py));
        let callback_for_closure = Arc::clone(&callback_arc);
        
        // Create Rust closure that calls Python callback
        let rust_callback = move |data: &WrappedIOData| {
            // Acquire GIL to call Python
            Python::with_gil(|py| {
                // Convert WrappedIOData to Python-friendly format
                let py_value = match data {
                    WrappedIOData::Percentage(p) => p.as_f32(),
                    WrappedIOData::SignedPercentage(sp) => sp.as_f32(),
                    WrappedIOData::Percentage_3D(p3d) => {
                        // For 3D percentages, pass a tuple (a, b, c)
                        let tuple = (p3d.a.as_f32(), p3d.b.as_f32(), p3d.c.as_f32());
                        if let Err(e) = callback_for_closure.call1(py, (tuple,)) {
                            eprintln!("Error calling Python motor callback: {}", e);
                        }
                        return;
                    }
                    _ => {
                        eprintln!("Unsupported WrappedIOData type in motor callback");
                        return;
                    }
                };
                
                // Call Python callback with value
                if let Err(e) = callback_for_closure.call1(py, (py_value,)) {
                    eprintln!("Error calling Python motor callback: {}", e);
                }
            });
        };
        
        // Lock the inner cache and register callback
        let mut cache = self.inner.lock().unwrap();
        let signal_index = match motor_type {
            MotorCorticalUnit::RotaryMotor => {
                cache.motor_rotary_motor_try_register_motor_callback(
                    group_idx,
                    channel_idx,
                    rust_callback,
                ).map_err(PyFeagiError::from)?
            }
            MotorCorticalUnit::PositionalServo => {
                cache.motor_positional_servo_try_register_motor_callback(
                    group_idx,
                    channel_idx,
                    rust_callback,
                ).map_err(PyFeagiError::from)?
            }
            MotorCorticalUnit::Shock => {
                cache.motor_gaze_control_try_register_motor_callback(
                    group_idx,
                    channel_idx,
                    rust_callback,
                ).map_err(PyFeagiError::from)?
            }
            MotorCorticalUnit::MiscData => {
                cache.motor_miscellaneous_try_register_motor_callback(
                    group_idx,
                    channel_idx,
                    rust_callback,
                ).map_err(PyFeagiError::from)?
            }
            _ => {
                return Err(PyFeagiError::from_string(&format!(
                    "Motor type {:?} callback not yet supported",
                    motor_type
                )).into());
            }
        };
        
        // Store callback reference to prevent garbage collection
        let signal_id: u32 = signal_index.into();
        self.callbacks.lock().unwrap().insert(signal_id as u64, callback_arc);
        
        Ok(signal_id)
    }

    /// Process neuron data from FEAGI and trigger callbacks
    /// 
    /// This method decodes neuron voxels into motor values and triggers
    /// all registered callbacks for affected channels.
    /// 
    /// # Arguments
    /// * `neuron_data` - Raw neuron voxel bytes from FEAGI
    /// 
    /// # Example
    /// ```python
    /// # Typically called by SDK when ZMQ receives motor data
    /// cache.process_neurons(neuron_bytes)
    /// ```
    pub fn process_neurons(&mut self, neuron_data: Vec<u8>) -> PyResult<()> {
        // TODO: Implement neuron decoding
        // For now, this is a placeholder - the actual decoding logic needs to be
        // exposed from MotorDeviceCache in feagi-connector-core
        Ok(())
    }

    /// Get the most recent motor value for a channel (after callbacks have been triggered)
    /// 
    /// # Arguments
    /// * `motor_unit` - The motor type
    /// * `group` - Cortical group index
    /// * `channel` - Motor channel index
    /// 
    /// # Returns
    /// The processed motor value as a float (percentage)
    pub fn get_value(
        &mut self,
        motor_unit: PyMotorCorticalType,
        group: u8,
        channel: u32,
    ) -> PyResult<f32> {
        let motor_type: MotorCorticalUnit = motor_unit.into();
        let group_idx = CorticalGroupIndex::from(group);
        let channel_idx = CorticalChannelIndex::from(channel);
        
        // Lock the inner cache
        let cache = self.inner.lock().unwrap();
        
        // Use the generic read method which returns &WrappedIOData
        let wrapped = cache.try_read_postprocessed_cached_value(motor_type, group_idx, channel_idx)
            .map_err(PyFeagiError::from)?;
        
        // Convert to f32 based on the type
        let value = match wrapped {
            WrappedIOData::Percentage(p) => p.as_f32(),
            WrappedIOData::SignedPercentage(sp) => sp.as_f32(),
            WrappedIOData::Percentage_3D(p3d) => {
                // For 3D percentages, return the first component
                // TODO: Consider returning a tuple instead
                p3d.a.as_f32()
            }
            _ => {
                return Err(PyFeagiError::from_string(&format!(
                    "Unsupported WrappedIOData type: {:?}",
                    wrapped
                )).into());
            }
        };
        
        Ok(value)
    }
}

