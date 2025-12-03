use std::{net::IpAddr, time::Duration};

use pyo3::{PyResult, prelude::*, pybacked::PyBackedStr};
use wreq::{
    Client, Proxy, Version,
    header::{self, HeaderMap, HeaderValue, OrigHeaderMap},
    redirect::Policy,
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

    /// Whether to allow redirects.
    allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    max_redirects: Option<usize>,

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
        let mut params = Self::default();
        extract_option!(ob, params, emulation);
        extract_option!(ob, params, proxy);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, timeout);
        extract_option!(ob, params, read_timeout);

        extract_option!(ob, params, version);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, orig_headers);
        extract_option!(ob, params, default_headers);
        extract_option!(ob, params, cookies);
        extract_option!(ob, params, allow_redirects);
        extract_option!(ob, params, max_redirects);
        extract_option!(ob, params, auth);
        extract_option!(ob, params, bearer_auth);
        extract_option!(ob, params, basic_auth);
        extract_option!(ob, params, query);
        extract_option!(ob, params, form);
        extract_option!(ob, params, json);
        extract_option!(ob, params, body);
        extract_option!(ob, params, multipart);

        extract_option!(ob, params, gzip);
        extract_option!(ob, params, brotli);
        extract_option!(ob, params, deflate);
        extract_option!(ob, params, zstd);

        Ok(params)
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

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
    if let Some(basic_auth) = params.basic_auth.take() {
        builder = builder.basic_auth(basic_auth.0, basic_auth.1);
    }

    // Cookies options.
    if let Some(cookies) = params.cookies.take() {
        for cookie in cookies.0 {
            builder = builder.header_append(header::COOKIE, cookie);
        }
    }

    // Allow redirects options.
    match params.allow_redirects {
        Some(false) => {
            builder = builder.redirect(Policy::none());
        }
        Some(true) => {
            builder = builder.redirect(
                params
                    .max_redirects
                    .take()
                    .map(Policy::limited)
                    .unwrap_or_default(),
            );
        }
        None => {}
    };

    // Compression options.
    apply_option!(set_if_some, builder, params.gzip, gzip);
    apply_option!(set_if_some, builder, params.brotli, brotli);
    apply_option!(set_if_some, builder, params.deflate, deflate);
    apply_option!(set_if_some, builder, params.zstd, zstd);

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Form options.
    apply_option!(set_if_some_ref, builder, params.form, form);

    // JSON options.
    apply_option!(set_if_some_ref, builder, params.json, json);

    // Multipart options.
    apply_option!(set_if_some_inner, builder, params.multipart, multipart);

    // Body options.
    if let Some(body) = params.body.take() {
        builder = builder.body(wreq::Body::try_from(body)?);
    }

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

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
    if let Some(basic_auth) = params.basic_auth.take() {
        builder = builder.basic_auth(basic_auth.0, basic_auth.1);
    }

    // Cookies options.
    if let Some(cookies) = params.cookies.take() {
        for cookie in cookies.0 {
            builder = builder.header_append(header::COOKIE, cookie);
        }
    }

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Send the WebSocket request.
    let response = builder.send().await.map_err(Error::Library)?;
    WebSocket::new(response)
        .await
        .map_err(Error::Library)
        .map_err(Into::into)
}
