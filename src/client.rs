use crate::{
    error::{wrap_invali_header_name_error, wrap_rquest_error},
    param::{ClientParams, RequestParams},
    resp::Response,
    types::{Impersonate, Method, Version},
    Result,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{
    header::{HeaderMap, HeaderName},
    RequestBuilder,
};
use std::{ops::Deref, time::Duration};

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
    (apply_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (apply_option_or_default, $builder:expr, $option:expr, $method:ident, $default:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method();
        }
    };
}

/// A client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct Client(rquest::Client);

impl Deref for Client {
    type Target = rquest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Client {
    /// Creates a new Client instance.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional request parameters as a dictionary.
    ///
    /// # Returns
    ///
    /// A new `Client` instance.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(
    ///     user_agent="my-app/0.0.1",
    ///     timeout=10,
    /// )
    /// response = client.get('https://httpbin.org/get')
    /// print(response.text)
    /// ```
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new<'rt>(mut kwds: Option<ClientParams>) -> PyResult<Client> {
        let params = kwds.get_or_insert_default();
        let mut builder = rquest::Client::builder();

        // Impersonation options.
        apply_option!(
            apply_transformed_option,
            builder,
            params.impersonate,
            impersonate,
            |v: Impersonate| v.into_inner()
        );

        // User agent options.
        apply_option!(apply_if_some, builder, params.user_agent, user_agent);

        // Headers options.
        if let Some(default_headers) = params.default_headers.take() {
            let mut headers = HeaderMap::with_capacity(default_headers.len());
            for (key, value) in default_headers.into_iter() {
                let name = HeaderName::from_bytes(key.as_bytes())
                    .map_err(wrap_invali_header_name_error)?;
                headers.insert(name, value);
            }
        }

        // Headers order options.
        if let Some(headers_order) = params.headers_order.take() {
            let mut names = Vec::with_capacity(headers_order.len());
            for name in headers_order {
                let name = HeaderName::from_bytes(name.as_bytes())
                    .map_err(wrap_invali_header_name_error)?;
                names.push(name);
            }
            builder = builder.headers_order(names);
        }

        // Referer options.
        apply_option!(apply_if_some, builder, params.referer, referer);

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
            params.connect_timeout,
            connect_timeout,
            Duration::from_secs
        );
        apply_option!(
            apply_transformed_option,
            builder,
            params.read_timeout,
            read_timeout,
            Duration::from_secs
        );
        apply_option!(
            apply_option_or_default,
            builder,
            params.no_keepalive,
            no_keepalive,
            false
        );
        apply_option!(
            apply_transformed_option,
            builder,
            params.tcp_keepalive,
            tcp_keepalive,
            Duration::from_secs
        );
        apply_option!(
            apply_transformed_option,
            builder,
            params.pool_idle_timeout,
            pool_idle_timeout,
            Duration::from_secs
        );
        apply_option!(
            apply_if_some,
            builder,
            params.pool_max_idle_per_host,
            pool_max_idle_per_host
        );
        apply_option!(apply_if_some, builder, params.pool_max_size, pool_max_size);

        // Protocol options.
        apply_option!(
            apply_option_or_default,
            builder,
            params.http1_only,
            http1_only,
            false
        );
        apply_option!(
            apply_option_or_default,
            builder,
            params.http2_only,
            http2_only,
            false
        );
        apply_option!(apply_if_some, builder, params.https_only, https_only);
        apply_option!(apply_if_some, builder, params.tcp_nodelay, tcp_nodelay);
        apply_option!(
            apply_if_some,
            builder,
            params.http2_max_retry_count,
            http2_max_retry_count
        );
        apply_option!(apply_if_some, builder, params.tls_info, tls_info);
        apply_option!(
            apply_if_some,
            builder,
            params.danger_accept_invalid_certs,
            danger_accept_invalid_certs
        );

        // Network options.
        if let Some(proxies) = params.proxies.take() {
            for proxy in proxies {
                builder = builder.proxy(proxy.into_inner());
            }
        }
        apply_option!(
            apply_option_or_default,
            builder,
            params.no_proxy,
            no_proxy,
            false
        );
        apply_option!(apply_if_some, builder, params.local_address, local_address);
        rquest::cfg_bindable_device!({
            apply_option!(apply_if_some, builder, params.interface, interface);
        });

        // Compression options.
        apply_option!(apply_if_some, builder, params.gzip, gzip);
        apply_option!(apply_if_some, builder, params.brotli, brotli);
        apply_option!(apply_if_some, builder, params.deflate, deflate);
        apply_option!(apply_if_some, builder, params.zstd, zstd);

        builder.build().map(Client).map_err(wrap_rquest_error)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Client {
    #[pyo3(signature = (url, **kwds))]
    fn get<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move { get(url, kwds).await })
    }
}

#[inline]
pub async fn get(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::GET, url, params).await
}

#[inline]
pub async fn post(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::POST, url, params).await
}

#[inline]
pub async fn put(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::PUT, url, params).await
}

#[inline]
pub async fn patch(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::PATCH, url, params).await
}

#[inline]
pub async fn delete(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::DELETE, url, params).await
}

#[inline]
pub async fn head(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::HEAD, url, params).await
}

#[inline]
pub async fn options(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::OPTIONS, url, params).await
}

#[inline]
pub async fn trace(url: String, params: Option<RequestParams>) -> Result<Response> {
    request(Method::TRACE, url, params).await
}

pub async fn request(
    method: Method,
    url: String,
    mut params: Option<RequestParams>,
) -> Result<Response> {
    let client = rquest::Client::builder()
        .build()
        .map_err(wrap_rquest_error)?;

    let builder = client.request(method.into_inner(), url);
    apply_params_to_request(builder, params.get_or_insert_default())
        .send()
        .await
        .map(Response::from)
        .map_err(wrap_rquest_error)
}

/// Apply the parameters to the request builder.
fn apply_params_to_request(
    mut builder: RequestBuilder,
    params: &mut RequestParams,
) -> RequestBuilder {
    // Version options.
    apply_option!(
        apply_transformed_option,
        builder,
        params.version,
        version,
        |v: Version| v.into_inner()
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
    apply_option!(apply_if_some, builder, params.local_address, local_address);
    rquest::cfg_bindable_device!(
        apply_option!(apply_if_some, builder, params.interface, interface);
    );

    // Headers options.
    if let Some(headers) = params.headers.take() {
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
    }

    // Authentication options.
    apply_option!(apply_if_some, builder, params.auth, auth);

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
    apply_option!(apply_if_some, builder, params.body, body);

    builder
}
