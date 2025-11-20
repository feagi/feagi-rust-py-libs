/*
 * PyO3 wrapper for AgentType enum
 */

use pyo3::prelude::*;

#[pyclass(name = "AgentType")]
#[derive(Clone, Debug)]
pub struct PyAgentType {
    inner: feagi_agent_sdk::AgentType,
}

#[pymethods]
impl PyAgentType {
    #[new]
    fn new(agent_type: &str) -> PyResult<Self> {
        let inner = match agent_type.to_lowercase().as_str() {
            "sensory" => feagi_agent_sdk::AgentType::Sensory,
            "motor" => feagi_agent_sdk::AgentType::Motor,
            "both" => feagi_agent_sdk::AgentType::Both,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid agent type: {}. Must be 'sensory', 'motor', or 'both'", agent_type)
            )),
        };
        Ok(PyAgentType { inner })
    }
    
    #[classattr]
    const SENSORY: &'static str = "sensory";
    
    #[classattr]
    const MOTOR: &'static str = "motor";
    
    #[classattr]
    const BOTH: &'static str = "both";
    
    /// Create a Sensory agent type
    #[staticmethod]
    fn sensory() -> Self {
        PyAgentType { inner: feagi_agent_sdk::AgentType::Sensory }
    }
    
    /// Create a Motor agent type
    #[staticmethod]
    fn motor() -> Self {
        PyAgentType { inner: feagi_agent_sdk::AgentType::Motor }
    }
    
    /// Create a Both (bidirectional) agent type
    #[staticmethod]
    fn both() -> Self {
        PyAgentType { inner: feagi_agent_sdk::AgentType::Both }
    }
    
    fn __repr__(&self) -> String {
        format!("AgentType({})", self.inner.to_string())
    }
    
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

impl PyAgentType {
    pub fn inner(&self) -> feagi_agent_sdk::AgentType {
        self.inner.clone()
    }
}

// Allow creating from Rust AgentType
impl From<feagi_agent_sdk::AgentType> for PyAgentType {
    fn from(inner: feagi_agent_sdk::AgentType) -> Self {
        PyAgentType { inner }
    }
}

