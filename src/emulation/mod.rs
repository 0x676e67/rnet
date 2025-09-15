mod http1;
mod http2;
mod tls;

use pyo3::prelude::*;

pub use self::{
    http1::Http1Options,
    http2::{Http2Options, StreamId, StreamDependency, Priority, PseudoId, SettingId},
    tls::TlsOptions,
};
use crate::http::header::{HeaderMap, OrigHeaderMap};

#[derive(Default)]
pub struct Builder {
    /// the HTTP/1 options configuration.
    pub http1_options: Option<Http1Options>,

    /// the HTTP/2 options configuration.
    pub http2_options: Option<Http2Options>,

    /// the TLS options configuration.
    pub tls_options: Option<TlsOptions>,

    /// the default headers.
    pub headers: Option<HeaderMap>,

    /// the original headers.
    pub orig_headers: Option<OrigHeaderMap>,
}

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, http1_options);
        extract_option!(ob, params, http2_options);
        extract_option!(ob, params, tls_options);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, orig_headers);
        Ok(params)
    }
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct Emulation(pub wreq::Emulation);

#[pymethods]
impl Emulation {
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, mut kwds: Option<Builder>) -> Self {
        py.detach(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::Emulation::builder();

            apply_option!(set_if_some_inner, builder, params.http1_options, http1_options);
            apply_option!(set_if_some_inner, builder, params.http2_options, http2_options);
            apply_option!(set_if_some_inner, builder, params.tls_options, tls_options);
            apply_option!(set_if_some_inner, builder, params.headers, headers);
            apply_option!(set_if_some_inner, builder, params.orig_headers, orig_headers);

            Self(builder.build())
        })
    }
}
