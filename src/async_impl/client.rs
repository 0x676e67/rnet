use super::request::{execute_request, execute_websocket_request};
use crate::{
    apply_option,
    buffer::{HeaderValueBuffer, PyBufferProtocol},
    dns,
    error::{wrap_rquest_error, wrap_url_parse_error},
    param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    typing::{Cookie, HeaderMap, ImpersonateOS, Method, SslVerify, TlsVersion},
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{RootCertStore, Url, redirect::Policy};
use std::time::Duration;
use std::{net::IpAddr, ops::Deref};

/// A client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
pub struct Client(rquest::Client);

impl Deref for Client {
    type Target = rquest::Client;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! define_http_method {
    ($(#[$meta:meta])* $name:ident, $method:expr) => {
        #[gen_stub_pymethods]
        #[pymethods]
        impl Client {
            /// Sends a request with the given URL
            ///
            /// # Arguments
            ///
            /// * `url` - The URL to send the request to.
            /// * `**kwds` - Additional request parameters.
            ///
            ///     proxy: typing.Optional[builtins.str]
            ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
            ///     interface: typing.Optional[builtins.str]
            ///     timeout: typing.Optional[builtins.int]
            ///     read_timeout: typing.Optional[builtins.int]
            ///     version: typing.Optional[Version]
            ///     headers: typing.Optional[typing.Dict[str, bytes]]
            ///     cookies: typing.Optional[typing.Dict[str, str]]
            ///     allow_redirects: typing.Optional[builtins.bool]
            ///     max_redirects: typing.Optional[builtins.int]
            ///     auth: typing.Optional[str]
            ///     bearer_auth: typing.Optional[str]
            ///     basic_auth: typing.Optional[tuple[str, typing.Optional[str]]]
            ///     query: typing.Optional[typing.List[typing.Tuple[str, str]]]
            ///     form: typing.Optional[typing.List[typing.Tuple[str, str]]]
            ///     json: typing.Optional[typing.Any]
            ///     body: typing.Optional[typing.Any]
            ///     multipart: typing.Optional[Multipart]
            ///
            /// # Returns
            ///
            /// A `Response` object.
            ///
            $(#[$meta])*
            #[pyo3(signature = (url, **kwds))]
            #[inline(always)]
            pub fn $name<'rt>(
                &self,
                py: Python<'rt>,
                url: PyBackedStr,
                kwds: Option<RequestParams>,
            ) -> PyResult<Bound<'rt, PyAny>> {
                self.request(py, $method, url, kwds)
            }
        }
    };
}

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.get("https://httpbin.org/anything")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    get,
    Method::GET
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.head("https://httpbin.org/anything")
    ///     print(response.status)
    ///
    /// asyncio.run(main())
    /// ```
    head,
    Method::HEAD
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.post("https://httpbin.org/anything", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    post,
    Method::POST
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.put("https://httpbin.org/anything", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    put,
    Method::PUT
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.delete("https://httpbin.org/anything")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    delete,
    Method::DELETE
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.patch("https://httpbin.org/anything", json={"key": "value"})
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    patch,
    Method::PATCH
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.options("https://httpbin.org/anything")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    options,
    Method::OPTIONS
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.Client()
    ///     response = await client.trace("https://httpbin.org/anything")
    ///     print(await response.text())
    ///
    /// asyncio.run(main())
    /// ```
    trace,
    Method::TRACE
);

#[gen_stub_pymethods]
#[pymethods]
impl Client {
    /// Sends a request with the given method and URL.
    ///
    /// # Arguments
    ///
    /// * `method` - The method to use for the request.
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    ///     proxy: typing.Optional[builtins.str]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     timeout: typing.Optional[builtins.int]
    ///     read_timeout: typing.Optional[builtins.int]
    ///     version: typing.Optional[Version]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     cookies: typing.Optional[typing.Dict[str, str]]
    ///     allow_redirects: typing.Optional[builtins.bool]
    ///     max_redirects: typing.Optional[builtins.int]
    ///     auth: typing.Optional[str]
    ///     bearer_auth: typing.Optional[str]
    ///     basic_auth: typing.Optional[tuple[str, typing.Optional[str]]]
    ///     query: typing.Optional[typing.List[typing.Tuple[str, str]]]
    ///     form: typing.Optional[typing.List[typing.Tuple[str, str]]]
    ///     json: typing.Optional[typing.Any]
    ///     body: typing.Optional[typing.Any]
    ///     multipart: typing.Optional[Multipart]
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
    ///     response = await client.request(Method.GET, "https://httpbin.org/anything")
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
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_request(client, method, url, kwds))
    }

    /// Sends a WebSocket request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the WebSocket request to.
    /// * `**kwds` - Additional WebSocket request parameters.
    ///
    ///     proxy: typing.Optional[builtins.str]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     cookies: typing.Optional[typing.Dict[str, str]]
    ///     protocols: typing.Optional[builtins.list[builtins.str]]
    ///     use_http2: typing.Optional[builtins.bool]
    ///     auth: typing.Optional[builtins.str]
    ///     bearer_auth: typing.Optional[builtins.str]
    ///     basic_auth: typing.Optional[tuple[builtins.str, typing.Optional[builtins.str]]]
    ///     query: typing.Optional[builtins.list[tuple[builtins.str, builtins.str]]]
    ///     read_buffer_size: typing.Optional[builtins.int]
    ///     write_buffer_size: typing.Optional[builtins.int]
    ///     max_write_buffer_size: typing.Optional[builtins.int]
    ///     max_message_size: typing.Optional[builtins.int]
    ///     max_frame_size: typing.Optional[builtins.int]
    ///     accept_unmasked_frames: typing.Optional[builtins.bool]
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
        url: PyBackedStr,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_websocket_request(client, url, kwds))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Client {
    /// Creates a new Client instance.
    ///
    /// # Arguments
    ///
    /// * `**kwds` - Optional request parameters as a dictionary.
    ///
    ///     impersonate: typing.Optional[Impersonate]
    ///     impersonate_os: typing.Optional[ImpersonateOS]
    ///     impersonate_skip_http2: typing.Optional[builtins.bool]
    ///     impersonate_skip_headers: typing.Optional[builtins.bool]
    ///     base_url: typing.Optional[str]
    ///     user_agent: typing.Optional[str]
    ///     default_headers: typing.Optional[typing.Dict[str, bytes]]
    ///     headers_order: typing.Optional[typing.List[str]]
    ///     referer: typing.Optional[builtins.bool]
    ///     allow_redirects: typing.Optional[builtins.bool]
    ///     max_redirects: typing.Optional[builtins.int]
    ///     cookie_store: typing.Optional[builtins.bool]
    ///     lookup_ip_strategy: typing.Optional[LookupIpStrategy]
    ///     timeout: typing.Optional[builtins.int]
    ///     connect_timeout: typing.Optional[builtins.int]
    ///     read_timeout: typing.Optional[builtins.int]
    ///     no_keepalive: typing.Optional[builtins.bool]
    ///     tcp_keepalive: typing.Optional[builtins.int]
    ///     pool_idle_timeout: typing.Optional[builtins.int]
    ///     pool_max_idle_per_host: typing.Optional[builtins.int]
    ///     pool_max_size: typing.Optional[builtins.int]
    ///     http1_only: typing.Optional[builtins.bool]
    ///     http2_only: typing.Optional[builtins.bool]
    ///     https_only: typing.Optional[builtins.bool]
    ///     tcp_nodelay: typing.Optional[builtins.bool]
    ///     http2_max_retry_count: typing.Optional[builtins.int]
    ///     verify: Optional[Union[bool, Path]]
    ///     tls_info: typing.Optional[builtins.bool]
    ///     min_tls_version: typing.Optional[TlsVersion]
    ///     max_tls_version: typing.Optional[TlsVersion]
    ///     no_proxy: typing.Optional[builtins.bool]
    ///     proxies: typing.Optional[builtins.list[Proxy]]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     gzip: typing.Optional[builtins.bool]
    ///     brotli: typing.Optional[builtins.bool]
    ///     deflate: typing.Optional[builtins.bool]
    ///     zstd: typing.Optional[builtins.bool]
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
                builder = builder.emulation(
                    rquest_util::EmulationOption::builder()
                        .emulation(impersonate.into_ffi())
                        .emulation_os(
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

            // User agent options.
            apply_option!(
                apply_transformed_option_ref,
                builder,
                params.user_agent,
                user_agent,
                AsRef::<str>::as_ref
            );

            // Default headers options.
            apply_option!(
                apply_if_some_inner,
                builder,
                params.default_headers,
                default_headers
            );

            // Headers order options.
            apply_option!(
                apply_if_some_inner,
                builder,
                params.headers_order,
                headers_order
            );

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

            // SSL Verification options.
            if let Some(verify) = params.verify.take() {
                builder = match verify {
                    SslVerify::DisableSslVerification(verify) => {
                        builder.danger_accept_invalid_certs(!verify)
                    }
                    SslVerify::RootCertificateFilepath(path_buf) => {
                        let store =
                            RootCertStore::from_pem_file(path_buf).map_err(wrap_rquest_error)?;
                        builder.root_cert_store(store)
                    }
                }
            }

            // Network options.
            if let Some(proxies) = params.proxies.take() {
                for proxy in proxies {
                    builder = builder.proxy(proxy);
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

            // Compression options.
            apply_option!(apply_if_some, builder, params.gzip, gzip);
            apply_option!(apply_if_some, builder, params.brotli, brotli);
            apply_option!(apply_if_some, builder, params.deflate, deflate);
            apply_option!(apply_if_some, builder, params.zstd, zstd);

            builder.build().map(Client).map_err(wrap_rquest_error)
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
                .user_agent()
                .and_then(|hv| hv.to_str().map(ToString::to_string).ok())
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
    pub fn headers(&self) -> HeaderMap {
        HeaderMap(self.0.headers())
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
    pub fn get_cookies<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        let cookies = py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(wrap_url_parse_error)?;
            let cookies = self.0.get_cookies(&url);
            Ok::<_, PyErr>(cookies)
        })?;

        cookies
            .map(HeaderValueBuffer::new)
            .map(|buffer| buffer.into_bytes_ref(py))
            .transpose()
    }

    /// Sets the cookies for the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL to set the cookies for.
    /// * `cookie` - The cookie to set.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// client.set_cookie("https://example.com", rnet.Cookie(name="foo", value="bar"))
    /// ```
    #[pyo3(signature = (url, cookie))]
    pub fn set_cookie(&self, py: Python, url: PyBackedStr, cookie: Cookie) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(wrap_url_parse_error)?;
            self.0.set_cookie(&url, cookie.0);
            Ok(())
        })
    }

    /// Removes the cookie with the given name for the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL to remove the cookie from.
    /// * `name` - The name of the cookie to remove.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// client.remove_cookie("https://example.com", "foo")
    #[pyo3(signature = (url, name))]
    pub fn remove_cookie(&self, py: Python, url: PyBackedStr, name: PyBackedStr) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(wrap_url_parse_error)?;
            self.0.remove_cookie(&url, &name);
            Ok(())
        })
    }

    /// Clears the cookies for the given URL.
    pub fn clear_cookies(&self, py: Python) {
        py.allow_threads(|| {
            self.0.clear_cookies();
        })
    }

    /// Updates the client with the given parameters.
    ///
    /// # Arguments
    /// * `**kwds` - The parameters to update the client with.
    ///
    ///     impersonate: typing.Optional[Impersonate]
    ///     impersonate_os: typing.Optional[ImpersonateOS]
    ///     impersonate_skip_http2: typing.Optional[builtins.bool]
    ///     impersonate_skip_headers: typing.Optional[builtins.bool]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     headers_order: typing.Optional[typing.List[str]]
    ///     proxies: typing.Optional[builtins.list[Proxy]]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
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
    pub fn update(&self, py: Python, mut kwds: Option<UpdateClientParams>) -> PyResult<()> {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();
            let mut update = self.0.update();

            // Impersonation options.
            if let Some(impersonate) = params.impersonate.take() {
                update = update.emulation(
                    rquest_util::EmulationOption::builder()
                        .emulation(impersonate.into_ffi())
                        .emulation_os(
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
            if let Some(mut default_headers) = params.headers.take() {
                update = update.headers(|headers| std::mem::swap(headers, &mut default_headers.0));
            }

            // Headers order options.
            apply_option!(
                apply_if_some_inner,
                update,
                params.headers_order,
                headers_order
            );

            // Network options.
            apply_option!(apply_if_some, update, params.proxies, proxies);
            apply_option!(
                apply_transformed_option,
                update,
                params.local_address,
                local_address,
                IpAddr::from
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
            apply_option!(apply_if_some, update, params.interface, interface);

            // Apply the changes.
            update.apply().map_err(wrap_rquest_error)
        })
    }
}
