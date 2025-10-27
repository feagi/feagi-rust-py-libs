//! LZ4 compression/decompression utilities for FEAGI data
//! 
//! ARCHITECTURE: Bridge operates in PASSTHROUGH mode - clients handle decompression
//! This module provides LZ4 decompression for Python clients receiving compressed data from FEAGI

use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Decompress LZ4-compressed data from FEAGI
/// 
/// ARCHITECTURE: FEAGI PNS → LZ4 compress → ZMQ → Bridge PASSTHROUGH → Client DECOMPRESS
/// 
/// Args:
///     compressed_data: LZ4-compressed bytes from FEAGI (via bridge passthrough)
/// 
/// Returns:
///     Decompressed bytes or raises exception on error
#[pyfunction]
pub fn decompress_lz4<'py>(py: Python<'py>, compressed_data: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyBytes>> {
    let compressed_bytes = compressed_data.as_bytes();
    
    if compressed_bytes.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err("Empty input data"));
    }
    
    // Decompress using LZ4 block decompression
    match lz4::block::decompress(compressed_bytes, None) {
        Ok(decompressed) => {
            // Convert Vec<u8> to PyBytes
            Ok(PyBytes::new_bound(py, &decompressed))
        }
        Err(e) => {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "LZ4 decompression failed: {:?}",
                e
            )))
        }
    }
}

/// Check if data is LZ4 compressed by examining magic header
/// 
/// LZ4 compressed data starts with magic number 0x04
/// 
/// Args:
///     data: Bytes to check
/// 
/// Returns:
///     True if data appears to be LZ4 compressed
#[pyfunction]
pub fn is_lz4_compressed(data: &Bound<'_, PyBytes>) -> bool {
    let bytes = data.as_bytes();
    !bytes.is_empty() && bytes[0] == 0x04
}

/// Compress data using LZ4
/// 
/// This is primarily for testing - most clients only need decompression
/// 
/// Args:
///     data: Raw bytes to compress
/// 
/// Returns:
///     LZ4-compressed bytes
#[pyfunction]
pub fn compress_lz4<'py>(py: Python<'py>, data: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyBytes>> {
    let raw_bytes = data.as_bytes();
    
    if raw_bytes.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err("Empty input data"));
    }
    
    // Compress using LZ4 with high compression
    match lz4::block::compress(raw_bytes, Some(lz4::block::CompressionMode::HIGHCOMPRESSION(9)), false) {
        Ok(compressed) => {
            Ok(PyBytes::new_bound(py, &compressed))
        }
        Err(e) => {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "LZ4 compression failed: {:?}",
                e
            )))
        }
    }
}

/// Decompress data if it's LZ4 compressed, otherwise return as-is
/// 
/// Convenience function that checks for LZ4 magic header and decompresses if needed
/// 
/// Args:
///     data: Bytes that may or may not be compressed
/// 
/// Returns:
///     Decompressed bytes (or original if not compressed)
#[pyfunction]
pub fn decompress_if_needed<'py>(py: Python<'py>, data: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyBytes>> {
    if is_lz4_compressed(data) {
        decompress_lz4(py, data)
    } else {
        // Return original data
        Ok(data.clone())
    }
}

