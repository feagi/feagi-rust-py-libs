use pyo3::{pyclass};
use pyo3::prelude::*;
use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronCoderVariantType;


#[pyclass(eq, eq_int)]
#[derive(Clone, Hash, PartialEq)]
#[pyo3(name = "NeuronCoderVariantType")]
pub enum PyNeuronCoderVariantType{
    F32Normalized0To1_Linear,
    F32NormalizedM1To1_PSPBidirectional,
    F32NormalizedM1To1_SplitSignDivided,
    ImageFrame,
    SegmentedImageFrame,
}

impl From<NeuronCoderVariantType> for PyNeuronCoderVariantType{
    fn from(v:NeuronCoderVariantType) -> PyNeuronCoderVariantType{
        match v {
            NeuronCoderVariantType::F32Normalized0To1_Linear => PyNeuronCoderVariantType::F32Normalized0To1_Linear,
            NeuronCoderVariantType::F32NormalizedM1To1_PSPBidirectional => PyNeuronCoderVariantType::F32NormalizedM1To1_PSPBidirectional,
            NeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided => PyNeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided,
            NeuronCoderVariantType::ImageFrame => PyNeuronCoderVariantType::ImageFrame,
            NeuronCoderVariantType::SegmentedImageFrame => PyNeuronCoderVariantType::SegmentedImageFrame
        }
    }
}

impl From<PyNeuronCoderVariantType> for NeuronCoderVariantType{
    fn from(v:PyNeuronCoderVariantType) -> NeuronCoderVariantType{
        match v { 
            PyNeuronCoderVariantType::F32Normalized0To1_Linear => NeuronCoderVariantType::F32Normalized0To1_Linear,
            PyNeuronCoderVariantType::F32NormalizedM1To1_PSPBidirectional => NeuronCoderVariantType::F32NormalizedM1To1_PSPBidirectional,
            PyNeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided => NeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided,
            PyNeuronCoderVariantType::ImageFrame => NeuronCoderVariantType::ImageFrame,
            PyNeuronCoderVariantType::SegmentedImageFrame => NeuronCoderVariantType::SegmentedImageFrame
        }
    }
}