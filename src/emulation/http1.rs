use pyo3::prelude::*;

#[derive(Default)]
pub struct Builder {
    /// Enable support for HTTP/0.9 responses.
    pub http09_responses: Option<bool>,

    /// Whether to use vectored writes for HTTP/1 connections.
    pub writev: Option<bool>,

    /// Maximum number of headers allowed in HTTP/1 responses.
    pub max_headers: Option<usize>,

    /// Exact size of the read buffer to use for HTTP/1 connections.
    pub read_buf_exact_size: Option<usize>,

    /// Maximum buffer size for HTTP/1 connections.
    pub max_buf_size: Option<usize>,

    /// Whether to allow spaces after header names in HTTP/1 responses.
    pub allow_spaces_after_header_name_in_responses: Option<bool>,

    /// Whether to ignore invalid headers in HTTP/1 responses.
    pub ignore_invalid_headers_in_responses: Option<bool>,

    /// Whether to allow obsolete multiline headers in HTTP/1 responses.
    pub allow_obsolete_multiline_headers_in_responses: Option<bool>,
}

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, http09_responses);
        extract_option!(ob, params, writev);
        extract_option!(ob, params, max_headers);
        extract_option!(ob, params, read_buf_exact_size);
        extract_option!(ob, params, max_buf_size);
        extract_option!(ob, params, allow_spaces_after_header_name_in_responses);
        extract_option!(ob, params, ignore_invalid_headers_in_responses);
        extract_option!(ob, params, allow_obsolete_multiline_headers_in_responses);
        Ok(params)
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
#[pyclass(subclass, frozen)]
pub struct Http1Options(pub wreq::http1::Http1Options);

#[pymethods]
impl Http1Options {
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, mut kwds: Option<Builder>) -> Self {
        py.detach(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::http1::Http1Options::builder();

            apply_option!(set_if_some, builder, params.http09_responses, http09_responses);
            apply_option!(set_if_some_map, builder, params.writev, writev, Some);
            apply_option!(set_if_some, builder, params.max_headers, max_headers);
            apply_option!(set_if_some_map, builder, params.read_buf_exact_size, read_buf_exact_size, Some);
            apply_option!(set_if_some, builder, params.max_buf_size, max_buf_size);
            apply_option!(set_if_some, builder, params.allow_spaces_after_header_name_in_responses, allow_spaces_after_header_name_in_responses);
            apply_option!(set_if_some, builder, params.ignore_invalid_headers_in_responses, ignore_invalid_headers_in_responses);
            apply_option!(set_if_some, builder, params.allow_obsolete_multiline_headers_in_responses, allow_obsolete_multiline_headers_in_responses);

            Self(builder.build())
        })
    }
}
