use crate::error::DNSResolverError;
use rquest::dns::{HickoryDnsResolver, LookupIpStrategy};
use std::sync::{Arc, OnceLock};

/// Initializes and returns a DNS resolver with the specified strategy.
///
/// This function initializes a global DNS resolver using the provided lookup IP strategy.
/// If the DNS resolver has already been initialized, it returns the existing instance.
///
/// # Arguments
///
/// * `strategy` - An optional `LookupIpStrategy` to use for the DNS resolver.
///
/// # Returns
///
/// A `Result` containing an `Arc` to the `HickoryDnsResolver` instance, or an error if initialization fails.
///
/// # Errors
///
/// This function returns an error if the DNS resolver fails to initialize.
///
/// # Examples
///
/// ```rust
/// use rnet::dns::get_or_try_init;
/// use rquest::dns::LookupIpStrategy;
///
/// let resolver = get_or_try_init(LookupIpStrategy::default()).unwrap();
/// ```
pub fn get_or_try_init() -> crate::Result<Arc<HickoryDnsResolver>> {
    static DNS_RESOLVER: OnceLock<Result<Arc<HickoryDnsResolver>, &'static str>> = OnceLock::new();

    DNS_RESOLVER
        .get_or_init(move || {
            HickoryDnsResolver::new(LookupIpStrategy::Ipv4AndIpv6)
                .map(Arc::new)
                .map_err(|err| {
                    #[cfg(feature = "logging")]
                    {
                        log::error!("failed to initialize the DNS resolver: {}", err);
                    }
                    #[cfg(not(feature = "logging"))]
                    {
                        eprintln!("failed to initialize the DNS resolver: {}", err);
                    }
                    "failed to initialize the DNS resolver"
                })
        })
        .as_ref()
        .map(Arc::clone)
        .map_err(DNSResolverError::new_err)
}
