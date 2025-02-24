use crate::define_enum_with_conversion;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

define_enum_with_conversion!(
    const,
    /// The lookup ip strategy.
    LookupIpStrategy, rquest::dns::LookupIpStrategy,
    Ipv4Only,
    Ipv6Only,
    Ipv4AndIpv6,
    Ipv6thenIpv4,
    Ipv4thenIpv6,
);
