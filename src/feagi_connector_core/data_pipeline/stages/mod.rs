mod identities;
mod image_quick_diff;
mod image_segmentor;
mod image_transformer;

pub use image_quick_diff::PyImageFrameQuickDiffStage;
pub use identities::{PyIdentityFloatStage, PyIdentityImageFrameStage, PyIdentitySegmentedImageFrameStage};
pub use image_segmentor::PyImageFrameSegmentatorStage;
pub use image_transformer::PyImageFrameProcessorStage;