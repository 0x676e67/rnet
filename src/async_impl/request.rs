use crate::apply_option;
use crate::{
    async_impl::{Response, WebSocket},
    error::wrap_rquest_error,
    param::{RequestParams, WebSocketParams},
    typing::{Method, Version},
    Result,
};

use rquest::header;
use rquest::redirect::Policy;
use std::net::IpAddr;
use std::ops::Deref;
use std::{sync::Arc, time::Duration};

/// Executes an HTTP request.
pub async fn execute_request<C, U>(
    client: C,
    method: Method,
    url: U,
    mut params: Option<RequestParams>,
) -> Result<Response>
where
    C: Deref<Target = rquest::Client>,
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

    // Allow redirects options.
    apply_option!(
        apply_option_or_default_with_value,
        builder,
        params.allow_redirects,
        redirect,
        false,
        params
            .max_redirects
            .take()
            .map(Policy::limited)
            .unwrap_or_default()
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
    apply_option!(apply_if_some, builder, params.proxy, proxy);
    apply_option!(
        apply_transformed_option,
        builder,
        params.local_address,
        local_address,
        IpAddr::from
    );
    rquest::cfg_bindable_device!(
        apply_option!(apply_if_some, builder, params.interface, interface);
    );

    // Headers options.
    apply_option!(apply_if_some_inner, builder, params.headers, headers);

    // Cookies options.
    if let Some(cookies) = params.cookies.take() {
        builder = builder.header(header::COOKIE, cookies.0);
    }

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

    // Query options.
    apply_option!(apply_if_some_ref, builder, params.query, query);

    // Form options.
    apply_option!(apply_if_some_ref, builder, params.form, form);

    // JSON options.
    apply_option!(apply_if_some_ref, builder, params.json, json);

    // Body options.
    if let Some(body) = params.body.take() {
        builder = builder.body(rquest::Body::try_from(body)?);
    }

    // Multipart options.
    apply_option!(apply_if_some, builder, params.multipart, multipart);

    // Send the request.
    builder
        .send()
        .await
        .map(Response::new)
        .map_err(wrap_rquest_error)
}

/// Executes a WebSocket request.
pub async fn execute_websocket_request<C, U>(
    client: C,
    url: U,
    mut params: Option<WebSocketParams>,
) -> Result<WebSocket>
where
    C: Deref<Target = rquest::Client>,
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

    // The origin to use for the request.
    builder = builder.with_builder(|mut builder| {
        // Network options.
        apply_option!(
            apply_transformed_option_ref,
            builder,
            params.proxy,
            proxy,
            AsRef::<str>::as_ref
        );
        apply_option!(
            apply_transformed_option,
            builder,
            params.local_address,
            local_address,
            IpAddr::from
        );
        rquest::cfg_bindable_device!(
            apply_option!(apply_if_some, builder, params.interface, interface);
        );

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
            builder = builder.header(header::COOKIE, cookies.0);
        }

        // Query options.
        apply_option!(apply_if_some_ref, builder, params.query, query);

        builder
    });

    WebSocket::new(builder).await
}

/// Executes an HTTP request.
#[inline(always)]
pub async fn execute_request2<C, U>(
    client: C,
    method: Method,
    url: U,
    params: Option<RequestParams>,
) -> Result<Response>
where
    C: Deref<Target = Arc<rquest::Client>>,
    U: AsRef<str>,
{
    execute_request(client.as_ref(), method, url, params).await
}

/// Executes a WebSocket request.
#[inline(always)]
pub async fn execute_websocket_request2<C, U>(
    client: C,
    url: U,
    params: Option<WebSocketParams>,
) -> Result<WebSocket>
where
    C: Deref<Target = Arc<rquest::Client>>,
    U: AsRef<str>,
{
    execute_websocket_request(client.as_ref(), url, params).await
}
