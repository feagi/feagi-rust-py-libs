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

### From TestPyPI (Staging)

```bash
pip install --index-url https://test.pypi.org/simple/ --extra-index-url https://pypi.org/simple/ feagi-rust-py-libs
```

### From PyPI (Production)

```bash
pip install feagi-rust-py-libs
```

## Usage

This library is primarily used by the FEAGI Python SDK and agent applications. Most Python classes are named after their Rust counterparts, with internal wrapper classes prefixed with "Py".

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






