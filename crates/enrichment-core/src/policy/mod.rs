//! Shared execution and network configuration facts (blueprint §10).
//!
//! Isolation exists to keep working environments intact and research reproducible. The rules
//! Native store plans enforce URL/address/redirect admission from these facts. Byte and archive
//! mechanisms consume explicit limits. A caller cannot change the configured enabled profiles
//! or trusted endpoint inventory.

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use url::Url;

use crate::config::Config;

crate::native_vocabulary! {
/// The three execution profiles (§10).
///
/// The values are, in order: `static`, `build`, `runtime`.
#[derive(Hash)]
#[schemars(inline)]
pub enum ExecutionProfile {
    Static = "static",
    Build = "build",
    Runtime = "runtime",
}
}

impl ExecutionProfile {
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
    /// Every redirect hop must fit the captured native policy.
    #[error("refusing `{url}`: redirect limit {limit} exceeded")]
    RedirectLimit { url: String, limit: u32 },
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
}

fn host_key(url: &Url) -> Option<String> {
    let host = url.host_str()?.to_ascii_lowercase();
    Some(match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host,
    })
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

    #[test]
    fn the_policy_derives_its_trusted_hosts_from_configuration() {
        let mut config = Config::default();
        config.producers.rust.docs_rs_url = "http://127.0.0.1:9000".to_owned();
        let policy = FetchPolicy::from_config(&config);
        assert!(policy.trusted_hosts.contains("127.0.0.1:9000"));
        assert!(policy.trusted_hosts.contains("index.crates.io"));
        assert!(policy.trusted_hosts.contains("crates.io"));
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
