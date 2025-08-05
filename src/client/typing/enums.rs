use pyo3::prelude::*;

define_enum_with_conversion!(
    /// An HTTP version.
    const,
    Version,
    wreq::Version,
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
);

define_enum_with_conversion!(
    /// An HTTP method.
    Method,
    wreq::Method,
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
    /// The lookup ip strategy.
    const,
    LookupIpStrategy,
    wreq::dns::LookupIpStrategy,
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
    wreq::tls::TlsVersion,
    TLS_1_0,
    TLS_1_1,
    TLS_1_2,
    TLS_1_3,
);

define_enum_with_conversion!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    wreq::cookie::SameSite,
    (Strict, Strict),
    (Lax, Lax),
    (Empty, None),
);
