//! Service configuration, read from `LIBENR_CONFIG`.
//!
//! Policy is enforced in the core, not in MCP tool annotations and not in the companion skill
//! (blueprint §10). This module is where "what the operator enabled" actually comes from, so
//! that `service_status` reports configuration rather than a compiled-in guess.
//!
//! The contract is `config/service.example.toml`, which is frozen and documents every key with
//! its default. Missing keys fall back to those documented defaults; an unparsable file is an
//! error rather than a silent fallback, because starting with different limits than the
//! operator wrote is worse than not starting.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Everything the daemon reads out of its configuration file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Result and transport budgets (§7.3).
    pub limits: Limits,
    /// Execution-profile policy (§10).
    pub policy: Policy,
    /// Where this came from, so `service_status` can say.
    #[serde(skip)]
    pub source: Source,
}

/// Where a [`Config`] came from.
///
/// Reported to callers verbatim: telling someone their configuration enabled a profile when the
/// value is a compiled-in default is a wrong answer about policy, not a cosmetic one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    /// No `LIBENR_CONFIG` was set, or the path did not exist.
    #[default]
    BuiltInDefaults,
    /// Read from this file.
    File(PathBuf),
}

impl Source {
    /// A phrase suitable for `service_status`.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::BuiltInDefaults => {
                "built-in defaults; LIBENR_CONFIG is unset or absent".to_owned()
            }
            Self::File(path) => format!("read from {}", path.display()),
        }
    }
}

/// Result and transport budgets.
///
/// Defaults match `config/service.example.toml` exactly. That file is frozen and digest-checked,
/// so the two cannot drift without `just provenance-check` noticing.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Limits {
    /// Largest inline result body before an artifact handle is used instead.
    pub inline_result_bytes: usize,
    /// Maximum search results per page.
    pub search_results: usize,
    /// Seconds to wait inline before returning a job receipt.
    pub inline_wait_seconds: u64,
    /// Maximum bounded wait in `job_control`.
    pub max_job_wait_seconds: u64,
    /// Largest NDJSON-RPC frame the daemon will accept (§2.1).
    pub rpc_message_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            inline_result_bytes: 12288,
            search_results: 12,
            inline_wait_seconds: 2,
            max_job_wait_seconds: 10,
            rpc_message_bytes: 1_048_576,
        }
    }
}

/// Execution-profile policy (§10).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Policy {
    /// Profiles a caller may select from.
    ///
    /// A caller selects from what configuration has enabled; it never grants itself permission.
    /// `runtime` is never a default — it is an explicitly enabled local profile.
    pub enabled_profiles: Vec<String>,
    /// Whether `build` requires an operational sandbox.
    pub require_sandbox_for_build: bool,
    /// Whether `runtime` requires an operational sandbox.
    pub require_sandbox_for_runtime: bool,
    /// Whether executed code may reach the network.
    pub execution_network: bool,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            // The frozen example enables `static` and `build`; `runtime` is deliberately absent.
            enabled_profiles: vec!["static".to_owned(), "build".to_owned()],
            require_sandbox_for_build: true,
            require_sandbox_for_runtime: true,
            execution_network: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            limits: Limits::default(),
            policy: Policy::default(),
            source: Source::BuiltInDefaults,
        }
    }
}

/// Why configuration could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The file exists but could not be read.
    #[error("cannot read {path}: {source}")]
    Read {
        /// The configuration path.
        path: PathBuf,
        /// The underlying I/O failure.
        source: std::io::Error,
    },
    /// The file is not valid TOML, or a key has the wrong shape.
    #[error("{path} is not valid service configuration: {message}")]
    Parse {
        /// The configuration path.
        path: PathBuf,
        /// What the parser objected to.
        message: String,
    },
}

impl Config {
    /// Load from `LIBENR_CONFIG`, or return documented defaults when it is unset.
    ///
    /// # Errors
    ///
    /// Fails if the file exists but cannot be read or parsed. An unset variable is not an
    /// error; a malformed file is, because silently running with different limits than the
    /// operator wrote is the failure mode this exists to prevent.
    pub fn from_env() -> Result<Self, ConfigError> {
        match std::env::var_os("LIBENR_CONFIG") {
            Some(value) if !value.is_empty() => {
                let path = PathBuf::from(value);
                if path.exists() {
                    Self::from_path(&path)
                } else {
                    // A configured-but-absent path is the ordinary state of a fresh checkout
                    // before `just setup`, so it falls back rather than refusing to start.
                    Ok(Self::default())
                }
            }
            _ => Ok(Self::default()),
        }
    }

    /// Load from an explicit path.
    ///
    /// # Errors
    ///
    /// Fails if the file cannot be read or is not valid service configuration.
    pub fn from_path(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let mut config: Self = toml::from_str(&text).map_err(|err| ConfigError::Parse {
            path: path.to_path_buf(),
            message: err.to_string(),
        })?;
        config.source = Source::File(path.to_path_buf());
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The frozen example documents the contract, so the defaults here must match it. If they
    /// drift, an operator reading the example gets different behaviour than they configured.
    const FROZEN_EXAMPLE: &str = include_str!("../../../config/service.example.toml");

    #[test]
    fn defaults_match_the_frozen_example() {
        let example: Config = toml::from_str(FROZEN_EXAMPLE).expect("the example parses");
        let defaults = Config::default();
        assert_eq!(example.limits, defaults.limits);
        assert_eq!(example.policy, defaults.policy);
    }

    #[test]
    fn the_frozen_example_does_not_enable_the_runtime_profile() {
        let example: Config = toml::from_str(FROZEN_EXAMPLE).expect("the example parses");
        assert!(
            !example
                .policy
                .enabled_profiles
                .contains(&"runtime".to_owned()),
            "runtime is an explicitly enabled local profile, never a default"
        );
    }

    #[test]
    fn a_configured_value_overrides_the_default() {
        let config: Config = toml::from_str(
            "[limits]\nrpc_message_bytes = 4096\n\n[policy]\nenabled_profiles = [\"static\"]\n",
        )
        .expect("parses");
        assert_eq!(config.limits.rpc_message_bytes, 4096);
        assert_eq!(config.policy.enabled_profiles, vec!["static".to_owned()]);
        // Unmentioned keys keep their documented defaults rather than becoming zero.
        assert_eq!(config.limits.inline_wait_seconds, 2);
    }

    #[test]
    fn an_empty_profile_list_is_honoured_rather_than_replaced() {
        // The regression this guards: reporting a built-in default as the operator's policy.
        // An operator who disables every profile must not be told `static` is enabled.
        let config: Config = toml::from_str("[policy]\nenabled_profiles = []\n").expect("parses");
        assert!(config.policy.enabled_profiles.is_empty());
    }

    #[test]
    fn malformed_configuration_is_an_error_not_a_silent_default() {
        let dir = std::env::temp_dir().join("libenr-config-test");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("broken.toml");
        std::fs::write(&path, "[limits]\nrpc_message_bytes = \"not a number\"\n").expect("write");

        let err = Config::from_path(&path).expect_err("must not fall back silently");
        assert!(matches!(err, ConfigError::Parse { .. }));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn the_source_is_reported_so_a_default_is_never_mistaken_for_configuration() {
        assert!(Config::default().source.describe().contains("built-in"));
        assert!(
            Source::File(PathBuf::from("/x/service.toml"))
                .describe()
                .contains("/x/service.toml")
        );
    }
}
