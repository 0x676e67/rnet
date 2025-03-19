use crate::stream::{AsyncStream, SyncStream};
use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::pybacked::{PyBackedBytes, PyBackedStr};
use pyo3::{FromPyObject, PyAny};

/// The body to use for the request.
pub enum FromPyBody {
    Text(Bytes),
    Bytes(Bytes),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

impl From<FromPyBody> for rquest::Body {
    fn from(value: FromPyBody) -> rquest::Body {
        match value {
            FromPyBody::Text(bytes) | FromPyBody::Bytes(bytes) => rquest::Body::from(bytes),
            FromPyBody::SyncStream(stream) => rquest::Body::from(stream),
            FromPyBody::AsyncStream(stream) => rquest::Body::from(stream),
        }
    }
}

impl FromPyObject<'_> for FromPyBody {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Text(Bytes::from_owner(text)));
        }

        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from_owner(bytes)));
        }

        if ob.hasattr("asend")? {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
                .map(AsyncStream::new)
                .map(Self::AsyncStream)
        } else {
            ob.extract::<PyObject>()
                .map(SyncStream::new)
                .map(Self::SyncStream)
        }
    }
}
