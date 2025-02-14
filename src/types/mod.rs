mod headers;
mod impersonate;
mod ipaddr;
mod json;
mod method;
mod proxy;
mod status_code;
mod version;

pub use self::{
    headers::HeaderMap,
    impersonate::{Impersonate, ImpersonateOS},
    ipaddr::SocketAddr,
    json::Json,
    method::Method,
    proxy::Proxy,
    status_code::StatusCode,
    version::Version,
};

#[macro_export]
macro_rules! define_constants {
    ($type:tt, $inner_type:ty, $($name:ident),*) => {
        #[allow(non_upper_case_globals)]
        #[gen_stub_pymethods]
        #[pymethods]
        impl $type {
            $(
                #[classattr]
                pub const $name: $type = $type(<$inner_type>::$name);
            )*
        }
    };
}
