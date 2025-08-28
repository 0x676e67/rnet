use bytes::Bytes;
use pyo3::{
    FromPyObject,
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyList},
};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use wreq::header::{self, HeaderName, HeaderValue};

use crate::{
    client::multipart::Multipart,
    emulation::{Emulation, EmulationOption},
    error::Error,
    http::header::{HeaderMap, OrigHeaderMap},
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

/// Extractor for emulation options as [`wreq_util::EmulationOption`].
impl FromPyObject<'_> for Extractor<wreq_util::EmulationOption> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(impersonate) = ob.downcast::<Emulation>() {
            let emulation = wreq_util::EmulationOption::builder()
                .emulation(impersonate.borrow().into_ffi())
                .build();

            return Ok(Self(emulation));
        }

        let option = ob.downcast::<EmulationOption>()?.borrow();
        Ok(Self(option.0.clone()))
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
