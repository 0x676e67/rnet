use crate::define_enum_with_conversion;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

define_enum_with_conversion!(
    const,
    /// An HTTP version.
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
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
);

define_enum_with_conversion!(
    const,
    /// An impersonate.
    Impersonate,
    rquest::Impersonate,
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
    Chrome133,
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
    SafariIos18_1_1,
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp5
);

define_enum_with_conversion!(
    const,
    /// An impersonate operating system.
    ImpersonateOS,
    rquest::ImpersonateOS,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
);

define_enum_with_conversion!(
    const,
    /// The lookup ip strategy.
    LookupIpStrategy,
    rquest::dns::LookupIpStrategy,
    Ipv4Only,
    Ipv6Only,
    Ipv4AndIpv6,
    Ipv6thenIpv4,
    Ipv4thenIpv6,
);

define_enum_with_conversion!(
    const,
    /// The TLS version.
    TlsVersion,
    rquest::TlsVersion,
    TLS_1_0,
    TLS_1_1,
    TLS_1_2,
    TLS_1_3,
);
