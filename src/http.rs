pub mod cookie;
pub mod header;
pub mod status;

use pyo3::prelude::*;

define_enum!(
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

define_enum!(
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
