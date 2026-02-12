use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use pyo3::{FromPyObject, prelude::*, types::PyList};

use crate::{
    emulation::{Emulation, EmulationOption},
    proxy::Proxy,
};

/// A generic extractor for various types.
pub struct Extractor<T>(pub T);

impl FromPyObject<'_, '_> for Extractor<wreq_util::EmulationOption> {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        if let Ok(impersonate) = ob.cast::<Emulation>() {
            let emulation = wreq_util::EmulationOption::builder()
                .emulation(impersonate.borrow().into_ffi())
                .build();

            return Ok(Self(emulation));
        }

        let option = ob.cast::<EmulationOption>()?.borrow();
        Ok(Self(option.0.clone()))
    }
}

impl FromPyObject<'_, '_> for Extractor<Vec<wreq::Proxy>> {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        let proxies = ob.cast::<PyList>()?;
        let len = proxies.len();
        proxies
            .iter()
            .try_fold(Vec::with_capacity(len), |mut list, proxy| {
                let proxy = proxy.cast::<Proxy>()?;
                list.push(proxy.borrow().0.clone());
                Ok::<_, PyErr>(list)
            })
            .map(Self)
    }
}

impl FromPyObject<'_, '_> for Extractor<(Option<Ipv4Addr>, Option<Ipv6Addr>)> {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        let (v4, v6) = ob.extract::<(Option<IpAddr>, Option<IpAddr>)>()?;
        Ok(Self((
            match v4 {
                Some(IpAddr::V4(addr)) => Some(addr),
                _ => None,
            },
            match v6 {
                Some(IpAddr::V6(addr)) => Some(addr),
                _ => None,
            },
        )))
    }
}
