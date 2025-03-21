use crate::{
    error::{Error, MIMEParseError},
    stream::{AsyncStream, SyncStream},
};
use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
#[cfg(feature = "docs")]
use pyo3_stub_gen::{
    PyStubType, TypeInfo,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};
use rquest::Body;
use std::path::PathBuf;

/// A part of a multipart form.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct Part {
    pub name: Option<String>,
    pub inner: Option<rquest::multipart::Part>,
}

/// The data for a part of a multipart form.
pub enum PartData {
    Text(Bytes),
    Bytes(Bytes),
    File(PathBuf),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

#[cfg(feature = "docs")]
impl PyStubType for PartData {
    fn type_output() -> TypeInfo {
        TypeInfo::any()
    }
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
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
                PartData::Text(bytes) | PartData::Bytes(bytes) => {
                    rquest::multipart::Part::stream(Body::from(bytes))
                }
                PartData::File(path) => pyo3_async_runtimes::tokio::get_runtime()
                    .block_on(rquest::multipart::Part::file(path))
                    .map_err(Error::from)?,
                PartData::SyncStream(stream) => {
                    rquest::multipart::Part::stream(Body::wrap_stream(stream))
                }
                PartData::AsyncStream(stream) => {
                    rquest::multipart::Part::stream(Body::wrap_stream(stream))
                }
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
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Text(Bytes::from_owner(text)));
        }

        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from_owner(bytes)));
        }

        if let Ok(path) = ob.extract::<PathBuf>() {
            return Ok(Self::File(path));
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
