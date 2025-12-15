use std::path::PathBuf;

use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
    types::PyTuple,
};
use wreq::{Body, multipart, multipart::Form};

use crate::{client::body::PyStream, error::Error, header::HeaderMap};

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
#[derive(FromPyObject)]
pub enum Value {
    Text(PyBackedStr),
    Bytes(PyBackedBytes),
    File(PathBuf),
    Stream(PyStream),
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
        headers: Option<HeaderMap>,
    ) -> PyResult<Part> {
        py.detach(|| {
            // Create the inner part
            let mut inner = match value {
                Value::Text(text) => multipart::Part::stream(Body::from(Bytes::from_owner(text))),
                Value::Bytes(bytes) => {
                    multipart::Part::stream(Body::from(Bytes::from_owner(bytes)))
                }
                Value::File(path) => pyo3_async_runtimes::tokio::get_runtime()
                    .block_on(multipart::Part::file(path))
                    .map_err(Error::from)?,
                Value::Stream(stream) => {
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
