pub mod cortical_type;
mod cortical_id;

pub mod descriptors;
mod brain_regions;
mod cortical_areas;

pub use cortical_id::PyCorticalID;
pub use cortical_type::{/* PyCorticalType, */ PySensorCorticalType, PyCoreCorticalType, PyMotorCorticalType};