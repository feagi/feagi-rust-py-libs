/*
 * PyO3 wrapper for AgentConfig
 */

use pyo3::prelude::*;
use super::py_agent_type::PyAgentType;

#[pyclass(name = "PyAgentConfig")]
#[derive(Clone)]
pub struct PyAgentConfig {
    inner: feagi_agent_sdk::AgentConfig,
}

#[pymethods]
impl PyAgentConfig {
    #[new]
    fn new(agent_id: String, agent_type: PyAgentType) -> Self {
        let inner = feagi_agent_sdk::AgentConfig::new(agent_id, agent_type.inner());
        PyAgentConfig { inner }
    }
    
    /// Set FEAGI host and ports (required for all endpoints)
    fn with_feagi_endpoints(
        &mut self,
        host: String,
        registration_port: u16,
        sensory_port: u16,
        motor_port: u16,
        visualization_port: u16,
        control_port: u16,
    ) -> PyResult<()> {
        self.inner = self.inner.clone()
            .with_feagi_endpoints(host, registration_port, sensory_port, motor_port, visualization_port, control_port);
        Ok(())
    }
    
    /// Set registration endpoint
    fn with_registration_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_registration_endpoint(endpoint);
        Ok(())
    }
    
    /// Set sensory data endpoint
    fn with_sensory_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_sensory_endpoint(endpoint);
        Ok(())
    }
    
    /// Set motor data endpoint
    fn with_motor_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_motor_endpoint(endpoint);
        Ok(())
    }
    
    /// Set heartbeat interval in seconds (0 to disable)
    fn with_heartbeat_interval(&mut self, interval: f64) -> PyResult<()> {
        self.inner = self.inner.clone().with_heartbeat_interval(interval);
        Ok(())
    }
    
    /// Set connection timeout in milliseconds
    fn with_connection_timeout_ms(&mut self, timeout: u64) -> PyResult<()> {
        self.inner = self.inner.clone().with_connection_timeout_ms(timeout);
        Ok(())
    }
    
    /// Set number of registration retries
    fn with_registration_retries(&mut self, retries: u32) -> PyResult<()> {
        self.inner = self.inner.clone().with_registration_retries(retries);
        Ok(())
    }
    
    /// Set sensory socket configuration (high water mark, linger, immediate)
    fn with_sensory_socket_config(&mut self, hwm: i32, linger_ms: i32, immediate: bool) -> PyResult<()> {
        self.inner = self.inner.clone().with_sensory_socket_config(hwm, linger_ms, immediate);
        Ok(())
    }
    
    /// Add vision capability
    #[pyo3(signature = (modality, width, height, channels, cortical_area))]
    fn with_vision_capability(
        &mut self,
        modality: String,
        width: usize,
        height: usize,
        channels: usize,
        cortical_area: String,
    ) -> PyResult<()> {
        self.inner = self.inner.clone().with_vision_capability(
            modality,
            (width, height),
            channels,
            cortical_area,
        );
        Ok(())
    }
    
    /// Add motor capability
    #[pyo3(signature = (modality, output_count, cortical_areas))]
    fn with_motor_capability(
        &mut self,
        modality: String,
        output_count: usize,
        cortical_areas: Vec<String>,
    ) -> PyResult<()> {
        self.inner = self.inner.clone().with_motor_capability(modality, output_count, cortical_areas);
        Ok(())
    }
    
    /// Add custom capability (takes JSON string)
    fn with_custom_capability(&mut self, key: String, value_json: String) -> PyResult<()> {
        let value: serde_json::Value = serde_json::from_str(&value_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid JSON: {}", e)
            ))?;
        self.inner = self.inner.clone().with_custom_capability(key, value);
        Ok(())
    }
    
    /// Validate configuration
    fn validate(&self) -> PyResult<()> {
        self.inner.validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    
    fn __repr__(&self) -> String {
        format!("PyAgentConfig(agent_id={})", self.inner.agent_id)
    }
}

impl PyAgentConfig {
    pub fn inner(&self) -> &feagi_agent_sdk::AgentConfig {
        &self.inner
    }
    
    pub fn into_inner(self) -> feagi_agent_sdk::AgentConfig {
        self.inner
    }
}

