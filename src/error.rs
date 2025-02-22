use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError},
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

pub fn memory_error() -> pyo3::PyErr {
    PyRuntimeError::new_err(RACE_CONDITION_ERROR_MSG)
}

pub fn wrap_serde_error(error: serde_json::Error) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("Serde error: {:?}", error))
}

pub fn wrap_invali_header_name_error(error: header::InvalidHeaderName) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("Invalid header name: {:?}", error))
}

pub fn wrap_invali_header_value_error(error: header::InvalidHeaderValue) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("Invalid header value: {:?}", error))
}

pub fn wrap_url_parse_error(error: url::ParseError) -> pyo3::PyErr {
    URLParseError::new_err(format!("URL parse error: {:?}", error))
}

pub fn wrap_io_error(error: std::io::Error) -> pyo3::PyErr {
    PyRuntimeError::new_err(format!("IO error: {:?}", error))
}

pub fn wrap_rquest_error(error: rquest::Error) -> pyo3::PyErr {
    if error.is_body() {
        BodyError::new_err(format!("Body related error: {:?}", error))
    } else if error.is_connect() {
        return ConnectionError::new_err(format!("Could not connect to host: {:?}", error));
    } else if error.is_decode() {
        return DecodingError::new_err(format!("Response body decoding error: {:?}", error));
    } else if error.is_redirect() {
        return RedirectError::new_err(format!("Maximum redirect count was reached: {:?}", error));
    } else if error.is_timeout() {
        return TimeoutError::new_err(format!("Timeout has been reached: {:?}", error));
    } else if error.is_status() {
        return StatusError::new_err(format!("Error status code in the response: {:?}", error));
    } else if error.is_request() {
        return RequestError::new_err(format!("Request error: {:?}", error));
    } else if error.is_builder() {
        return BuilderError::new_err(format!("Could not build the request: {:?}", error));
    } else {
        return UnknownError::new_err(format!("Unknown error occured: {:?}", error));
    }
}
