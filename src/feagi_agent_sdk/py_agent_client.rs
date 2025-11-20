/*
 * PyO3 wrapper for AgentClient
 */

use pyo3::prelude::*;
use pyo3::types::{PyList, PyAny};
use super::py_agent_config::PyAgentConfig;
use std::sync::{Arc, Mutex};

#[pyclass(name = "PyAgentClient")]
pub struct PyAgentClient {
    inner: Arc<Mutex<feagi_agent_sdk::AgentClient>>,
}

#[pymethods]
impl PyAgentClient {
    #[new]
    fn new(config: &PyAgentConfig) -> PyResult<Self> {
        let client = feagi_agent_sdk::AgentClient::new(config.inner().clone())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(PyAgentClient {
            inner: Arc::new(Mutex::new(client)),
        })
    }
    
    /// Connect and register with FEAGI
    fn connect(&mut self) -> PyResult<()> {
        let mut client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        client.connect()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
    
    /// Send sensory data as list of (neuron_id, potential) tuples
    fn send_sensory_data(&self, py: Python, neuron_pairs: Bound<'_, PyAny>) -> PyResult<()> {
        let list = neuron_pairs.downcast::<PyList>()?;
        let mut pairs: Vec<(i32, f64)> = Vec::new();
        
        for item in list {
            let tuple = item.downcast::<pyo3::types::PyTuple>()?;
            if tuple.len() != 2 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Each item must be a (neuron_id, potential) tuple"
                ));
            }
            let neuron_id: i32 = tuple.get_item(0)?.extract()?;
            let potential: f64 = tuple.get_item(1)?.extract()?;
            pairs.push((neuron_id, potential));
        }
        
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        // Send data (ZMQ sockets are not Sync, so we can't release GIL)
        client.send_sensory_data(pairs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
    
    /// Receive motor data (non-blocking, returns None if no data)
    /// Returns motor data as JSON string
    fn receive_motor_data(&self, py: Python) -> PyResult<Option<String>> {
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        // Receive data (ZMQ sockets are not Sync, so we can't release GIL)
        match client.receive_motor_data() {
            Ok(Some(data)) => {
                // Convert to a simple format for Python (for now, just return debug format)
                // TODO: Implement proper serialization for CorticalMappedXYZPNeuronVoxels
                Ok(Some(format!("{:?}", data)))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    /// Check if agent is registered
    fn is_registered(&self) -> PyResult<bool> {
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        Ok(client.is_registered())
    }
    
    /// Note: Disconnect happens automatically on drop, no explicit disconnect needed
    /// This method is kept for compatibility but is a no-op
    fn disconnect(&mut self) -> PyResult<()> {
        // Agent automatically deregisters on drop
        Ok(())
    }
    
    fn __repr__(&self) -> String {
        let registered = self.is_registered().unwrap_or(false);
        format!("PyAgentClient(registered={})", registered)
    }
}

