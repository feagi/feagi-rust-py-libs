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
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize Rust tracing (call once from Python)
#[pyfunction]
fn init_rust_logging() {
    INIT.call_once(|| {
        use tracing_subscriber::{fmt, EnvFilter};
        
        // Default to INFO level if RUST_LOG not set
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        
        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .init();
    });
}

/// Register the feagi_agent_sdk module with Python
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "feagi_agent_sdk")?;
    
    // Register types
    submodule.add_class::<PyAgentClient>()?;
    submodule.add_class::<PyAgentConfig>()?;
    submodule.add_class::<AgentType>()?;
    
    // Register functions
    submodule.add_function(wrap_pyfunction!(init_rust_logging, &submodule)?)?;
    
    parent_module.add_submodule(&submodule)?;
    Ok(())
}

