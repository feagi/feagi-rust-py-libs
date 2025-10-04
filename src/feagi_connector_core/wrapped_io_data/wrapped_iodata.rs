use feagi_connector_core::data_types::{ImageFrame, MiscData, Percentage, Percentage2D, Percentage3D, Percentage4D, SegmentedImageFrame, SignedPercentage, SignedPercentage2D, SignedPercentage3D, SignedPercentage4D};
use feagi_connector_core::wrapped_io_data::WrappedIOData;
use pyo3::{PyResult };
use pyo3::prelude::*;
use feagi_data_structures::FeagiDataError;
use pyo3::types::PyFloat;
use crate::feagi_connector_core::data::{PyImageFrame, PyMiscData, PySegmentedImageFrame, PyPercentage, PyPercentage2D, PyPercentage3D, PyPercentage4D, PySignedPercentage, PySignedPercentage2D, PySignedPercentage3D, PySignedPercentage4D};
use crate::{project_display, py_object_cast_generic, py_type_casts};
use crate::py_error::PyFeagiError;


// Conversion functions for backward compatibility
pub fn wrapped_io_data_to_py_object(wrapped_iodata: WrappedIOData) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        Ok(match wrapped_iodata {
            WrappedIOData::F32(number) => PyFloat::new(py, number as f64).into(),
            WrappedIOData::F32_2D(tuple) => tuple.into_py(py),
            WrappedIOData::F32_3D(tuple) => tuple.into_py(py),
            WrappedIOData::F32_4D(tuple) => tuple.into_py(py),
            WrappedIOData::Percentage(percentage) => {
                let py_percentage = PyPercentage::from(percentage);
                py_percentage.into_py(py)
            },
            WrappedIOData::Percentage_2D(percentage_2d) => {
                let py_percentage_2d = PyPercentage2D::from(percentage_2d);
                py_percentage_2d.into_py(py)
            },
            WrappedIOData::Percentage_3D(percentage_3d) => {
                let py_percentage_3d = PyPercentage3D::from(percentage_3d);
                py_percentage_3d.into_py(py)
            },
            WrappedIOData::Percentage_4D(percentage_4d) => {
                let py_percentage_4d = PyPercentage4D::from(percentage_4d);
                py_percentage_4d.into_py(py)
            },
            WrappedIOData::SignedPercentage(signed_percentage) => {
                let py_signed_percentage = PySignedPercentage::from(signed_percentage);
                py_signed_percentage.into_py(py)
            },
            WrappedIOData::SignedPercentage_2D(signed_percentage_2d) => {
                let py_signed_percentage_2d = PySignedPercentage2D::from(signed_percentage_2d);
                py_signed_percentage_2d.into_py(py)
            },
            WrappedIOData::SignedPercentage_3D(signed_percentage_3d) => {
                let py_signed_percentage_3d = PySignedPercentage3D::from(signed_percentage_3d);
                py_signed_percentage_3d.into_py(py)
            },
            WrappedIOData::SignedPercentage_4D(signed_percentage_4d) => {
                let py_signed_percentage_4d = PySignedPercentage4D::from(signed_percentage_4d);
                py_signed_percentage_4d.into_py(py)
            },
            WrappedIOData::ImageFrame(frame) => {
                let py_frame = PyImageFrame::from(frame);
                py_frame.into_py(py)
            },
            WrappedIOData::SegmentedImageFrame(segmented_frame) => {
                let py_segmented_frame = PySegmentedImageFrame::from(segmented_frame);
                py_segmented_frame.into_py(py)
            },
            WrappedIOData::MiscData(misc_data) => {
                let py_misc_data = PyMiscData::from(misc_data);
                py_misc_data.into_py(py)
            },
        })
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
        _ if bound.is_instance_of::<PyMiscData>() => {
            let py_obj = py_wrapped.extract::<PyMiscData>(py).unwrap();
            let misc_data: MiscData = py_obj.into();
            Ok(misc_data.into())
        }
        _ if bound.is_instance_of::<PyPercentage>() => {
            let py_obj = py_wrapped.extract::<PyPercentage>(py).unwrap();
            let percentage: Percentage = py_obj.into();
            Ok(percentage.into())
        }
        _ if bound.is_instance_of::<PyPercentage2D>() => {
            let py_obj = py_wrapped.extract::<PyPercentage2D>(py).unwrap();
            let percentage_2d: Percentage2D = py_obj.into();
            Ok(percentage_2d.into())
        }
        _ if bound.is_instance_of::<PyPercentage3D>() => {
            let py_obj = py_wrapped.extract::<PyPercentage3D>(py).unwrap();
            let percentage_3d: Percentage3D = py_obj.into();
            Ok(percentage_3d.into())
        }
        _ if bound.is_instance_of::<PyPercentage4D>() => {
            let py_obj = py_wrapped.extract::<PyPercentage4D>(py).unwrap();
            let percentage_4d: Percentage4D = py_obj.into();
            Ok(percentage_4d.into())
        }
        _ if bound.is_instance_of::<PySignedPercentage>() => {
            let py_obj = py_wrapped.extract::<PySignedPercentage>(py).unwrap();
            let signed_percentage: SignedPercentage = py_obj.into();
            Ok(signed_percentage.into())
        }
        _ if bound.is_instance_of::<PySignedPercentage2D>() => {
            let py_obj = py_wrapped.extract::<PySignedPercentage2D>(py).unwrap();
            let signed_percentage_2d: SignedPercentage2D = py_obj.into();
            Ok(signed_percentage_2d.into())
        }
        _ if bound.is_instance_of::<PySignedPercentage3D>() => {
            let py_obj = py_wrapped.extract::<PySignedPercentage3D>(py).unwrap();
            let signed_percentage_3d: SignedPercentage3D = py_obj.into();
            Ok(signed_percentage_3d.into())
        }
        _ if bound.is_instance_of::<PySignedPercentage4D>() => {
            let py_obj = py_wrapped.extract::<PySignedPercentage4D>(py).unwrap();
            let signed_percentage_4d: SignedPercentage4D = py_obj.into();
            Ok(signed_percentage_4d.into())
        }
        _ if bound.is_instance_of::<PyFloat>() => {
            let py_f32 = py_wrapped.extract::<f32>(py).unwrap();
            Ok(WrappedIOData::F32(py_f32))
        }
        // Handle tuples for multi-dimensional f32 types
        _ => {
            // Try to extract as various tuple types
            if let Ok(tuple_2d) = py_wrapped.extract::<(f32, f32)>(py) {
                Ok(WrappedIOData::F32_2D(tuple_2d))
            } else if let Ok(tuple_3d) = py_wrapped.extract::<(f32, f32, f32)>(py) {
                Ok(WrappedIOData::F32_3D(tuple_3d))
            } else if let Ok(tuple_4d) = py_wrapped.extract::<(f32, f32, f32, f32)>(py) {
                Ok(WrappedIOData::F32_4D(tuple_4d))
            } else {
                Err(FeagiDataError::BadParameters("Unable to convert given data into Wrapped IO Data!".into()))
            }
        }
    }
}