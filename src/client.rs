use crate::{
    error::wrap_rquest_error, impersonate::Impersonate, method::Method, req::RequestParams,
    resp::Response, Result,
};
use pyo3::prelude::*;
use rquest::RequestBuilder;
use std::{ops::Deref, time::Duration};

#[pyclass]
#[derive(Clone, Debug)]
pub struct Client(rquest::Client);

impl Deref for Client {
    type Target = rquest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[inline]
pub async fn get(url: String, params: RequestParams) -> Result<Response> {
    request(Method::GET, url, params).await
}

#[inline]
pub async fn post(url: String, params: RequestParams) -> Result<Response> {
    request(Method::POST, url, params).await
}

#[inline]
pub async fn put(url: String, params: RequestParams) -> Result<Response> {
    request(Method::PUT, url, params).await
}

#[inline]
pub async fn patch(url: String, params: RequestParams) -> Result<Response> {
    request(Method::PATCH, url, params).await
}

#[inline]
pub async fn delete(url: String, params: RequestParams) -> Result<Response> {
    request(Method::DELETE, url, params).await
}

#[inline]
pub async fn head(url: String, params: RequestParams) -> Result<Response> {
    request(Method::HEAD, url, params).await
}

#[inline]
pub async fn options(url: String, params: RequestParams) -> Result<Response> {
    request(Method::OPTIONS, url, params).await
}

#[inline]
pub async fn trace(url: String, params: RequestParams) -> Result<Response> {
    request(Method::TRACE, url, params).await
}

pub async fn request(method: Method, url: String, mut params: RequestParams) -> Result<Response> {
    let client = build_client_from_params(&mut params)?;
    let builder = client.request(method.into_inner(), url);
    apply_params_to_request(builder, params)
        .send()
        .await
        .map(Response::from)
        .map_err(wrap_rquest_error)
}

macro_rules! apply_option {
    ($builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    ($builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (empty, $builder:expr, $option:expr, $method:ident, $default:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method();
        }
    };
}

/// Build a client from the parameters.
fn build_client_from_params(params: &mut RequestParams) -> Result<rquest::Client> {
    let mut builder = rquest::Client::builder();
    // Impersonation options.
    apply_option!(
        builder,
        params.impersonate,
        impersonate,
        |v: Impersonate| v.into_inner()
    );
    apply_option!(builder, params.user_agent, user_agent);

    // Timeout options.
    apply_option!(builder, params.timeout, timeout, Duration::from_secs);
    apply_option!(
        builder,
        params.connect_timeout,
        connect_timeout,
        Duration::from_secs
    );
    apply_option!(
        builder,
        params.read_timeout,
        read_timeout,
        Duration::from_secs
    );
    apply_option!(empty, builder, params.no_keepalive, no_keepalive, false);

    // Other options.
    apply_option!(empty, builder, params.no_proxy, no_proxy, false);
    apply_option!(empty, builder, params.http1_only, http1_only, false);
    apply_option!(empty, builder, params.http2_only, http2_only, false);
    apply_option!(builder, params.referer, referer);

    builder.build().map_err(wrap_rquest_error)
}

/// Apply the parameters to the request builder.
fn apply_params_to_request(
    mut builder: RequestBuilder,
    mut params: RequestParams,
) -> RequestBuilder {
    // Apply the version setting to the request.
    if let Some(version) = params.version.take() {
        builder = builder.version(version.into_inner());
    }

    // Apply the headers setting to the request.
    if let Some(headers) = params.headers.take() {
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
    }

    builder
}
