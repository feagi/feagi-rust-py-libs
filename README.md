# feagi-data-processing-py
Wrapper for [Feagi Data Processing](https://github.com/feagi/feagi-data-processing) for Python (Rust compiled by [Maturin](https://github.com/PyO3/maturin)).

[Test-PyPi](https://test.pypi.org/project/feagi-data-processing/)

This Python package allows for fast processing of computer data to and from neuronal forms for use in FEAGI. This crate is primarily intended to be used by FEAGI-Connector

To see implementations of the algorithms, please see the Rust source Here (TODO)

Most Python wrappings in this source code are named after the original Rust Structure names, but with "Py" prepended to them. However, their Python class names use the original Rust structure names instead.

To learn about the Python wrappings, consult the following:

- Genomic Structures
- IO Data
- IO Processing
- [Neuron Data](src/neuron_data/README.md)






