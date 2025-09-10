use feagi_connector_core::data_pipeline::PipelineStage;
use feagi_data_structures::FeagiDataError;

pub trait PipelineStagePyTrait {
    fn copy_as_box(&self) -> Result<Box<dyn PipelineStage>, FeagiDataError>;
    
    
}