use std::{sync::LazyLock, time::Duration};

use pyo3::PyResult;
use wreq::{Client, header, redirect::Policy};

use super::{
    dns,
    param::{RequestParams, WebSocketParams},
};
use crate::{
    client::{
        async_impl::response::{Response, WebSocket},
        typing::{LookupIpStrategy, Method, Version},
    },
    error::Error,
};

static DEFAULT_CLIENT: LazyLock<wreq::Client> = LazyLock::new(|| {
    let mut builder = wreq::Client::builder();
    apply_option!(
        apply_if_ok,
        builder,
        || dns::get_or_try_init(LookupIpStrategy::Ipv4AndIpv6),
        dns_resolver
    );
    builder
        .no_hickory_dns()
        .no_keepalive()
        .http1(|mut http| {
            http.title_case_headers(true);
        })
        .build()
        .expect("Failed to build the default client.")
});

#[inline]
pub async fn shortcut_request<U>(
    url: U,
    method: Method,
    params: Option<RequestParams>,
) -> PyResult<Response>
where
    U: AsRef<str>,
{
    execute_request(DEFAULT_CLIENT.clone(), method, url, params).await
}

#[inline]
pub async fn shortcut_websocket_request<U>(
    url: U,
    params: Option<WebSocketParams>,
) -> PyResult<WebSocket>
where
    U: AsRef<str>,
{
    execute_websocket_request(DEFAULT_CLIENT.clone(), url, params).await
}

/// Executes an HTTP request.
pub async fn execute_request<U>(
    client: Client,
    method: Method,
    url: U,
    mut params: Option<RequestParams>,
) -> PyResult<Response>
where
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = client.request(method.into_ffi(), url.as_ref());

    // Version options.
    apply_option!(
        apply_transformed_option,
        builder,
        params.version,
        version,
        Version::into_ffi
    );

    // Timeout options.
    apply_option!(
        apply_transformed_option,
        builder,
        params.timeout,
        timeout,
        Duration::from_secs
    );
    apply_option!(
        apply_transformed_option,
        builder,
        params.read_timeout,
        read_timeout,
        Duration::from_secs
    );

    // Network options.
    apply_option!(apply_if_some_inner, builder, params.proxy, proxy);
    apply_option!(
        apply_if_some_inner,
        builder,
        params.local_address,
        local_address
    );
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "ios",
        target_os = "visionos",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    apply_option!(apply_if_some, builder, params.interface, interface);

    // Headers options.
    apply_option!(apply_if_some_inner, builder, params.headers, headers);

    // Authentication options.
    apply_option!(
        apply_transformed_option_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );

    // Bearer authentication options.
    apply_option!(apply_if_some, builder, params.bearer_auth, bearer_auth);

    // Basic authentication options.
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

    // Query options.
    apply_option!(apply_if_some_ref, builder, params.query, query);

    // Form options.
    apply_option!(apply_if_some_ref, builder, params.form, form);

    // JSON options.
    apply_option!(apply_if_some_ref, builder, params.json, json);

    // Body options.
    apply_option!(apply_if_some, builder, params.body, body);

    // Multipart options.
    apply_option!(apply_if_some_inner, builder, params.multipart, multipart);

    // Send the request.
    builder
        .send()
        .await
        .map(Response::new)
        .map_err(Error::Request)
        .map_err(Into::into)
}

/// Executes a WebSocket request.
pub async fn execute_websocket_request<U>(
    client: Client,
    url: U,
    mut params: Option<WebSocketParams>,
) -> PyResult<WebSocket>
where
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = client.websocket(url.as_ref());

    // The protocols to use for the request.
    apply_option!(apply_if_some, builder, params.protocols, protocols);

    // The WebSocket config
    apply_option!(
        apply_if_some,
        builder,
        params.read_buffer_size,
        read_buffer_size
    );
    apply_option!(
        apply_if_some,
        builder,
        params.write_buffer_size,
        write_buffer_size
    );
    apply_option!(
        apply_if_some,
        builder,
        params.max_write_buffer_size,
        max_write_buffer_size
    );
    apply_option!(
        apply_if_some,
        builder,
        params.max_frame_size,
        max_frame_size
    );
    apply_option!(
        apply_if_some,
        builder,
        params.max_message_size,
        max_message_size
    );
    apply_option!(
        apply_if_some,
        builder,
        params.accept_unmasked_frames,
        accept_unmasked_frames
    );

    // Use http2 options.
    apply_option!(
        apply_option_or_default,
        builder,
        params.use_http2,
        use_http2,
        false
    );

    // Network options.
    apply_option!(apply_if_some_inner, builder, params.proxy, proxy);
    apply_option!(
        apply_if_some_inner,
        builder,
        params.local_address,
        local_address
    );
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "ios",
        target_os = "visionos",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    apply_option!(apply_if_some, builder, params.interface, interface);

    // Authentication options.
    apply_option!(
        apply_transformed_option_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );

    // Bearer authentication options.
    apply_option!(apply_if_some, builder, params.bearer_auth, bearer_auth);

    // Basic authentication options.
    if let Some(basic_auth) = params.basic_auth.take() {
        builder = builder.basic_auth(basic_auth.0, basic_auth.1);
    }

    // Headers options.
    apply_option!(apply_if_some_inner, builder, params.headers, headers);

    // Cookies options.
    if let Some(cookies) = params.cookies.take() {
        for cookie in cookies.0 {
            builder = builder.header_append(header::COOKIE, cookie);
        }
    }

    // Query options.
    apply_option!(apply_if_some_ref, builder, params.query, query);

    WebSocket::new(builder)
        .await
        .map_err(Error::Request)
        .map_err(Into::into)
}
