mod client;
mod request;
mod response;

use std::sync::LazyLock;

use pyo3::PyResult;
pub use request::{execute_request, execute_websocket_request};

pub use self::{
    client::Client,
    response::{Message, Response, Streamer, WebSocket},
};
use crate::{
    dns,
    typing::{
        LookupIpStrategy, Method,
        param::{RequestParams, WebSocketParams},
    },
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

/// Send a shortcut HTTP request.
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

/// Send a shortcut WebSocket request.
#[inline(always)]
pub async fn shortcut_websocket_request<U>(
    url: U,
    params: Option<WebSocketParams>,
) -> PyResult<WebSocket>
where
    U: AsRef<str>,
{
    execute_websocket_request(DEFAULT_CLIENT.clone(), url, params).await
}
