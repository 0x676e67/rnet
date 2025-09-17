//! WebSocket Message Utilities
//!
//! This module provides the `Message` type for representing WebSocket messages,
//! including text, binary, ping, pong, and close frames. It offers constructors
//! for creating messages of various types, as well as methods and getters for
//! extracting message content (such as text, binary data, ping/pong payloads, and close reason).
//!
//! The `Message` type is used for sending and receiving WebSocket messages in a unified way.

use std::fmt;

use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use wreq::ws::message::{self, CloseCode, CloseFrame, Utf8Bytes};

use crate::{buffer::PyBuffer, client::body::Json, error::Error};

/// A WebSocket message.
#[derive(Debug, Clone)]
#[pyclass(subclass, str, frozen)]
pub struct Message(pub message::Message);

#[pymethods]
impl Message {
    /// Returns the JSON representation of the message.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        py.detach(|| {
            self.0
                .json::<Json>()
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Returns the data of the message as bytes.
    #[getter]
    pub fn data(&self) -> Option<PyBuffer> {
        let bytes = match &self.0 {
            message::Message::Text(text) => text.clone().into(),
            message::Message::Binary(bytes)
            | message::Message::Ping(bytes)
            | message::Message::Pong(bytes) => bytes.clone(),
            _ => return None,
        };
        Some(PyBuffer::from(bytes))
    }

    /// Returns the text content of the message if it is a text message.
    #[getter]
    pub fn text(&self) -> Option<&str> {
        if let message::Message::Text(text) = &self.0 {
            Some(text)
        } else {
            None
        }
    }

    /// Returns the binary data of the message if it is a binary message.
    #[getter]
    pub fn binary(&self) -> Option<PyBuffer> {
        if let message::Message::Binary(data) = &self.0 {
            Some(PyBuffer::from(data))
        } else {
            None
        }
    }

    /// Returns the ping data of the message if it is a ping message.
    #[getter]
    pub fn ping(&self) -> Option<PyBuffer> {
        if let message::Message::Ping(data) = &self.0 {
            Some(PyBuffer::from(data))
        } else {
            None
        }
    }

    /// Returns the pong data of the message if it is a pong message.
    #[getter]
    pub fn pong(&self) -> Option<PyBuffer> {
        if let message::Message::Pong(data) = &self.0 {
            Some(PyBuffer::from(data))
        } else {
            None
        }
    }

    /// Returns the close code and reason of the message if it is a close message.
    #[getter]
    pub fn close(&self) -> Option<(u16, Option<&str>)> {
        if let message::Message::Close(Some(s)) = &self.0 {
            Some((u16::from(s.code.clone()), Some(s.reason.as_str())))
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
        py.detach(|| {
            message::Message::text_from_json(&json)
                .map(Message)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Creates a new binary message from the JSON representation.
    #[staticmethod]
    #[pyo3(signature = (json))]
    pub fn binary_from_json(py: Python, json: Json) -> PyResult<Self> {
        py.detach(|| {
            message::Message::binary_from_json(&json)
                .map(Message)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Creates a new text message.
    #[staticmethod]
    #[pyo3(signature = (text))]
    pub fn from_text(text: PyBackedStr) -> Self {
        // If the string is not valid UTF-8, this will panic.
        let msg = message::Message::text(
            Utf8Bytes::try_from(Bytes::from_owner(text)).expect("valid UTF-8"),
        );
        Self(msg)
    }

    /// Creates a new binary message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_binary(data: PyBackedBytes) -> Self {
        Self(message::Message::binary(Bytes::from_owner(data)))
    }

    /// Creates a new ping message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_ping(data: PyBackedBytes) -> Self {
        Self(message::Message::ping(Bytes::from_owner(data)))
    }

    /// Creates a new pong message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    pub fn from_pong(data: PyBackedBytes) -> Self {
        Self(message::Message::pong(Bytes::from_owner(data)))
    }

    /// Creates a new close message.
    #[staticmethod]
    #[pyo3(signature = (code, reason=None))]
    pub fn from_close(code: u16, reason: Option<PyBackedStr>) -> Self {
        let reason = reason
            .map(Bytes::from_owner)
            .and_then(|b| Utf8Bytes::try_from(b).ok())
            .unwrap_or_else(|| Utf8Bytes::from_static("Goodbye"));
        let msg = message::Message::close(CloseFrame {
            code: CloseCode::from(code),
            reason,
        });
        Self(msg)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
