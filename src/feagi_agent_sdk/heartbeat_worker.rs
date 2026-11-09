/*
 * Background heartbeat worker for PyAgentClient.
 *
 * FEAGI prunes an agent that sends nothing inbound for `client_heartbeat_timeout`.
 * Heartbeats piggybacked on send/receive calls stop when a controller loop pauses, so
 * a worker thread owned by the client sends them on its own. The worker never touches
 * Python or the GIL: it only calls `AgentClientCompat::background_heartbeat_turn`, which
 * shares the single rate limit (`maybe_send_heartbeat`) with the piggybacked calls.
 */

use super::py_agent_client::AgentClientCompat;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, TryLockError, Weak};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// The worker wakes this many times per heartbeat interval: it checks the stop flag and the
/// rate limit at a fraction of `heartbeat_interval_secs`, so a heartbeat goes out at most
/// `interval / STEPS_PER_INTERVAL` late. Dimensionless; the time base is the configured interval.
pub(crate) const STEPS_PER_INTERVAL: u32 = 4;

/// Convert the configured `heartbeat_interval_secs` into a worker interval.
///
/// Returns `None` when no worker must run: the interval is unset, not a positive finite
/// number, or too large to represent. This mirrors `maybe_send_heartbeat`, which sends
/// nothing for an unset or non-positive interval.
pub(crate) fn interval_from_config(heartbeat_interval_secs: Option<f64>) -> Option<Duration> {
    let secs = heartbeat_interval_secs?;
    if !secs.is_finite() || secs <= 0.0 {
        return None;
    }
    Duration::try_from_secs_f64(secs).ok()
}

/// A running heartbeat thread plus the handles needed to stop it.
///
/// Owned by one `AgentClientHandle`. Dropping it signals the stop flag and joins the thread
/// (bounded by the heartbeat interval), so the thread never outlives its owner.
pub(crate) struct HeartbeatWorker {
    /// Set to ask the thread to exit at its next wake-up.
    stop: Arc<AtomicBool>,
    /// The thread; `None` once joined or detached.
    thread: Option<JoinHandle<()>>,
    /// Closed (sender dropped) when the thread exits, so joins can wait with a timeout.
    exited: Receiver<()>,
    /// Longest wait for the thread to exit: one heartbeat interval.
    join_bound: Duration,
}

impl HeartbeatWorker {
    /// Spawn the worker for `client`, waking every `interval / STEPS_PER_INTERVAL`.
    ///
    /// The worker holds only a `Weak` reference: it exits when the stop flag is set or the
    /// client has been dropped.
    pub(crate) fn spawn(
        client: Weak<Mutex<AgentClientCompat>>,
        interval: Duration,
    ) -> std::io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let step = interval / STEPS_PER_INTERVAL;
        let (exit_signal, exited) = mpsc::channel::<()>();
        let thread = thread::Builder::new()
            .name("feagi-agent-heartbeat".to_string())
            .spawn(move || run(client, stop_for_thread, step, exit_signal))?;
        Ok(Self {
            stop,
            thread: Some(thread),
            exited,
            join_bound: interval,
        })
    }

    /// Whether the worker thread is still alive.
    pub(crate) fn is_running(&self) -> bool {
        self.thread
            .as_ref()
            .is_some_and(|thread| !thread.is_finished())
    }

    /// Signal the stop flag and wait for the thread to exit, for at most one heartbeat interval.
    ///
    /// Returns `true` when the thread exited. When it did not exit in time it is detached (its
    /// stop flag stays set, so it exits on its own at its next wake-up) and `false` is returned.
    /// Must be called before taking the client lock: the worker only ever uses `try_lock`, so
    /// this never waits on the lock either way.
    pub(crate) fn stop_and_join(&mut self) -> bool {
        self.stop.store(true, Ordering::Release);
        let Some(thread) = self.thread.take() else {
            return true;
        };
        // Wake the thread out of its timed park so it sees the flag now, not a step later.
        thread.thread().unpark();
        match self.exited.recv_timeout(self.join_bound) {
            // The only message the channel carries is the sender dropping: the thread exited.
            Ok(()) | Err(RecvTimeoutError::Disconnected) => {
                let _ = thread.join();
                true
            }
            Err(RecvTimeoutError::Timeout) => {
                tracing::warn!(
                    target: "feagi-rust-py-libs",
                    "heartbeat worker did not stop within {:?}; detaching it",
                    self.join_bound
                );
                false
            }
        }
    }
}

impl Drop for HeartbeatWorker {
    fn drop(&mut self) {
        self.stop_and_join();
    }
}

/// Worker thread body. No unwrap/expect/panic: every failure path is a skip or an exit.
fn run(
    client: Weak<Mutex<AgentClientCompat>>,
    stop: Arc<AtomicBool>,
    step: Duration,
    // Held for the thread's lifetime; dropping it on exit is the "exited" signal.
    _exit_signal: Sender<()>,
) {
    while !stop.load(Ordering::Acquire) {
        // Timed park: `stop_and_join` unparks for a prompt exit; a spurious wake-up only
        // brings a turn forward, and the shared rate limit decides whether it sends.
        thread::park_timeout(step);
        if stop.load(Ordering::Acquire) {
            break;
        }
        let Some(client) = client.upgrade() else {
            break;
        };
        beat_once(&client);
    }
}

/// One worker turn: send a heartbeat if the shared rate limit allows it.
///
/// Skips the turn when the client is busy (another thread holds the lock) or poisoned, so the
/// worker never blocks a Python call and never panics on a poisoned mutex.
fn beat_once(client: &Mutex<AgentClientCompat>) {
    let mut guard = match client.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) | Err(TryLockError::Poisoned(_)) => return,
    };
    guard.background_heartbeat_turn();
}
