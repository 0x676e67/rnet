use std::net::IpAddr;

use indexmap::IndexMap;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;

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
        Ok(params)
    }
}
