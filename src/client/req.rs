use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Duration,
};

use futures_util::TryFutureExt;
use http::header::COOKIE;
use pyo3::{PyResult, prelude::*, pybacked::PyBackedStr};
use wreq::{
    Client, Proxy, Version,
    header::{HeaderMap, HeaderValue, OrigHeaderMap},
};
use wreq_util::EmulationOption;

use crate::{
    client::{
        body::{Body, Form, Json},
        query::Query,
        resp::{Response, WebSocket},
    },
    error::Error,
    extractor::Extractor,
    http::Method,
    redirect,
};

/// The parameters for a request.
#[derive(Default)]
#[non_exhaustive]
pub struct Request {
    /// The Emulation settings for the request.
    emulation: Option<Extractor<EmulationOption>>,

    /// The proxy to use for the request.
    proxy: Option<Extractor<Proxy>>,

    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,

    /// Bind to local IP Addresses (IPv4, IPv6).
    local_addresses: Option<Extractor<(Option<Ipv4Addr>, Option<Ipv6Addr>)>>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    interface: Option<String>,

    /// The timeout to use for the request.
    timeout: Option<u64>,

    /// The read timeout to use for the request.
    read_timeout: Option<u64>,

    /// The HTTP version to use for the request.
    version: Option<Extractor<Version>>,

    /// The headers to use for the request.
    headers: Option<Extractor<HeaderMap>>,

    /// The original headers to use for the request.
    orig_headers: Option<Extractor<OrigHeaderMap>>,

    /// The option enables default headers.
    default_headers: Option<bool>,

    /// The cookies to use for the request.
    cookies: Option<Extractor<Vec<HeaderValue>>>,

    /// The redirect policy to use for the request.
    redirect: Option<redirect::Policy>,

    /// Sets gzip as an accepted encoding.
    gzip: Option<bool>,

    /// Sets brotli as an accepted encoding.
    brotli: Option<bool>,

    /// Sets deflate as an accepted encoding.
    deflate: Option<bool>,

    /// Sets zstd as an accepted encoding.
    zstd: Option<bool>,

    /// The authentication to use for the request.
    auth: Option<PyBackedStr>,

    /// The bearer authentication to use for the request.
    bearer_auth: Option<PyBackedStr>,

    /// The basic authentication to use for the request.
    basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,

    /// The query parameters to use for the request.
    query: Option<Query>,

    /// The form parameters to use for the request.
    form: Option<Form>,

    /// The JSON body to use for the request.
    json: Option<Json>,

    /// The body to use for the request.
    body: Option<Body>,

    /// The multipart form to use for the request.
    multipart: Option<Extractor<wreq::multipart::Form>>,
}

impl FromPyObject<'_, '_> for Request {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Request> {
        let mut request = Self::default();
        extract_option!(ob, request, emulation);
        extract_option!(ob, request, proxy);
        extract_option!(ob, request, local_address);
        extract_option!(ob, request, local_addresses);
        extract_option!(ob, request, interface);

        extract_option!(ob, request, timeout);
        extract_option!(ob, request, read_timeout);

        extract_option!(ob, request, version);
        extract_option!(ob, request, headers);
        extract_option!(ob, request, orig_headers);
        extract_option!(ob, request, default_headers);
        extract_option!(ob, request, cookies);
        extract_option!(ob, request, redirect);
        extract_option!(ob, request, auth);
        extract_option!(ob, request, bearer_auth);
        extract_option!(ob, request, basic_auth);
        extract_option!(ob, request, query);
        extract_option!(ob, request, form);
        extract_option!(ob, request, json);
        extract_option!(ob, request, body);
        extract_option!(ob, request, multipart);

        extract_option!(ob, request, gzip);
        extract_option!(ob, request, brotli);
        extract_option!(ob, request, deflate);
        extract_option!(ob, request, zstd);

        Ok(request)
    }
}

/// The parameters for a WebSocket request.
#[derive(Default)]
#[non_exhaustive]
pub struct WebSocketRequest {
    /// The Emulation settings for the request.
    emulation: Option<Extractor<EmulationOption>>,

    /// The proxy to use for the request.
    proxy: Option<Extractor<Proxy>>,

    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,

    /// Bind to local IP Addresses (IPv4, IPv6).
    local_addresses: Option<Extractor<(Option<Ipv4Addr>, Option<Ipv6Addr>)>>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    interface: Option<String>,

    /// The headers to use for the request.
    headers: Option<Extractor<HeaderMap>>,

    /// The original headers to use for the request.
    orig_headers: Option<Extractor<OrigHeaderMap>>,

    /// The option enables default headers.
    default_headers: Option<bool>,

    /// The cookies to use for the request.
    cookies: Option<Extractor<Vec<HeaderValue>>>,

    /// The protocols to use for the request.
    protocols: Option<Vec<String>>,

    /// Whether to use HTTP/2 for the websocket.
    force_http2: Option<bool>,

    /// The authentication to use for the request.
    auth: Option<PyBackedStr>,

    /// The bearer authentication to use for the request.
    bearer_auth: Option<PyBackedStr>,

    /// The basic authentication to use for the request.
    basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,

    /// The query parameters to use for the request.
    query: Option<Query>,

    /// Read buffer capacity. This buffer is eagerly allocated and used for receiving
    /// messages.
    ///
    /// For high read load scenarios a larger buffer, e.g. 128 KiB, improves performance.
    ///
    /// For scenarios where you expect a lot of connections and don't need high read load
    /// performance a smaller buffer, e.g. 4 KiB, would be appropriate to lower total
    /// memory usage.
    ///
    /// The default value is 128 KiB.
    read_buffer_size: Option<usize>,

    /// The target minimum size of the write buffer to reach before writing the data
    /// to the underlying stream.
    /// The default value is 128 KiB.
    ///
    /// If set to `0` each message will be eagerly written to the underlying stream.
    /// It is often more optimal to allow them to buffer a little, hence the default value.
    ///
    /// Note: [`flush`](WebSocket::flush) will always fully write the buffer regardless.
    write_buffer_size: Option<usize>,

    /// The max size of the write buffer in bytes. Setting this can provide backpressure
    /// in the case the write buffer is filling up due to write errors.
    /// The default value is unlimited.
    ///
    /// Note: The write buffer only builds up past [`write_buffer_size`](Self::write_buffer_size)
    /// when writes to the underlying stream are failing. So the **write buffer can not
    /// fill up if you are not observing write errors even if not flushing**.
    ///
    /// Note: Should always be at least [`write_buffer_size + 1 message`](Self::write_buffer_size)
    /// and probably a little more depending on error handling strategy.
    max_write_buffer_size: Option<usize>,

    /// The maximum size of an incoming message. `None` means no size limit. The default value is
    /// 64 MiB which should be reasonably big for all normal use-cases but small enough to
    /// prevent memory eating by a malicious user.
    max_message_size: Option<usize>,

    /// The maximum size of a single incoming message frame. `None` means no size limit. The limit
    /// is for frame payload NOT including the frame header. The default value is 16 MiB which
    /// should be reasonably big for all normal use-cases but small enough to prevent memory
    /// eating by a malicious user.
    max_frame_size: Option<usize>,

    /// When set to `true`, the server will accept and handle unmasked frames
    /// from the client. According to the RFC 6455, the server must close the
    /// connection to the client in such cases, however it seems like there are
    /// some popular libraries that are sending unmasked frames, ignoring the RFC.
    /// By default this option is set to `false`, i.e. according to RFC 6455.
    accept_unmasked_frames: Option<bool>,
}

impl FromPyObject<'_, '_> for WebSocketRequest {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, emulation);
        extract_option!(ob, params, proxy);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, local_addresses);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, force_http2);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, orig_headers);
        extract_option!(ob, params, default_headers);
        extract_option!(ob, params, cookies);
        extract_option!(ob, params, protocols);
        extract_option!(ob, params, auth);
        extract_option!(ob, params, bearer_auth);
        extract_option!(ob, params, basic_auth);
        extract_option!(ob, params, query);

        extract_option!(ob, params, read_buffer_size);
        extract_option!(ob, params, write_buffer_size);
        extract_option!(ob, params, max_write_buffer_size);
        extract_option!(ob, params, max_message_size);
        extract_option!(ob, params, max_frame_size);
        extract_option!(ob, params, accept_unmasked_frames);
        Ok(params)
    }
}

pub async fn execute_request<U>(
    client: Client,
    method: Method,
    url: U,
    mut params: Option<Request>,
) -> PyResult<Response>
where
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = client.request(method.into_ffi(), url.as_ref());

    // Emulation options.
    apply_option!(set_if_some_inner, builder, params.emulation, emulation);

    // Version options.
    apply_option!(set_if_some_inner, builder, params.version, version);

    // Timeout options.
    apply_option!(
        set_if_some_map,
        builder,
        params.timeout,
        timeout,
        Duration::from_secs
    );
    apply_option!(
        set_if_some_map,
        builder,
        params.read_timeout,
        read_timeout,
        Duration::from_secs
    );

    // Network options.
    apply_option!(set_if_some_inner, builder, params.proxy, proxy);
    apply_option!(set_if_some, builder, params.local_address, local_address);
    apply_option!(
        set_if_some_tuple_inner,
        builder,
        params.local_addresses,
        local_addresses
    );

    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "solaris",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "watchos",
    ))]
    apply_option!(set_if_some, builder, params.interface, interface);

    // Headers options.
    apply_option!(set_if_some_inner, builder, params.headers, headers);
    apply_option!(
        set_if_some_inner,
        builder,
        params.orig_headers,
        orig_headers
    );
    apply_option!(
        set_if_some,
        builder,
        params.default_headers,
        default_headers
    );
    apply_option!(
        set_if_some_iter_inner_with_key,
        builder,
        params.cookies,
        header,
        COOKIE
    );

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
    apply_option!(set_if_some_tuple, builder, params.basic_auth, basic_auth);

    // Allow redirects options.
    apply_option!(set_if_some_inner, builder, params.redirect, redirect);

    // Compression options.
    apply_option!(set_if_some, builder, params.gzip, gzip);
    apply_option!(set_if_some, builder, params.brotli, brotli);
    apply_option!(set_if_some, builder, params.deflate, deflate);
    apply_option!(set_if_some, builder, params.zstd, zstd);

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Body options.
    apply_option!(set_if_some_ref, builder, params.form, form);
    apply_option!(set_if_some_ref, builder, params.json, json);
    apply_option!(set_if_some_inner, builder, params.multipart, multipart);
    apply_option!(
        set_if_some_map_try,
        builder,
        params.body,
        body,
        wreq::Body::try_from
    );

    // Send request.
    builder
        .send()
        .await
        .map(Response::new)
        .map_err(Error::Library)
        .map_err(Into::into)
}

pub async fn execute_websocket_request<U>(
    client: Client,
    url: U,
    mut params: Option<WebSocketRequest>,
) -> PyResult<WebSocket>
where
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = client.websocket(url.as_ref());

    // The protocols to use for the request.
    apply_option!(set_if_some, builder, params.protocols, protocols);

    // The WebSocket config
    apply_option!(
        set_if_some,
        builder,
        params.read_buffer_size,
        read_buffer_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.write_buffer_size,
        write_buffer_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.max_write_buffer_size,
        max_write_buffer_size
    );
    apply_option!(set_if_some, builder, params.max_frame_size, max_frame_size);
    apply_option!(
        set_if_some,
        builder,
        params.max_message_size,
        max_message_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.accept_unmasked_frames,
        accept_unmasked_frames
    );

    // Use http2 options.
    apply_option!(set_if_true, builder, params.force_http2, force_http2, false);

    // Network options.
    apply_option!(set_if_some_inner, builder, params.proxy, proxy);
    apply_option!(set_if_some, builder, params.local_address, local_address);
    apply_option!(
        set_if_some_tuple_inner,
        builder,
        params.local_addresses,
        local_addresses
    );
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "solaris",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "watchos",
    ))]
    apply_option!(set_if_some, builder, params.interface, interface);

    // Headers options.
    apply_option!(set_if_some_inner, builder, params.headers, headers);
    apply_option!(
        set_if_some_inner,
        builder,
        params.orig_headers,
        orig_headers
    );
    apply_option!(
        set_if_some,
        builder,
        params.default_headers,
        default_headers
    );
    apply_option!(
        set_if_some_iter_inner_with_key,
        builder,
        params.cookies,
        header,
        COOKIE
    );

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
    apply_option!(set_if_some_tuple, builder, params.basic_auth, basic_auth);

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Send the WebSocket request.
    builder
        .send()
        .and_then(WebSocket::new)
        .await
        .map_err(Error::Library)
        .map_err(Into::into)
}
