use feagi_data_structures::data::{ImageFrame, SegmentedImageFrame};
use pyo3::{PyResult};
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use feagi_data_structures::wrapped_io_data::WrappedIOData;
use pyo3::types::PyFloat;
use crate::feagi_data_structures::data::{PyImageFrame, PySegmentedImageFrame};
// NOTE: We don't need the actual data type as we just use PyObject. Instead, here are some conversion functions

pub fn wrapped_io_data_to_py_object(wrapped_iodata: WrappedIOData) -> PyResult<PyObject> {
    Ok(match wrapped_iodata {
        WrappedIOData::ImageFrame(frame) => frame.into(),
        WrappedIOData::SegmentedImageFrame(segmented_frame) => segmented_frame.into(),
        WrappedIOData::F32(number) => number.into(),
        WrappedIOData::F32Normalized0To1(number) => number.into(),
        WrappedIOData::F32NormalizedM1To1(number) => number.into(),
    })
}

pub fn py_object_to_wrapped_io_data<'py>(py: Python<'_>, py_wrapped: PyObject) -> Result<WrappedIOData, FeagiDataError> {
    let bound = py_wrapped.bind(py);

    match () {
        _ if bound.is_instance_of::<PyImageFrame>() => {
            let py_obj = py_wrapped.extract::<PyImageFrame>(py).unwrap();
            let image_frame: ImageFrame = py_obj.into();
            Ok(image_frame.into())
        }
        _ if bound.is_instance_of::<PySegmentedImageFrame>() => {
            let py_obj = py_wrapped.extract::<PySegmentedImageFrame>(py).unwrap();
            let segmented_frame: SegmentedImageFrame = py_obj.into();
            Ok(segmented_frame.into())
        }
        _ if bound.is_instance_of::<PyFloat>() => {
            let py_f32 = py_wrapped.extract::<f32>(py).unwrap();
            Ok(WrappedIOData::F32(py_f32))
        }
        _ => Err(FeagiDataError::BadParameters("Unable to convert given data into Wrapped IO Data!".into()))
    }
}