pub mod cortical_type;
mod cortical_id;

pub mod descriptors;
mod brain_regions;
mod cortical_areas;
mod motor_cortical_unit;
mod sensory_cortical_unit;

pub use cortical_id::PyCorticalID;
pub use cortical_type::{/* PyCorticalType, */ PyCoreCorticalType, PyMotorCorticalType, PySensorCorticalType};