/*
 * Copyright 2025 Neuraville Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! ZMQ API Client for Python
//!
//! Provides a simple DEALER socket client for the API process to communicate
//! with the FEAGI core via ZMQ IPC (eliminates pyzmq dependency).

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;
use std::sync::Mutex;

#[pyclass]
pub struct ZmqApiClient {
    context: zmq::Context,
    socket: Mutex<Option<zmq::Socket>>,
    address: String,
}

#[pymethods]
impl ZmqApiClient {
    #[new]
    fn new(address: String) -> PyResult<Self> {
        let context = zmq::Context::new();
        Ok(Self {
            context,
            socket: Mutex::new(None),
            address,
        })
    }

    /// Connect to FEAGI API control stream
    fn connect(&self) -> PyResult<()> {
        let socket = self.context.socket(zmq::DEALER)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create socket: {}", e)))?;
        
        // Use unique identity for each client (avoids ZMQ ROUTER conflicts)
        // Each instance gets a random ID to prevent identity collisions
        use std::sync::atomic::{AtomicU64, Ordering};
        static INSTANCE_COUNTER: AtomicU64 = AtomicU64::new(0);
        let instance_id = INSTANCE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let identity = format!("api-process-{}-{}", std::process::id(), instance_id);
        
        // Log identity for debugging
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/nadji/code/FEAGI-2.0/feagi-py/tmp/zmq_api_client.log") {
            let _ = writeln!(f, "🦀 [ZMQ-API-CLIENT] Connecting with identity: {}", identity);
            let _ = f.flush();
        }
        
        socket.set_identity(identity.as_bytes())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set identity: {}", e)))?;
        
        socket.set_linger(1000)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set linger: {}", e)))?;
        
        socket.set_rcvtimeo(5000)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set rcvtimeo: {}", e)))?;
        
        socket.set_sndtimeo(5000)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set sndtimeo: {}", e)))?;
        
        socket.connect(&self.address)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to connect: {}", e)))?;
        
        // CRITICAL: ZMQ connect() is async - give it time to establish before use
        // Without this, first messages can be silently dropped!
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        *self.socket.lock().unwrap() = Some(socket);
        Ok(())
    }

    /// Send request and receive response
    /// 
    /// Args:
    ///     method: HTTP method (e.g., "GET", "POST")
    ///     path: API path (e.g., "/v1/health")
    ///     body: Optional JSON string body for POST/PUT requests
    /// 
    /// Returns:
    ///     Dictionary with 'status' (int) and 'body' (dict)
    #[pyo3(signature = (method, path, body=None))]
    fn request(&self, py: Python<'_>, method: String, path: String, body: Option<String>) -> PyResult<Py<PyDict>> {
        let socket_guard = self.socket.lock().unwrap();
        let socket = socket_guard.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Not connected - call connect() first"))?;
        
        // Parse body if provided
        let body_value = if let Some(body_str) = body {
            serde_json::from_str::<Value>(&body_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON body: {}", e)))?
        } else {
            Value::Null
        };
        
        // Build request
        let request = serde_json::json!({
            "method": method,
            "path": path,
            "body": body_value,
            "query_params": Value::Null,
        });
        
        let request_json = serde_json::to_string(&request)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize request: {}", e)))?;
        
        // Send request (DEALER: empty frame + data)
        // File-based logging for debugging
        use std::io::Write;
        let mut log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/Users/nadji/code/FEAGI-2.0/feagi-py/tmp/zmq_api_client.log")
            .ok();
        
        if let Some(ref mut f) = log_file {
            let _ = writeln!(f, "🦀 [ZMQ-API-CLIENT] Sending request: {} {} ({} bytes)", method, path, request_json.len());
            let _ = f.flush();
        }
        
        println!("🦀 [ZMQ-API-CLIENT] Sending request: {} {} ({} bytes)", method, path, request_json.len());
        
        socket.send("", zmq::SNDMORE)
            .map_err(|e| {
                if let Some(ref mut f) = log_file {
                    let _ = writeln!(f, "🦀 [ZMQ-API-CLIENT] ❌ Failed to send delimiter: {}", e);
                }
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to send delimiter: {}", e))
            })?;
        
        socket.send(request_json.as_bytes(), 0)
            .map_err(|e| {
                if let Some(ref mut f) = log_file {
                    let _ = writeln!(f, "🦀 [ZMQ-API-CLIENT] ❌ Failed to send request: {}", e);
                }
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to send request: {}", e))
            })?;
        
        if let Some(ref mut f) = log_file {
            let _ = writeln!(f, "🦀 [ZMQ-API-CLIENT] ✅ Request sent, waiting for response...");
            let _ = f.flush();
        }
        
        println!("🦀 [ZMQ-API-CLIENT] Request sent, waiting for response...");
        
        // Receive response
        let mut msg_parts = Vec::new();
        loop {
            let mut msg = zmq::Message::new();
            socket.recv(&mut msg, 0)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to receive: {}", e)))?;
            
            msg_parts.push(msg);
            
            let more = socket.get_rcvmore()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to check RCVMORE: {}", e)))?;
            
            if !more {
                break;
            }
        }
        
        if msg_parts.len() < 2 {
            return Err(pyo3::exceptions::PyValueError::new_err("Invalid response: expected at least 2 parts"));
        }
        
        // Parse response (skip empty delimiter, use data frame)
        let response_data = &msg_parts[1];
        let response_str = std::str::from_utf8(response_data.as_ref())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid UTF-8: {}", e)))?;
        
        let response: Value = serde_json::from_str(response_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse response: {}", e)))?;
        
        // Convert to Python dict
        let py_dict = PyDict::new_bound(py);
        
        if let Some(status) = response.get("status").and_then(|v| v.as_u64()) {
            py_dict.set_item("status", status)?;
        } else {
            py_dict.set_item("status", 500)?;
        }
        
        if let Some(body) = response.get("body") {
            let body_str = serde_json::to_string(body)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize body: {}", e)))?;
            
            let body_value: Py<PyAny> = pyo3::types::PyModule::import_bound(py, "json")?
                .getattr("loads")?
                .call1((body_str,))?
                .unbind();
            
            py_dict.set_item("body", body_value)?;
        }
        
        Ok(py_dict.unbind())
    }
}

