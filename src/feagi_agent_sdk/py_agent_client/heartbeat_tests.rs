//! Unit tests for the background heartbeat worker, ack draining and error surfacing.
//!
//! The command/control transport is a scripted in-memory requester (`Wire`) behind the real
//! `CommandControlAgent`, so registration state, heartbeat encoding and the deregistration
//! handshake all run through the production code. Every duration derives from `INTERVAL`.

use super::*;
use feagi_agent::command_and_control::agent_registration_message::{
    AgentRegistrationMessage, DeregistrationResponse, RegistrationResponse,
};
use feagi_io::traits_and_enums::client::{
    FeagiClient, FeagiClientRequester, FeagiClientRequesterProperties,
};
use std::collections::{HashMap, VecDeque};

/// Heartbeat interval used by every test; all waits are multiples of it.
const INTERVAL: Duration = Duration::from_millis(40);
/// Deregistration ack budget configured on the test sessions.
const CONNECTION_TIMEOUT_MS: u64 = 2000;

/// Scripted state of the in-memory command/control link.
#[derive(Default)]
struct Wire {
    /// Messages the client published, with the time each was published.
    sent: Vec<(Instant, FeagiMessage)>,
    /// Encoded server messages waiting to be polled by the client.
    incoming: VecDeque<Vec<u8>>,
    /// Answer every HeartBeat with a HeartBeat (what FEAGI does).
    auto_ack_heartbeats: bool,
    /// Answer a deregistration request with a Success response (what FEAGI does).
    ack_deregistration: bool,
    /// Fail every publish with a transport error.
    fail_sends: bool,
}

type SharedWire = Arc<Mutex<Wire>>;

fn with_wire<T>(wire: &SharedWire, f: impl FnOnce(&mut Wire) -> T) -> T {
    let mut guard = wire.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut guard)
}

fn encode(message: &FeagiMessage) -> Vec<u8> {
    let mut container = FeagiByteContainer::new_empty();
    message
        .serialize_to_byte_container(&mut container, AgentID::new_random(), 0)
        .expect("encode message");
    container.get_byte_ref().to_vec()
}

fn endpoint() -> TransportProtocolEndpoint {
    TransportProtocolEndpoint::Zmq(ZmqUrl::new("tcp://example:1").expect("valid endpoint"))
}

struct WireRequester {
    wire: SharedWire,
    state: FeagiEndpointState,
    current: Vec<u8>,
}

impl FeagiClient for WireRequester {
    fn poll(&mut self) -> &FeagiEndpointState {
        if matches!(
            self.state,
            FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData
        ) {
            self.state = if with_wire(&self.wire, |w| w.incoming.is_empty()) {
                FeagiEndpointState::ActiveWaiting
            } else {
                FeagiEndpointState::ActiveHasData
            };
        }
        &self.state
    }

    fn request_connect(&mut self) -> Result<(), FeagiNetworkError> {
        self.state = FeagiEndpointState::ActiveWaiting;
        Ok(())
    }

    fn request_disconnect(&mut self) -> Result<(), FeagiNetworkError> {
        self.state = FeagiEndpointState::Inactive;
        Ok(())
    }

    fn confirm_error_and_close(&mut self) -> Result<(), FeagiNetworkError> {
        self.state = FeagiEndpointState::Inactive;
        Ok(())
    }

    fn get_endpoint_target(&self) -> TransportProtocolEndpoint {
        endpoint()
    }
}

impl FeagiClientRequester for WireRequester {
    fn publish_request(&mut self, request: &[u8]) -> Result<(), FeagiNetworkError> {
        let mut container = FeagiByteContainer::new_empty();
        container
            .try_write_data_by_copy_and_verify(request)
            .map_err(|e| FeagiNetworkError::SendFailed(e.to_string()))?;
        let message: FeagiMessage = (&container)
            .try_into()
            .map_err(|e: FeagiAgentError| FeagiNetworkError::SendFailed(e.to_string()))?;
        with_wire(&self.wire, |wire| {
            if wire.fail_sends {
                return Err(FeagiNetworkError::SendFailed(
                    "injected send failure".to_string(),
                ));
            }
            match &message {
                FeagiMessage::HeartBeat if wire.auto_ack_heartbeats => {
                    wire.incoming.push_back(encode(&FeagiMessage::HeartBeat));
                }
                FeagiMessage::AgentRegistration(
                    AgentRegistrationMessage::ClientRequestDeregistration(_),
                ) if wire.ack_deregistration => {
                    wire.incoming
                        .push_back(encode(&FeagiMessage::AgentRegistration(
                            AgentRegistrationMessage::ServerRespondsDeregistration(
                                DeregistrationResponse::Success,
                            ),
                        )));
                }
                _ => {}
            }
            wire.sent.push((Instant::now(), message));
            Ok(())
        })
    }

    fn consume_retrieved_response(&mut self) -> Result<&[u8], FeagiNetworkError> {
        let next = with_wire(&self.wire, |w| w.incoming.pop_front());
        match next {
            Some(bytes) => {
                self.current = bytes;
                Ok(&self.current)
            }
            None => Err(FeagiNetworkError::ReceiveFailed(
                "no response queued".to_string(),
            )),
        }
    }

    fn as_boxed_requester_properties(&self) -> Box<dyn FeagiClientRequesterProperties> {
        Box::new(WireProperties {
            wire: self.wire.clone(),
        })
    }
}

struct WireProperties {
    wire: SharedWire,
}

impl FeagiClientRequesterProperties for WireProperties {
    fn as_boxed_client_requester(&self) -> Box<dyn FeagiClientRequester> {
        Box::new(WireRequester {
            wire: self.wire.clone(),
            state: FeagiEndpointState::Inactive,
            current: Vec::new(),
        })
    }

    fn get_endpoint_target(&self) -> TransportProtocolEndpoint {
        endpoint()
    }
}

/// A registered session on `wire` whose registration went through the real client state machine.
fn registered_compat(wire: &SharedWire, heartbeat_interval_secs: Option<f64>) -> AgentClientCompat {
    let mut config = AgentConfigCompat::new("unit".to_string(), AgentTypeCompat::Sensory);
    config.heartbeat_interval_secs = heartbeat_interval_secs;
    config.connection_timeout_ms = Some(CONNECTION_TIMEOUT_MS);
    config.registration_retries = Some(1);
    let mut compat = AgentClientCompat::new(config);

    let mut control = CommandControlAgent::new(Box::new(WireProperties { wire: wire.clone() }));
    control.request_connect().expect("connect control");
    with_wire(wire, |w| {
        w.incoming
            .push_back(encode(&FeagiMessage::AgentRegistration(
                AgentRegistrationMessage::ServerRespondsRegistration(
                    RegistrationResponse::Success(AgentID::new_random(), HashMap::new()),
                ),
            )));
    });
    control.poll_for_messages().expect("poll registration");
    assert!(matches!(
        control.registration_status(),
        AgentRegistrationStatus::Registered(_, _)
    ));
    compat.command_control = Some(control);
    compat.registered = true;
    compat.last_heartbeat_sent_at = Some(Instant::now());
    compat
}

/// A handle around a registered session (no worker started yet).
fn handle_for(wire: &SharedWire) -> AgentClientHandle {
    let compat = registered_compat(wire, Some(INTERVAL.as_secs_f64()));
    AgentClientHandle {
        lifecycle: Mutex::new(Lifecycle {
            worker: None,
            workers_started: 0,
        }),
        inner: Arc::new(Mutex::new(compat)),
        heartbeat_interval: Some(INTERVAL),
    }
}

fn start_worker(handle: &AgentClientHandle) {
    let mut lifecycle = handle.lock_lifecycle().expect("lifecycle lock");
    handle.ensure_worker(&mut lifecycle).expect("start worker");
}

fn heartbeats_on_wire(wire: &SharedWire) -> usize {
    with_wire(wire, |w| {
        w.sent
            .iter()
            .filter(|(_, m)| matches!(m, FeagiMessage::HeartBeat))
            .count()
    })
}

fn wait_until(limit: Duration, condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + limit;
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    condition()
}

//region interval derivation

#[test]
fn interval_from_config_disables_the_worker_for_unset_or_non_positive_values() {
    assert_eq!(interval_from_config(None), None);
    assert_eq!(interval_from_config(Some(0.0)), None);
    assert_eq!(interval_from_config(Some(-1.0)), None);
    assert_eq!(interval_from_config(Some(f64::NAN)), None);
    assert_eq!(interval_from_config(Some(f64::INFINITY)), None);
    assert_eq!(
        interval_from_config(Some(0.5)),
        Some(Duration::from_millis(500))
    );
}

#[test]
fn no_worker_runs_when_the_interval_is_not_positive() {
    let wire = SharedWire::default();
    let mut handle = handle_for(&wire);
    handle.heartbeat_interval = None;
    start_worker(&handle);
    assert!(!handle.heartbeat_worker_running());
    assert_eq!(handle.heartbeat_workers_started(), 0);
}

//endregion

//region worker lifecycle

#[test]
fn worker_sends_heartbeats_without_any_client_call_and_stops_on_request() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    start_worker(&handle);
    assert!(handle.heartbeat_worker_running());

    assert!(
        wait_until(INTERVAL * 20, || heartbeats_on_wire(&wire) >= 3),
        "worker must send heartbeats on its own"
    );

    handle.stop_heartbeat_worker().expect("stop worker");
    assert!(!handle.heartbeat_worker_running());
    let after_stop = heartbeats_on_wire(&wire);
    std::thread::sleep(INTERVAL * 4);
    assert_eq!(
        heartbeats_on_wire(&wire),
        after_stop,
        "no heartbeat may be sent after the worker is stopped"
    );
}

#[test]
fn one_worker_per_client_and_restart_after_stop() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    for _ in 0..5 {
        start_worker(&handle);
    }
    assert_eq!(handle.heartbeat_workers_started(), 1);
    assert!(handle.heartbeat_worker_running());

    handle.stop_heartbeat_worker().expect("stop worker");
    start_worker(&handle);
    assert_eq!(handle.heartbeat_workers_started(), 2);
    assert!(handle.heartbeat_worker_running());
}

#[test]
fn dropping_the_handle_joins_the_worker_before_releasing_the_session() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    start_worker(&handle);
    let session = Arc::downgrade(&handle.inner);
    drop(handle);
    assert_eq!(
        session.strong_count(),
        0,
        "the worker must be gone and the session released once the handle is dropped"
    );
    // No worker is left sending: the wire stays quiet.
    let sent = heartbeats_on_wire(&wire);
    std::thread::sleep(INTERVAL * 4);
    assert_eq!(heartbeats_on_wire(&wire), sent);
}

#[test]
fn worker_exits_by_itself_when_the_client_is_dropped() {
    let wire = SharedWire::default();
    let session = Arc::new(Mutex::new(registered_compat(
        &wire,
        Some(INTERVAL.as_secs_f64()),
    )));
    let worker = HeartbeatWorker::spawn(Arc::downgrade(&session), INTERVAL).expect("spawn");
    assert!(worker.is_running());
    drop(session);
    assert!(
        wait_until(INTERVAL * 8, || !worker.is_running()),
        "worker must exit once the Weak reference no longer upgrades"
    );
}

#[test]
fn worker_skips_its_turn_while_the_client_is_busy_and_resumes_after() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    start_worker(&handle);
    assert!(wait_until(INTERVAL * 20, || heartbeats_on_wire(&wire) >= 1));

    let held = handle.inner.lock().expect("client lock");
    // Let any in-flight turn finish, then count while the lock is held.
    std::thread::sleep(INTERVAL);
    let before = heartbeats_on_wire(&wire);
    std::thread::sleep(INTERVAL * 6);
    assert_eq!(
        heartbeats_on_wire(&wire),
        before,
        "worker must not send (or block) while another thread holds the client"
    );
    drop(held);

    assert!(
        wait_until(INTERVAL * 8, || heartbeats_on_wire(&wire) > before),
        "worker must resume once the client is free"
    );
}

#[test]
fn stopping_the_worker_does_not_wait_for_a_held_client_lock() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    start_worker(&handle);
    let held = handle.inner.lock().expect("client lock");
    let started = Instant::now();
    handle.stop_heartbeat_worker().expect("stop worker");
    assert!(
        started.elapsed() < INTERVAL,
        "stop must finish within the join bound even while the client lock is held"
    );
    drop(held);
}

#[test]
fn worker_survives_a_poisoned_client_mutex_and_still_stops_cleanly() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    let inner = Arc::clone(&handle.inner);
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let _guard = inner.lock().expect("lock before poisoning");
        panic!("poison the client mutex");
    }));
    assert!(handle.inner.is_poisoned());

    start_worker(&handle);
    std::thread::sleep(INTERVAL * 4);
    assert!(
        handle.heartbeat_worker_running(),
        "a poisoned client must not kill or panic the worker"
    );
    handle.stop_heartbeat_worker().expect("stop worker");
    assert!(!handle.heartbeat_worker_running());
}

#[test]
fn reconnect_restarts_the_worker_even_when_the_rebuild_fails() {
    let wire = SharedWire::default();
    with_wire(&wire, |w| w.ack_deregistration = true);
    let handle = handle_for(&wire);
    start_worker(&handle);
    assert_eq!(handle.heartbeat_workers_started(), 1);

    // The test session has no registration endpoint, so the rebuild's connect step fails fast.
    let outcome = handle.reconnect("unit test");
    assert!(outcome.is_err(), "rebuild must fail without an endpoint");
    assert!(handle.heartbeat_worker_running());
    assert_eq!(
        handle.heartbeat_workers_started(),
        2,
        "reconnect stops the old worker and starts exactly one new one"
    );
}

//endregion

//region shared rate limit

#[test]
fn worker_and_piggybacked_calls_share_one_rate_limit() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    start_worker(&handle);

    // Hammer the piggybacked path while the worker runs.
    let started = Instant::now();
    while started.elapsed() < INTERVAL * 10 {
        handle.is_registered().expect("is_registered");
        std::thread::sleep(Duration::from_millis(1));
    }
    let elapsed = started.elapsed();

    let sent_at: Vec<Instant> = with_wire(&wire, |w| {
        w.sent
            .iter()
            .filter(|(_, m)| matches!(m, FeagiMessage::HeartBeat))
            .map(|(at, _)| *at)
            .collect()
    });
    let allowed = (elapsed.as_secs_f64() / INTERVAL.as_secs_f64()) as usize + 1;
    assert!(
        sent_at.len() >= 3,
        "heartbeats must flow ({})",
        sent_at.len()
    );
    assert!(
        sent_at.len() <= allowed,
        "{} heartbeats in {:?} exceeds the one-per-interval limit ({allowed})",
        sent_at.len(),
        elapsed
    );
    for pair in sent_at.windows(2) {
        assert!(
            pair[1].duration_since(pair[0]) >= INTERVAL * 9 / 10,
            "two heartbeats closer than the configured interval"
        );
    }
}

//endregion

//region error surfacing

#[test]
fn background_heartbeat_error_is_raised_by_the_next_transport_call_then_cleared() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    with_wire(&wire, |w| w.fail_sends = true);
    start_worker(&handle);

    assert!(
        wait_until(INTERVAL * 20, || matches!(
            handle.last_heartbeat_error(),
            Ok(Some(_))
        )),
        "the worker must record its send error"
    );
    with_wire(&wire, |w| w.fail_sends = false);
    handle.stop_heartbeat_worker().expect("stop worker");

    // Reading the error does not clear it, and the piggybacked liveness check does not raise it.
    let stored = handle
        .last_heartbeat_error()
        .expect("read")
        .expect("stored");
    assert!(stored.contains("injected send failure"), "{stored}");
    assert!(handle.last_heartbeat_error().expect("read").is_some());

    let raised = handle
        .send_sensory_bytes(&[1, 2, 3])
        .expect_err("pending heartbeat error must be raised");
    assert!(raised.contains("Background heartbeat failed"), "{raised}");
    assert!(raised.contains("injected send failure"), "{raised}");
    assert_eq!(handle.last_heartbeat_error().expect("read"), None);

    // Raised once: the next call reports whatever is really wrong, not the old error.
    let next = handle
        .send_sensory_bytes(&[1, 2, 3])
        .expect_err("sensory socket is absent in this fixture");
    assert!(!next.contains("Background heartbeat failed"), "{next}");
}

#[test]
fn every_transport_call_raises_the_pending_heartbeat_error() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    for call in 0..3 {
        handle
            .inner
            .lock()
            .expect("client lock")
            .last_heartbeat_error = Some("link down".to_string());
        let raised = match call {
            0 => handle.receive_motor_data().map(|_| ()),
            1 => handle.receive_motor_data_raw().map(|_| ()),
            _ => handle.send_device_configuration("{}"),
        }
        .expect_err("pending error must be raised");
        assert!(raised.contains("link down"), "call {call}: {raised}");
    }
}

#[test]
fn disconnect_clears_a_stale_heartbeat_error() {
    let wire = SharedWire::default();
    with_wire(&wire, |w| w.ack_deregistration = true);
    let handle = handle_for(&wire);
    handle
        .inner
        .lock()
        .expect("client lock")
        .last_heartbeat_error = Some("old session".to_string());
    handle.disconnect().expect("disconnect");
    assert_eq!(handle.last_heartbeat_error().expect("read"), None);
}

//endregion

//region ack draining

#[test]
fn heartbeat_acks_do_not_accumulate_and_disconnect_completes_promptly() {
    let wire = SharedWire::default();
    with_wire(&wire, |w| {
        w.auto_ack_heartbeats = true;
        w.ack_deregistration = true;
    });
    let handle = handle_for(&wire);

    // Many heartbeats, each answered by FEAGI, with nothing else polling the channel.
    let beats = 60;
    for _ in 0..beats {
        std::thread::sleep(INTERVAL);
        handle.is_registered().expect("heartbeat");
        let backlog = with_wire(&wire, |w| w.incoming.len());
        assert!(backlog <= 1, "ack backlog grew to {backlog}");
    }
    assert!(heartbeats_on_wire(&wire) >= beats * 9 / 10);

    let started = Instant::now();
    handle.disconnect().expect("disconnect");
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_millis(CONNECTION_TIMEOUT_MS) / 10,
        "disconnect took {elapsed:?} after {beats} heartbeats"
    );
    assert!(!handle.lock_inner().expect("client lock").registered);
    assert_eq!(
        with_wire(&wire, |w| w.incoming.len()),
        0,
        "the deregistration ack must have been consumed"
    );
}

#[test]
fn draining_consumes_unexpected_messages_without_blocking_the_heartbeat() {
    let wire = SharedWire::default();
    let handle = handle_for(&wire);
    let unexpected = encode(&FeagiMessage::AgentRegistration(
        AgentRegistrationMessage::ServerRespondsDeregistration(
            DeregistrationResponse::NotRegistered,
        ),
    ));
    with_wire(&wire, |w| {
        w.incoming.push_back(encode(&FeagiMessage::HeartBeat));
        w.incoming.push_back(unexpected);
        w.incoming.push_back(encode(&FeagiMessage::HeartBeat));
    });
    std::thread::sleep(INTERVAL);
    handle.is_registered().expect("heartbeat");
    assert_eq!(with_wire(&wire, |w| w.incoming.len()), 0);
    assert_eq!(heartbeats_on_wire(&wire), 1);
    // A NotRegistered reply does not change the session state the disconnect relies on.
    assert!(matches!(
        handle
            .lock_inner()
            .expect("client lock")
            .command_control
            .as_ref()
            .map(|c| c.registration_status().clone()),
        Some(AgentRegistrationStatus::Registered(_, _))
    ));
}

//endregion

#[test]
fn handle_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AgentClientHandle>();
}
