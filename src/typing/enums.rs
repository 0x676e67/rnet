use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

macro_rules! define_enum_with_conversion {
    ($(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        $(#[$meta])*
        #[gen_stub_pyclass_enum]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    ($(#[$meta:meta])* const, $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
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

define_enum_with_conversion!(
    /// An HTTP version.
    const,
    Version,
    rquest::Version,
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
);

define_enum_with_conversion!(
    /// An HTTP method.
    Method,
    rquest::Method,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    PATCH,
);

define_enum_with_conversion!(
    /// An impersonate.
    const,
    Impersonate,
    rquest_util::Emulation,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome126,
    Chrome127,
    Chrome128,
    Chrome129,
    Chrome130,
    Chrome131,
    Chrome132,
    Chrome133,
    Chrome134,
    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Firefox109,
    Firefox117,
    Firefox128,
    Firefox133,
    Firefox135,
    FirefoxPrivate135,
    FirefoxAndroid135,
    Firefox136,
    FirefoxPrivate136,
    SafariIos17_2,
    SafariIos17_4_1,
    SafariIos16_5,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_0,
    Safari17_2_1,
    Safari17_4_1,
    Safari17_5,
    Safari18,
    SafariIPad18,
    Safari18_2,
    Safari18_3,
    SafariIos18_1_1,
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp4_12,
    OkHttp5
);

define_enum_with_conversion!(
    /// An impersonate operating system.
    const,
    ImpersonateOS,
    rquest_util::EmulationOS,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
);

define_enum_with_conversion!(
    /// The lookup ip strategy.
    const,
    LookupIpStrategy,
    rquest::dns::LookupIpStrategy,
    Ipv4Only,
    Ipv6Only,
    Ipv4AndIpv6,
    Ipv6thenIpv4,
    Ipv4thenIpv6,
);

define_enum_with_conversion!(
    /// The TLS version.
    const,
    TlsVersion,
    rquest::TlsVersion,
    TLS_1_0,
    TLS_1_1,
    TLS_1_2,
    TLS_1_3,
);

define_enum_with_conversion!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    rquest::cookie::SameSite,
    Strict,
    Lax,
    None,
);
