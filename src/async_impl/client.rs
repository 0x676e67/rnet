use super::request::{execute_request, execute_websocket_request};
use crate::{
    buffer::{HeaderValueBuffer, PyBufferProtocol},
    dns,
    error::Error,
    typing::{
        Cookie, HeaderMap, Method, SslVerify, TlsVersion,
        param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    },
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use std::ops::Deref;
use std::time::Duration;
use wreq::{
    CertStore, Url,
    header::{Entry, OccupiedEntry},
    redirect::Policy,
};

/// A client for making HTTP requests.
#[pyclass(subclass)]
pub struct Client(wreq::Client);

impl Deref for Client {
    type Target = wreq::Client;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl Client {
    /// Make a GET request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn get<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::GET, url, kwds)
    }

    /// Make a HEAD request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn head<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::HEAD, url, kwds)
    }

    /// Make a POST request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn post<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::POST, url, kwds)
    }

    /// Make a PUT request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn put<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PUT, url, kwds)
    }

    /// Make a DELETE request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn delete<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::DELETE, url, kwds)
    }

    /// Make a PATCH request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn patch<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PATCH, url, kwds)
    }

    /// Make a OPTIONS request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn options<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::OPTIONS, url, kwds)
    }

    /// Make a TRACE request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn trace<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::TRACE, url, kwds)
    }

    /// Make a request with the given method and URL.
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request<'py>(
        &self,
        py: Python<'py>,
        method: Method,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_request(client, method, url, kwds))
    }

    /// Make a WebSocket request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn websocket<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_websocket_request(client, url, kwds))
    }
}

#[pymethods]
impl Client {
    /// Creates a new Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    pub fn new(py: Python, mut kwds: Option<ClientParams>) -> PyResult<Client> {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::Client::builder().no_hickory_dns();

            // Impersonation options.
            if let Some(impersonate) = params.impersonate.take() {
                builder = builder.emulation(impersonate.0);
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
                    SslVerify::DisableSslVerification(verify) => builder.cert_verification(verify),
                    SslVerify::RootCertificateFilepath(path_buf) => {
                        let store = CertStore::from_pem_file(path_buf).map_err(Error::Request)?;
                        builder.cert_store(store)
                    }
                }
            }

            // Network options.
            if let Some(proxies) = params.proxies.take() {
                for proxy in proxies.0 {
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

            // Compression options.
            apply_option!(apply_if_some, builder, params.gzip, gzip);
            apply_option!(apply_if_some, builder, params.brotli, brotli);
            apply_option!(apply_if_some, builder, params.deflate, deflate);
            apply_option!(apply_if_some, builder, params.zstd, zstd);

            builder
                .http1(|mut http1| {
                    http1.title_case_headers(true);
                })
                .build()
                .map(Client)
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Returns the user agent of the client.
    #[getter]
    pub fn user_agent(&self, py: Python) -> Option<String> {
        py.allow_threads(|| {
            self.0
                .user_agent()
                .and_then(|hv| hv.to_str().map(ToString::to_string).ok())
        })
    }

    /// Returns the headers of the client.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        HeaderMap(self.0.headers())
    }

    /// Returns the cookies for the given URL.
    #[pyo3(signature = (url))]
    pub fn get_cookies<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        let cookies = py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::from)?;
            let cookies = self.0.get_cookies(&url);
            Ok::<_, PyErr>(cookies.map(HeaderValueBuffer::new))
        })?;

        cookies.map(|buffer| buffer.into_bytes_ref(py)).transpose()
    }

    /// Sets the cookies for the given URL.
    #[pyo3(signature = (url, cookie))]
    pub fn set_cookie(&self, py: Python, url: PyBackedStr, cookie: Cookie) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::from)?;
            self.0.set_cookie(&url, cookie.0);
            Ok(())
        })
    }

    /// Removes the cookie with the given name for the given URL.
    #[pyo3(signature = (url, name))]
    pub fn remove_cookie(&self, py: Python, url: PyBackedStr, name: PyBackedStr) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::from)?;
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
    #[pyo3(signature = (**kwds))]
    pub fn update(&self, py: Python, mut kwds: Option<UpdateClientParams>) -> PyResult<()> {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();

            // Create a new client with the current configuration.
            let mut update = self.0.update();

            // Impersonation options.
            apply_option!(apply_if_some_inner, update, params.impersonate, emulation);

            // Updated headers options.
            if let Some(src) = params.headers.take() {
                update = update.headers(|dst| {
                    // IntoIter of HeaderMap yields (Option<HeaderName>, HeaderValue).
                    // The first time a name is yielded, it will be Some(name), and if
                    // there are more values with the same name, the next yield will be
                    // None.

                    let mut prev_entry: Option<OccupiedEntry<_>> = None;
                    for (key, value) in src.0 {
                        match key {
                            Some(key) => match dst.entry(key) {
                                Entry::Occupied(mut e) => {
                                    e.insert(value);
                                    prev_entry = Some(e);
                                }
                                Entry::Vacant(e) => {
                                    let e = e.insert_entry(value);
                                    prev_entry = Some(e);
                                }
                            },
                            None => match prev_entry {
                                Some(ref mut entry) => {
                                    entry.append(value);
                                }
                                None => unreachable!("HeaderMap::into_iter yielded None first"),
                            },
                        }
                    }
                });
            }

            // Headers order options.
            apply_option!(
                apply_if_some_inner,
                update,
                params.headers_order,
                headers_order
            );

            // Network options.
            apply_option!(apply_if_some_inner, update, params.proxies, proxies);
            apply_option!(
                apply_if_some_inner,
                update,
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
            apply_option!(apply_if_some, update, params.interface, interface);

            // Apply the changes.
            update.apply().map_err(Error::Request).map_err(Into::into)
        })
    }
}
