pub mod header;
mod status;

use pyo3::prelude::*;

pub use self::status::StatusCode;

define_enum_with_conversion!(
    /// An HTTP version.
    const,
    Version,
    wreq::Version,
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
);

define_enum_with_conversion!(
    /// An HTTP method.
    Method,
    wreq::Method,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    PATCH,
);
