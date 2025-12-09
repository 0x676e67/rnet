//! Types and utilities for representing HTTP request bodies.

mod form;
mod json;
pub mod multipart;
mod stream;

use bytes::Bytes;
use pyo3::{
    FromPyObject, PyResult,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};

pub use self::{
    form::Form,
    json::Json,
    stream::{PyStream, Streamer},
};

/// Represents the body of an HTTP request.
#[derive(FromPyObject)]
pub enum Body {
    Text(PyBackedStr),
    Bytes(PyBackedBytes),
    Form(Form),
    Json(Json),
    Stream(PyStream),
}

impl TryFrom<Body> for wreq::Body {
    type Error = PyErr;

    fn try_from(value: Body) -> PyResult<wreq::Body> {
        match value {
            Body::Form(form) => serde_urlencoded::to_string(form)
                .map(wreq::Body::from)
                .map_err(crate::Error::Form)
                .map_err(Into::into),
            Body::Json(json) => serde_json::to_vec(&json)
                .map_err(crate::Error::Json)
                .map(wreq::Body::from)
                .map_err(Into::into),
            Body::Text(s) => Ok(wreq::Body::from(Bytes::from_owner(s))),
            Body::Bytes(bytes) => Ok(wreq::Body::from(Bytes::from_owner(bytes))),
            Body::Stream(stream) => Ok(wreq::Body::wrap_stream(stream)),
        }
    }
}
