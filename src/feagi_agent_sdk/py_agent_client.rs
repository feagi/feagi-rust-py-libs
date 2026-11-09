/*
 * PyO3 wrapper for AgentClient on top of FEAGI 2.0 client primitives.
 */

use super::heartbeat_worker::{interval_from_config, HeartbeatWorker};
use super::py_agent_config::{AgentConfigCompat, PyAgentConfig};
use super::py_agent_type::AgentTypeCompat;
use feagi_agent::clients::recovery::session::RebuildableSession;
use feagi_agent::clients::{AgentRegistrationStatus, CommandControlAgent};
use feagi_agent::command_and_control::agent_embodiment_configuration_message::AgentEmbodimentConfigurationMessage;
use feagi_agent::command_and_control::FeagiMessage;
use feagi_agent::{AgentCapabilities, AgentDescriptor, AuthToken, FeagiAgentError};
use feagi_data_structures::genomic::cortical_area::CorticalID;
use feagi_data_structures::neuron_voxels::xyzp::{
    CorticalMappedXYZPNeuronVoxels, NeuronVoxelXYZPArrays,
};
use feagi_io::protocol_implementations::websocket::WebSocketUrl;
use feagi_io::protocol_implementations::zmq::ZmqUrl;
use feagi_io::traits_and_enums::client::{FeagiClientPusher, FeagiClientSubscriber};
use feagi_io::traits_and_enums::shared::{FeagiEndpointState, TransportProtocolEndpoint};
use feagi_io::{AgentID, FeagiNetworkError};
use feagi_sensorimotor::configuration::jsonable::JSONInputOutputDefinition;
use feagi_serialization::FeagiByteContainer;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyList, PyTuple};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

pub(crate) struct AgentClientCompat {
    config: AgentConfigCompat,
    command_control: Option<CommandControlAgent>,
    sensory_client: Option<Box<dyn FeagiClientPusher>>,
    motor_client: Option<Box<dyn FeagiClientSubscriber>>,
    registered: bool,
    last_heartbeat_sent_at: Option<Instant>,
    /// Number of heartbeats sent over the command/control channel (worker and piggybacked).
    heartbeats_sent: u64,
    /// Last error from a background heartbeat turn. The next call that uses the transport
    /// raises it (so the Python recovery path sees a transport failure); cleared when raised
    /// and on connect/disconnect. The worker itself never retries beyond its normal cadence.
    last_heartbeat_error: Option<String>,
    /// AgentID (base64) assigned at registration; used for device_registrations import
    registration_agent_id_b64: Option<String>,
    /// Cached device registrations JSON (most recent successful send) so a
    /// reconnect re-establishes the same device topology without needing
    /// the controller to re-supply it.
    last_device_registrations_json: Option<String>,
}

impl AgentClientCompat {
    /// Pause between polling attempts in the connect/disconnect/rebuild wait
    /// loops. These loops previously used `std::thread::yield_now()`, a
    /// busy-spin that pegs a CPU core with no actual pause; a small bounded
    /// sleep is a good citizen on shared/embedded hardware while adding
    /// negligible latency relative to the sockets' own round-trip time.
    const POLL_SLEEP: Duration = Duration::from_millis(2);

    fn new(config: AgentConfigCompat) -> Self {
        Self {
            config,
            command_control: None,
            sensory_client: None,
            motor_client: None,
            registered: false,
            last_heartbeat_sent_at: None,
            heartbeats_sent: 0,
            last_heartbeat_error: None,
            registration_agent_id_b64: None,
            last_device_registrations_json: None,
        }
    }

    fn parse_endpoint(endpoint: &str) -> Result<TransportProtocolEndpoint, String> {
        if endpoint.starts_with("tcp://") {
            let parsed = ZmqUrl::new(endpoint).map_err(|e| e.to_string())?;
            Ok(TransportProtocolEndpoint::Zmq(parsed))
        } else if endpoint.starts_with("ws://") || endpoint.starts_with("wss://") {
            let parsed = WebSocketUrl::new(endpoint).map_err(|e| e.to_string())?;
            Ok(TransportProtocolEndpoint::WebSocket(parsed))
        } else {
            Err(format!(
                "Unsupported endpoint scheme for '{}'; expected tcp://, ws://, or wss://",
                endpoint
            ))
        }
    }

    fn requested_capabilities(agent_type: AgentTypeCompat) -> Vec<AgentCapabilities> {
        match agent_type {
            AgentTypeCompat::Sensory => vec![AgentCapabilities::SendSensorData],
            AgentTypeCompat::Motor => vec![AgentCapabilities::ReceiveMotorData],
            AgentTypeCompat::Both => vec![
                AgentCapabilities::SendSensorData,
                AgentCapabilities::ReceiveMotorData,
            ],
        }
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.registered {
            tracing::info!(
                target: "feagi-rust-py-libs",
                "connect: short-circuit, already registered"
            );
            return Ok(());
        }

        let connect_started_at = Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "connect: start (registration_endpoint={})",
            self.config.registration_endpoint
        );

        let endpoint = Self::parse_endpoint(&self.config.registration_endpoint)?;
        let requester = endpoint
            .try_create_boxed_client_requester_properties()
            .map_err(|e| e.to_string())?;
        let mut control = CommandControlAgent::new(requester);
        control.request_connect().map_err(|e| e.to_string())?;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "connect: control socket request_connect issued"
        );

        let descriptor_cfg = self
            .config
            .descriptor
            .as_ref()
            .ok_or_else(|| "Agent descriptor is not configured".to_string())?;
        let descriptor = AgentDescriptor::new(
            &descriptor_cfg.manufacturer,
            &descriptor_cfg.agent_name,
            descriptor_cfg.agent_version,
        )
        .map_err(|e| e.to_string())?;
        let auth_bytes = self
            .config
            .auth_token
            .ok_or_else(|| "Auth token is not configured".to_string())?;
        let auth = AuthToken::new(auth_bytes);
        let requested_capabilities = Self::requested_capabilities(self.config.agent_type);

        let connect_timeout_ms = self
            .config
            .connection_timeout_ms
            .ok_or_else(|| "connection_timeout_ms is not configured".to_string())?;
        let registration_retries = self
            .config
            .registration_retries
            .ok_or_else(|| "registration_retries is not configured".to_string())?;
        let timeout_ms = connect_timeout_ms
            .saturating_mul(registration_retries as u64)
            .max(1);
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        tracing::info!(
            target: "feagi-rust-py-libs",
            "connect: registration deadline = {} ms (connect_timeout_ms={} * retries={})",
            timeout_ms,
            connect_timeout_ms,
            registration_retries
        );
        let mut sent_registration = false;

        let mut session_id: Option<AgentID> = None;
        let mut endpoints: Option<
            std::collections::HashMap<AgentCapabilities, TransportProtocolEndpoint>,
        > = None;

        while Instant::now() < deadline {
            let (state, _message) = control.poll_for_messages().map_err(|e| e.to_string())?;

            if !sent_registration
                && matches!(
                    state,
                    FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData
                )
            {
                control
                    .request_registration(
                        descriptor.clone(),
                        auth.clone(),
                        requested_capabilities.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                sent_registration = true;
                tracing::info!(
                    target: "feagi-rust-py-libs",
                    "connect: request_registration sent (capabilities={})",
                    requested_capabilities.len()
                );
            }

            if let AgentRegistrationStatus::Registered(id, map) = control.registration_status() {
                session_id = Some(*id);
                endpoints = Some(map.clone());
                tracing::info!(
                    target: "feagi-rust-py-libs",
                    "connect: registered after {} ms (endpoints={})",
                    connect_started_at.elapsed().as_millis(),
                    map.len()
                );
                break;
            }

            std::thread::sleep(Self::POLL_SLEEP);
        }

        let session_id = session_id.ok_or_else(|| {
            let msg = format!(
                "Registration timed out after {} ms",
                connect_started_at.elapsed().as_millis()
            );
            tracing::warn!(target: "feagi-rust-py-libs", "connect: {}", msg);
            msg
        })?;
        let endpoint_map =
            endpoints.ok_or_else(|| "No endpoint map after registration".to_string())?;

        if matches!(
            self.config.agent_type,
            AgentTypeCompat::Sensory | AgentTypeCompat::Both
        ) {
            let sensory_endpoint = endpoint_map
                .get(&AgentCapabilities::SendSensorData)
                .cloned()
                .ok_or_else(|| "FEAGI registration did not provide sensory endpoint".to_string())?;
            tracing::info!(
                target: "feagi-rust-py-libs",
                "connect: opening sensory pusher to FEAGI-provided endpoint"
            );
            let props = sensory_endpoint
                .try_create_boxed_client_pusher_properties()
                .map_err(|e| e.to_string())?;
            let mut sensory_client = props.as_boxed_client_pusher();
            sensory_client
                .request_connect()
                .map_err(|e| e.to_string())?;
            self.wait_until_active_waiting_pusher(sensory_client.as_mut(), deadline)?;
            self.sensory_client = Some(sensory_client);
            tracing::info!(
                target: "feagi-rust-py-libs",
                "connect: sensory pusher active"
            );
        }

        if matches!(
            self.config.agent_type,
            AgentTypeCompat::Motor | AgentTypeCompat::Both
        ) {
            let motor_endpoint = endpoint_map
                .get(&AgentCapabilities::ReceiveMotorData)
                .cloned()
                .ok_or_else(|| "FEAGI registration did not provide motor endpoint".to_string())?;
            tracing::info!(
                target: "feagi-rust-py-libs",
                "connect: opening motor subscriber to FEAGI-provided endpoint"
            );
            let props = motor_endpoint
                .try_create_boxed_client_subscriber_properties()
                .map_err(|e| e.to_string())?;
            let mut motor_client = props.as_boxed_client_subscriber();
            motor_client.request_connect().map_err(|e| e.to_string())?;
            self.wait_until_active_waiting_subscriber(motor_client.as_mut(), deadline)?;
            self.motor_client = Some(motor_client);
            tracing::info!(
                target: "feagi-rust-py-libs",
                "connect: motor subscriber active"
            );
        }

        self.registration_agent_id_b64 = Some(session_id.to_base64());
        self.command_control = Some(control);
        self.registered = true;
        self.last_heartbeat_sent_at = Some(Instant::now());
        self.last_heartbeat_error = None;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "connect: complete in {} ms (session_id={})",
            connect_started_at.elapsed().as_millis(),
            self.registration_agent_id_b64
                .as_deref()
                .unwrap_or("<unset>")
        );
        Ok(())
    }

    /// Send device registrations (AgentConfiguration) to FEAGI over the command/control channel.
    /// Must be called after connect() when the agent has motors registered.
    fn send_device_configuration(&mut self, device_registrations_json: &str) -> Result<(), String> {
        self.raise_pending_heartbeat_error()?;
        if !self.registered {
            return Err("Agent is not registered; call connect() first".to_string());
        }
        let control = self
            .command_control
            .as_mut()
            .ok_or_else(|| "Command control channel not available".to_string())?;
        let device_def: JSONInputOutputDefinition = serde_json::from_str(device_registrations_json)
            .map_err(|e| format!("Invalid device_registrations JSON: {}", e))?;
        let message = FeagiMessage::AgentConfiguration(
            AgentEmbodimentConfigurationMessage::AgentConfigurationDetails(device_def),
        );
        let started_at = Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "send_device_configuration: sending payload ({} bytes)",
            device_registrations_json.len()
        );
        control
            .send_message(message, 0)
            .map_err(|e| format!("Failed to send device configuration: {}", e))?;
        self.last_device_registrations_json = Some(device_registrations_json.to_string());
        tracing::info!(
            target: "feagi-rust-py-libs",
            "send_device_configuration: sent in {} ms",
            started_at.elapsed().as_millis()
        );
        Ok(())
    }

    /// Whether a previous call to `send_device_configuration` cached a payload
    /// that a reconnect would replay.
    pub(crate) fn has_cached_device_registrations(&self) -> bool {
        self.last_device_registrations_json.is_some()
    }

    fn sensory_socket_ready_for_publish(state: &FeagiEndpointState) -> Result<(), String> {
        match state {
            FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData => Ok(()),
            FeagiEndpointState::Pending => Err("Sensory socket is pending".to_string()),
            FeagiEndpointState::Inactive => Err("Sensory socket is inactive".to_string()),
            FeagiEndpointState::Errored(err) => Err(format!("Sensory socket errored: {}", err)),
        }
    }

    fn is_transient_zmq_publish_would_block(err: &FeagiNetworkError) -> bool {
        matches!(
            err,
            FeagiNetworkError::SendFailed(msg) if msg.contains("Socket would block")
        )
    }

    /// Bounded retry for ZMQ `DONTWAIT` / `EAGAIN`, matching `feagi-agent` tokio helpers.
    fn publish_sensory_with_transient_retry(
        pusher: &mut dyn FeagiClientPusher,
        bytes: &[u8],
    ) -> Result<(), String> {
        const MAX_ATTEMPTS: usize = 32;
        const PAUSE: Duration = Duration::from_millis(1);
        for attempt in 0..MAX_ATTEMPTS {
            Self::sensory_socket_ready_for_publish(pusher.poll())?;
            match pusher.publish_data(bytes) {
                Ok(()) => return Ok(()),
                Err(e) if Self::is_transient_zmq_publish_would_block(&e) => {
                    if attempt + 1 < MAX_ATTEMPTS {
                        std::thread::sleep(PAUSE);
                        continue;
                    }
                    return Err(format!(
                        "Sensory publish would block after {} attempts: {}",
                        MAX_ATTEMPTS, e
                    ));
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        Err("Sensory publish exhausted transient retries".to_string())
    }

    fn wait_until_active_waiting_pusher(
        &self,
        pusher: &mut dyn FeagiClientPusher,
        deadline: Instant,
    ) -> Result<(), String> {
        while Instant::now() < deadline {
            match pusher.poll() {
                FeagiEndpointState::ActiveWaiting => return Ok(()),
                FeagiEndpointState::ActiveHasData => {
                    return Err("Sensory pusher entered invalid ActiveHasData state".to_string());
                }
                FeagiEndpointState::Errored(err) => {
                    return Err(format!("Sensory connect failed: {}", err));
                }
                _ => {
                    std::thread::sleep(Self::POLL_SLEEP);
                }
            }
        }
        Err("Timed out waiting for sensory channel connection".to_string())
    }

    fn wait_until_active_waiting_subscriber(
        &self,
        subscriber: &mut dyn FeagiClientSubscriber,
        deadline: Instant,
    ) -> Result<(), String> {
        while Instant::now() < deadline {
            match subscriber.poll() {
                FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData => {
                    return Ok(());
                }
                FeagiEndpointState::Errored(err) => {
                    return Err(format!("Motor connect failed: {}", err));
                }
                _ => {
                    std::thread::sleep(Self::POLL_SLEEP);
                }
            }
        }
        Err("Timed out waiting for motor channel connection".to_string())
    }

    fn send_sensory_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.raise_pending_heartbeat_error()?;
        if !self.registered {
            return Err("Agent is not registered".to_string());
        }
        self.maybe_send_heartbeat()?;
        let sensory_client = self
            .sensory_client
            .as_mut()
            .ok_or_else(|| "Sensory socket is not available for this agent type".to_string())?;
        Self::publish_sensory_with_transient_retry(sensory_client.as_mut(), bytes)
    }

    fn encode_sensory_pairs_to_container(
        &self,
        pairs: &[(i32, f64)],
    ) -> Result<FeagiByteContainer, String> {
        let vision = self
            .config
            .vision_capability
            .as_ref()
            .ok_or_else(|| "Vision capability is required for send_sensory_data".to_string())?;

        let cortical_area = vision.cortical_area.as_ref().ok_or_else(|| {
            "send_sensory_data requires with_vision_capability(target_cortical_area)".to_string()
        })?;
        if cortical_area.is_empty() {
            return Err("target_cortical_area cannot be empty".to_string());
        }
        if vision.width == 0 || vision.height == 0 || vision.channels == 0 {
            return Err("Vision dimensions and channels must all be > 0".to_string());
        }

        let mut id_bytes = [b' '; 8];
        let area_bytes = cortical_area.as_bytes();
        let copy_len = area_bytes.len().min(8);
        id_bytes[..copy_len].copy_from_slice(&area_bytes[..copy_len]);
        let cortical_id = CorticalID::try_from_bytes(&id_bytes)
            .map_err(|e| format!("Invalid cortical area id: {:?}", e))?;

        let width = vision.width as u32;
        let height = vision.height as u32;
        let channels = vision.channels as u32;
        let plane = width
            .checked_mul(height)
            .ok_or_else(|| "Vision width*height overflowed u32".to_string())?;
        let max_index = plane
            .checked_mul(channels)
            .ok_or_else(|| "Vision width*height*channels overflowed u32".to_string())?;

        let mut x_coords = Vec::with_capacity(pairs.len());
        let mut y_coords = Vec::with_capacity(pairs.len());
        let mut z_coords = Vec::with_capacity(pairs.len());
        let mut potentials = Vec::with_capacity(pairs.len());

        for (neuron_id, potential) in pairs.iter().copied() {
            if neuron_id < 0 {
                return Err(format!("Neuron id must be non-negative, got {}", neuron_id));
            }
            let idx = neuron_id as u32;
            if idx >= max_index {
                return Err(format!(
                    "Neuron id {} is out of bounds for configured vision shape {}x{}x{}",
                    idx, width, height, channels
                ));
            }
            let z = idx / plane;
            let in_plane = idx % plane;
            let y = in_plane / width;
            let x = in_plane % width;

            x_coords.push(x);
            y_coords.push(y);
            z_coords.push(z);
            potentials.push(potential as f32);
        }

        let arrays =
            NeuronVoxelXYZPArrays::new_from_vectors(x_coords, y_coords, z_coords, potentials)
                .map_err(|e| format!("Failed to create neuron arrays: {}", e))?;
        let mut mapped = CorticalMappedXYZPNeuronVoxels::new();
        mapped.insert(cortical_id, arrays);

        let mut container = FeagiByteContainer::new_empty();
        container
            .overwrite_byte_data_with_single_struct_data(&mapped, 0)
            .map_err(|e| format!("Failed to serialize sensory data: {}", e))?;
        Ok(container)
    }

    fn receive_motor_data(&mut self) -> Result<Option<String>, String> {
        self.raise_pending_heartbeat_error()?;
        if !self.registered {
            return Err("Agent is not registered".to_string());
        }
        self.maybe_send_heartbeat()?;
        let motor_client = self
            .motor_client
            .as_mut()
            .ok_or_else(|| "Motor socket is not available for this agent type".to_string())?;

        let raw_data: Vec<u8> = match motor_client.poll() {
            FeagiEndpointState::ActiveHasData => motor_client
                .consume_retrieved_data()
                .map_err(|e| e.to_string())?
                .to_vec(),
            FeagiEndpointState::ActiveWaiting
            | FeagiEndpointState::Pending
            | FeagiEndpointState::Inactive => return Ok(None),
            FeagiEndpointState::Errored(err) => {
                return Err(format!("Motor socket errored: {}", err));
            }
        };

        let mut buffer = FeagiByteContainer::new_empty();
        buffer
            .try_write_data_by_copy_and_verify(&raw_data)
            .map_err(|e| e.to_string())?;

        let structure_count = buffer
            .try_get_number_contained_structures()
            .map_err(|e| e.to_string())?;
        if structure_count == 0 {
            return Ok(None);
        }

        let boxed_struct = buffer
            .try_create_new_struct_from_index(0)
            .map_err(|e| e.to_string())?;
        let motor_data = boxed_struct
            .as_any()
            .downcast_ref::<CorticalMappedXYZPNeuronVoxels>()
            .ok_or_else(|| {
                "Received motor payload is not CorticalMappedXYZPNeuronVoxels".to_string()
            })?;

        let mut result = serde_json::Map::new();
        for (cortical_id, neuron_voxels) in motor_data.mappings.iter() {
            let (x_vec, y_vec, z_vec, p_vec) = neuron_voxels.borrow_xyzp_vectors();
            let mut area_data = serde_json::Map::new();
            area_data.insert("x".to_string(), serde_json::json!(x_vec));
            area_data.insert("y".to_string(), serde_json::json!(y_vec));
            area_data.insert("z".to_string(), serde_json::json!(z_vec));
            area_data.insert("p".to_string(), serde_json::json!(p_vec));
            result.insert(
                cortical_id.as_base_64(),
                serde_json::Value::Object(area_data),
            );
        }

        Ok(Some(serde_json::Value::Object(result).to_string()))
    }

    /// Polls the motor socket and returns the raw, verified FEAGI byte payload.
    ///
    /// Unlike [`Self::receive_motor_data`], this performs NO XYZP-to-JSON conversion;
    /// the bytes are returned untouched so the Rust motor decoder (`MotorDeviceCache`)
    /// can decode them directly. Returns `None` when no data is currently available.
    fn receive_motor_data_raw(&mut self) -> Result<Option<Vec<u8>>, String> {
        self.raise_pending_heartbeat_error()?;
        if !self.registered {
            return Err("Agent is not registered".to_string());
        }
        self.maybe_send_heartbeat()?;
        let motor_client = self
            .motor_client
            .as_mut()
            .ok_or_else(|| "Motor socket is not available for this agent type".to_string())?;

        let raw_data: Vec<u8> = match motor_client.poll() {
            FeagiEndpointState::ActiveHasData => motor_client
                .consume_retrieved_data()
                .map_err(|e| e.to_string())?
                .to_vec(),
            FeagiEndpointState::ActiveWaiting
            | FeagiEndpointState::Pending
            | FeagiEndpointState::Inactive => return Ok(None),
            FeagiEndpointState::Errored(err) => {
                return Err(format!("Motor socket errored: {}", err));
            }
        };

        // Verify the payload decodes to the expected motor structure before handing the
        // bytes to the decoder, preserving the validation contract of receive_motor_data.
        let mut buffer = FeagiByteContainer::new_empty();
        buffer
            .try_write_data_by_copy_and_verify(&raw_data)
            .map_err(|e| e.to_string())?;
        let structure_count = buffer
            .try_get_number_contained_structures()
            .map_err(|e| e.to_string())?;
        if structure_count == 0 {
            return Ok(None);
        }

        Ok(Some(raw_data))
    }

    fn maybe_send_heartbeat(&mut self) -> Result<(), String> {
        if !self.registered {
            return Ok(());
        }
        let interval_secs = self
            .config
            .heartbeat_interval_secs
            .ok_or_else(|| "heartbeat_interval_secs is not configured".to_string())?;
        if interval_secs <= 0.0 {
            return Ok(());
        }

        let should_send = match self.last_heartbeat_sent_at {
            None => true,
            Some(last) => last.elapsed() >= Duration::from_secs_f64(interval_secs),
        };
        if !should_send {
            return Ok(());
        }

        // Consume the acks of earlier heartbeats before sending the next one, so they never
        // pile up in front of the deregistration ack that `disconnect_internal` waits for.
        self.drain_control_messages()?;

        let control = self
            .command_control
            .as_mut()
            .ok_or_else(|| "Command control channel is not initialized".to_string())?;
        control.send_heartbeat().map_err(|e| e.to_string())?;
        self.last_heartbeat_sent_at = Some(Instant::now());
        self.heartbeats_sent += 1;
        Ok(())
    }

    /// Consume every command/control message FEAGI has already sent, without blocking.
    ///
    /// FEAGI answers each HeartBeat (and each AgentConfiguration) with a HeartBeat reply, and
    /// nothing else polls the channel between connect and disconnect. `poll_for_messages`
    /// applies registration and deregistration state changes itself (the status that
    /// `disconnect_internal` checks), so draining here cannot hide them; no other code in this
    /// crate reads the returned messages. Anything other than a HeartBeat is logged and dropped.
    fn drain_control_messages(&mut self) -> Result<(), String> {
        let control = self
            .command_control
            .as_mut()
            .ok_or_else(|| "Command control channel is not initialized".to_string())?;
        loop {
            let (_, message) = control.poll_for_messages().map_err(|e| e.to_string())?;
            match message {
                None => return Ok(()),
                Some(FeagiMessage::HeartBeat) => {}
                Some(other) => {
                    tracing::debug!(
                        target: "feagi-rust-py-libs",
                        "drain_control_messages: dropped unexpected message {:?}",
                        other
                    );
                }
            }
        }
    }

    /// One turn of the background heartbeat worker (the worker holds the client lock).
    ///
    /// Runs the same rate-limited send as the piggybacked calls. A failure (or a panic inside
    /// the transport, caught here so it cannot poison the client mutex) is stored in
    /// `last_heartbeat_error` for the next transport call to raise.
    pub(crate) fn background_heartbeat_turn(&mut self) {
        let error = match catch_unwind(AssertUnwindSafe(|| self.maybe_send_heartbeat())) {
            Ok(Ok(())) => return,
            Ok(Err(message)) => message,
            Err(_) => "heartbeat worker panicked while sending".to_string(),
        };
        tracing::warn!(
            target: "feagi-rust-py-libs",
            "background heartbeat failed: {}",
            error
        );
        self.last_heartbeat_error = Some(error);
    }

    /// Raise (and clear) the error stored by the background heartbeat worker, if any.
    fn raise_pending_heartbeat_error(&mut self) -> Result<(), String> {
        match self.last_heartbeat_error.take() {
            Some(error) => Err(format!("Background heartbeat failed: {error}")),
            None => Ok(()),
        }
    }

    /// Tear down the active session.
    ///
    /// Polls for FEAGI's deregistration response (within `connection_timeout_ms`)
    /// before tearing down local sockets. Both the public `disconnect()` and
    /// `rebuild()` use it: a new registration is a different socket from the old
    /// session's, so FEAGI has no ordering between the two. Registering again
    /// before the old session is deregistered is rejected as `AlreadyRegistered`
    /// while the old session is still fresh. If no ack arrives within the budget
    /// (dead link, or FEAGI already dropped the session) teardown proceeds anyway.
    fn disconnect_internal(&mut self) -> Result<(), String> {
        if !self.registered {
            tracing::info!(
                target: "feagi-rust-py-libs",
                "disconnect: short-circuit, not registered"
            );
            return Ok(());
        }

        let disconnect_started_at = Instant::now();
        let timeout_ms = self
            .config
            .connection_timeout_ms
            .ok_or_else(|| "connection_timeout_ms is not configured".to_string())?;
        let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(1));
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: start (deregistration ack budget = {} ms)",
            timeout_ms
        );

        if let Some(control) = self.command_control.as_mut() {
            control
                .request_deregistration(Some("python-sdk disconnect".to_string()))
                .map_err(|e| e.to_string())?;
            tracing::info!(
                target: "feagi-rust-py-libs",
                "disconnect: request_deregistration sent"
            );

            let mut acked = false;
            while Instant::now() < deadline {
                let _ = control.poll_for_messages().map_err(|e| e.to_string())?;
                if matches!(
                    control.registration_status(),
                    AgentRegistrationStatus::NotRegistered
                ) {
                    acked = true;
                    break;
                }
                std::thread::sleep(Self::POLL_SLEEP);
            }
            tracing::info!(
                target: "feagi-rust-py-libs",
                "disconnect: deregistration {} after {} ms",
                if acked { "acknowledged" } else { "TIMED OUT (FEAGI never replied)" },
                disconnect_started_at.elapsed().as_millis()
            );
            if let Err(err) = control.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("Cannot disconnect: client is not in Active state") {
                    return Err(msg);
                }
                tracing::info!(
                    target: "feagi-rust-py-libs",
                    "disconnect: request_disconnect skipped ({})",
                    msg
                );
            }
        }

        if let Some(sensory) = self.sensory_client.as_mut() {
            if let Err(err) = sensory.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("not in Active state") {
                    return Err(msg);
                }
            }
            let _ = sensory.poll();
            tracing::info!(
                target: "feagi-rust-py-libs",
                "disconnect: sensory pusher closed"
            );
        }
        if let Some(motor) = self.motor_client.as_mut() {
            if let Err(err) = motor.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("not in Active state") {
                    return Err(msg);
                }
            }
            let _ = motor.poll();
            tracing::info!(
                target: "feagi-rust-py-libs",
                "disconnect: motor subscriber closed"
            );
        }

        // Drop the boxed sockets one at a time, with tracing on either side of
        // each `Drop` so a future hang at `zmq_close` is immediately visible.
        // `zmq_close` blocks for the socket's linger period; client sockets
        // default to linger=0 (see `feagi-io::client_implementations`), but
        // operators may override that via `FEAGI_ZMQ_LINGER_MS`.
        let drop_started_at = std::time::Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: dropping sensory pusher box"
        );
        self.sensory_client = None;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: sensory pusher dropped ({} ms)",
            drop_started_at.elapsed().as_millis()
        );

        let motor_drop_started_at = std::time::Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: dropping motor subscriber box"
        );
        self.motor_client = None;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: motor subscriber dropped ({} ms)",
            motor_drop_started_at.elapsed().as_millis()
        );

        let control_drop_started_at = std::time::Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: dropping command/control agent"
        );
        self.command_control = None;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: command/control agent dropped ({} ms)",
            control_drop_started_at.elapsed().as_millis()
        );

        self.registered = false;
        self.last_heartbeat_sent_at = None;
        self.last_heartbeat_error = None;
        tracing::info!(
            target: "feagi-rust-py-libs",
            "disconnect: complete in {} ms",
            disconnect_started_at.elapsed().as_millis()
        );
        Ok(())
    }

    /// Public disconnect that preserves the historical "wait for FEAGI ack
    /// before tearing down sockets" behaviour. Called from
    /// `PyAgentClient.disconnect()` (i.e., explicit Python-side shutdown).
    fn disconnect(&mut self) -> Result<(), String> {
        self.disconnect_internal()
    }
}

/// `RebuildableSession` plug-in for the shared `feagi-agent` recovery loop.
///
/// `rebuild` performs a best-effort `disconnect` (waiting for the
/// deregistration ack within `connection_timeout_ms`) followed by a fresh
/// `connect`, then replays the most recently cached device registrations
/// (if any) so the new session has the same device topology as the old
/// one. This is the same surface every other SDK (Java) will implement so
/// behavior cannot drift.
impl RebuildableSession for AgentClientCompat {
    fn rebuild(&mut self, reason: &str) -> Result<(), FeagiAgentError> {
        let rebuild_started_at = Instant::now();
        tracing::info!(
            target: "feagi-rust-py-libs",
            "rebuild: start (reason={reason}, has_cached_device_registrations={})",
            self.last_device_registrations_json.is_some()
        );
        // Wait (bounded by `connection_timeout_ms`) for FEAGI to acknowledge the
        // deregistration so the old session is gone server-side before the new
        // registration is sent; on a dead link the wait expires and we proceed.
        if let Err(err) = self.disconnect_internal() {
            tracing::warn!(
                target: "feagi-rust-py-libs",
                "rebuild: best-effort disconnect failed (reason={reason}): {err}"
            );
        }
        tracing::info!(
            target: "feagi-rust-py-libs",
            "rebuild: about to call connect() (elapsed={} ms)",
            rebuild_started_at.elapsed().as_millis()
        );
        self.connect().map_err(|e| {
            tracing::warn!(
                target: "feagi-rust-py-libs",
                "rebuild: connect() failed after {} ms: {}",
                rebuild_started_at.elapsed().as_millis(),
                e
            );
            FeagiAgentError::ConnectionFailed(e)
        })?;

        if let Some(payload) = self.last_device_registrations_json.clone() {
            tracing::info!(
                target: "feagi-rust-py-libs",
                "rebuild: replaying cached device_registrations (elapsed={} ms)",
                rebuild_started_at.elapsed().as_millis()
            );
            self.send_device_configuration(&payload).map_err(|e| {
                tracing::warn!(
                    target: "feagi-rust-py-libs",
                    "rebuild: send_device_configuration failed after {} ms: {}",
                    rebuild_started_at.elapsed().as_millis(),
                    e
                );
                FeagiAgentError::Other(e)
            })?;
        } else {
            tracing::info!(
                target: "feagi-rust-py-libs",
                "rebuild: no cached device_registrations to replay"
            );
        }
        tracing::info!(
            target: "feagi-rust-py-libs",
            "rebuild: complete in {} ms (reason={reason})",
            rebuild_started_at.elapsed().as_millis()
        );
        Ok(())
    }
}

/// Lifecycle state guarded by `AgentClientHandle::lifecycle`.
struct Lifecycle {
    /// The heartbeat worker, present while one is (or was last) running for this client.
    worker: Option<HeartbeatWorker>,
    /// How many workers this client has started in its lifetime (one live worker at a time).
    workers_started: usize,
}

/// Rust-level client handle: the shared session plus the background heartbeat worker.
///
/// `PyAgentClient` is a thin Python wrapper around this type. The worker's lifetime is the
/// handle's lifetime, not one session: it is started when the client first connects and is
/// restarted after a disconnect/reconnect, and it keeps covering sessions that
/// `RebuildableSession::rebuild` re-creates. Every method that tears down or rebuilds the
/// session stops and joins the worker BEFORE taking the session lock.
pub struct AgentClientHandle {
    // Declaration order matters: `lifecycle` drops first, which stops and joins the worker
    // before the session `Arc` is released.
    lifecycle: Mutex<Lifecycle>,
    inner: Arc<Mutex<AgentClientCompat>>,
    /// Heartbeat interval from the config; `None` means no worker runs (unset or <= 0).
    heartbeat_interval: Option<Duration>,
}

impl AgentClientHandle {
    /// Create a handle for `config`. Nothing is started until `connect`.
    pub fn new(config: AgentConfigCompat) -> Self {
        let heartbeat_interval = interval_from_config(config.heartbeat_interval_secs);
        Self {
            lifecycle: Mutex::new(Lifecycle {
                worker: None,
                workers_started: 0,
            }),
            inner: Arc::new(Mutex::new(AgentClientCompat::new(config))),
            heartbeat_interval,
        }
    }

    fn lock_lifecycle(&self) -> Result<MutexGuard<'_, Lifecycle>, String> {
        self.lifecycle
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))
    }

    fn lock_inner(&self) -> Result<MutexGuard<'_, AgentClientCompat>, String> {
        self.inner
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))
    }

    /// Start the heartbeat worker unless one is already running or the config disables it.
    fn ensure_worker(&self, lifecycle: &mut Lifecycle) -> Result<(), String> {
        if lifecycle
            .worker
            .as_ref()
            .is_some_and(HeartbeatWorker::is_running)
        {
            return Ok(());
        }
        // A worker that exited on its own (or was detached) is replaced, never duplicated.
        lifecycle.worker = None;
        let Some(interval) = self.heartbeat_interval else {
            return Ok(());
        };
        let worker = HeartbeatWorker::spawn(Arc::downgrade(&self.inner), interval)
            .map_err(|e| format!("Failed to start heartbeat worker: {}", e))?;
        lifecycle.worker = Some(worker);
        lifecycle.workers_started += 1;
        Ok(())
    }

    /// Signal the stop flag and join the worker. Callers do this before taking the session lock.
    fn stop_worker(lifecycle: &mut Lifecycle) {
        if let Some(mut worker) = lifecycle.worker.take() {
            worker.stop_and_join();
        }
    }

    /// Connect and register, then make sure the heartbeat worker is running.
    ///
    /// If the worker cannot start, the fresh session would be pruned for silence, so it is
    /// torn down again and the error is returned.
    pub fn connect(&self) -> Result<(), String> {
        let mut lifecycle = self.lock_lifecycle()?;
        self.lock_inner()?.connect()?;
        if let Err(start_error) = self.ensure_worker(&mut lifecycle) {
            let teardown = self.lock_inner().and_then(|mut client| client.disconnect());
            return Err(match teardown {
                Ok(()) => start_error,
                Err(teardown_error) => {
                    format!("{start_error}; teardown also failed: {teardown_error}")
                }
            });
        }
        Ok(())
    }

    /// Stop the worker, then disconnect (waiting for FEAGI's deregistration ack).
    pub fn disconnect(&self) -> Result<(), String> {
        let mut lifecycle = self.lock_lifecycle()?;
        Self::stop_worker(&mut lifecycle);
        self.lock_inner()?.disconnect()
    }

    /// Stop the worker, rebuild the session, then restart the worker.
    ///
    /// The worker is restarted whatever the rebuild outcome: the caller retries a failed
    /// reconnect, and the worker does nothing while the client is not registered.
    pub fn reconnect(&self, reason: &str) -> Result<(), String> {
        let mut lifecycle = self.lock_lifecycle()?;
        Self::stop_worker(&mut lifecycle);
        let rebuilt = self
            .lock_inner()
            .and_then(|mut client| client.rebuild(reason).map_err(|e| e.to_string()));
        let restarted = self.ensure_worker(&mut lifecycle);
        rebuilt.and(restarted)
    }

    /// Stop the heartbeat worker without touching the session (it restarts on the next
    /// `connect`/`reconnect`). Used to prove that the worker is what keeps a session alive.
    #[doc(hidden)]
    pub fn stop_heartbeat_worker(&self) -> Result<(), String> {
        let mut lifecycle = self.lock_lifecycle()?;
        Self::stop_worker(&mut lifecycle);
        Ok(())
    }

    /// Whether a heartbeat worker thread is currently alive.
    pub fn heartbeat_worker_running(&self) -> bool {
        self.lock_lifecycle()
            .map(|lifecycle| {
                lifecycle
                    .worker
                    .as_ref()
                    .is_some_and(HeartbeatWorker::is_running)
            })
            .unwrap_or(false)
    }

    /// How many heartbeat workers this client has started so far.
    pub fn heartbeat_workers_started(&self) -> usize {
        self.lock_lifecycle()
            .map(|lifecycle| lifecycle.workers_started)
            .unwrap_or(0)
    }

    /// Number of heartbeats sent so far (worker and piggybacked).
    pub fn heartbeats_sent(&self) -> Result<u64, String> {
        Ok(self.lock_inner()?.heartbeats_sent)
    }

    /// The last background heartbeat error that has not been raised yet (does not clear it).
    pub fn last_heartbeat_error(&self) -> Result<Option<String>, String> {
        Ok(self.lock_inner()?.last_heartbeat_error.clone())
    }

    /// Encode vision `(neuron_id, potential)` pairs and send them as sensory data.
    pub fn send_sensory_pairs(&self, pairs: &[(i32, f64)]) -> Result<(), String> {
        let mut client = self.lock_inner()?;
        let container = client.encode_sensory_pairs_to_container(pairs)?;
        client.send_sensory_bytes(container.get_byte_ref())
    }

    /// Send pre-encoded sensory bytes.
    pub fn send_sensory_bytes(&self, bytes: &[u8]) -> Result<(), String> {
        self.lock_inner()?.send_sensory_bytes(bytes)
    }

    /// Poll the motor socket and return the data as JSON text.
    pub fn receive_motor_data(&self) -> Result<Option<String>, String> {
        self.lock_inner()?.receive_motor_data()
    }

    /// Poll the motor socket and return the raw verified FEAGI bytes.
    pub fn receive_motor_data_raw(&self) -> Result<Option<Vec<u8>>, String> {
        self.lock_inner()?.receive_motor_data_raw()
    }

    /// Whether the client is registered (piggybacks a rate-limited heartbeat, as before).
    pub fn is_registered(&self) -> Result<bool, String> {
        let mut client = self.lock_inner()?;
        client.maybe_send_heartbeat()?;
        Ok(client.registered)
    }

    /// AgentID (base64) assigned at registration, or `None` when not connected.
    pub fn registration_agent_id_b64(&self) -> Result<Option<String>, String> {
        Ok(self.lock_inner()?.registration_agent_id_b64.clone())
    }

    /// Send the device registrations JSON over command/control.
    pub fn send_device_configuration(&self, device_registrations_json: &str) -> Result<(), String> {
        self.lock_inner()?
            .send_device_configuration(device_registrations_json)
    }

    /// Whether a device registrations payload is cached for replay on reconnect.
    pub fn has_cached_device_registrations(&self) -> Result<bool, String> {
        Ok(self.lock_inner()?.has_cached_device_registrations())
    }
}

/// Convert a Rust-level error message into the `RuntimeError` Python callers already see.
fn runtime_error(message: String) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(message)
}

#[pyclass(name = "PyAgentClient")]
pub struct PyAgentClient {
    handle: AgentClientHandle,
}

#[pymethods]
impl PyAgentClient {
    #[new]
    fn new(config: &PyAgentConfig) -> PyResult<Self> {
        config
            .validate_internal()
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
        Ok(PyAgentClient {
            handle: AgentClientHandle::new(config.inner().clone()),
        })
    }

    /// Connect and register with FEAGI.
    ///
    /// This blocks for up to `connection_timeout_ms * registration_retries`
    /// (per `AgentConfigCompat`) while polling the control socket. That work
    /// is dispatched via [`Python::detach`] so the GIL is released for
    /// its duration: without this, no other Python thread in the process
    /// (e.g. a controller's manual-control HTTP server thread) can run while
    /// `connect()` is polling, which previously froze concurrent Python
    /// threads for the full connection deadline on every attempt.
    ///
    /// On success a background worker keeps heartbeats flowing, paced by
    /// `heartbeat_interval_secs`, until `disconnect()` or the client is dropped.
    fn connect(&mut self, py: Python<'_>) -> PyResult<()> {
        let handle = &self.handle;
        py.detach(|| handle.connect()).map_err(runtime_error)
    }

    fn send_sensory_data(&self, _py: Python, neuron_pairs: Bound<'_, PyAny>) -> PyResult<()> {
        let list = neuron_pairs.cast::<PyList>()?;
        let mut pairs: Vec<(i32, f64)> = Vec::new();
        for item in list {
            let tuple = item.cast::<PyTuple>()?;
            if tuple.len() != 2 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Each item must be a (neuron_id, potential) tuple",
                ));
            }
            let neuron_id: i32 = tuple.get_item(0)?.extract()?;
            let potential: f64 = tuple.get_item(1)?.extract()?;
            pairs.push((neuron_id, potential));
        }

        self.handle
            .send_sensory_pairs(&pairs)
            .map_err(runtime_error)
    }

    fn send_sensory_bytes(&self, _py: Python, payload: Bound<'_, PyAny>) -> PyResult<()> {
        let py_bytes = payload.cast::<PyBytes>()?;
        let bytes = py_bytes.as_bytes().to_vec();
        self.handle
            .send_sensory_bytes(&bytes)
            .map_err(runtime_error)
    }

    fn receive_motor_data(&self, _py: Python) -> PyResult<Option<String>> {
        self.handle.receive_motor_data().map_err(runtime_error)
    }

    /// Polls the motor socket and returns the raw verified FEAGI byte payload, or
    /// `None` when no data is available.
    ///
    /// Intended to feed the Rust motor decoder directly (no Python-side XYZP decode).
    fn receive_motor_data_raw<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyBytes>>> {
        let maybe_bytes = self
            .handle
            .receive_motor_data_raw()
            .map_err(runtime_error)?;
        Ok(maybe_bytes.map(|bytes| PyBytes::new(py, &bytes)))
    }

    fn is_registered(&self) -> PyResult<bool> {
        self.handle.is_registered().map_err(runtime_error)
    }

    /// Return AgentID (base64) assigned at registration, or None if not connected.
    fn get_registration_agent_id_b64(&self) -> PyResult<Option<String>> {
        self.handle
            .registration_agent_id_b64()
            .map_err(runtime_error)
    }

    /// Send device registrations to FEAGI over ZMQ (AgentConfiguration message).
    /// Call after connect() when the agent has motors. Replaces HTTP device_registrations import.
    fn send_device_configuration(&self, device_registrations_json: &str) -> PyResult<()> {
        self.handle
            .send_device_configuration(device_registrations_json)
            .map_err(runtime_error)
    }

    /// Disconnect from FEAGI, waiting (up to `connection_timeout_ms`) for the
    /// deregistration ack. Dispatched via [`Python::detach`] for the
    /// same reason as `connect()`: it polls in a loop and must not hold the
    /// GIL for that duration. Stops the background heartbeat worker first.
    fn disconnect(&mut self, py: Python<'_>) -> PyResult<()> {
        let handle = &self.handle;
        py.detach(|| handle.disconnect()).map_err(runtime_error)
    }

    /// Force a fresh registration cycle: disconnect (best-effort), connect,
    /// and replay the most recent device registrations payload if one was
    /// previously sent.
    ///
    /// Use this when the controller has detected a FEAGI lifecycle event
    /// (genome reload, FEAGI restart, prolonged unreachability) that
    /// invalidated the prior session. The decision logic for *when* to
    /// call this lives in the Rust `feagi-agent::clients::recovery`
    /// module so behavior is identical across Python, Rust, and Java SDKs.
    ///
    /// On success the agent is registered again and any cached device
    /// registrations have been re-sent. Raises `RuntimeError` on failure.
    /// Disconnects and reconnects, replaying cached device registrations.
    ///
    /// Like `connect()`/`disconnect()`, `rebuild()` polls in a loop for up to
    /// `connection_timeout_ms * registration_retries`; dispatched via
    /// [`Python::detach`] so it does not hold the GIL for that
    /// duration. The heartbeat worker is stopped before the rebuild and
    /// restarted after it.
    #[pyo3(signature = (reason=None))]
    fn reconnect(&self, py: Python<'_>, reason: Option<&str>) -> PyResult<()> {
        let handle = &self.handle;
        let reason_text = reason.unwrap_or("python-sdk reconnect").to_string();
        py.detach(|| handle.reconnect(&reason_text))
            .map_err(runtime_error)
    }

    /// Whether the most recent successful `send_device_configuration` is
    /// cached and will be replayed on the next `reconnect()`.
    fn has_cached_device_registrations(&self) -> PyResult<bool> {
        self.handle
            .has_cached_device_registrations()
            .map_err(runtime_error)
    }

    /// The last error raised by the background heartbeat worker that no call has
    /// raised yet, or `None`. Read-only: the next send/receive call raises it once.
    fn last_heartbeat_error(&self) -> PyResult<Option<String>> {
        self.handle.last_heartbeat_error().map_err(runtime_error)
    }

    fn __repr__(&self) -> String {
        let registered = self.is_registered().unwrap_or(false);
        format!("PyAgentClient(registered={})", registered)
    }
}

#[cfg(test)]
mod heartbeat_tests;

#[cfg(test)]
mod tests {
    use super::AgentClientCompat;
    use crate::feagi_agent_sdk::py_agent_config::{
        AgentConfigCompat, AgentDescriptorCompat, VisionCapabilityCompat,
    };
    use crate::feagi_agent_sdk::py_agent_type::AgentTypeCompat;

    fn configured_client() -> AgentClientCompat {
        let mut cfg = AgentConfigCompat::new("agent-1".to_string(), AgentTypeCompat::Sensory);
        cfg.registration_endpoint = "tcp://127.0.0.1:30001".to_string();
        cfg.sensory_endpoint = "tcp://127.0.0.1:5558".to_string();
        cfg.connection_timeout_ms = Some(1000);
        cfg.registration_retries = Some(3);
        cfg.heartbeat_interval_secs = Some(1.0);
        cfg.descriptor = Some(AgentDescriptorCompat {
            manufacturer: "Neuraville".to_string(),
            agent_name: "agent-1".to_string(),
            agent_version: 1,
        });
        cfg.auth_token = Some([9_u8; 32]);
        cfg.vision_capability = Some(VisionCapabilityCompat {
            modality: "vision".to_string(),
            width: 2,
            height: 2,
            channels: 1,
            cortical_area: Some("iv00".to_string()),
            unit: None,
            group: None,
        });
        AgentClientCompat::new(cfg)
    }

    /// `PyAgentClient::connect`/`disconnect`/`reconnect` dispatch their
    /// blocking work through `Python::detach`, which requires the
    /// closure (and everything it captures, including `AgentClientCompat`
    /// behind the shared `Arc<Mutex<_>>`) to be `Send`. This compile-time
    /// assertion guards that invariant: if a future change to
    /// `AgentClientCompat`'s fields makes it `!Send`, this test fails to
    /// *compile* (not just to run), catching the regression immediately
    /// instead of resurfacing as a GIL-freeze bug report.
    #[test]
    fn agent_client_compat_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AgentClientCompat>();
    }

    #[test]
    fn encode_sensory_pairs_to_container_produces_single_structure() {
        let client = configured_client();
        let container = client
            .encode_sensory_pairs_to_container(&[(0, 0.1), (1, 0.2), (2, 0.3)])
            .expect("encoding should succeed");

        let count = container
            .try_get_number_contained_structures()
            .expect("container read should succeed");
        assert_eq!(count, 1);
    }

    #[test]
    fn encode_sensory_pairs_to_container_rejects_out_of_bounds_neuron() {
        let client = configured_client();
        let err = client
            .encode_sensory_pairs_to_container(&[(4, 0.5)])
            .expect_err("expected bounds check failure");
        assert!(err.contains("out of bounds"));
    }
}
