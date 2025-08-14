use pyo3::{
    PyErr, create_exception,
    exceptions::{PyException, PyRuntimeError, PyStopAsyncIteration, PyStopIteration},
};
use wreq::header;

const RACE_CONDITION_ERROR_MSG: &str = r#"Due to Rust's memory management with borrowing,
you cannot use certain instances multiple times as they may be consumed.

This error can occur in the following cases:
1) You passed a non-clonable instance to a function that requires ownership.
2) You attempted to use a method that consumes ownership more than once (e.g., reading a response body twice).
3) You tried to reference an instance after it was borrowed.

Potential solutions:
1) Avoid sharing instances; create a new instance each time you use it.
2) Refrain from performing actions that consume ownership multiple times.
3) Change the order of operations to reference the instance before borrowing it.
"#;

// System-level and runtime errors
create_exception!(exceptions, DNSResolverError, PyRuntimeError);

// Network connection errors
create_exception!(exceptions, ConnectionError, PyException);
create_exception!(exceptions, ConnectionResetError, PyException);
create_exception!(exceptions, TlsError, PyException);

// HTTP protocol and request/response errors
create_exception!(exceptions, RequestError, PyException);
create_exception!(exceptions, StatusError, PyException);
create_exception!(exceptions, RedirectError, PyException);
create_exception!(exceptions, TimeoutError, PyException);

// Data processing and encoding errors
create_exception!(exceptions, BodyError, PyException);
create_exception!(exceptions, DecodingError, PyException);

// Configuration and builder errors
create_exception!(exceptions, BuilderError, PyException);

// Protocol upgrade and WebSocket errors
create_exception!(exceptions, UpgradeError, PyException);
create_exception!(exceptions, WebSocketError, PyException);

// Input validation and parsing errors
create_exception!(exceptions, URLParseError, PyException);
create_exception!(exceptions, MIMEParseError, PyException);

macro_rules! wrap_error {
    ($error:expr, $($variant:ident => $exception:ident),*) => {
        {
            $(
                if $error.$variant() {
                    return $exception::new_err(format!(concat!(stringify!($variant), " error: {:?}"), $error));
                }
            )*
            UpgradeError::new_err(format!("error: {:?}", $error))
        }
    };
}

/// Unified error enum
#[derive(Debug)]
pub enum Error {
    Memory,
    StopIteration,
    StopAsyncIteration,
    WebSocketDisconnect,
    InvalidHeaderName(header::InvalidHeaderName),
    InvalidHeaderValue(header::InvalidHeaderValue),
    UrlParse(url::ParseError),
    IO(std::io::Error),
    Decode(cookie_crate::ParseError),
    Library(wreq::Error),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Memory => PyRuntimeError::new_err(RACE_CONDITION_ERROR_MSG),
            Error::StopIteration => PyStopIteration::new_err("The iterator is exhausted"),
            Error::StopAsyncIteration => PyStopAsyncIteration::new_err("The iterator is exhausted"),
            Error::WebSocketDisconnect => {
                PyRuntimeError::new_err("The WebSocket has been disconnected")
            }
            Error::InvalidHeaderName(err) => {
                PyRuntimeError::new_err(format!("Invalid header name: {err:?}"))
            }
            Error::InvalidHeaderValue(err) => {
                PyRuntimeError::new_err(format!("Invalid header value: {err:?}"))
            }
            Error::UrlParse(err) => URLParseError::new_err(format!("URL parse error: {err:?}")),
            Error::IO(err) => PyRuntimeError::new_err(format!("IO error: {err:?}")),
            Error::Decode(err) => MIMEParseError::new_err(format!("Decode error: {err:?}")),
            Error::Library(err) => wrap_error!(err,
                is_body => BodyError,
                is_tls => TlsError,
                is_websocket => WebSocketError,
                is_connect => ConnectionError,
                is_connection_reset => ConnectionResetError,
                is_decode => DecodingError,
                is_redirect => RedirectError,
                is_timeout => TimeoutError,
                is_status => StatusError,
                is_request => RequestError,
                is_builder => BuilderError
            ),
        }
    }
}

impl From<header::InvalidHeaderName> for Error {
    fn from(err: header::InvalidHeaderName) -> Self {
        Error::InvalidHeaderName(err)
    }
}

impl From<header::InvalidHeaderValue> for Error {
    fn from(err: header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::UrlParse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<wreq::Error> for Error {
    fn from(err: wreq::Error) -> Self {
        Error::Library(err)
    }
}
