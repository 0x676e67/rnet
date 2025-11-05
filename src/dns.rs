//! DNS resolution via the [hickory-resolver](https://github.com/hickory-dns/hickory-dns) crate

use std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, LazyLock},
};

use hickory_resolver::{
    TokioResolver, config::ResolverConfig, lookup_ip::LookupIpIntoIter,
    name_server::TokioConnectionProvider,
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use wreq::dns::{Addrs, Name, Resolve, Resolving};

define_enum!(
    /// The lookup ip strategy.
    const,
    LookupIpStrategy,
    hickory_resolver::config::LookupIpStrategy,
    Ipv4Only,
    Ipv6Only,
    Ipv4AndIpv6,
    Ipv6thenIpv4,
    Ipv4thenIpv6,
);

/// DNS resolver options for customizing DNS resolution behavior.
#[derive(Clone)]
#[pyclass]
pub struct ResolverOptions {
    pub(crate) lookup_ip_strategy: LookupIpStrategy,
    pub(crate) resolve_to_addrs: Vec<(Arc<PyBackedStr>, Vec<SocketAddr>)>,
}

#[pymethods]
impl ResolverOptions {
    /// Create a new [`ResolverOptions`] with the given lookup ip strategy.
    #[new]
    #[pyo3(signature=(lookup_ip_strategy=LookupIpStrategy::Ipv4AndIpv6))]
    pub fn new(lookup_ip_strategy: LookupIpStrategy) -> Self {
        ResolverOptions {
            lookup_ip_strategy,
            resolve_to_addrs: Vec::new(),
        }
    }

    /// Add a custom DNS resolve mapping.
    #[pyo3(signature=(domain, addrs))]
    pub fn add_resolve(&mut self, domain: PyBackedStr, addrs: Vec<IpAddr>) {
        self.resolve_to_addrs.push((
            Arc::new(domain),
            addrs.into_iter().map(|ip| SocketAddr::new(ip, 0)).collect(),
        ));
    }
}

/// Wrapper around an [`TokioResolver`], which implements the `Resolve` trait.
#[derive(Debug, Clone)]
pub struct HickoryDnsResolver {
    /// Shared, lazily-initialized Tokio-based DNS resolver.
    ///
    /// Backed by [`LazyLock`] to guarantee thread-safe, one-time creation.
    /// On initialization, it attempts to load the system's DNS configuration;
    /// if unavailable, it falls back to sensible default settings.
    resolver: &'static LazyLock<TokioResolver>,
}

impl HickoryDnsResolver {
    /// Create a new resolver with the default configuration,
    /// which reads from `/etc/resolve.conf`. The options are
    /// overriden to look up for both IPv4 and IPv6 addresses
    /// to work with "happy eyeballs" algorithm.
    pub fn new(ip_strategy: LookupIpStrategy) -> HickoryDnsResolver {
        static RESOLVER: LazyLock<TokioResolver> = LazyLock::new(move || {
            let mut builder = match TokioResolver::builder_tokio() {
                Ok(resolver) => resolver,
                Err(err) => {
                    eprintln!("error reading DNS system conf: {}, using defaults", err);
                    TokioResolver::builder_with_config(
                        ResolverConfig::default(),
                        TokioConnectionProvider::default(),
                    )
                }
            };
            builder.options_mut().ip_strategy = ip_strategy.into_ffi();
            builder.build()
        });

        HickoryDnsResolver {
            resolver: &RESOLVER,
        }
    }
}

struct SocketAddrs {
    iter: LookupIpIntoIter,
}

impl Resolve for HickoryDnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let resolver = self.clone();
        Box::pin(async move {
            let lookup = resolver.resolver.lookup_ip(name.as_str()).await?;
            let addrs: Addrs = Box::new(SocketAddrs {
                iter: lookup.into_iter(),
            });
            Ok(addrs)
        })
    }
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ip_addr| SocketAddr::new(ip_addr, 0))
    }
}
