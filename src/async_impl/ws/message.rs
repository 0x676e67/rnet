use pyo3::{prelude::*, types::PyBytes};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::{error::wrap_rquest_error, types::Json};

/// A WebSocket message.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Message(pub rquest::Message);

#[gen_stub_pymethods]
#[pymethods]
impl Message {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

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
    pub fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let bytes = match &self.0 {
            rquest::Message::Text(text) => text.as_bytes(),
            rquest::Message::Binary(bytes)
            | rquest::Message::Ping(bytes)
            | rquest::Message::Pong(bytes) => bytes,
            _ => &[],
        };
        PyBytes::new(py, bytes)
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
    pub fn binary<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        if let rquest::Message::Binary(data) = &self.0 {
            Some(PyBytes::new(py, data))
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
    pub fn ping<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        if let rquest::Message::Ping(data) = &self.0 {
            Some(PyBytes::new(py, data))
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
    pub fn pong<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        if let rquest::Message::Pong(data) = &self.0 {
            Some(PyBytes::new(py, data))
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
        if let rquest::Message::Close { code, reason } = &self.0 {
            Some((u16::from(*code), reason.as_deref()))
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
    pub fn from_text(text: &str) -> Self {
        Message(rquest::Message::Text(text.to_owned()))
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
    pub fn from_binary(data: &Bound<PyBytes>) -> Self {
        Message(rquest::Message::Binary(data.as_bytes().to_vec()))
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
    pub fn from_ping(data: &Bound<PyBytes>) -> Self {
        Message(rquest::Message::Ping(data.as_bytes().to_vec()))
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
    pub fn from_pong(data: &Bound<PyBytes>) -> Self {
        Message(rquest::Message::Pong(data.as_bytes().to_vec()))
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
        Message(rquest::Message::Close {
            code: rquest::CloseCode::from(code),
            reason,
        })
    }
}
