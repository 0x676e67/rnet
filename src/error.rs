use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError, PyStopAsyncIteration, PyStopIteration},
};
use rquest::header;

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

create_exception!(exceptions, BorrowingError, PyRuntimeError);
create_exception!(exceptions, DNSResolverError, PyRuntimeError);

create_exception!(exceptions, BaseError, PyException);
create_exception!(exceptions, BodyError, BaseError);
create_exception!(exceptions, BuilderError, BaseError);
create_exception!(exceptions, ConnectionError, BaseError);
create_exception!(exceptions, DecodingError, BaseError);
create_exception!(exceptions, RedirectError, BaseError);
create_exception!(exceptions, TimeoutError, BaseError);
create_exception!(exceptions, StatusError, BaseError);
create_exception!(exceptions, RequestError, BaseError);
create_exception!(exceptions, UnknownError, BaseError);

create_exception!(exceptions, HTTPMethodParseError, PyException);
create_exception!(exceptions, URLParseError, PyException);
create_exception!(exceptions, MIMEParseError, PyException);

#[inline(always)]
pub fn memory_error() -> pyo3::PyErr {
    PyRuntimeError::new_err(RACE_CONDITION_ERROR_MSG)
}

#[inline(always)]
pub fn py_stop_iteration_error() -> pyo3::PyErr {
    PyStopIteration::new_err("The iterator is exhausted")
}

#[inline(always)]
pub fn py_stop_async_iteration_error() -> pyo3::PyErr {
    PyStopAsyncIteration::new_err("The iterator is exhausted")
}

#[inline(always)]
pub fn websocket_disconnect_error() -> pyo3::PyErr {
    PyRuntimeError::new_err("The WebSocket has been disconnected")
}

#[inline(always)]
pub fn stream_consumed_error() -> pyo3::PyErr {
    BodyError::new_err("Stream is already consumed")
}

#[inline(always)]
pub fn wrap_invali_header_name_error(error: header::InvalidHeaderName) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("Invalid header name: {:?}", error))
}

#[inline(always)]
pub fn wrap_invali_header_value_error(error: header::InvalidHeaderValue) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("Invalid header value: {:?}", error))
}

#[inline(always)]
pub fn wrap_url_parse_error(error: url::ParseError) -> pyo3::PyErr {
    URLParseError::new_err(format!("URL parse error: {:?}", error))
}

#[inline(always)]
pub fn wrap_io_error(error: std::io::Error) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("IO error: {:?}", error))
}

macro_rules! wrap_error {
    ($error:expr, $($variant:ident => $exception:ident),*) => {
        {
            $(
                if $error.$variant() {
                    return $exception::new_err(format!(concat!(stringify!($variant), " error: {:?}"), $error));
                }
            )*
            UnknownError::new_err(format!("Unknown error occurred: {:?}", $error))
        }
    };
}

pub fn wrap_rquest_error(error: rquest::Error) -> pyo3::PyErr {
    wrap_error!(error,
        is_body => BodyError,
        is_connect => ConnectionError,
        is_connection_reset => ConnectionError,
        is_decode => DecodingError,
        is_redirect => RedirectError,
        is_timeout => TimeoutError,
        is_status => StatusError,
        is_request => RequestError,
        is_builder => BuilderError
    )
}
