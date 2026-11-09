# feagi-rust-py-libs

High-performance Rust-powered Python libraries for FEAGI data processing, sensorimotor encoding, and agent communication.

Built with [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin), this package provides Python bindings to FEAGI's core Rust libraries.

## Features

- **Data Processing**: Fast processing of sensory data to and from neuronal forms
- **Sensorimotor System**: Efficient encoding/decoding for vision, text, and motor control
- **Agent SDK**: Python bindings for building FEAGI agents in Rust-accelerated Python
- **Data Structures**: Core genomic and neuron voxel data structures
- **Serialization**: Efficient serialization/deserialization for FEAGI protocols

## Installation

### From PyPI (recommended)

Pre-built wheels are published for Linux, Windows, and macOS (x86_64 and aarch64). Prefer this so you do not need a Rust toolchain:

```bash
pip install feagi-rust-py-libs
```

### From TestPyPI (staging)

```bash
pip install --index-url https://test.pypi.org/simple/ --extra-index-url https://pypi.org/simple/ feagi-rust-py-libs
```

### Building from source

If pip falls back to building from source (e.g. no wheel for your platform), you need **Rust 1.85 or newer**. A transitive dependency uses the Rust 2024 edition, which is not supported by older Cargo/Rust.

- Check version: `rustc --version` and `cargo --version`
- Install or upgrade: <https://rustup.rs/> then `rustup update stable`
- Then: `pip install feagi-rust-py-libs`

## Usage

This library is primarily used by the FEAGI Python SDK and agent applications. Most Python classes are named after their Rust counterparts, with internal wrapper classes prefixed with "Py".

## Agent heartbeats

`PyAgentClient` keeps its FEAGI session alive from a background worker thread. FEAGI prunes an agent
that sends nothing inbound for `zmq.client_heartbeat_timeout`, and data FEAGI pushes to the agent does
not count, so a controller loop that pauses must not stop the heartbeats.

- The worker starts when `connect()` first succeeds and lives as long as the client object. It wakes
  at a fraction of `heartbeat_interval_secs` and sends through the same rate limit as the heartbeats
  piggybacked on send/receive calls, so the configured interval is never exceeded. With
  `heartbeat_interval_secs` unset or <= 0 no worker runs.
- The worker never touches Python or the GIL and skips a turn while another call holds the client.
  `disconnect()` and `reconnect()` stop it before taking the client lock; `reconnect()` restarts it.
- Each heartbeat first drains FEAGI's pending replies, so acknowledgements never pile up in front of
  the deregistration reply that `disconnect()` waits for.
- A heartbeat send error is stored; the next send/receive/device-configuration call raises it once as
  `RuntimeError("Background heartbeat failed: ...")` (so the transport-failure recovery path runs), and
  `last_heartbeat_error()` reads it without clearing.

Tests: unit tests in `src/feagi_agent_sdk/py_agent_client/heartbeat_tests.rs`; integration tests in
`tests/heartbeat_liveness_tests.rs` run a real FEAGI agent server over ZMQ (`cargo test`).

## Percentage coder functions

`connector_core` exposes feagi-sensorimotor's percentage coder interface:
`percentage_coding_variant`, `decode_percentage_from_z_indexes`, `encode_percentage_to_z_indexes`,
`target_to_z_bin` and `validate_physical_range` (`from feagi_rust_py_libs import connector_core`).
A unit is passed as the agent registration JSON carries it: the section key
(`output_units_and_decoder_properties` or `input_units_and_encoder_properties`), the unit key
(for example `PositionalServo`) and the unit definition object as a JSON string. Rust derives the
unit's cortical IDs, reads the IO flag and selects the coding with `PercentageCodingVariant::from_io_flag`;
Python never chooses a variant. `area_index` picks the cortical area of the unit (default 0). Failures raise
`ValueError` with the Rust message; there are no defaults or fallbacks.

Tests: unit tests in `src/feagi_connector_core/percentage_coding.rs` (`cargo test --lib percentage_coding`);
Python tests in `tests/test_percentage_coding.py` (run `pytest` after `maturin develop`).

Local monorepo builds need the `[patch.crates-io]` block of `.cargo/config.toml.example` in `.cargo/config.toml`
(feagi-core sibling checkout). Do not commit that block when publishing wheels.

## Documentation

For detailed information about the wrapped types and functions:
- Genomic Structures
- IO Data Processing
- [Neuron Voxel Data](src/neuron_data/README.md)
- Agent Communication

## Related Projects

- [FEAGI Python SDK](https://github.com/Neuraville/FEAGI-2.0/tree/main/feagi-python-sdk)
- [FEAGI Core (Rust)](https://github.com/Neuraville/FEAGI-2.0/tree/main/feagi-core)

## License

Apache-2.0






