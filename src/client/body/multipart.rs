use std::path::PathBuf;

use bytes::Bytes;
use pyo3::{
    intern,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
    types::PyTuple,
};
use wreq::{Body, header::HeaderMap, multipart, multipart::Form};

use crate::{
    bridge::Runtime,
    client::body::{AsyncStream, SyncStream},
    error::Error,
    extractor::Extractor,
};

/// A multipart form for a request.
#[pyclass(subclass)]
pub struct Multipart(pub Option<Form>);

#[pymethods]
impl Multipart {
    /// Creates a new multipart form.
    #[new]
    #[pyo3(signature = (*parts))]
    pub fn new(parts: &Bound<PyTuple>) -> PyResult<Multipart> {
        let mut form = Form::new();
        for part in parts {
            let part = part.cast::<Part>()?;
            let mut part = part.borrow_mut();
            form = part
                .name
                .take()
                .zip(part.inner.take())
                .map(|(name, inner)| form.part(name, inner))
                .ok_or_else(|| Error::Memory)?;
        }
        Ok(Multipart(Some(form)))
    }
}

/// A part of a multipart form.
#[pyclass(subclass)]
pub struct Part {
    pub name: Option<String>,
    pub inner: Option<multipart::Part>,
}

/// The data for a part value of a multipart form.
pub enum Value {
    Text(Bytes),
    Bytes(Bytes),
    File(PathBuf),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

#[pymethods]
impl Part {
    /// Creates a new part.
    #[new]
    #[pyo3(signature = (
        name,
        value,
        filename = None,
        mime = None,
        length = None,
        headers = None
    ))]
    pub fn new(
        py: Python,
        name: String,
        value: Value,
        filename: Option<String>,
        mime: Option<&str>,
        length: Option<u64>,
        headers: Option<Extractor<HeaderMap>>,
    ) -> PyResult<Part> {
        py.detach(|| {
            // Create the inner part
            let mut inner = match value {
                Value::Text(bytes) | Value::Bytes(bytes) => {
                    multipart::Part::stream(Body::from(bytes))
                }
                Value::File(path) => {
                    Runtime::block_on(multipart::Part::file(path)).map_err(Error::from)?
                }
                Value::SyncStream(stream) => {
                    let stream = Body::wrap_stream(stream);
                    match length {
                        Some(length) => multipart::Part::stream_with_length(stream, length),
                        None => multipart::Part::stream(stream),
                    }
                }
                Value::AsyncStream(stream) => {
                    let stream = Body::wrap_stream(stream);
                    match length {
                        Some(length) => multipart::Part::stream_with_length(stream, length),
                        None => multipart::Part::stream(stream),
                    }
                }
            };

            // Set the filename and MIME type if provided
            if let Some(filename) = filename {
                inner = inner.file_name(filename);
            }

            // Set the MIME type if provided
            if let Some(mime) = mime {
                inner = inner.mime_str(mime).map_err(Error::Library)?;
            }

            // Set the headers if provided
            if let Some(headers) = headers {
                inner = inner.headers(headers.0);
            }

            Ok(Part {
                name: Some(name),
                inner: Some(inner),
            })
        })
    }
}

impl FromPyObject<'_> for Value {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Try extracting string
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Value::Text(Bytes::from_owner(text)));
        }

        // Try extracting bytes
        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Value::Bytes(Bytes::from_owner(bytes)));
        }

        // Try extracting file path
        if let Ok(path) = ob.extract::<PathBuf>() {
            return Ok(Value::File(path));
        }

        // Determine if it's an async or sync stream
        if ob.hasattr(intern!(ob.py(), "asend"))? {
            Runtime::into_stream(ob.to_owned())
                .map(AsyncStream::new)
                .map(Value::AsyncStream)
        } else {
            ob.extract::<Py<PyAny>>()
                .map(SyncStream::new)
                .map(Value::SyncStream)
        }
    }
}
