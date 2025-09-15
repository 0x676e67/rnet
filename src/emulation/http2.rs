use std::time::Duration;

use pyo3::prelude::*;

use crate::extractor::Extractor;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[pyclass(frozen, eq, hash)]
pub struct StreamId(pub wreq::http2::StreamId);

#[pymethods]
impl StreamId {
    #[classattr]
    const ZERO: Self = Self(wreq::http2::StreamId::ZERO);

    #[classattr]
    const MAX: Self = Self(wreq::http2::StreamId::MAX);
    
    #[new]
    fn new(src: u32) -> Self {
        Self(wreq::http2::StreamId::from(src))
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
#[pyclass(frozen, eq, hash)]
pub struct StreamDependency(pub wreq::http2::StreamDependency);

#[pymethods]
impl StreamDependency {
    #[new]
    fn new(
        dependency_id: Extractor<wreq::http2::StreamId>,
        weight: u8,
        is_exclusive: bool,
    ) -> Self {
        Self(wreq::http2::StreamDependency::new(
            dependency_id.0,
            weight,
            is_exclusive,
        ))
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[pyclass(frozen, eq, hash)]
pub struct Priority(pub wreq::http2::Priority);

#[pymethods]
impl Priority {
    #[new]
    fn new(
        stream_id: Extractor<wreq::http2::StreamId>,
        dependency: Extractor<wreq::http2::StreamDependency>,
    ) -> Self {
        Self(wreq::http2::Priority::new(
            stream_id.0,
            dependency.0,
        ))
    }
}

define_enum!(
    /// The HTTP/2 pseudo-header fields.
    const,
    PseudoId,
    wreq::http2::PseudoId,
    Method,
    Scheme,
    Authority,
    Path,
    Protocol,
    Status,
);

define_enum!(
    /// The HTTP/2 experimental settings.
    const,
    SettingId,
    wreq::http2::SettingId,
    HeaderTableSize,
    EnablePush,
    MaxConcurrentStreams,
    InitialWindowSize,
    MaxFrameSize,
    MaxHeaderListSize,
    EnableConnectProtocol,
    NoRfc7540Priorities,
);

#[derive(Default)]
pub struct Builder {
    /// The initial window size for HTTP/2 streams.
    pub initial_window_size: Option<u32>,

    /// The initial window size for HTTP/2 connection-level flow control.
    pub initial_connection_window_size: Option<u32>,

    /// The initial maximum number of locally initiated (send) streams.
    pub initial_max_send_streams: Option<usize>,

    /// The initial stream ID for the connection.
    pub initial_stream_id: Option<u32>,

    /// Whether to use adaptive flow control.
    pub adaptive_window: Option<bool>,

    /// The maximum frame size to use for HTTP/2.
    pub max_frame_size: Option<u32>,

    /// The maximum size of the header list.
    pub max_header_list_size: Option<u32>,

    /// The header table size for HPACK compression.
    pub header_table_size: Option<u32>,

    /// The maximum number of concurrent streams initiated by the remote peer.
    pub max_concurrent_streams: Option<u32>,

    /// The interval for HTTP/2 keep-alive ping frames.
    pub keep_alive_interval: Option<Duration>,

    /// The timeout for receiving an acknowledgement of the keep-alive ping.
    pub keep_alive_timeout: Option<Duration>,

    /// Whether HTTP/2 keep-alive should apply while the connection is idle.
    pub keep_alive_while_idle: Option<bool>,

    /// Whether to enable push promises.
    pub enable_push: Option<bool>,

    /// Whether to enable the CONNECT protocol.
    pub enable_connect_protocol: Option<bool>,

    /// Whether to disable RFC 7540 Stream Priorities.
    pub no_rfc7540_priorities: Option<bool>,

    /// The maximum number of concurrent locally reset streams.
    pub max_concurrent_reset_streams: Option<usize>,

    /// The maximum size of the send buffer for HTTP/2 streams.
    pub max_send_buf_size: Option<usize>,

    /// The maximum number of pending accept reset streams.
    pub max_pending_accept_reset_streams: Option<usize>,

    /// The stream dependency for the outgoing HEADERS frame.
    pub headers_stream_dependency: Option<Extractor<wreq::http2::StreamDependency>>,

    /// The HTTP/2 pseudo-header field order for outgoing HEADERS frames.
    pub headers_pseudo_order: Option<Extractor<wreq::http2::PseudoOrder>>,

    /// Custom experimental HTTP/2 settings.
    pub experimental_settings: Option<Extractor<wreq::http2::ExperimentalSettings>>,

    /// The order of settings parameters in the initial SETTINGS frame.
    pub settings_order: Option<Extractor<wreq::http2::SettingsOrder>>,

    /// The list of PRIORITY frames to be sent after connection establishment.
    pub priorities: Option<Extractor<wreq::http2::Priorities>>,
}

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, initial_window_size);
        extract_option!(ob, params, initial_connection_window_size);
        extract_option!(ob, params, initial_max_send_streams);
        extract_option!(ob, params, initial_stream_id);
        extract_option!(ob, params, adaptive_window);
        extract_option!(ob, params, max_frame_size);
        extract_option!(ob, params, max_header_list_size);
        extract_option!(ob, params, header_table_size);
        extract_option!(ob, params, max_concurrent_streams);
        extract_option!(ob, params, keep_alive_interval);
        extract_option!(ob, params, keep_alive_timeout);
        extract_option!(ob, params, keep_alive_while_idle);
        extract_option!(ob, params, enable_push);
        extract_option!(ob, params, enable_connect_protocol);
        extract_option!(ob, params, no_rfc7540_priorities);
        extract_option!(ob, params, max_concurrent_reset_streams);
        extract_option!(ob, params, max_send_buf_size);
        extract_option!(ob, params, max_pending_accept_reset_streams);
        extract_option!(ob, params, headers_stream_dependency);
        extract_option!(ob, params, headers_pseudo_order);
        extract_option!(ob, params, experimental_settings);
        extract_option!(ob, params, settings_order);
        extract_option!(ob, params, priorities);
        Ok(params)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[pyclass(subclass, frozen)]
pub struct Http2Options(pub wreq::http2::Http2Options);

#[pymethods]
impl Http2Options {
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, mut kwds: Option<Builder>) -> Self {
        py.detach(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::http2::Http2Options::builder();

            apply_option!(set_if_some, builder, params.initial_window_size, initial_window_size);
            apply_option!(set_if_some, builder, params.initial_connection_window_size, initial_connection_window_size);
            apply_option!(set_if_some, builder, params.initial_max_send_streams, initial_max_send_streams);
            apply_option!(set_if_some, builder, params.initial_stream_id, initial_stream_id);
            apply_option!(set_if_some, builder, params.adaptive_window, adaptive_window);
            apply_option!(set_if_some, builder, params.max_frame_size, max_frame_size);
            apply_option!(set_if_some, builder, params.max_header_list_size, max_header_list_size);
            apply_option!(set_if_some, builder, params.header_table_size, header_table_size);
            apply_option!(set_if_some, builder, params.max_concurrent_streams, max_concurrent_streams);
            apply_option!(set_if_some, builder, params.enable_push, enable_push);
            apply_option!(set_if_some, builder, params.enable_connect_protocol, enable_connect_protocol);
            apply_option!(set_if_some, builder, params.no_rfc7540_priorities, no_rfc7540_priorities);
            apply_option!(set_if_some, builder, params.max_concurrent_reset_streams, max_concurrent_reset_streams);
            apply_option!(set_if_some, builder, params.max_send_buf_size, max_send_buf_size);
            apply_option!(set_if_some, builder, params.max_pending_accept_reset_streams, max_pending_accept_reset_streams);
            apply_option!(set_if_some_inner, builder, params.headers_stream_dependency, headers_stream_dependency);
            apply_option!(set_if_some_inner, builder, params.headers_pseudo_order, headers_pseudo_order);
            apply_option!(set_if_some_inner, builder, params.experimental_settings, experimental_settings);
            apply_option!(set_if_some_inner, builder, params.settings_order, settings_order);
            apply_option!(set_if_some_inner, builder, params.priorities, priorities);

            apply_option!(set_if_some_inplace, builder, params.keep_alive_interval, keep_alive_interval);
            apply_option!(set_if_some_inplace, builder, params.keep_alive_timeout, keep_alive_timeout);
            apply_option!(set_if_some_inplace, builder, params.keep_alive_while_idle, keep_alive_while_idle);

            Self(builder.build())
        })
    }
}
