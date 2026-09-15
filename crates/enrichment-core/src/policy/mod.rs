//! Execution policy (blueprint §10), enforced here in the core.
//!
//! Isolation exists to keep working environments intact and research reproducible. The rules
//! in this module are the ones with a cheap oracle at the point of use: which profile a request
//! may run under, which URLs the fetcher may open and how much it may pull, and what an archive
//! may contain before a byte of it lands outside scratch. A caller selects from what
//! configuration enabled; it never grants itself permission.

use std::collections::BTreeSet;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::{Host, Url};

use crate::config::Config;

/// The three execution profiles (§10).
///
/// The values are, in order: `static`, `build`, `runtime`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ExecutionProfile {
    Static,
    Build,
    Runtime,
}

impl ExecutionProfile {
    /// The configuration spelling of this profile.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Build => "build",
            Self::Runtime => "runtime",
        }
    }

    /// Whether configuration has enabled this profile. Detection of a sandbox never enables one.
    #[must_use]
    pub fn is_enabled(self, config: &Config) -> bool {
        config
            .policy
            .enabled_profiles
            .iter()
            .any(|p| p == self.as_str())
    }
}

impl FromStr for ExecutionProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "static" => Ok(Self::Static),
            "build" => Ok(Self::Build),
            "runtime" => Ok(Self::Runtime),
            other => Err(format!("unknown execution profile `{other}`")),
        }
    }
}

impl fmt::Display for ExecutionProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What the fetcher may open and how much it may pull (§10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchPolicy {
    /// Largest response body accepted, in bytes, before decompression.
    pub max_download_bytes: u64,
    /// Largest decompressed payload accepted, in bytes.
    pub max_decompressed_bytes: u64,
    /// Redirect hops followed; each destination is re-checked against this policy.
    pub max_redirects: u32,
    /// Total deadline for one request.
    pub request_timeout_seconds: u64,
    /// Hosts (with port when non-default) that may be reached even when loopback or plain
    /// `http`. Populated from the configured base URLs, never from a request.
    pub trusted_hosts: BTreeSet<String>,
    /// The identifying `User-Agent` upstream policies require.
    pub user_agent: String,
}

/// Why a URL was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PolicyViolation {
    /// Only `https` is allowed to an untrusted host.
    #[error("refusing `{url}`: scheme `{scheme}` is only allowed for configured endpoints")]
    SchemeNotAllowed {
        /// The URL.
        url: String,
        /// Its scheme.
        scheme: String,
    },
    /// The URL has no host at all.
    #[error("refusing `{url}`: no host")]
    HostMissing {
        /// The URL.
        url: String,
    },
    /// The host is a private, loopback, link-local or otherwise non-public address.
    #[error(
        "refusing `{url}`: `{host}` is a private, loopback or link-local address and is not a \
         configured endpoint (blueprint §10)"
    )]
    PrivateAddress {
        /// The URL.
        url: String,
        /// The host as written.
        host: String,
    },
}

impl FetchPolicy {
    /// Derive the fetch policy from configuration.
    ///
    /// The trusted set is exactly the hosts of the configured registry and documentation base
    /// URLs. That is what lets a test point the daemon at a loopback fixture upstream without
    /// weakening the rule for anything else.
    #[must_use]
    pub fn from_config(config: &Config) -> Self {
        let mut trusted_hosts = BTreeSet::new();
        for base in [
            &config.producers.github_api_url,
            &config.producers.rust.crates_io_index_url,
            &config.producers.rust.crates_io_api_url,
            &config.producers.rust.docs_rs_url,
            &config.producers.python.pypi_url,
            &config.producers.python.simple_url,
        ] {
            if let Some(authority) = Url::parse(base).ok().and_then(|u| host_key(&u)) {
                trusted_hosts.insert(authority);
            }
        }
        Self {
            max_download_bytes: config.network.max_download_bytes,
            max_decompressed_bytes: config.network.max_decompressed_bytes,
            max_redirects: config.network.max_redirects,
            request_timeout_seconds: config.network.request_timeout_seconds,
            trusted_hosts,
            user_agent: config.producers.rust.user_agent.clone(),
        }
    }

    /// Whether this host (with non-default port) is a configured endpoint.
    #[must_use]
    pub fn is_trusted(&self, url: &Url) -> bool {
        host_key(url).is_some_and(|key| self.trusted_hosts.contains(&key))
    }

    /// Check a URL before opening it, and again after every redirect.
    ///
    /// # Errors
    ///
    /// Refuses non-`https` schemes to untrusted hosts, hostless URLs, and private, loopback,
    /// link-local, unspecified, multicast or broadcast addresses that are not configured
    /// endpoints. Names are not resolved here, so a public name that resolves to a private
    /// address is not caught by this check; that limitation is recorded rather than hidden.
    pub fn check_url(&self, url: &Url) -> Result<(), PolicyViolation> {
        let trusted = self.is_trusted(url);
        if url.scheme() != "https" && !(url.scheme() == "http" && trusted) {
            return Err(PolicyViolation::SchemeNotAllowed {
                url: url.to_string(),
                scheme: url.scheme().to_owned(),
            });
        }
        let Some(host) = url.host() else {
            return Err(PolicyViolation::HostMissing {
                url: url.to_string(),
            });
        };
        if trusted {
            return Ok(());
        }
        let non_public = match host {
            Host::Ipv4(ip) => !is_public_v4(ip),
            Host::Ipv6(ip) => !is_public_v6(ip),
            Host::Domain(name) => {
                let lower = name.to_ascii_lowercase();
                lower == "localhost" || lower.ends_with(".localhost")
            }
        };
        if non_public {
            return Err(PolicyViolation::PrivateAddress {
                url: url.to_string(),
                host: host.to_string(),
            });
        }
        Ok(())
    }
}

fn host_key(url: &Url) -> Option<String> {
    let host = url.host_str()?.to_ascii_lowercase();
    Some(match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host,
    })
}

fn is_public_v4(ip: Ipv4Addr) -> bool {
    !(ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_unspecified()
        || ip.is_broadcast()
        || ip.is_multicast()
        || ip.is_documentation()
        // 100.64.0.0/10, carrier-grade NAT: reachable only inside a provider network.
        || (ip.octets()[0] == 100 && (64..=127).contains(&ip.octets()[1])))
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return is_public_v4(mapped);
    }
    !(ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || ip.is_unique_local()
        || ip.is_unicast_link_local())
}

/// Classify a bare address the same way [`FetchPolicy::check_url`] does. Exposed for tests and
/// for the fetcher's post-connect check.
#[must_use]
pub fn is_public_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_public_v4(v4),
        IpAddr::V6(v6) => is_public_v6(v6),
    }
}

/// What an archive may contain before extraction writes anything (§10, gate C11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivePolicy {
    /// Maximum number of entries.
    pub max_entries: usize,
    /// Maximum total decompressed bytes across all entries.
    pub max_total_bytes: u64,
    /// Maximum decompressed bytes for one entry.
    pub max_entry_bytes: u64,
}

impl Default for ArchivePolicy {
    fn default() -> Self {
        Self {
            max_entries: 20_000,
            max_total_bytes: 512 * 1024 * 1024,
            max_entry_bytes: 64 * 1024 * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy_with(trusted: &[&str]) -> FetchPolicy {
        FetchPolicy {
            max_download_bytes: 1,
            max_decompressed_bytes: 1,
            max_redirects: 1,
            request_timeout_seconds: 1,
            trusted_hosts: trusted.iter().map(|s| (*s).to_owned()).collect(),
            user_agent: "test".to_owned(),
        }
    }

    fn check(policy: &FetchPolicy, url: &str) -> Result<(), PolicyViolation> {
        policy.check_url(&Url::parse(url).expect("test url parses"))
    }

    #[test]
    fn public_https_hosts_are_allowed() {
        let policy = policy_with(&[]);
        assert_eq!(
            check(&policy, "https://docs.rs/crate/serde/1.0.0/json"),
            Ok(())
        );
        assert_eq!(
            check(&policy, "https://static.crates.io/crates/x/x-1.0.0.crate"),
            Ok(())
        );
    }

    #[test]
    fn plain_http_is_refused_unless_the_host_is_configured() {
        let policy = policy_with(&[]);
        assert!(matches!(
            check(&policy, "http://docs.rs/x"),
            Err(PolicyViolation::SchemeNotAllowed { .. })
        ));
        let trusting = policy_with(&["127.0.0.1:8123"]);
        assert_eq!(
            check(&trusting, "http://127.0.0.1:8123/index/se/rd/serde"),
            Ok(())
        );
    }

    #[test]
    fn loopback_private_and_link_local_targets_are_refused() {
        let policy = policy_with(&[]);
        for url in [
            "https://127.0.0.1/x",
            "https://10.0.0.1/x",
            "https://172.16.5.5/x",
            "https://192.168.1.1/x",
            "https://169.254.169.254/latest/meta-data",
            "https://100.100.1.1/x",
            "https://0.0.0.0/x",
            "https://[::1]/x",
            "https://[fe80::1]/x",
            "https://[fd00::1]/x",
            "https://[::ffff:127.0.0.1]/x",
            "https://localhost/x",
            "https://api.localhost/x",
        ] {
            assert!(
                matches!(
                    check(&policy, url),
                    Err(PolicyViolation::PrivateAddress { .. })
                ),
                "{url} should be refused"
            );
        }
    }

    #[test]
    fn a_configured_loopback_endpoint_is_allowed_only_on_its_own_port() {
        let policy = policy_with(&["127.0.0.1:8123"]);
        assert_eq!(check(&policy, "http://127.0.0.1:8123/x"), Ok(()));
        assert!(check(&policy, "http://127.0.0.1:8124/x").is_err());
        assert!(check(&policy, "https://127.0.0.1/x").is_err());
    }

    #[test]
    fn the_policy_derives_its_trusted_hosts_from_configuration() {
        let mut config = Config::default();
        config.producers.rust.docs_rs_url = "http://127.0.0.1:9000".to_owned();
        let policy = FetchPolicy::from_config(&config);
        assert!(policy.trusted_hosts.contains("127.0.0.1:9000"));
        assert!(policy.trusted_hosts.contains("index.crates.io"));
        assert!(policy.trusted_hosts.contains("crates.io"));
        assert_eq!(
            check(&policy, "http://127.0.0.1:9000/crate/x/1/json"),
            Ok(())
        );
    }

    #[test]
    fn profiles_are_enabled_by_configuration_alone() {
        let mut config = Config::default();
        config.policy.enabled_profiles = vec!["static".to_owned()];
        assert!(ExecutionProfile::Static.is_enabled(&config));
        assert!(!ExecutionProfile::Build.is_enabled(&config));
        assert!(!ExecutionProfile::Runtime.is_enabled(&config));
        assert_eq!(
            "build".parse::<ExecutionProfile>(),
            Ok(ExecutionProfile::Build)
        );
        assert!("shell".parse::<ExecutionProfile>().is_err());
    }
}
