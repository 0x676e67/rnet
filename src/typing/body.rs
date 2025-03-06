use crate::error::stream_consumed_error;
use crate::stream::{AsyncStream, SyncStream};
use arc_swap::ArcSwapOption;
use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::pybacked::{PyBackedBytes, PyBackedStr};
use pyo3::{FromPyObject, PyAny};
use std::sync::Arc;

/// The body to use for the request.
pub enum FromPyBody {
    Text(Bytes),
    Bytes(Bytes),
    Iterator(Arc<ArcSwapOption<SyncStream>>),
    Stream(Arc<ArcSwapOption<AsyncStream>>),
}

impl TryFrom<FromPyBody> for rquest::Body {
    type Error = PyErr;

    fn try_from(value: FromPyBody) -> Result<rquest::Body, Self::Error> {
        match value {
            FromPyBody::Text(bytes) | FromPyBody::Bytes(bytes) => Ok(rquest::Body::from(bytes)),
            FromPyBody::Iterator(iterator) => iterator
                .swap(None)
                .and_then(Arc::into_inner)
                .map(Into::into)
                .ok_or_else(stream_consumed_error),
            FromPyBody::Stream(stream) => stream
                .swap(None)
                .and_then(Arc::into_inner)
                .map(Into::into)
                .ok_or_else(stream_consumed_error),
        }
    }
}

impl FromPyObject<'_> for FromPyBody {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Text(Bytes::from(text.as_bytes().to_vec())));
        }

        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from(bytes.as_ref().to_vec())));
        }

        if ob.hasattr("asend")? {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
                .map(AsyncStream::new)
                .map(ArcSwapOption::from_pointee)
                .map(Arc::new)
                .map(Self::Stream)
        } else {
            ob.extract::<PyObject>()
                .map(SyncStream::new)
                .map(ArcSwapOption::from_pointee)
                .map(Arc::new)
                .map(Self::Iterator)
        }
    }
}
