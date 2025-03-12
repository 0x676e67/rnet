use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::Utf8Bytes;

use crate::{
    buffer::{BytesBuffer, PyBufferProtocol},
    error::wrap_rquest_error,
    typing::Json,
};

/// A WebSocket message.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Message(pub rquest::Message);

#[gen_stub_pymethods]
#[pymethods]
impl Message {
    /// Returns the JSON representation of the message.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing the JSON representation of the message.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        py.allow_threads(|| self.0.json::<Json>().map_err(wrap_rquest_error))
    }

    /// Returns the data of the message as bytes.
    ///
    /// # Returns
    ///
    /// A byte slice representing the data of the message.
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
    ///
    /// # Returns
    ///
    /// An optional string representing the text content of the message.
    #[getter]
    pub fn text(&self) -> Option<&str> {
        if let rquest::Message::Text(text) = &self.0 {
            Some(text)
        } else {
            None
        }
    }

    /// Returns the binary data of the message if it is a binary message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the binary data of the message.
    #[getter]
    pub fn binary<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Binary(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the ping data of the message if it is a ping message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the ping data of the message.
    #[getter]
    pub fn ping<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Ping(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the pong data of the message if it is a pong message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the pong data of the message.
    #[getter]
    pub fn pong<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        if let rquest::Message::Pong(data) = &self.0 {
            BytesBuffer::new(data.clone()).into_bytes_ref(py).ok()
        } else {
            None
        }
    }

    /// Returns the close code and reason of the message if it is a close message.
    ///
    /// # Returns
    ///
    /// An optional tuple containing the close code and reason.
    #[getter]
    pub fn close(&self) -> Option<(u16, Option<&str>)> {
        if let rquest::Message::Close(Some(s)) = &self.0 {
            Some((s.code.0, Some(s.reason.as_str())))
        } else {
            None
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Message {
    /// Creates a new text message from the JSON representation.
    ///
    /// # Arguments
    /// * `json` - The JSON representation of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the message.
    #[staticmethod]
    #[pyo3(signature = (json))]
    #[inline(always)]
    pub fn text_from_json(py: Python, json: Json) -> PyResult<Self> {
        py.allow_threads(|| {
            rquest::Message::text_from_json(&json)
                .map(Message)
                .map_err(wrap_rquest_error)
        })
    }

    /// Creates a new binary message from the JSON representation.
    ///
    /// # Arguments
    /// * `json` - The JSON representation of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the message.
    #[staticmethod]
    #[pyo3(signature = (json))]
    #[inline(always)]
    pub fn binary_from_json(py: Python, json: Json) -> PyResult<Self> {
        py.allow_threads(|| {
            rquest::Message::binary_from_json(&json)
                .map(Message)
                .map_err(wrap_rquest_error)
        })
    }

    /// Creates a new text message.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the text message.
    #[staticmethod]
    #[pyo3(signature = (text))]
    #[inline(always)]
    pub fn from_text(text: PyBackedStr) -> Self {
        let msg = rquest::Message::text(text.as_ref() as &str);
        Message(msg)
    }

    /// Creates a new binary message.
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the binary message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline(always)]
    pub fn from_binary(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::binary(data.as_ref().to_owned());
        Message(msg)
    }

    /// Creates a new ping message.
    ///
    /// # Arguments
    ///
    /// * `data` - The ping data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the ping message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline(always)]
    pub fn from_ping(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::ping(data.as_ref().to_owned());
        Message(msg)
    }

    /// Creates a new pong message.
    ///
    /// # Arguments
    ///
    /// * `data` - The pong data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the pong message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline(always)]
    pub fn from_pong(data: PyBackedBytes) -> Self {
        let msg = rquest::Message::pong(data.as_ref().to_owned());
        Message(msg)
    }

    /// Creates a new close message.
    ///
    /// # Arguments
    ///
    /// * `code` - The close code.
    /// * `reason` - An optional reason for closing.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the close message.
    #[staticmethod]
    #[pyo3(signature = (code, reason=None))]
    #[inline(always)]
    pub fn from_close(code: u16, reason: Option<String>) -> Self {
        let msg = rquest::Message::close(rquest::CloseFrame {
            code: rquest::CloseCode(code),
            reason: Utf8Bytes::from(reason.as_deref().unwrap_or("Goodbye")),
        });
        Message(msg)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Message {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
