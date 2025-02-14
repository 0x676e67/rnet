/*
 *   Copyright (c) 2025
 *   All rights reserved.
 */
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
    redirect::Policy,
};
use std::time::Duration;

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
    (apply_option_or_default_with_value, $builder:expr, $option:expr, $method:ident, $default:expr, $value:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method($value);
        }
    };
}

/// A client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Default)]
pub struct Client(rquest::Client);

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
    /// import asyncio
    /// import rnet
    ///
    /// client = rnet.Client(
    ///     user_agent="my-app/0.0.1",
    ///     timeout=10,
    /// )
    /// response = await client.get('https://httpbin.org/get')
    /// print(response.text)
    /// ```
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(mut kwds: Option<ClientParams>) -> PyResult<Client> {
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

        // Allow redirects options.
        apply_option!(
            apply_option_or_default_with_value,
            builder,
            params.allow_redirects,
            redirect,
            false,
            Policy::default()
        );

        // Cookie store options.
        apply_option!(apply_if_some, builder, params.cookie_store, cookie_store);

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

    /// Sends a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.get("https://httpbin.org/get")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn get<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::GET, url, kwds),
        )
    }

    /// Sends a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.post("://httpbin.org/post", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn post<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::POST, url, kwds),
        )
    }

    /// Sends a PUT request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.put("https://httpbin.org/put", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn put<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::PUT, url, kwds),
        )
    }

    /// Sends a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.patch("https://httpbin.org/patch", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn patch<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::PATCH, url, kwds),
        )
    }

    /// Sends a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.delete("https://httpbin.org/delete")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn delete<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::DELETE, url, kwds),
        )
    }

    /// Sends a HEAD request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.head("https://httpbin.org/head")
    ///     print(response.status_code)
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn head<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::HEAD, url, kwds),
        )
    }

    /// Sends an OPTIONS request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.options("https://httpbin.org/options")
    ///     print(response.headers)
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn options<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::OPTIONS, url, kwds),
        )
    }

    /// Sends a TRACE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.trace("https://httpbin.org/trace")
    ///     print(response.headers)
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    pub fn trace<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(client, Method::TRACE, url, kwds),
        )
    }

    /// Sends a request with the given method and URL.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use.
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.request(Method.GET, "https://httpbin.org/get")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request<'rt>(
        &self,
        py: Python<'rt>,
        method: Method,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, execute_request(client, method, url, kwds))
    }
}

/// Executes an HTTP request.
async fn execute_request(
    client: rquest::Client,
    method: Method,
    url: String,
    mut params: Option<RequestParams>,
) -> Result<Response> {
    let params = params.get_or_insert_default();
    let mut builder = client.request(method.into_inner(), url);

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

    // Send the request.
    builder
        .send()
        .await
        .map(Response::from)
        .map_err(wrap_rquest_error)
}
