#[macro_export]
macro_rules! apply_option {
    (apply_if_some, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    (apply_if_some_ref, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&value);
        }
    };
    (apply_if_some_inner, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value.0);
        }
    };
    (apply_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (apply_if_ok, $builder:expr, $result:expr, $method:ident) => {
        if let Ok(value) = $result() {
            $builder = $builder.$method(value);
        }
    };
    (apply_transformed_option_ref, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(&value));
        }
    };
    (apply_ref_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&$transform(value));
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

macro_rules! request_impl {
    ($type_name:ty, function_name:ty, $method:expr, $return_type:ty) => {
        #[cfg_attr(feature = "docs", pyo3_stub_gen::derive::gen_stub_pymethods)]
        #[pyo3::pymethods]
        impl $type_name {
            #[pyo3::pyo3(signature = (
                        url,
                        proxy = None,
                        ocal_address = None,
                        interface = None,
                        timeout = None,
                        read_timeout = None,
                        version = None,
                        headers = None,
                        cookies = None,
                        allow_redirects = None,
                        max_redirects = None,
                        auth = None,
                        bearer_auth = None,
                        basic_auth = None,
                        query = None,
                        form = None,
                        json = None,
                        body = None,
                        multipart = None
                    ))]
            #[inline(always)]
            fn $function_name<'rt>(
                &self,
                py: Python<'rt>,
                url: pyo3::pybacked::PyBackedStr,
                method: $method,
                proxy: Option<crate::typing::ProxyExtractor>,
                local_address: Option<crate::typing::IpAddrExtractor>,
                interface: Option<String>,
                timeout: Option<u64>,
                read_timeout: Option<u64>,
                version: Option<crate::typing::Version>,
                headers: Option<crate::typing::HeaderMapExtractor>,
                cookies: Option<crate::typing::CookieExtractor>,
                allow_redirects: Option<bool>,
                max_redirects: Option<usize>,
                auth: Option<pyo3::pybacked::PyBackedStr>,
                bearer_auth: Option<pyo3::pybacked::PyBackedStr>,
                basic_auth: Option<(
                    pyo3::pybacked::PyBackedStr,
                    Option<pyo3::pybacked::PyBackedStr>,
                )>,
                query: Option<crate::typing::UrlEncodedValuesExtractor>,
                form: Option<UrlEncodedValuesExtractor>,
                json: Option<crate::typing::Json>,
                body: Option<crate::typing::BodyExtractor>,
                multipart: Option<crate::typing::MultipartExtractor>,
            ) -> PyResult<$return_type> {
                let params = crate::async_impl::RequestParams {
                    proxy,
                    local_address,
                    interface,
                    timeout,
                    read_timeout,
                    version,
                    headers,
                    cookies,
                    allow_redirects,
                    max_redirects,
                    auth,
                    bearer_auth,
                    basic_auth,
                    query,
                    form,
                    json,
                    body,
                    multipart,
                };
            }
        }
    };
}
