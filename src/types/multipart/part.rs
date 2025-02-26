use crate::{
    error::{stream_consumed_error, wrap_io_error, MIMEParseError},
    types::PyIterator,
};
use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::StreamExt;
use pyo3::prelude::*;
use pyo3_stub_gen::{
    derive::{gen_stub_pyclass, gen_stub_pymethods},
    PyStubType, TypeInfo,
};
use std::{fmt::Debug, path::PathBuf, pin::Pin, sync::Arc};

/// A part of a multipart form.
#[gen_stub_pyclass]
#[pyclass]
pub struct Part {
    pub name: Option<String>,
    pub inner: Option<rquest::multipart::Part>,
}

/// The data for a part of a multipart form.
pub enum PartData {
    Text(String),
    Bytes(Vec<u8>),
    File(PathBuf),
    Iterator(Arc<ArcSwapOption<Box<dyn Iterator<Item = Vec<u8>> + Send + Sync + 'static>>>),
    Stream(
        Arc<
            ArcSwapOption<
                Pin<Box<dyn futures_util::Stream<Item = PyObject> + Send + Sync + 'static>>,
            >,
        >,
    ),
}

impl Debug for PartData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(inner) => write!(f, "PartData::Text({:?})", inner),
            Self::Bytes(inner) => write!(f, "PartData::Bytes({:?})", inner),
            Self::File(inner) => write!(f, "PartData::File({:?})", inner),
            Self::Iterator(_) => write!(f, "PartData::Iterator(...)"),
            Self::Stream(_) => write!(f, "PartData::Stream(...)"),
        }
    }
}

impl PyStubType for PartData {
    fn type_output() -> TypeInfo {
        TypeInfo::any()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Part {
    /// Creates a new part.
    #[new]
    #[pyo3(signature = (name, value, filename = None, mime = None))]
    pub fn new(
        py: Python,
        name: String,
        value: PartData,
        filename: Option<String>,
        mime: Option<&str>,
    ) -> PyResult<Part> {
        py.allow_threads(|| {
            // Create the inner part
            let mut inner = match value {
                PartData::Text(text) => rquest::multipart::Part::text(text),
                PartData::Bytes(bytes) => rquest::multipart::Part::bytes(bytes),
                PartData::File(path) => pyo3_async_runtimes::tokio::get_runtime()
                    .block_on(rquest::multipart::Part::file(path))
                    .map_err(wrap_io_error)?,
                PartData::Iterator(iterator) => iterator
                    .swap(None)
                    .and_then(Arc::into_inner)
                    .map(|iter| iter.map(|item| Ok::<_, PyErr>(Bytes::from(item))))
                    .map(futures_util::stream::iter)
                    .map(rquest::Body::wrap_stream)
                    .map(rquest::multipart::Part::stream)
                    .ok_or_else(stream_consumed_error)?,
                PartData::Stream(stream) => stream
                    .swap(None)
                    .and_then(Arc::into_inner)
                    .map(|stream| {
                        stream.map(|item| Python::with_gil(|py| item.extract::<Vec<u8>>(py)))
                    })
                    .map(rquest::Body::wrap_stream)
                    .map(rquest::multipart::Part::stream)
                    .ok_or_else(stream_consumed_error)?,
            };

            // Set the filename and MIME type if provided
            if let Some(filename) = filename {
                inner = inner.file_name(filename);
            }

            // Set the MIME type if provided
            if let Some(mime) = mime {
                inner = inner.mime_str(mime).map_err(|e| {
                    MIMEParseError::new_err(format!("Cannot parse MIME type: {:?}", e))
                })?;
            }

            Ok(Part {
                name: Some(name),
                inner: Some(inner),
            })
        })
    }
}

impl FromPyObject<'_> for PartData {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<String>() {
            Ok(Self::Text(text))
        } else if let Ok(bytes) = ob.extract::<Vec<u8>>() {
            Ok(Self::Bytes(bytes))
        } else if let Ok(path) = ob.extract::<PathBuf>() {
            Ok(Self::File(path))
        } else if let Ok(iter) = ob.extract::<PyObject>() {
            Ok(Self::Iterator(Arc::new(ArcSwapOption::from_pointee(
                Box::new(PyIterator::new(iter))
                    as Box<dyn Iterator<Item = Vec<u8>> + Send + Sync + 'static>,
            ))))
        } else {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
                .map(|s| {
                    Box::pin(s)
                        as Pin<
                            Box<dyn futures_util::Stream<Item = PyObject> + Send + Sync + 'static>,
                        >
                })
                .map(ArcSwapOption::from_pointee)
                .map(Arc::new)
                .map(Self::Stream)
        }
    }
}
