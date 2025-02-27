use crate::error::{wrap_invali_header_name_error, wrap_invali_header_value_error};
use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict},
};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::header::{self, HeaderName, HeaderValue};
use std::{collections::HashSet, ops::Deref, str::FromStr};

/// A HTTP header map.
///
/// # Examples
///
/// ```python
/// import rnet
/// from rnet import Method
///
/// async def main():
///     resp = await rnet.request(Method.GET, "https://www.google.com/")
///     print("Headers: ", resp.headers)
///
/// if __name__ == "__main__":
///     import asyncio
///     asyncio.run(main())
/// ```
#[derive(Clone, Debug)]
pub struct HeaderMap(header::HeaderMap);

impl From<header::HeaderMap> for HeaderMap {
    fn from(map: header::HeaderMap) -> Self {
        HeaderMap(map)
    }
}

impl From<HeaderMap> for header::HeaderMap {
    fn from(map: HeaderMap) -> Self {
        map.0
    }
}

impl Deref for HeaderMap {
    type Target = header::HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PyStubType for HeaderMap {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Dict[str, bytes]".to_owned(),
            import: HashSet::new(),
        }
    }
}

impl FromPyObject<'_> for HeaderMap {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        let mut headers = header::HeaderMap::new();
        for (key, value) in dict.iter() {
            let key = key.extract::<&str>()?;
            let value = value.extract::<&str>()?;
            headers.insert(
                HeaderName::from_str(key).map_err(wrap_invali_header_name_error)?,
                HeaderValue::from_str(value).map_err(wrap_invali_header_value_error)?,
            );
        }

        Ok(HeaderMap(headers))
    }
}
impl<'py> IntoPyObject<'py> for HeaderMap {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);
        for (header, value) in &self.0 {
            dict.set_item(header.as_str(), PyBytes::new(py, value.as_ref()))?;
        }
        Ok(dict)
    }
}
