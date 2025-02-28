use std::fmt::Debug;
use std::sync::Arc;

use crate::error::stream_consumed_error;
use crate::stream::{AsyncStream, SyncStream};
use arc_swap::ArcSwapOption;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{FromPyObject, IntoPyObject, PyAny};
use pyo3_stub_gen::{PyStubType, TypeInfo};

/// The body to use for the request.
#[derive(Clone)]
pub enum Body {
    Text(String),
    Bytes(Vec<u8>),
    Iterator(Arc<ArcSwapOption<SyncStream>>),
    Stream(Arc<ArcSwapOption<AsyncStream>>),
}

impl TryFrom<Body> for rquest::Body {
    type Error = PyErr;

    fn try_from(value: Body) -> Result<rquest::Body, Self::Error> {
        match value {
            Body::Text(text) => Ok(rquest::Body::from(text)),
            Body::Bytes(bytes) => Ok(rquest::Body::from(bytes)),
            Body::Iterator(iterator) => iterator
                .swap(None)
                .and_then(Arc::into_inner)
                .map(Into::into)
                .ok_or_else(stream_consumed_error),
            Body::Stream(stream) => stream
                .swap(None)
                .and_then(Arc::into_inner)
                .map(Into::into)
                .ok_or_else(stream_consumed_error),
        }
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(inner) => write!(f, "Body::Text({:?})", inner),
            Self::Bytes(inner) => write!(f, "Body::Bytes({:?})", inner),
            Self::Iterator(_) => write!(f, "Body::Iterator(...)"),
            Self::Stream(_) => write!(f, "Body::Stream(...)"),
        }
    }
}

impl PyStubType for Body {
    fn type_output() -> TypeInfo {
        TypeInfo::any()
    }
}

impl FromPyObject<'_> for Body {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<String>() {
            Ok(Self::Text(text))
        } else if let Ok(bytes) = ob.downcast::<PyBytes>() {
            Ok(Self::Bytes(bytes.as_bytes().to_vec()))
        } else if let Ok(iter) = ob.extract::<PyObject>() {
            Ok(Self::Iterator(Arc::new(ArcSwapOption::from_pointee(
                SyncStream::new(iter),
            ))))
        } else {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
                .map(AsyncStream::new)
                .map(ArcSwapOption::from_pointee)
                .map(Arc::new)
                .map(Self::Stream)
        }
    }
}

impl<'rt> IntoPyObject<'rt> for Body {
    type Error = PyErr;
    type Output = Bound<'rt, Self::Target>;
    type Target = PyAny;

    fn into_pyobject(self, _: Python<'rt>) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
