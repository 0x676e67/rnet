use crate::typing::{CookieFromPyDict, HeaderMapFromPyDict, IpAddr, QueryOrForm};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_stub_gen::{PyStubType, TypeInfo};

/// The parameters for a WebSocket request.
#[derive(Default)]
pub struct WebSocketParams {
    /// The proxy to use for the request.
    pub proxy: Option<PyBackedStr>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    pub interface: Option<String>,

    /// The headers to use for the request.
    pub headers: Option<HeaderMapFromPyDict>,

    /// The cookies to use for the request.
    pub cookies: Option<CookieFromPyDict>,

    /// The protocols to use for the request.
    pub protocols: Option<Vec<String>>,

    /// Whether to use HTTP/2 for the websocket.
    pub use_http2: Option<bool>,

    /// The authentication to use for the request.
    pub auth: Option<PyBackedStr>,

    /// The bearer authentication to use for the request.
    pub bearer_auth: Option<PyBackedStr>,

    /// The basic authentication to use for the request.
    pub basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,

    /// The query parameters to use for the request.
    pub query: Option<QueryOrForm>,

    /// Read buffer capacity. This buffer is eagerly allocated and used for receiving
    /// messages.
    ///
    /// For high read load scenarios a larger buffer, e.g. 128 KiB, improves performance.
    ///
    /// For scenarios where you expect a lot of connections and don't need high read load
    /// performance a smaller buffer, e.g. 4 KiB, would be appropriate to lower total
    /// memory usage.
    ///
    /// The default value is 128 KiB.
    pub read_buffer_size: Option<usize>,

    /// The target minimum size of the write buffer to reach before writing the data
    /// to the underlying stream.
    /// The default value is 128 KiB.
    ///
    /// If set to `0` each message will be eagerly written to the underlying stream.
    /// It is often more optimal to allow them to buffer a little, hence the default value.
    ///
    /// Note: [`flush`](WebSocket::flush) will always fully write the buffer regardless.
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
    pub max_write_buffer_size: Option<usize>,

    /// The maximum size of an incoming message. `None` means no size limit. The default value is 64 MiB
    /// which should be reasonably big for all normal use-cases but small enough to prevent
    /// memory eating by a malicious user.
    pub max_message_size: Option<usize>,

    /// The maximum size of a single incoming message frame. `None` means no size limit. The limit is for
    /// frame payload NOT including the frame header. The default value is 16 MiB which should
    /// be reasonably big for all normal use-cases but small enough to prevent memory eating
    /// by a malicious user.
    pub max_frame_size: Option<usize>,

    /// When set to `true`, the server will accept and handle unmasked frames
    /// from the client. According to the RFC 6455, the server must close the
    /// connection to the client in such cases, however it seems like there are
    /// some popular libraries that are sending unmasked frames, ignoring the RFC.
    /// By default this option is set to `false`, i.e. according to RFC 6455.
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

        extract_option!(ob, params, use_http2);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, cookies);
        extract_option!(ob, params, protocols);
        extract_option!(ob, params, auth);
        extract_option!(ob, params, bearer_auth);
        extract_option!(ob, params, basic_auth);
        extract_option!(ob, params, query);

        extract_option!(ob, params, read_buffer_size);
        extract_option!(ob, params, write_buffer_size);
        extract_option!(ob, params, max_write_buffer_size);
        extract_option!(ob, params, max_message_size);
        extract_option!(ob, params, max_frame_size);
        extract_option!(ob, params, accept_unmasked_frames);
        Ok(params)
    }
}

impl PyStubType for WebSocketParams {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, typing.Any]", "typing".into())
    }
}
