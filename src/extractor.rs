use bytes::Bytes;
use pyo3::{
    FromPyObject,
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyList},
};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use wreq::{
    EmulationFactory,
    header::{self, HeaderName, HeaderValue},
};

use crate::{
    browser::{Browser, BrowserOption},
    client::body::multipart::Multipart,
    emulation::{Emulation, Priority, PseudoId, SettingId, StreamId},
    error::Error,
    http::{
        Version,
        header::{HeaderMap, OrigHeaderMap},
    },
    proxy::Proxy,
};

/// A generic extractor for various types.
pub struct Extractor<T>(pub T);

/// Serialize implementation for [`Vec<(PyBackedStr, PyBackedStr)>`].
impl Serialize for Extractor<Vec<(PyBackedStr, PyBackedStr)>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            seq.serialize_element::<(&str, &str)>(&(key.as_ref(), value.as_ref()))?;
        }
        seq.end()
    }
}

/// Extractor for URL-encoded values as [`Vec<(PyBackedStr, PyBackedStr)>`].
impl FromPyObject<'_> for Extractor<Vec<(PyBackedStr, PyBackedStr)>> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self)
    }
}

/// Extractor for HTTP Version as [`wreq::Version`].
impl FromPyObject<'_> for Extractor<wreq::Version> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract::<Version>().map(Version::into_ffi).map(Self)
    }
}

/// Extractor for cookies as [`Vec<HeaderValue>`].
impl FromPyObject<'_> for Extractor<Vec<HeaderValue>> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        dict.iter()
            .try_fold(Vec::with_capacity(dict.len()), |mut cookies, (k, v)| {
                let cookie = {
                    let mut cookie = String::with_capacity(10);
                    cookie.push_str(k.extract::<PyBackedStr>()?.as_ref());
                    cookie.push('=');
                    cookie.push_str(v.extract::<PyBackedStr>()?.as_ref());
                    HeaderValue::from_maybe_shared(Bytes::from(cookie)).map_err(Error::from)?
                };

                cookies.push(cookie);
                Ok(cookies)
            })
            .map(Self)
    }
}

/// Extractor for headers as [`wreq::header::HeaderMap`].
impl FromPyObject<'_> for Extractor<wreq::header::HeaderMap> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(headers) = ob.downcast::<HeaderMap>() {
            return Ok(Self(headers.borrow().0.clone()));
        }

        let dict = ob.downcast::<PyDict>()?;
        dict.iter()
            .try_fold(
                header::HeaderMap::with_capacity(dict.len()),
                |mut headers, (name, value)| {
                    let name = {
                        let name = name.extract::<PyBackedStr>()?;
                        HeaderName::from_bytes(name.as_bytes()).map_err(Error::from)?
                    };

                    let value = {
                        let value = value.extract::<PyBackedStr>()?;
                        HeaderValue::from_maybe_shared(Bytes::from_owner(value))
                            .map_err(Error::from)?
                    };

                    headers.insert(name, value);
                    Ok(headers)
                },
            )
            .map(Self)
    }
}

/// Extractor for headers as [`wreq::header::OrigHeaderMap`].
impl FromPyObject<'_> for Extractor<wreq::header::OrigHeaderMap> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(headers) = ob.downcast::<OrigHeaderMap>() {
            return Ok(Self(headers.borrow().0.clone()));
        }

        let list = ob.downcast::<PyList>()?;
        list.iter()
            .try_fold(
                header::OrigHeaderMap::with_capacity(list.len()),
                |mut headers, name| {
                    let name = {
                        let name = name.extract::<PyBackedStr>()?;
                        Bytes::from_owner(name)
                    };
                    headers.insert(name);
                    Ok(headers)
                },
            )
            .map(Self)
    }
}

/// Extractor for emulation options as [`wreq::Emulation`].
impl FromPyObject<'_> for Extractor<wreq::Emulation> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(emulation) = ob.downcast::<Emulation>() {
            let emulation = emulation.borrow().0.clone();
            return Ok(Self(emulation));
        }

        if let Ok(browser) = ob.downcast::<Browser>() {
            let browser = browser.borrow().into_ffi();
            return Ok(Self(browser.emulation()));
        }

        let option = ob.downcast::<BrowserOption>()?;
        let option = option.borrow().0.clone();
        Ok(Self(option.emulation()))
    }
}

/// Extractor for a single proxy as [`wreq::Proxy`].
impl FromPyObject<'_> for Extractor<wreq::Proxy> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let proxy = ob.downcast::<Proxy>()?;
        let proxy = proxy.borrow().0.clone();
        Ok(Self(proxy))
    }
}

/// Extractor for a vector of proxies as [`Vec<wreq::Proxy>`].
impl FromPyObject<'_> for Extractor<Vec<wreq::Proxy>> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let proxies = ob.downcast::<PyList>()?;
        let len = proxies.len();
        proxies
            .into_iter()
            .try_fold(Vec::with_capacity(len), |mut list, proxy| {
                let proxy = proxy.downcast::<Proxy>()?;
                list.push(proxy.borrow().0.clone());
                Ok::<_, PyErr>(list)
            })
            .map(Self)
    }
}

/// Extractor for multipart forms as [`wreq::multipart::Form`].
impl FromPyObject<'_> for Extractor<wreq::multipart::Form> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let form = ob.downcast::<Multipart>()?;
        form.borrow_mut()
            .0
            .take()
            .map(Self)
            .ok_or_else(|| Error::Memory)
            .map_err(Into::into)
    }
}

/// Extractor for a single IP address as [`std::net::IpAddr`].
impl FromPyObject<'_> for Extractor<std::net::IpAddr> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self)
    }
}

/// Extractor for a stream ID as [`wreq::http2::StreamId`].
impl FromPyObject<'_> for Extractor<wreq::http2::StreamId> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(id) = ob.downcast::<StreamId>() {
            let id = id.borrow().into_ffi();
            return Ok(Self(id));
        }

        let id = ob.extract::<u32>()?;
        Ok(Self(id.into()))
    }
}

/// Extractor for pseudo headers order as [`wreq::http2::PseudoOrder`].
impl FromPyObject<'_> for Extractor<wreq::http2::PseudoOrder> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        let builder =
            list.into_iter()
                .try_fold(wreq::http2::PseudoOrder::builder(), |builder, id| {
                    let id = id.downcast::<PseudoId>()?;
                    Ok::<_, PyErr>(builder.push(id.borrow().into_ffi()))
                })?;
        Ok(Self(builder.build()))
    }
}

/// Extractor for setting ID as [`wreq::http2::SettingId`].
impl FromPyObject<'_> for Extractor<wreq::http2::SettingId> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(id) = ob.downcast::<SettingId>() {
            let id = id.borrow().into_ffi();
            return Ok(Self(id));
        }

        let id = ob.extract::<u16>()?;
        Ok(Self(id.into()))
    }
}

/// Extractor for experimental settings as [`wreq::http2::ExperimentalSettings`].
impl FromPyObject<'_> for Extractor<wreq::http2::ExperimentalSettings> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        let builder = dict.iter().try_fold(
            wreq::http2::ExperimentalSettings::builder(),
            |builder, (id, value)| {
                let id = id.extract::<Extractor<wreq::http2::SettingId>>()?;
                let value = value.extract::<u32>()?;
                let setting = wreq::http2::Setting::from_id(id.0, value);
                Ok::<_, PyErr>(builder.push(setting))
            },
        )?;
        Ok(Self(builder.build()))
    }
}

/// Extractor for settings order as [`wreq::http2::SettingsOrder`].
impl FromPyObject<'_> for Extractor<wreq::http2::SettingsOrder> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        let builder =
            list.into_iter()
                .try_fold(wreq::http2::SettingsOrder::builder(), |builder, id| {
                    let id = id.extract::<Extractor<wreq::http2::SettingId>>()?;
                    Ok::<_, PyErr>(builder.push(id.0))
                })?;
        Ok(Self(builder.build()))
    }
}

/// Extractor for priorities as [`wreq::http2::Priorities`].
impl FromPyObject<'_> for Extractor<wreq::http2::Priorities> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        let builder = list.into_iter().try_fold(
            wreq::http2::Priorities::builder(),
            |builder, priority| {
                let priority = priority.downcast::<Priority>()?;
                Ok::<_, PyErr>(builder.push(priority.borrow().0.clone()))
            },
        )?;
        Ok(Self(builder.build()))
    }
}
