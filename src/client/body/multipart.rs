use std::path::PathBuf;

use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
    types::PyTuple,
};
use wreq::{Body, multipart};

use crate::{client::body::PyStream, error::Error, header::HeaderMap};

/// A multipart form for a request.
#[pyclass(subclass)]
pub struct Multipart {
    pub form: Option<multipart::Form>,
    pub parts: Vec<Part>,
}

/// The data for a part value of a multipart form.
#[derive(FromPyObject)]
pub enum Value {
    Text(PyBackedStr),
    Bytes(PyBackedBytes),
    File(PathBuf),
    Stream(PyStream),
}

/// A part of a multipart form.
#[pyclass(subclass)]
pub struct Part {
    pub name: String,
    pub value: Option<Value>,
    pub filename: Option<String>,
    pub mime: Option<String>,
    pub length: Option<u64>,
    pub headers: Option<HeaderMap>,
}

// ===== impl Multipart =====

#[pymethods]
impl Multipart {
    /// Creates a new multipart.
    #[new]
    #[pyo3(signature = (*parts))]
    pub fn new(py: Python, parts: &Bound<PyTuple>) -> PyResult<Multipart> {
        let mut new_parts = Vec::with_capacity(parts.len());
        for part in parts {
            let part = part.cast::<Part>()?;
            let mut part = part.borrow_mut();
            new_parts.push(part.try_clone(py)?);
        }

        Ok(Self {
            form: None,
            parts: new_parts,
        })
    }
}

impl Multipart {
    fn build_form(&mut self, py: Python) -> PyResult<multipart::Form> {
        let mut form = multipart::Form::new();
        for part in &mut self.parts {
            let (name, inner) = part.build_form_part(py)?;
            form = form.part(name, inner);
        }
        Ok(form)
    }
}

impl FromPyObject<'_, '_> for Multipart {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        let multipart = ob.cast::<Multipart>()?;
        let mut multipart = multipart.borrow_mut();
        let form = multipart.build_form(ob.py())?;

        Ok(Multipart {
            form: Some(form),
            parts: Vec::new(),
        })
    }
}

// ===== impl Value =====

impl Value {
    fn try_clone(&self, py: Python) -> Option<Self> {
        match self {
            Value::Text(text) => {
                let text = text.clone_ref(py);
                Some(Value::Text(text))
            }
            Value::Bytes(bytes) => {
                let bytes = bytes.clone_ref(py);
                Some(Value::Bytes(bytes))
            }
            Value::File(path) => {
                let path = path.clone();
                Some(Value::File(path))
            }
            Value::Stream(_) => None,
        }
    }
}

// ===== impl Part =====

impl Part {
    fn with_value(&self, value: Value) -> Part {
        Part {
            name: self.name.clone(),
            value: Some(value),
            filename: self.filename.clone(),
            mime: self.mime.clone(),
            length: self.length,
            headers: self.headers.clone(),
        }
    }

    fn build_inner(value: Value, length: Option<u64>) -> Result<multipart::Part, Error> {
        Ok(match value {
            Value::Text(text) => multipart::Part::stream(Body::from(Bytes::from_owner(text))),
            Value::Bytes(bytes) => multipart::Part::stream(Body::from(Bytes::from_owner(bytes))),
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
        })
    }

    fn clone_value_or_take(&mut self, py: Python) -> PyResult<Value> {
        self.value
            .as_ref()
            .and_then(|value| value.try_clone(py))
            .or_else(|| self.value.take())
            .ok_or_else(|| Error::Memory.into())
    }

    fn build_form_part(&mut self, py: Python) -> PyResult<(String, multipart::Part)> {
        let value = self.clone_value_or_take(py)?;
        let name = self.name.clone();
        let filename = self.filename.clone();
        let mime = self.mime.clone();
        let length = self.length;
        let headers = self.headers.clone();

        py.detach(move || {
            let mut inner = Self::build_inner(value, length)?;

            if let Some(filename) = filename {
                inner = inner.file_name(filename);
            }

            if let Some(mime) = mime {
                inner = inner.mime_str(&mime).map_err(Error::Library)?;
            }

            if let Some(headers) = headers {
                inner = inner.headers(headers.0);
            }

            Ok((name, inner))
        })
    }

    fn try_clone(&mut self, py: Python) -> PyResult<Part> {
        if let Some(part) = self
            .value
            .as_ref()
            .and_then(|value| value.try_clone(py))
            .map(|value| self.with_value(value))
        {
            return Ok(part);
        }

        self.value
            .take()
            .map(|value| self.with_value(value))
            .ok_or_else(|| Error::Memory)
            .map_err(Into::into)
    }
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
        name: String,
        value: Value,
        filename: Option<String>,
        mime: Option<&str>,
        length: Option<u64>,
        headers: Option<HeaderMap>,
    ) -> Part {
        Part {
            name,
            value: Some(value),
            filename,
            mime: mime.map(ToOwned::to_owned),
            length,
            headers,
        }
    }
}
