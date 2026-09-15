//! Service configuration, read from `LIBENR_CONFIG`.
//!
//! Policy is enforced in the core, not in MCP tool annotations and not in the companion skill
//! (blueprint §10). This module is where "what the operator enabled" actually comes from, so
//! that `service_status` reports configuration rather than a compiled-in guess.
//!
//! The contract is `config/service.example.toml`, which is frozen and documents every key with
//! its default. Missing keys fall back to those documented defaults; an unparsable file is an
//! error rather than a silent fallback, because starting with different limits than the
//! operator wrote is worse than not starting. Keys the frozen example does not carry --
//! `[network]` and the registry base URLs -- were added for Phase 1 and are documented in
//! `docs/operations/README.md`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Everything the daemon reads out of its configuration file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Daemon-wide Arrow query and storage admission budgets (ADR-0024).
    pub arrow: ArrowConfig,
    /// Isolated producer configuration; never enabled by a client request.
    pub execution: Execution,
    /// Result and transport budgets (§7.3).
    pub limits: Limits,
    /// Execution-profile policy (§10).
    pub policy: Policy,
    /// Registry and document freshness (§3.3).
    pub freshness: FreshnessConfig,
    /// Per-ecosystem producer settings.
    pub producers: Producers,
    /// Outbound network limits (§10).
    pub network: Network,
    /// Where this came from, so `service_status` can say.
    #[serde(skip)]
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ArrowConfig {
    pub memory_bytes: usize,
    pub spill_bytes: u64,
    pub metadata_cache_bytes: usize,
    pub batch_rows: usize,
    pub partitions: usize,
    pub concurrency: usize,
    pub result_rows: usize,
    pub result_bytes: usize,
    pub query_deadline_seconds: u64,
    pub admission_deadline_seconds: u64,
    pub cached_snapshots: usize,
    pub record_bytes: usize,
    pub batch_bytes: usize,
    pub file_bytes: u64,
    pub table_rows: usize,
    pub row_groups: usize,
}

impl Default for ArrowConfig {
    fn default() -> Self {
        Self {
            memory_bytes: 128 * 1024 * 1024,
            spill_bytes: 512 * 1024 * 1024,
            metadata_cache_bytes: 16 * 1024 * 1024,
            batch_rows: 1024,
            partitions: 2,
            concurrency: 4,
            result_rows: 10_000,
            result_bytes: 16 * 1024 * 1024,
            query_deadline_seconds: 30,
            admission_deadline_seconds: 30,
            cached_snapshots: 32,
            record_bytes: 1024 * 1024,
            batch_bytes: 16 * 1024 * 1024,
            file_bytes: 256 * 1024 * 1024,
            table_rows: 1_000_000,
            row_groups: 1024,
        }
    }
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct Limits {
    /// Largest inline result body before an artifact handle is used instead.
    pub inline_result_bytes: usize,
    /// Maximum search results per page.
    pub search_results: usize,
    /// Maximum characters in one evidence excerpt.
    pub excerpt_characters: usize,
    /// Maximum lines in one source excerpt.
    pub source_lines: usize,
    /// Maximum child entries per namespace in an overview.
    pub namespace_entries: usize,
    /// Largest verification snippet accepted.
    pub verification_input_bytes: usize,
    /// Seconds to wait inline before returning a job receipt.
    pub inline_wait_seconds: u64,
    /// Maximum bounded wait in `job_control`.
    pub max_job_wait_seconds: u64,
    /// Concurrent expensive build/probe workers.
    pub expensive_worker_concurrency: usize,
    /// Warm language-server sessions kept alive.
    pub warm_lsp_sessions: usize,
    /// Largest NDJSON-RPC frame the daemon will accept (§2.1).
    pub rpc_message_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            inline_result_bytes: 12288,
            search_results: 12,
            excerpt_characters: 800,
            source_lines: 80,
            namespace_entries: 20,
            verification_input_bytes: 32768,
            inline_wait_seconds: 2,
            max_job_wait_seconds: 10,
            expensive_worker_concurrency: 2,
            warm_lsp_sessions: 2,
            rpc_message_bytes: 1_048_576,
        }
    }
}

/// Execution-profile policy (§10).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
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

impl Policy {
    /// Refuse a configuration this build cannot honour, rather than reading past it.
    ///
    /// These three keys are in the frozen example configuration, so an operator can set them and
    /// reasonably expect them to mean something. Two of them only have one supported value here:
    /// this service always contains execution and always runs target code without a network, and
    /// a configuration asking otherwise is a request we cannot satisfy. Accepting the file and
    /// quietly doing the opposite is the failure mode worth preventing -- it would leave an
    /// operator believing they had loosened a boundary that in fact still held, or tightened one
    /// that in fact did not exist.
    ///
    /// # Errors
    ///
    /// Returns the unsupported setting and what this build does instead.
    pub fn validate(&self) -> Result<(), String> {
        if self.execution_network {
            return Err(
                "[policy].execution_network = true is not supported: target execution always \
                 runs with no network, and dependency acquisition is a separate, bounded step \
                 (blueprint §10). Remove the key or set it to false"
                    .to_owned(),
            );
        }
        if !self.require_sandbox_for_build || !self.require_sandbox_for_runtime {
            return Err(
                "[policy].require_sandbox_for_build and require_sandbox_for_runtime cannot be \
                 false: this build has no host-execution path to fall back to, and an \
                 unavailable sandbox is POLICY_DENIED rather than a looser profile"
                    .to_owned(),
            );
        }
        Ok(())
    }
}

/// Freshness policy (§3.3): mutable lookups get finite TTLs; immutable artifacts are reused by
/// digest.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct FreshnessConfig {
    /// How long a registry answer is trusted under `cache_ok`.
    pub registry_ttl_seconds: u64,
    /// How long a mutable documentation page is trusted under `cache_ok`.
    pub mutable_docs_ttl_seconds: u64,
    /// How long an "unavailable" answer is remembered. Shorter than the positive TTLs on
    /// purpose: a timeout is not a permanent absence (§8.4).
    pub negative_cache_ttl_seconds: u64,
    /// Whether a "latest" question always revalidates against the registry.
    pub latest_requires_revalidation: bool,
}

impl Default for FreshnessConfig {
    fn default() -> Self {
        Self {
            registry_ttl_seconds: 900,
            mutable_docs_ttl_seconds: 86400,
            negative_cache_ttl_seconds: 300,
            latest_requires_revalidation: true,
        }
    }
}

/// Per-ecosystem producer settings.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct Producers {
    /// GitHub REST base for public immutable source acquisition.
    pub github_api_url: String,
    /// Rust producers.
    pub rust: RustProducers,
    /// Static Python distribution and worker settings.
    pub python: PythonProducers,
}

impl Default for Producers {
    fn default() -> Self {
        Self {
            github_api_url: "https://api.github.com".into(),
            rust: RustProducers::default(),
            python: PythonProducers::default(),
        }
    }
}

/// Python producers. The worker executable belongs to the service installation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct PythonProducers {
    /// PyPI JSON API base, including /pypi.
    pub pypi_url: String,
    /// Simple API base for artifact corroboration.
    pub simple_url: String,
    /// Service-owned Python interpreter containing enrichment_worker and pinned Griffe.
    pub worker_python: PathBuf,
    /// Total worker deadline.
    pub worker_timeout_seconds: u64,
}
impl Default for PythonProducers {
    fn default() -> Self {
        Self {
            pypi_url: "https://pypi.org/pypi".into(),
            simple_url: "https://pypi.org/simple".into(),
            worker_python: PathBuf::from("python3"),
            worker_timeout_seconds: 30,
        }
    }
}

/// Rust producer settings (`[producers.rust]`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct RustProducers {
    /// Download hosted rustdoc JSON before any local compilation (§4.1).
    pub prefer_hosted_rustdoc_json: bool,
    /// Never explore all feature combinations by default (§4.3).
    pub all_features_by_default: bool,
    /// The Rust semantic engine.
    pub lsp: String,
    /// Base URL of the sparse registry index. Its host becomes a trusted endpoint.
    pub crates_io_index_url: String,
    /// Base URL of the registry web API. Its host becomes a trusted endpoint.
    pub crates_io_api_url: String,
    /// Base URL of the documentation host. Its host becomes a trusted endpoint.
    pub docs_rs_url: String,
    /// The identifying `User-Agent` sent with every request; crates.io's policy requires one.
    pub user_agent: String,
}

impl Default for RustProducers {
    fn default() -> Self {
        Self {
            prefer_hosted_rustdoc_json: true,
            all_features_by_default: false,
            lsp: "rust-analyzer".to_owned(),
            crates_io_index_url: "https://index.crates.io".to_owned(),
            crates_io_api_url: "https://crates.io/api/v1".to_owned(),
            docs_rs_url: "https://docs.rs".to_owned(),
            user_agent: format!(
                "library-enrichment/{} (+https://github.com/paul-heyse/library-enrichment)",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

/// Outbound network limits (§10). Not in the frozen example; added for Phase 1.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default)]
pub struct Network {
    /// Largest response body accepted before decompression.
    pub max_download_bytes: u64,
    /// Largest decompressed payload accepted.
    pub max_decompressed_bytes: u64,
    /// Redirect hops followed, each re-checked against policy.
    pub max_redirects: u32,
    /// Total deadline for one HTTP request.
    pub request_timeout_seconds: u64,
    /// Deadline for one durable acquisition. On expiry cancel producer work and settle cleanup;
    /// a publication already admitted commits before the terminal job result is reported.
    pub acquisition_timeout_seconds: u64,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            max_download_bytes: 64 * 1024 * 1024,
            max_decompressed_bytes: 512 * 1024 * 1024,
            max_redirects: 5,
            request_timeout_seconds: 30,
            acquisition_timeout_seconds: 120,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            arrow: ArrowConfig::default(),
            execution: Execution::default(),
            limits: Limits::default(),
            policy: Policy::default(),
            freshness: FreshnessConfig::default(),
            producers: Producers::default(),
            network: Network::default(),
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
        config
            .execution
            .resources()
            .map_err(|err| ConfigError::Parse {
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
        assert_eq!(example.freshness, defaults.freshness);
        // The example predates the base-URL keys, so only the keys it carries are compared.
        assert_eq!(
            example.producers.rust.prefer_hosted_rustdoc_json,
            defaults.producers.rust.prefer_hosted_rustdoc_json
        );
        assert_eq!(
            example.producers.rust.all_features_by_default,
            defaults.producers.rust.all_features_by_default
        );
        assert_eq!(example.producers.rust.lsp, defaults.producers.rust.lsp);
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
        assert_eq!(config.limits.excerpt_characters, 800);
    }

    #[test]
    fn the_registry_base_urls_are_configurable() {
        // What lets a test point the daemon at a loopback fixture upstream, and nothing else.
        let config: Config = toml::from_str(
            "[producers.rust]\ndocs_rs_url = \"http://127.0.0.1:9/\"\n\n[network]\nmax_redirects = 0\n",
        )
        .expect("parses");
        assert_eq!(config.producers.rust.docs_rs_url, "http://127.0.0.1:9/");
        assert_eq!(
            config.producers.rust.crates_io_index_url,
            "https://index.crates.io"
        );
        assert_eq!(config.network.max_redirects, 0);
        assert_eq!(config.network.request_timeout_seconds, 30);
    }

    #[test]
    fn the_user_agent_identifies_the_service() {
        let ua = RustProducers::default().user_agent;
        assert!(ua.starts_with("library-enrichment/"));
        assert!(ua.contains("github.com/paul-heyse/library-enrichment"));
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

    #[test]
    fn workstation_capacity_is_preserved_and_invalid_requests_are_rejected() {
        let config: Config =
            toml::from_str(include_str!("../../../config/service.workstation.toml")).unwrap();
        let resource = config.execution.resources().unwrap();
        assert_eq!(resource.cpu_quota_micros, 3_200_000);
        assert_eq!(resource.memory_bytes, 32 * 1024 * 1024 * 1024);
        assert_eq!(resource.scratch_bytes, 16 * 1024 * 1024 * 1024);
        assert_eq!(resource.pids, 2048);
        assert_eq!(resource.swap_bytes, 0);
        for field in ["cpus", "memory_mib", "scratch_mib", "pids"] {
            let config: Execution = toml::from_str(&format!("{field} = 0")).unwrap();
            assert!(config.resources().is_err(), "{field}");
        }
        let small = Execution {
            memory_mib: 256,
            ..Execution::default()
        };
        assert_eq!(small.resources().unwrap().scratch_bytes, 256 * 1024 * 1024);
        let maximum = Execution {
            scratch_mib: 65_536,
            memory_mib: 65_536,
            ..Execution::default()
        };
        assert_eq!(
            maximum.resources().unwrap().scratch_bytes,
            crate::capsule_protocol::DATA_LIMIT
        );
    }
}

/// Service-owned rootless execution backend. Immutable image IDs are operator admitted.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Execution {
    pub storage_root: Option<PathBuf>,
    pub python_image: Option<String>,
    pub rust_image: Option<String>,
    pub deadline_seconds: u64,
    pub output_bytes: usize,
    /// CPU bandwidth in logical CPUs, not affinity or a guaranteed scheduling allocation.
    pub cpus: u32,
    pub memory_mib: u64,
    /// Hard writable tmpfs ceiling, additionally limited by the effective memory bound.
    pub scratch_mib: u64,
    /// First-party executor binary, mounted read-only and bound into qualification.
    pub executor_path: Option<PathBuf>,
    pub pids: u32,
    pub queue_limit: usize,
    /// The container broker executable. Absent means the packaged `/usr/bin/podman`.
    ///
    /// Configurable because an operator may install Podman elsewhere, and because a cleanup
    /// failure is otherwise unreachable from a test: the F3 oracle points this at a wrapper
    /// that fails `rm` a bounded number of times.
    pub broker_path: Option<PathBuf>,
    /// How long the cleanup supervisor keeps retrying removal before the owned container is
    /// declared unresolved. Its worker permit is held for the whole window.
    pub cleanup_deadline_seconds: u64,
    /// Aggregate ceiling on writable capsule storage, in MiB.
    pub capsule_budget_mib: u64,
    /// Idle seconds before a warm session is evicted and its container removed.
    pub lsp_idle_seconds: u64,
}
impl Default for Execution {
    fn default() -> Self {
        Self {
            storage_root: None,
            python_image: None,
            rust_image: None,
            deadline_seconds: 60,
            output_bytes: 65536,
            cpus: 2,
            memory_mib: 1024,
            scratch_mib: 512,
            executor_path: None,
            pids: 128,
            queue_limit: 32,
            broker_path: None,
            cleanup_deadline_seconds: 120,
            capsule_budget_mib: 8192,
            lsp_idle_seconds: 300,
        }
    }
}

impl Execution {
    /// One validated resource contract for container flags, qualification and provenance.
    /// Reject unsupported requests instead of silently reducing the operator's capacity.
    pub fn resources(&self) -> std::io::Result<ExecutionResources> {
        if !(1..=4096).contains(&self.cpus)
            || !(128..=1_048_576).contains(&self.memory_mib)
            || !(16..=65_536).contains(&self.scratch_mib)
            || !(16..=1_048_576).contains(&self.pids)
        {
            return Err(std::io::Error::other(
                "execution resources require cpus 1..4096, memory_mib 128..1048576, \
                 scratch_mib 16..65536, and pids 16..1048576",
            ));
        }
        Ok(ExecutionResources {
            cpu_quota_micros: u64::from(self.cpus) * 100_000,
            cpu_period_micros: 100_000,
            memory_bytes: self.memory_mib * 1024 * 1024,
            // Equal Podman memory and memory-swap limits prohibit additional swap.
            swap_bytes: 0,
            scratch_bytes: self.scratch_bytes(),
            pids: self.pids,
        })
    }

    pub fn scratch_bytes(&self) -> u64 {
        // All entry points validate resources before reserving or creating workspaces.
        self.scratch_mib
            .min(self.memory_mib)
            .saturating_mul(1024 * 1024)
    }

    pub fn executor(&self) -> std::io::Result<PathBuf> {
        match &self.executor_path {
            Some(path) => path.canonicalize(),
            None => {
                let executable = std::env::current_exe()?;
                let mut parent = executable
                    .parent()
                    .ok_or_else(|| std::io::Error::other("executable has no parent"))?;
                if parent.file_name().is_some_and(|name| name == "deps") {
                    parent = parent.parent().ok_or_else(|| {
                        std::io::Error::other("test executable has no profile directory")
                    })?;
                }
                parent.join("library-enrichment-executor").canonicalize()
            }
        }
    }
    /// The broker executable actually invoked.
    #[must_use]
    pub fn broker(&self) -> PathBuf {
        self.broker_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("/usr/bin/podman"))
    }
}

/// Requested cgroup and writable-workspace limits. These are capacities, not eager allocations.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionResources {
    pub cpu_quota_micros: u64,
    pub cpu_period_micros: u64,
    pub memory_bytes: u64,
    pub swap_bytes: u64,
    pub scratch_bytes: u64,
    pub pids: u32,
}
