# FEAGI Rust-Python Bindings: Architecture Migration Status

**Date**: November 28, 2024  
**Status**: IN PROGRESS - Blocked by API incompatibilities

---

## Overview

This document tracks the migration of `feagi-rust-py-libs` from `feagi-data-processing` beta.47 to beta.56, with the goal of creating a clean, maintainable Python API for FEAGI agent development.

---

## Architectural Vision

### Goals
1. **Zero Cortical ID Exposure**: Users define capabilities semantically (e.g., `"rotary_motor"`, not base64 IDs)
2. **Clean Callback API**: Simple `motor.on_command(callback)` interface
3. **Type-Safe**: Rust enforces correctness, Python gets simple API
4. **Performance**: Direct Rust callbacks, zero-copy where possible
5. **Future-Proof**: Easy to add new sensor/motor types

### User-Facing API (Target)
```python
from feagi.agent import BaseAgent
from feagi.motors import Motor

class CutebotAgent(BaseAgent):
    def __init__(self):
        super().__init__(
            feagi_host="localhost",
            capabilities={
                "motor": {
                    "devices": [
                        {"type": "rotary_motor", "count": 2, "group": 0}
                    ]
                }
            }
        )
        
        self.left_wheel = Motor(self, device_type="rotary_motor", group=0, channel=0)
        self.left_wheel.on_command(self.handle_left_wheel)
    
    def handle_left_wheel(self, value):
        self.robot.set_left_speed(value)
```

---

## Progress Summary

### ✅ Completed
1. **PyMotorDeviceCache** structure created with:
   - Generic `register(motor_unit, group, channels, ...)` method
   - Callback registration with `register_callback(motor_unit, group, channel, callback)`
   - Neuron processing with `process_neurons(bytes)`
   - Value retrieval with `get_value(motor_unit, group, channel)`

2. **Cortical type enums** (`PyMotorCorticalType`, `PySensorCorticalType`) properly generated via macros

3. **Old IOCache disabled**: Commented out 11k-line auto-generated code

4. **Module structure updated**: New caching module exports `PyMotorDeviceCache`

5. **Error handling**: Added `PyFeagiError::from_string()` helper

### ⚠️ Blocked Issues

#### **Critical API Mismatches (beta.47 → beta.56)**

1. **Descriptor Types Changed**:
   ```rust
   // beta.47 (worked)
   CorticalChannelCount(10)
   
   // beta.56 (private constructor)
   CorticalChannelCount(10)  // ❌ Error: private fields
   ```
   **Impact**: Cannot create descriptor types in Python bindings

2. **Motor Type Variants Changed**:
   ```rust
   // Expected
   MotorCorticalUnit::RotaryMotorIncrementalLinear
   
   // Actual (beta.56)
   MotorCorticalUnit::RotaryMotor
   ```
   **Status**: ✅ Fixed in code

3. **FrameChangeHandling Variants Changed**:
   ```rust
   // Expected
   FrameChangeHandling::Cumulative
   
   // Actual (beta.56)
   FrameChangeHandling::Absolute
   ```
   **Status**: ✅ Fixed in code

4. **Method Names Changed**:
   ```rust
   // beta.47
   self.inner.rotary_motor_incremental_linear_register(...)
   
   // beta.56
   self.inner.rotary_motor_register(...)
   ```
   **Status**: ✅ Fixed in code

5. **WrappedIOData API Changed**:
   ```rust
   // beta.47
   wrapped.as_f32()  // Simple conversion
   
   // beta.56
   // No as_f32() method - need to match on enum
   ```
   **Impact**: Cannot easily convert motor values to Python floats

6. **FeagiSignalIndex Type Changed**:
   ```rust
   // beta.47
   let id: u64 = signal_index.into();
   
   // beta.56
   // No Into<u64> implementation
   ```
   **Impact**: Cannot return signal IDs to Python

7. **Neuron Processing API Changed**:
   ```rust
   // Expected
   self.inner.decode_neurons_to_cached_motor_values(bytes)
   
   // Actual (beta.56)
   // Method name/signature different or doesn't exist
   ```
   **Impact**: Cannot process incoming motor commands

8. **Closure Trait Bounds**:
   ```rust
   // Error
   `(dyn for<'a> FnMut(&'a WrappedIOData) + Send + 'static)` cannot be shared between threads safely
   ```
   **Impact**: Callback closures need `Sync` trait but `FnMut` isn't `Sync`

---

## Recommended Strategy

### **Option 1: Minimal Viable API (Recommended)**

Focus on getting ONE motor type working end-to-end:

1. **Fix descriptor type creation**:
   - Check if beta.56 exposes public constructors
   - If not, add helper methods to `feagi-connector-core`

2. **Simplify callback API**:
   - Use `Fn` instead of `FnMut` (immutable closures are `Sync`)
   - Pass values, not references

3. **Add conversion helpers**:
   - Implement `as_f32()` for `WrappedIOData` types in `feagi-connector-core`
   - Add `Into<u64>` for `FeagiSignalIndex`

4. **Target ONE motor type**:
   - Focus on `RotaryMotor` only
   - Get full end-to-end working (Rust → Python callback → robot)

5. **Expand incrementally**:
   - Once `RotaryMotor` works, add others

### **Option 2: Wait for Upstream Fixes**

If API changes are too extensive:

1. Create GitHub issues for `feagi-data-processing` with required helper methods
2. Work with upstream maintainers to stabilize beta.56 API
3. Resume Python bindings once upstream is stable

### **Option 3: Fork and Fix Upstream**

If changes are needed urgently:

1. Fork `feagi-data-processing`
2. Add public constructors/helper methods
3. Use forked version in `feagi-rust-py-libs`
4. Submit PRs upstream later

---

## Required Upstream Changes (for Option 1)

### In `feagi-connector-core`:

```rust
// 1. Public constructors for descriptors
impl CorticalChannelCount {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

// 2. Value extraction for WrappedIOData
impl Percentage {
    pub fn as_f32(&self) -> f32 {
        self.0  // or whatever the internal representation is
    }
}

// 3. Signal index conversion
impl From<FeagiSignalIndex> for u64 {
    fn from(index: FeagiSignalIndex) -> u64 {
        index.0  // or whatever the internal representation is
    }
}

// 4. Neuron processing method
impl MotorDeviceCache {
    pub fn decode_neurons(&mut self, bytes: &[u8]) -> Result<(), FeagiDataError> {
        // Process neurons and trigger callbacks
    }
}

// 5. Sync closures
impl MotorDeviceCache {
    pub fn register_callback<F>(&mut self, ..., callback: F)
    where F: Fn(&WrappedIOData) + Send + Sync + 'static  // Fn, not FnMut
    {
        // ...
    }
}
```

---

## Files Modified (This Session)

### Rust (`feagi-rust-py-libs`)
- ✅ `src/feagi_connector_core/caching/motor_device_cache.rs` (NEW)
- ✅ `src/feagi_connector_core/caching/mod.rs` (updated exports)
- ✅ `src/feagi_data_structures/genomic/cortical_type.rs` (disabled PyCorticalType)
- ✅ `src/feagi_data_structures/genomic/cortical_id.rs` (disabled old methods)
- ✅ `src/feagi_data_structures/genomic/mod.rs` (updated exports)
- ✅ `src/py_error.rs` (added `from_string` helper)
- ✅ `src/lib.rs` (exported PyMotorDeviceCache)

### Python (feagi-python-sdk)
- ⏸️ Not started (blocked by Rust compilation)

---

## Next Steps

### Immediate (if proceeding with Option 1):

1. **Check `feagi-data-processing` beta.56 source** for public APIs:
   ```bash
   cd feagi-data-processing
   grep -r "pub fn new" feagi_data_structures/src/genomic/cortical_area/descriptors.rs
   grep -r "pub fn as_f32" feagi_connector_core/src/data_types/
   ```

2. **Add missing methods** to `feagi-connector-core` (if not present)

3. **Rebuild and test** `feagi-rust-py-libs`

4. **Build Python wheel**:
   ```bash
   cd feagi-rust-py-libs
   maturin build --release
   ```

5. **Create Python SDK layer** (`feagi/motors.py`, `feagi/sensors.py`)

6. **Write tests**

7. **Update Cutebot controller**

### Estimated Effort

- **Option 1 (with upstream help)**: 8-12 hours
- **Option 2 (wait)**: Unknown timeline
- **Option 3 (fork)**: 12-16 hours

---

## Questions for User

1. **Do you have access to modify `feagi-data-processing`?**  
   If yes → Option 1 or 3  
   If no → Option 2

2. **Is beta.56 API stable?**  
   If yes → Proceed with Option 1  
   If no → Consider waiting

3. **Priority: Speed vs. Clean API?**  
   Speed → Add quick hacks/workarounds  
   Clean → Do it right with upstream changes

---

## Conclusion

We've made significant architectural progress but are blocked by API incompatibilities in `feagi-data-processing` beta.56. The vision is clear, the structure is sound, but we need either upstream API fixes or workarounds to proceed.

**Recommendation**: Pause Python bindings, fix `feagi-data-processing` API to be Python-binding-friendly, then resume.

