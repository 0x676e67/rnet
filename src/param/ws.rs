use indexmap::IndexMap;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use std::net::IpAddr;

/// The parameters for a WebSocket request.
///
/// # Examples
///
/// ```python
/// import rnet
/// from rnet import Impersonate, Version
///
/// params = rnet.WebSocketParams(
///     proxy="http://proxy.example.com",
///     local_address="192.168.1.1",
///     interface="eth0",
///     headers={"Content-Type": "application/json"},
///     auth="Basic dXNlcjpwYXNzd29yZA==",
///     bearer_auth="Bearer token",
///     basic_auth=("user", "password"),
///     query=[("key1", "value1"), ("key2", "value2")]
/// )
///
/// async with rnet.websocket("wss://echo.websocket.org") as ws:
///    await ws.send("Hello, World!")
///    message = await ws.recv()
///    print(message)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyclass]
#[pyclass]
#[derive(Default, Debug)]
pub struct WebSocketParams {
    /// The proxy to use for the request.
    #[pyo3(get)]
    pub proxy: Option<String>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    #[pyo3(get)]
    pub interface: Option<String>,

    /// The headers to use for the request.
    pub headers: Option<IndexMap<String, String>>,

    /// The protocols to use for the request.
    #[pyo3(get)]
    pub protocols: Option<Vec<String>>,

    /// The authentication to use for the request.
    #[pyo3(get)]
    pub auth: Option<String>,

    /// The bearer authentication to use for the request.
    #[pyo3(get)]
    pub bearer_auth: Option<String>,

    /// The basic authentication to use for the request.
    #[pyo3(get)]
    pub basic_auth: Option<(String, Option<String>)>,

    /// The query parameters to use for the request.
    #[pyo3(get)]
    pub query: Option<Vec<(String, String)>>,

    /// The target minimum size of the write buffer to reach before writing the data
    /// to the underlying stream.
    /// The default value is 128 KiB.
    ///
    /// If set to `0` each message will be eagerly written to the underlying stream.
    /// It is often more optimal to allow them to buffer a little, hence the default value.
    ///
    /// Note: [`flush`](WebSocket::flush) will always fully write the buffer regardless.
    #[pyo3(get)]
    pub write_buffer_size: Option<usize>,

    /// The max size of the write buffer in bytes. Setting this can provide backpressure
    /// in the case the write buffer is filling up due to write errors.
    /// The default value is unlimited.
    ///
    /// Note: The write buffer only builds up past [`write_buffer_size`](Self::write_buffer_size)
    /// when writes to the underlying stream are failing. So the **write buffer can not
    /// fill up if you are not observing write errors even if not flushing**.
    ///
    /// Note: Should always be at least [`write_buffer_size + 1 message`](Self::write_buffer_size)
    /// and probably a little more depending on error handling strategy.
    #[pyo3(get)]
    pub max_write_buffer_size: Option<usize>,

    /// The maximum size of an incoming message. `None` means no size limit. The default value is 64 MiB
    /// which should be reasonably big for all normal use-cases but small enough to prevent
    /// memory eating by a malicious user.
    #[pyo3(get)]
    pub max_message_size: Option<usize>,

    /// The maximum size of a single incoming message frame. `None` means no size limit. The limit is for
    /// frame payload NOT including the frame header. The default value is 16 MiB which should
    /// be reasonably big for all normal use-cases but small enough to prevent memory eating
    /// by a malicious user.
    #[pyo3(get)]
    pub max_frame_size: Option<usize>,

    /// When set to `true`, the server will accept and handle unmasked frames
    /// from the client. According to the RFC 6455, the server must close the
    /// connection to the client in such cases, however it seems like there are
    /// some popular libraries that are sending unmasked frames, ignoring the RFC.
    /// By default this option is set to `false`, i.e. according to RFC 6455.
    #[pyo3(get)]
    pub accept_unmasked_frames: Option<bool>,
}

macro_rules! extract_option {
    ($ob:expr, $params:expr, $field:ident) => {
        if let Ok(value) = $ob.get_item(stringify!($field)) {
            $params.$field = value.extract()?;
        }
    };
}

impl<'py> FromPyObject<'py> for WebSocketParams {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, proxy);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, headers);
        extract_option!(ob, params, protocols);
        extract_option!(ob, params, auth);
        extract_option!(ob, params, bearer_auth);
        extract_option!(ob, params, basic_auth);
        extract_option!(ob, params, query);

        extract_option!(ob, params, write_buffer_size);
        extract_option!(ob, params, max_write_buffer_size);
        extract_option!(ob, params, max_message_size);
        extract_option!(ob, params, max_frame_size);
        extract_option!(ob, params, accept_unmasked_frames);
        Ok(params)
    }
}
