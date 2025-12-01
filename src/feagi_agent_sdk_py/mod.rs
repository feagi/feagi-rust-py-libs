/*
 * PyO3 bindings for feagi-agent-sdk
 * 
 * Exposes the Rust AgentClient to Python as PyAgentClient
 */

pub mod py_agent_client;
pub mod py_agent_config;
pub mod py_agent_type;

pub use py_agent_client::PyAgentClient;
pub use py_agent_config::PyAgentConfig;
pub use py_agent_type::PyAgentType as AgentType;

use pyo3::prelude::*;

/// Register the feagi_agent_sdk_py module with Python
pub fn register_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "feagi_agent_sdk_py")?;
    
    // Register types
    submodule.add_class::<PyAgentClient>()?;
    submodule.add_class::<PyAgentConfig>()?;
    submodule.add_class::<AgentType>()?;
    
    parent_module.add_submodule(submodule)?;
    Ok(())
}

