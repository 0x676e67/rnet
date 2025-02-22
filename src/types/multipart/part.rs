use crate::error::{wrap_io_error, MIMEParseError};
use pyo3::prelude::*;
use pyo3_stub_gen::{
    derive::{gen_stub_pyclass, gen_stub_pymethods},
    PyStubType, TypeInfo,
};
use std::path::PathBuf;

/// A part of a multipart form.
#[gen_stub_pyclass]
#[pyclass]
pub struct Part {
    pub name: Option<String>,
    pub inner: Option<rquest::multipart::Part>,
}

/// The data for a part of a multipart form.
#[derive(Debug, FromPyObject)]
pub enum PartData {
    Text(String),
    Bytes(Vec<u8>),
    File(PathBuf),
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
        name: String,
        value: PartData,
        filename: Option<String>,
        mime: Option<&str>,
    ) -> PyResult<Part> {
        // Create the inner part
        let mut inner = match value {
            PartData::Text(text) => rquest::multipart::Part::text(text),
            PartData::Bytes(bytes) => rquest::multipart::Part::bytes(bytes),
            PartData::File(path) => pyo3_async_runtimes::tokio::get_runtime()
                .block_on(rquest::multipart::Part::file(path))
                .map_err(wrap_io_error)?,
        };

        // Set the filename and MIME type if provided
        if let Some(filename) = filename {
            inner = inner.file_name(filename);
        }

        // Set the MIME type if provided
        if let Some(mime) = mime {
            inner = inner
                .mime_str(mime)
                .map_err(|e| MIMEParseError::new_err(format!("Cannot parse MIME type: {:?}", e)))?;
        }

        Ok(Part {
            name: Some(name),
            inner: Some(inner),
        })
    }
}
