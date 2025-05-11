use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use rquest::Utf8Bytes;

use crate::{
    buffer::{BytesBuffer, PyBufferProtocol},
    error::Error,
    typing::Json,
};

/// A WebSocket message.
#[pyclass]
#[derive(Clone)]
pub struct Message(pub rquest::Message);

#[pymethods]
impl Message {
    /// Returns the JSON representation of the message.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        py.allow_threads(|| {
            self.0
                .json::<Json>()
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Returns the data of the message as bytes.
    #[getter]
    pub fn data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        let bytes = match &self.0 {
            rquest::Message::Text(text) => text.clone().into(),
            rquest::Message::Binary(bytes)
            | rquest::Message::Ping(bytes)
            | rquest::Message::Pong(bytes) => bytes.clone(),
            _ => return None,
        };
        BytesBuffer::new(bytes).into_bytes_ref(py).ok()
    }

    /// Returns the text content of the message if it is a text message.
    #[getter]
    pub fn text(&self) -> Option<&str> {
        if let rquest::Message::Text(text) = &self.0 {
            Some(text)
        } else {
            None
        }
    }

    /// Returns the binary data of the message if it is a binary message.
    #[getter]
    pub fn binary<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Binary(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the ping data of the message if it is a ping message.
    #[getter]
    pub fn ping<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Ping(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the pong data of the message if it is a pong message.
    #[getter]
    pub fn pong<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Pong(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the close code and reason of the message if it is a close message.
    #[getter]
    pub fn close(&self) -> Option<(u16, Option<&str>)> {
        if let rquest::Message::Close(Some(s)) = &self.0 {
            Some((s.code.0, Some(s.reason.as_str())))
        } else {
            None
        }
    }
}

#[pymethods]
impl Message {
    /// Creates a new text message from the JSON representation.
    #[staticmethod]
    #[pyo3(signature = (json))]
    pub fn text_from_json(py: Python, json: Json) -> PyResult<Self> {
        py.allow_threads(|| {
            rquest::Message::text_from_json(&json)
                .map(Message)
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Creates a new binary message from the JSON representation.
    #[staticmethod]
    #[pyo3(signature = (json))]
    pub fn binary_from_json(py: Python, json: Json) -> PyResult<Self> {
        py.allow_threads(|| {
            rquest::Message::binary_from_json(&json)
                .map(Message)
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Creates a new text message.
    #[staticmethod]
    #[pyo3(signature = (text))]
    pub fn from_text(text: PyBackedStr) -> Self {
        let msg = rquest::Message::text(Utf8Bytes::from_bytes_unchecked(Bytes::from_owner(text)));
        Message(msg)
    }

    /// Creates a new binary message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_binary(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::binary(Bytes::from_owner(data));
        Message(msg)
    }

    /// Creates a new ping message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_ping(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::ping(Bytes::from_owner(data));
        Message(msg)
    }

    /// Creates a new pong message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_pong(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::pong(Bytes::from_owner(data));
        Message(msg)
    }

    /// Creates a new close message.
    #[staticmethod]
    #[pyo3(signature = (code, reason=None))]
    pub fn from_close(code: u16, reason: Option<PyBackedStr>) -> Self {
        let reason = reason
            .map(Bytes::from_owner)
            .map(Utf8Bytes::from_bytes_unchecked)
            .unwrap_or_else(|| rquest::Utf8Bytes::from_static("Goodbye"));
        let msg = rquest::Message::close(rquest::CloseFrame {
            code: rquest::CloseCode(code),
            reason,
        });
        Message(msg)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
