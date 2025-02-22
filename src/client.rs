use crate::{
    dns,
    error::{
        wrap_invali_header_name_error, wrap_invali_header_value_error, wrap_rquest_error,
        wrap_url_parse_error,
    },
    param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    response::{Response, WebSocket},
    types::{ImpersonateOS, Method, Version},
    Result,
};
use arc_swap::{ArcSwap, Guard};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect::Policy,
    Url,
};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

macro_rules! apply_option {
    (apply_if_some, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    (apply_if_ok, $builder:expr, $result:expr, $method:ident) => {
        if let Ok(value) = $result() {
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
pub struct Client(ArcSwap<rquest::Client>);

impl Client {
    /// Creates a new default `Client` instance.
    ///
    /// This method initializes a `Client` with default settings, including disabling
    /// Hickory DNS and keepalive.
    ///
    /// # Panics
    ///
    /// This method will panic if the Client cannot be created.
    ///
    /// # Note
    ///
    /// This client does not maintain a session, meaning it does not share cookies,
    /// headers, or other parameters between requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rnet::Client;
    ///
    /// let client = Client::default();
    /// ```
    pub fn default() -> &'static Self {
        static CLIENT: LazyLock<Client> = LazyLock::new(|| {
            let mut builder = rquest::Client::builder();
            apply_option!(apply_if_ok, builder, dns::get_or_try_init, dns_resolver);
            builder
                .no_hickory_dns()
                .no_keepalive()
                .build()
                .map(ArcSwap::from_pointee)
                .map(Client)
                .expect("Failed to build the default client.")
        });

        &*CLIENT
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
    pub fn get<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.load();
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
    ///     response = await client.post("https://httpbin.org/post", json={"key": "value"})
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
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
        let client = self.0.load();
        pyo3_async_runtimes::tokio::future_into_py(py, execute_request(client, method, url, kwds))
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
    pub fn websocket<'rt>(
        &self,
        py: Python<'rt>,
        url: String,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.load();
        pyo3_async_runtimes::tokio::future_into_py(py, execute_websocket_request(client, url, kwds))
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
    fn new(mut kwds: Option<ClientParams>) -> PyResult<Client> {
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
        if let Some(default_headers) = params.default_headers.take() {
            let len = default_headers.len();
            let default_headers = default_headers.into_iter().try_fold(
                HeaderMap::with_capacity(len),
                |mut headers, (key, value)| {
                    let name = HeaderName::from_bytes(key.as_bytes())
                        .map_err(wrap_invali_header_name_error)?;
                    let value = HeaderValue::from_bytes(value.as_bytes())
                        .map_err(wrap_invali_header_value_error)?;
                    headers.insert(name, value);
                    Ok::<_, PyErr>(headers)
                },
            )?;

            builder = builder.default_headers(default_headers);
        }

        // Headers order options.
        if let Some(headers_order) = params.headers_order.take() {
            builder = builder.headers_order(
                headers_order
                    .into_iter()
                    .map(|name| {
                        HeaderName::from_bytes(name.as_bytes())
                            .map_err(wrap_invali_header_name_error)
                    })
                    .filter_map(Result::ok)
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
            Policy::default()
        );

        // Cookie store options.
        apply_option!(apply_if_some, builder, params.cookie_store, cookie_store);

        // Async resolver options.
        apply_option!(apply_if_ok, builder, dns::get_or_try_init, dns_resolver);

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
        apply_option!(apply_if_some, builder, params.local_address, local_address);
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
    fn user_agent(&self) -> Option<String> {
        self.0
            .load()
            .user_agent()
            .and_then(|hv| hv.to_str().ok())
            .map(ToString::to_string)
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
    fn headers(&self) -> crate::HeaderMap {
        let binding = self.0.load();
        let headers = binding.headers();
        crate::HeaderMap::from(headers.clone())
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
    /// client = rnet.Client()
    /// cookies = client.get_cookies("https://example.com")
    /// print(cookies)
    /// ```
    #[pyo3(signature = (url))]
    fn get_cookies(&self, url: &str) -> PyResult<Vec<String>> {
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
    /// client = rnet.Client()
    /// client.set_cookies("https://example.com", ["cookie1=value1", "cookie2=value2"])
    /// ```
    #[pyo3(signature = (url, value))]
    fn set_cookies(&self, url: &str, value: Vec<String>) -> PyResult<()> {
        let url = Url::parse(url).map_err(wrap_url_parse_error)?;
        let value = value
            .into_iter()
            .map(|value| {
                HeaderValue::from_bytes(value.as_bytes()).map_err(wrap_invali_header_value_error)
            })
            .flat_map(Result::ok)
            .collect::<Vec<HeaderValue>>();

        self.0.load().set_cookies(&url, value);
        Ok(())
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
    fn update(&self, mut kwds: Option<UpdateClientParams>) {
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
            let len = default_headers.len();
            let _ = default_headers
                .into_iter()
                .try_fold(
                    HeaderMap::with_capacity(len),
                    |mut headers, (key, value)| {
                        let name = HeaderName::from_bytes(key.as_bytes())
                            .map_err(wrap_invali_header_name_error)?;
                        let value = HeaderValue::from_bytes(value.as_bytes())
                            .map_err(wrap_invali_header_value_error)?;
                        headers.insert(name, value);
                        Ok::<_, PyErr>(headers)
                    },
                )
                .map(|mut headers| std::mem::swap(client_mut.headers(), &mut headers));
        });

        // Headers order options.
        params.headers_order.take().map(|value| {
            client_mut.headers_order(
                value
                    .into_iter()
                    .map(|name| {
                        HeaderName::from_bytes(name.as_bytes())
                            .map_err(wrap_invali_header_name_error)
                    })
                    .filter_map(Result::ok)
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
            .map(|value| client_mut.local_address(value));
        rquest::cfg_bindable_device!({
            params
                .interface
                .take()
                .map(|value| client_mut.interface(value));
        });

        // Apply the changes.
        self.0.store(this);
    }
}

/// Executes an HTTP request.
async fn execute_request(
    client: Guard<Arc<rquest::Client>>,
    method: Method,
    url: String,
    mut params: Option<RequestParams>,
) -> Result<Response> {
    let params = params.get_or_insert_default();
    let mut builder = client.request(method.into_ffi(), url);

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
        Policy::default()
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

    // Multipart options.
    if let Some(multipart) = params.multipart.take() {
        let multipart = Python::with_gil(|py| multipart.borrow_mut(py).0.take());
        if let Some(multipart) = multipart {
            builder = builder.multipart(multipart);
        }
    }

    // Send the request.
    builder
        .send()
        .await
        .map(Response::from)
        .map_err(wrap_rquest_error)
}

/// Executes a WebSocket request.
async fn execute_websocket_request(
    client: Guard<Arc<rquest::Client>>,
    url: String,
    mut params: Option<WebSocketParams>,
) -> Result<WebSocket> {
    let params = params.get_or_insert_default();
    let mut builder = client.websocket(url);

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
        apply_option!(apply_if_some, builder, params.proxy, proxy);
        apply_option!(apply_if_some, builder, params.local_address, local_address);
        rquest::cfg_bindable_device!(
            apply_option!(apply_if_some, builder, params.interface, interface);
        );

        // Authentication options.
        apply_option!(apply_if_some, builder, params.auth, auth);

        // Bearer authentication options.
        apply_option!(apply_if_some, builder, params.bearer_auth, bearer_auth);

        // Basic authentication options.
        if let Some(basic_auth) = params.basic_auth.take() {
            builder = builder.basic_auth(basic_auth.0, basic_auth.1);
        }

        // Headers options.
        if let Some(headers) = params.headers.take() {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        // Query options.
        apply_option!(apply_if_some_ref, builder, params.query, query);

        builder
    });

    WebSocket::new(builder).await
}
