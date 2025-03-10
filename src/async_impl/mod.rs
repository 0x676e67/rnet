mod client;
mod http;
mod request;
mod ws;

pub use self::{
    client::Client,
    http::{Response, Streamer},
    ws::{Message, WebSocket},
};
use crate::param::{RequestParams, WebSocketParams};
use crate::typing::{LookupIpStrategy, Method};
use crate::{apply_option, dns};
pub use request::{execute_request, execute_websocket_request};
use std::sync::LazyLock;

#[macro_export]
macro_rules! apply_option {
    (apply_if_some, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    (apply_if_some_ref, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&value);
        }
    };
    (apply_if_some_inner, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value.0);
        }
    };
    (apply_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (apply_if_ok, $builder:expr, $result:expr, $method:ident) => {
        if let Ok(value) = $result() {
            $builder = $builder.$method(value);
        }
    };
    (apply_transformed_option_ref, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(&value));
        }
    };
    (apply_ref_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&$transform(value));
        }
    };
    (apply_option_or_default, $builder:expr, $option:expr, $method:ident, $default:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method();
        }
    };
    (apply_option_or_default_with_value, $builder:expr, $option:expr, $method:ident, $default:expr, $value:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method($value);
        }
    };
}

static DEFAULT_CLIENT: LazyLock<rquest::Client> = LazyLock::new(|| {
    let mut builder = rquest::Client::builder();
    apply_option!(
        apply_if_ok,
        builder,
        || dns::get_or_try_init(LookupIpStrategy::Ipv4AndIpv6),
        dns_resolver
    );
    builder
        .no_hickory_dns()
        .no_keepalive()
        .build()
        .expect("Failed to build the default client.")
});

/// Send a shortcut HTTP request.
#[inline(always)]
pub async fn shortcut_request<U>(
    url: U,
    method: Method,
    params: Option<RequestParams>,
) -> crate::Result<Response>
where
    U: AsRef<str>,
{
    execute_request(
        DEFAULT_CLIENT.clone(),
        method,
        url.as_ref().to_string(),
        params,
    )
    .await
}

/// Send a shortcut WebSocket request.
#[inline(always)]
pub async fn shortcut_websocket_request<U>(
    url: U,
    params: Option<WebSocketParams>,
) -> crate::Result<WebSocket>
where
    U: AsRef<str>,
{
    execute_websocket_request(DEFAULT_CLIENT.clone(), url.as_ref().to_string(), params).await
}
