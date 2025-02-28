use super::{execute_request2, execute_websocket_request2};
use crate::{
    apply_option, dns,
    error::{wrap_rquest_error, wrap_url_parse_error},
    param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    typing::{ImpersonateOS, Method, TlsVersion},
};
use arc_swap::ArcSwap;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect::Policy,
    Url,
};
use std::{net::IpAddr, num::NonZeroUsize, ops::Deref};
use std::{sync::Arc, time::Duration};

/// A client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
pub struct Client(ArcSwap<rquest::Client>);

impl Deref for Client {
    type Target = ArcSwap<rquest::Client>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Client {
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
    #[inline(always)]
    pub fn get<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::GET, url, kwds)
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
    ///     response = await client.post("https://httpbin.org/post", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn post<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::POST, url, kwds)
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
    #[inline(always)]
    pub fn put<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::PUT, url, kwds)
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
    #[inline(always)]
    pub fn patch<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::PATCH, url, kwds)
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
    #[inline(always)]
    pub fn delete<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::DELETE, url, kwds)
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
    #[inline(always)]
    pub fn head<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::HEAD, url, kwds)
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
    #[inline(always)]
    pub fn options<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::OPTIONS, url, kwds)
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
    #[inline(always)]
    pub fn trace<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        self.request(py, Method::TRACE, url, kwds)
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
    #[inline(always)]
    pub fn request<'rt>(
        &self,
        py: Python<'rt>,
        method: Method,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.load();
        future_into_py(py, execute_request2(client, method, url, kwds))
    }

    /// Sends a WebSocket request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the WebSocket request to.
    /// * `**kwds` - Additional WebSocket request parameters.
    ///
    /// # Returns
    ///
    /// A `WebSocket` object representing the WebSocket connection.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     ws = await client.websocket("wss://echo.websocket.org")
    ///     await ws.send(rnet.Message.from_text("Hello, WebSocket!"))
    ///     message = await ws.recv()
    ///     print("Received:", message.data)
    ///     await ws.close()
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn websocket<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.load();
        future_into_py(py, execute_websocket_request2(client, url, kwds))
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
    pub fn new(py: Python, mut kwds: Option<ClientParams>) -> PyResult<Client> {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = rquest::Client::builder().no_hickory_dns();

            // Impersonation options.
            if let Some(impersonate) = params.impersonate.take() {
                builder = builder.impersonate(
                    rquest::Impersonate::builder()
                        .impersonate(impersonate.into_ffi())
                        .impersonate_os(
                            params
                                .impersonate_os
                                .map(ImpersonateOS::into_ffi)
                                .unwrap_or_default(),
                        )
                        .skip_http2(params.impersonate_skip_http2.unwrap_or(false))
                        .skip_headers(params.impersonate_skip_headers.unwrap_or(false))
                        .build(),
                );
            }

            // Base URL options.
            apply_option!(apply_if_some, builder, params.base_url, base_url);

            // User agent options.
            apply_option!(apply_if_some, builder, params.user_agent, user_agent);

            // Default headers options.
            apply_option!(
                apply_transformed_option,
                builder,
                params.default_headers,
                default_headers,
                HeaderMap::from
            );

            // Headers order options.
            if let Some(headers_order) = params.headers_order.take() {
                builder = builder.headers_order(
                    headers_order
                        .into_iter()
                        .map(|name| HeaderName::from_bytes(name.as_bytes()))
                        .filter_map(std::result::Result::ok)
                        .collect::<Vec<_>>(),
                );
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
                params
                    .max_redirects
                    .take()
                    .map(Policy::limited)
                    .unwrap_or_default()
            );

            // Cookie store options.
            apply_option!(apply_if_some, builder, params.cookie_store, cookie_store);

            // Async resolver options.
            apply_option!(
                apply_if_ok,
                builder,
                || dns::get_or_try_init(params.lookup_ip_strategy),
                dns_resolver
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
            apply_option!(
                apply_transformed_option,
                builder,
                params.pool_max_size,
                pool_max_size,
                NonZeroUsize::new
            );

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

            // TLS options.
            apply_option!(
                apply_transformed_option,
                builder,
                params.min_tls_version,
                min_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.max_tls_version,
                max_tls_version,
                TlsVersion::into_ffi
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
                    builder = builder.proxy(proxy.into());
                }
            }
            apply_option!(
                apply_option_or_default,
                builder,
                params.no_proxy,
                no_proxy,
                false
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.local_address,
                local_address,
                IpAddr::from
            );
            rquest::cfg_bindable_device!({
                apply_option!(apply_if_some, builder, params.interface, interface);
            });

            // Compression options.
            apply_option!(apply_if_some, builder, params.gzip, gzip);
            apply_option!(apply_if_some, builder, params.brotli, brotli);
            apply_option!(apply_if_some, builder, params.deflate, deflate);
            apply_option!(apply_if_some, builder, params.zstd, zstd);

            builder
                .build()
                .map(ArcSwap::from_pointee)
                .map(Client)
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the user agent of the client.
    ///
    /// # Returns
    ///
    /// An optional string containing the user agent of the client.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client()
    /// user_agent = client.user_agent()
    /// print(user_agent)
    /// ```
    #[getter]
    pub fn user_agent(&self, py: Python) -> Option<String> {
        py.allow_threads(|| {
            self.0
                .load()
                .user_agent()
                .and_then(|hv| hv.to_str().ok())
                .map(ToString::to_string)
        })
    }

    /// Returns the headers of the client.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object containing the headers of the client.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client()
    /// headers = client.headers()
    /// print(headers)
    /// ```
    #[getter]
    pub fn headers(&self, py: Python) -> crate::HeaderMap {
        py.allow_threads(|| {
            let binding = self.0.load();
            let headers = binding.headers();
            crate::HeaderMap::from(headers.clone())
        })
    }

    /// Returns the cookies for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to get the cookies for.
    ///
    /// # Returns
    ///
    /// A list of cookie strings.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// cookies = client.get_cookies("https://example.com")
    /// print(cookies)
    /// ```
    #[pyo3(signature = (url))]
    pub fn get_cookies(&self, py: Python, url: &str) -> PyResult<Vec<String>> {
        py.allow_threads(|| {
            let url = Url::parse(url).map_err(wrap_url_parse_error)?;
            let cookies = self
                .0
                .load()
                .get_cookies(&url)
                .iter()
                .filter_map(|hv| hv.to_str().ok())
                .map(ToString::to_string)
                .collect::<Vec<String>>();

            Ok(cookies)
        })
    }

    /// Sets cookies for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to set the cookies for.
    /// * `value` - A list of cookie strings to set.
    ///
    /// # Returns
    ///
    /// A `PyResult` indicating success or failure.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// client.set_cookies("https://example.com", ["cookie1=value1", "cookie2=value2"])
    /// ```
    #[pyo3(signature = (url, value))]
    pub fn set_cookies(&self, py: Python, url: &str, value: Vec<String>) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url).map_err(wrap_url_parse_error)?;
            let value = value
                .into_iter()
                .map(|value| HeaderValue::from_bytes(value.as_bytes()))
                .flat_map(std::result::Result::ok)
                .collect::<Vec<HeaderValue>>();

            self.0.load().set_cookies(&url, value);
            Ok(())
        })
    }

    /// Updates the client with the given parameters.
    ///
    /// # Arguments
    /// * `params` - The parameters to update the client with.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client()
    /// client.update(
    ///    impersonate=rnet.Impersonate.Firefox135,
    ///    headers={"X-My-Header": "value"},
    ///    proxies=[rnet.Proxy.all("http://proxy.example.com:8080")],
    /// )
    /// ```
    #[pyo3(signature = (**kwds))]
    pub fn update(&self, py: Python, mut kwds: Option<UpdateClientParams>) {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();
            let mut this = self.0.load_full();
            let mut client_mut = Arc::make_mut(&mut this).as_mut();

            // Impersonation options.
            if let Some(impersonate) = params.impersonate.take() {
                client_mut.impersonate(
                    rquest::Impersonate::builder()
                        .impersonate(impersonate.into_ffi())
                        .impersonate_os(
                            params
                                .impersonate_os
                                .map(ImpersonateOS::into_ffi)
                                .unwrap_or_default(),
                        )
                        .skip_http2(params.impersonate_skip_http2.unwrap_or(false))
                        .skip_headers(params.impersonate_skip_headers.unwrap_or(false))
                        .build(),
                );
            }

            // Default headers options.
            params.headers.take().map(|default_headers| {
                let mut default_headers = HeaderMap::from(default_headers);
                std::mem::swap(client_mut.headers(), &mut default_headers)
            });

            // Headers order options.
            params.headers_order.take().map(|value| {
                client_mut.headers_order(
                    value
                        .into_iter()
                        .map(|name| HeaderName::from_bytes(name.as_bytes()))
                        .filter_map(std::result::Result::ok)
                        .collect::<Vec<_>>(),
                );
            });

            // Network options.
            params.proxies.take().map(|proxies| {
                client_mut.proxies(proxies.into_iter().map(Into::into).collect::<Vec<_>>());
            });
            params
                .local_address
                .take()
                .map(|value| client_mut.local_address::<std::net::IpAddr>(value.into()));
            rquest::cfg_bindable_device!({
                params
                    .interface
                    .take()
                    .map(|value| client_mut.interface(value));
            });

            // Apply the changes.
            self.0.store(this);
        })
    }
}
