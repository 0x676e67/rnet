use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError},
};
use rquest::header;

const RACE_CONDITION_ERROR_MSG: &'static str = r#"Due Rust's memory managment approach of borrowing,
you cannot use some instances for some kind of
stuff twice as they are gone.

There are 3 cases you've got this error:
1) You passed a non-clonable instance to another that requires owning
2) You tried using method with owning twice (i.e. reading response's body twice)
3) You tried using referencing after borrowing

Potential solutions:
1) Do not share instances, create new on every time you use it
2) Do not do this. Try another way to solve your problem
3) Swap calls order (referencing first)
inner types "#;

create_exception!(exceptions, BorrowingError, PyRuntimeError);

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

pub fn wrap_rquest_error(error: rquest::Error) -> pyo3::PyErr {
    if error.is_body() {
        return BodyError::new_err(format!("Body related error: {:?}", error));
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
