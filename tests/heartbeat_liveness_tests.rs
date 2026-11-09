//! Heartbeat liveness against a real FEAGI agent server over ZMQ.
//!
//! The server is a real `FeagiAgentHandler` (feagi-agent with the `agent-server` feature) with a
//! ZMQ ROUTER for command/control and a ZMQ PULL endpoint for sensory data, all on OS-assigned
//! loopback ports. The client is `AgentClientHandle`, the Rust core of `PyAgentClient`, whose
//! command/control transport is a ZMQ DEALER. The server prunes a client after
//! `SERVER_HEARTBEAT_TIMEOUT` without inbound messages, so liveness is provable in seconds.
//! Every client timing derives from `SERVER_HEARTBEAT_TIMEOUT`.
//!
//! - a connected client that makes NO send/receive calls survives more than 3x the timeout with
//!   the same agent id (and, as a control, a client whose worker is stopped is pruned);
//! - after `disconnect` the agent is gone from the server;
//! - after many heartbeats `disconnect` still completes promptly (acks are drained);
//! - `reconnect` restarts the worker and the new session stays alive too.

use std::net::{Ipv4Addr, TcpListener};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use feagi_agent::server::auth::DummyAuth;
use feagi_agent::server::{AgentLivenessConfig, FeagiAgentHandler};
use feagi_io::protocol_implementations::zmq::{
    FeagiZmqServerPullerProperties, FeagiZmqServerRouterProperties,
};
use feagi_rust_py_libs::feagi_agent_sdk::py_agent_client::AgentClientHandle;
use feagi_rust_py_libs::feagi_agent_sdk::py_agent_config::{
    AgentConfigCompat, AgentDescriptorCompat,
};
use feagi_rust_py_libs::feagi_agent_sdk::py_agent_type::AgentTypeCompat;

/// Server-side prune threshold. All other timings derive from it.
const SERVER_HEARTBEAT_TIMEOUT: Duration = Duration::from_millis(600);
/// Server stale-scan interval, as a fraction of the timeout.
const STALE_SCAN_DIVISOR: u32 = 12;
/// The client heartbeats this many times per server timeout in the survival tests.
const BEATS_PER_TIMEOUT: u32 = 6;
/// The client heartbeats this many times per server timeout in the ack-drain test.
const DENSE_BEATS_PER_TIMEOUT: u32 = 60;
/// Server loop pause between polls; only bounds busy-looping.
const SERVER_STEP: Duration = Duration::from_millis(2);
/// Deregistration ack budget on the client; also the upper bound for registration waits.
const CONNECTION_TIMEOUT_MS: u64 = 3000;

/// OS-assigned free TCP port.
fn reserve_free_port() -> u16 {
    TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .expect("failed to bind ephemeral port")
        .local_addr()
        .expect("failed to read local addr")
        .port()
}

fn tcp_url(port: u16) -> String {
    format!("tcp://{}:{port}", Ipv4Addr::LOCALHOST)
}

/// What the server thread publishes for assertions.
#[derive(Default)]
struct ServerProbe {
    registered_agent_ids: Mutex<Vec<String>>,
    stop: AtomicBool,
}

impl ServerProbe {
    fn registered(&self) -> Vec<String> {
        self.registered_agent_ids
            .lock()
            .map(|ids| ids.clone())
            .unwrap_or_default()
    }
}

struct TestServer {
    probe: Arc<ServerProbe>,
    thread: Option<JoinHandle<()>>,
    registration_url: String,
    sensory_url: String,
}

impl TestServer {
    /// Start a real agent server and wait until its handler is polling.
    fn start() -> Self {
        let registration_url = tcp_url(reserve_free_port());
        let sensory_url = tcp_url(reserve_free_port());
        let probe = Arc::new(ServerProbe::default());
        let probe_for_thread = Arc::clone(&probe);
        let registration_for_thread = registration_url.clone();
        let sensory_for_thread = sensory_url.clone();
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<(), String>>();

        let thread = thread::spawn(move || {
            // The handler holds non-Send transport objects, so it lives on this thread only.
            let mut handler = FeagiAgentHandler::new_with_liveness_config(
                Box::new(DummyAuth {}),
                AgentLivenessConfig {
                    heartbeat_timeout: SERVER_HEARTBEAT_TIMEOUT,
                    stale_check_interval: SERVER_HEARTBEAT_TIMEOUT / STALE_SCAN_DIVISOR,
                },
            );
            let started = (|| -> Result<(), String> {
                let router = FeagiZmqServerRouterProperties::new(
                    &registration_for_thread,
                    &registration_for_thread,
                )
                .map_err(|e| format!("router properties: {e}"))?;
                handler
                    .add_and_start_command_control_server(Box::new(router))
                    .map_err(|e| format!("start router: {e}"))?;
                let puller =
                    FeagiZmqServerPullerProperties::new(&sensory_for_thread, &sensory_for_thread)
                        .map_err(|e| format!("puller properties: {e}"))?;
                handler.add_puller_server(Box::new(puller));
                Ok(())
            })();
            let started_ok = started.is_ok();
            let _ = ready_tx.send(started);
            if !started_ok {
                return;
            }

            while !probe_for_thread.stop.load(Ordering::SeqCst) {
                if let Err(e) = handler.poll_command_and_control() {
                    eprintln!("server poll error: {e}");
                }
                if let Err(e) = handler.poll_agent_sensors() {
                    eprintln!("server sensor poll error: {e}");
                }
                let ids: Vec<String> = handler
                    .get_all_registered_agents()
                    .keys()
                    .map(|id| id.to_base64())
                    .collect();
                if let Ok(mut slot) = probe_for_thread.registered_agent_ids.lock() {
                    *slot = ids;
                }
                thread::sleep(SERVER_STEP);
            }
        });

        ready_rx
            .recv_timeout(Duration::from_millis(CONNECTION_TIMEOUT_MS))
            .expect("server thread did not report readiness")
            .expect("server failed to start");
        Self {
            probe,
            thread: Some(thread),
            registration_url,
            sensory_url,
        }
    }

    /// A client whose heartbeat interval is `SERVER_HEARTBEAT_TIMEOUT / beats_per_timeout`.
    fn new_client(&self, agent_name: &str, beats_per_timeout: u32) -> AgentClientHandle {
        let mut config = AgentConfigCompat::new(agent_name.to_string(), AgentTypeCompat::Sensory);
        config.registration_endpoint = self.registration_url.clone();
        config.sensory_endpoint = self.sensory_url.clone();
        config.heartbeat_interval_secs =
            Some((SERVER_HEARTBEAT_TIMEOUT / beats_per_timeout).as_secs_f64());
        config.connection_timeout_ms = Some(CONNECTION_TIMEOUT_MS);
        config.registration_retries = Some(1);
        config.descriptor = Some(AgentDescriptorCompat {
            manufacturer: "neuraville".to_string(),
            agent_name: agent_name.to_string(),
            agent_version: 1,
        });
        config.auth_token = Some([0_u8; 32]);
        AgentClientHandle::new(config)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.probe.stop.store(true, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn wait_until(limit: Duration, condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + limit;
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        thread::sleep(SERVER_STEP);
    }
    condition()
}

/// Connect `client` and return its agent id once the server lists it.
fn connect_and_get_id(server: &TestServer, client: &AgentClientHandle) -> String {
    client.connect().expect("registration must succeed");
    let agent_id = client
        .registration_agent_id_b64()
        .expect("read agent id")
        .expect("registered client has an agent id");
    assert!(
        wait_until(SERVER_HEARTBEAT_TIMEOUT, || server
            .probe
            .registered()
            .contains(&agent_id)),
        "server must list the freshly registered agent"
    );
    agent_id
}

#[test]
fn idle_client_survives_more_than_three_server_timeouts_with_the_same_agent_id() {
    let server = TestServer::start();
    let client = server.new_client("idle-survivor", BEATS_PER_TIMEOUT);
    let agent_id = connect_and_get_id(&server, &client);
    assert!(client.heartbeat_worker_running());

    // The client makes no send, receive or status calls for the whole period.
    thread::sleep(SERVER_HEARTBEAT_TIMEOUT * 4);

    assert!(
        server.probe.registered().contains(&agent_id),
        "agent must still be registered under the same id after 4x the server timeout"
    );
    assert_eq!(
        client.registration_agent_id_b64().expect("read agent id"),
        Some(agent_id)
    );
    assert_eq!(client.heartbeat_workers_started(), 1);
    assert_eq!(client.last_heartbeat_error().expect("read error"), None);
    let sent = client.heartbeats_sent().expect("heartbeat count");
    // BEATS_PER_TIMEOUT per timeout over 4 timeouts, with generous slack for scheduling.
    assert!(
        sent >= u64::from(BEATS_PER_TIMEOUT) * 4 / 2,
        "only {sent} heartbeats were sent"
    );
}

#[test]
fn client_without_its_worker_is_pruned_which_proves_the_worker_keeps_it_alive() {
    let server = TestServer::start();
    let client = server.new_client("worker-stopped", BEATS_PER_TIMEOUT);
    let agent_id = connect_and_get_id(&server, &client);

    client.stop_heartbeat_worker().expect("stop worker");
    assert!(!client.heartbeat_worker_running());

    assert!(
        wait_until(SERVER_HEARTBEAT_TIMEOUT * 4, || !server
            .probe
            .registered()
            .contains(&agent_id)),
        "a silent client must be pruned by the server's stale scan"
    );
}

#[test]
fn agent_is_gone_from_the_server_after_disconnect() {
    let server = TestServer::start();
    let client = server.new_client("disconnects", BEATS_PER_TIMEOUT);
    let agent_id = connect_and_get_id(&server, &client);

    client.disconnect().expect("disconnect");

    assert!(!client.heartbeat_worker_running());
    assert!(
        wait_until(SERVER_HEARTBEAT_TIMEOUT, || !server
            .probe
            .registered()
            .contains(&agent_id)),
        "a disconnected client must be deregistered"
    );
    assert_eq!(
        client.registration_agent_id_b64().expect("read agent id"),
        Some(agent_id),
        "the last session id stays readable after disconnect"
    );
}

#[test]
fn disconnect_completes_promptly_after_many_heartbeats() {
    let server = TestServer::start();
    let client = server.new_client("dense-heartbeats", DENSE_BEATS_PER_TIMEOUT);
    let agent_id = connect_and_get_id(&server, &client);

    // About 5 timeouts of dense heartbeats; FEAGI answers every one and nothing else polls.
    thread::sleep(SERVER_HEARTBEAT_TIMEOUT * 5);
    let sent = client.heartbeats_sent().expect("heartbeat count");
    assert!(
        sent >= u64::from(DENSE_BEATS_PER_TIMEOUT),
        "the test needs many heartbeats, got {sent}"
    );

    let started = Instant::now();
    client.disconnect().expect("disconnect");
    let elapsed = started.elapsed();

    // Undrained acks cost one poll step each in the deregistration wait; the ack budget
    // is far larger, so bound the wait by a fraction of the budget instead.
    assert!(
        elapsed < Duration::from_millis(CONNECTION_TIMEOUT_MS) / 10,
        "disconnect took {elapsed:?} after {sent} heartbeats"
    );
    assert!(
        wait_until(SERVER_HEARTBEAT_TIMEOUT, || !server
            .probe
            .registered()
            .contains(&agent_id)),
        "the agent must be deregistered after the prompt disconnect"
    );
}

#[test]
fn reconnect_restarts_the_worker_and_the_new_session_stays_alive() {
    let server = TestServer::start();
    let client = server.new_client("reconnects", BEATS_PER_TIMEOUT);
    let first_id = connect_and_get_id(&server, &client);

    client.reconnect("integration test").expect("reconnect");
    let second_id = client
        .registration_agent_id_b64()
        .expect("read agent id")
        .expect("re-registered client has an agent id");
    assert_ne!(first_id, second_id, "reconnect registers a new session");
    assert!(client.heartbeat_worker_running());
    assert_eq!(client.heartbeat_workers_started(), 2);

    thread::sleep(SERVER_HEARTBEAT_TIMEOUT * 4);

    let registered = server.probe.registered();
    assert!(registered.contains(&second_id));
    assert!(
        !registered.contains(&first_id),
        "the old session must be gone after the rebuild"
    );
}
