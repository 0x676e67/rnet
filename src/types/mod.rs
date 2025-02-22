mod headers;
mod impersonate;
mod ipaddr;
mod json;
mod method;
mod multipart;
mod proxy;
mod status;
mod version;

pub use self::{
    headers::HeaderMap,
    impersonate::{Impersonate, ImpersonateOS},
    ipaddr::SocketAddr,
    json::Json,
    method::Method,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
    version::Version,
};

#[macro_export]
macro_rules! define_enum_with_conversion {
    ($(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, { $($variant:ident),* $(,)? }) => {
        $(#[$meta])*
        #[gen_stub_pyclass_enum]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        pub enum $enum_type {
            $($variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$variant => <$ffi_type>::$variant,)*
                }
            }

            pub fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$variant => <$enum_type>::$variant,)*
                    _ => unreachable!(),
                }
            }
        }

    };

    (const, $(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, { $($variant:ident),* $(,)? }) => {
        $(#[$meta])*
        #[gen_stub_pyclass_enum]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        pub enum $enum_type {
            $($variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$variant => <$ffi_type>::$variant,)*
                }
            }

            pub const fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$variant => <$enum_type>::$variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}
