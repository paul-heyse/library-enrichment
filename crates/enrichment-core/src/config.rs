//! Service configuration, read from `LIBENR_CONFIG`.
//!
//! Policy is enforced in the core, not in MCP tool annotations and not in the companion skill
//! (blueprint §10). This module is where "what the operator enabled" actually comes from, so
//! that `service_status` reports configuration rather than a compiled-in guess.
//!
//! Generated native declarations below own the current configuration contract.
//! `config/service.example.toml` is frozen historical provenance, not a runtime configuration. Missing keys fall back to those documented defaults; an unparsable file is an
//! error rather than a silent fallback, because starting with different limits than the
//! operator wrote is worse than not starting. Keys the frozen example does not carry --
//! `[network]` and the registry base URLs -- were added for Phase 1 and are documented in
//! `docs/operations/README.md`.

use std::path::{Path, PathBuf};

/// TOML is an operator format with exact integer tokens. Project those tokens through the
/// declared native widths before using the strict decimal-string research wire codec.
pub fn parse<T: crate::native_union::Cell>(text: &str) -> Result<T, String> {
    fn project(value: &mut serde_json::Value, kind: &arrow::datatypes::DataType) {
        use arrow::datatypes::DataType;
        match kind {
            DataType::UInt64 => {
                if let Some(number) = value.as_u64() {
                    *value = serde_json::Value::String(number.to_string());
                }
            }
            DataType::Int64 => {
                if let Some(number) = value.as_i64() {
                    *value = serde_json::Value::String(number.to_string());
                }
            }
            DataType::Struct(fields) => {
                if let Some(object) = value.as_object_mut() {
                    for field in fields {
                        if let Some(member) = object.get_mut(field.name()) {
                            project(member, field.data_type());
                        }
                    }
                }
            }
            DataType::List(item) => {
                if let Some(values) = value.as_array_mut() {
                    for value in values {
                        project(value, item.data_type());
                    }
                }
            }
            _ => {}
        }
    }
    let parsed: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
    let mut value = serde_json::to_value(parsed).map_err(|e| e.to_string())?;
    project(&mut value, &T::data_type());
    serde_json::from_value(value).map_err(|e| e.to_string())
}

crate::native_struct! {
/// Everything the daemon reads out of its configuration file.
#[serde(default)]
pub struct Config {
    /// Daemon-wide Arrow query and storage admission budgets (ADR-0024).
    arrow: ArrowConfig => crate::native_union::Rule::Text,
    /// Isolated producer configuration; never enabled by a client request.
    execution: Execution => crate::native_union::Rule::Text,
    /// Result and transport budgets (§7.3).
    limits: Limits => crate::native_union::Rule::Text,
    /// Execution-profile policy (§10).
    policy: Policy => crate::native_union::Rule::Text,
    /// Registry and document freshness (§3.3).
    freshness: FreshnessConfig => crate::native_union::Rule::Text,
    /// Per-ecosystem producer settings.
    producers: Producers => crate::native_union::Rule::Text,
    /// Outbound network limits (§10).
    network: Network => crate::native_union::Rule::Text,
} ephemeral { source: Source = Source::BuiltInDefaults }
}

crate::native_struct! {
#[serde(default)]
pub struct ArrowConfig {
    native: NativeQueryConfig => crate::native_union::Rule::Text,
    memory_bytes: usize => crate::native_union::Rule::Text,
    spill_bytes: u64 => crate::native_union::Rule::Text,
    descriptor_cache_bytes: usize => crate::native_union::Rule::Text,
    metadata_cache_bytes: usize => crate::native_union::Rule::Text,
    batch_rows: usize => crate::native_union::Rule::Text,
    partitions: usize => crate::native_union::Rule::Text,
    concurrency: usize => crate::native_union::Rule::Text,
    result_rows: usize => crate::native_union::Rule::Text,
    result_bytes: usize => crate::native_union::Rule::Text,
    query_deadline_seconds: u64 => crate::native_union::Rule::Text,
    admission_deadline_seconds: u64 => crate::native_union::Rule::Text,
    record_bytes: usize => crate::native_union::Rule::Text,
    batch_bytes: usize => crate::native_union::Rule::Text,
    file_bytes: u64 => crate::native_union::Rule::Text,
    table_rows: usize => crate::native_union::Rule::Text,
}
}

impl Default for ArrowConfig {
    fn default() -> Self {
        Self {
            native: NativeQueryConfig::default(),
            // Performance-first defaults for the operator's 16-core / 192 GiB
            // workstation. These are shared ceilings, not eager allocations.
            memory_bytes: 32 * 1024 * 1024 * 1024,
            spill_bytes: 64 * 1024 * 1024 * 1024,
            descriptor_cache_bytes: 1024 * 1024 * 1024,
            metadata_cache_bytes: 2 * 1024 * 1024 * 1024,
            batch_rows: 1024,
            partitions: 16,
            concurrency: 16,
            result_rows: 10_000,
            result_bytes: 16 * 1024 * 1024,
            query_deadline_seconds: 30,
            admission_deadline_seconds: 30,
            record_bytes: 1024 * 1024,
            batch_bytes: 16 * 1024 * 1024,
            file_bytes: 256 * 1024 * 1024,
            table_rows: 1_000_000,
        }
    }
}

crate::native_struct! {
/// Effective native choices; measurements select defaults, configuration captures each run.
#[serde(default)]
pub struct NativeQueryConfig {
    /// Stack capacity for each native async and blocking worker; separate from Arrow memory.
    worker_stack_bytes: usize => crate::native_union::Rule::Text,
    blocking_threads: usize => crate::native_union::Rule::Text,
    decoder_filter: bool => crate::native_union::Rule::Text,
    observation_bloom: bool => crate::native_union::Rule::Text,
    reorder_filters: bool => crate::native_union::Rule::Text,
    row_group_rows: usize => crate::native_union::Rule::Text,
    row_group_bytes: usize => crate::native_union::Rule::Text,
    target_file_bytes: u64 => crate::native_union::Rule::Text,
    claim_lease_seconds: u64 => crate::native_union::Rule::Text,
    normalization_depth: u32 => crate::native_union::Rule::Text,
    retention: crate::operation::retention::RetentionPolicy => crate::native_union::Rule::Text,
}
}

impl Default for NativeQueryConfig {
    fn default() -> Self {
        Self {
            worker_stack_bytes: 16 * 1024 * 1024,
            blocking_threads: 16,
            decoder_filter: true,
            observation_bloom: true,
            reorder_filters: true,
            row_group_rows: 1024,
            row_group_bytes: 8 * 1024 * 1024,
            target_file_bytes: 64 * 1024 * 1024,
            claim_lease_seconds: 600,
            normalization_depth: 64,
            retention: Default::default(),
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

crate::native_struct! {
/// Result and transport budgets.
///
/// Defaults match `config/service.example.toml` exactly. That file is frozen and digest-checked,
/// so the two cannot drift without `just provenance-check` noticing.
#[serde(default)]
pub struct Limits {
    /// Largest inline result body before an artifact handle is used instead.
    inline_result_bytes: usize => crate::native_union::Rule::Text,
    /// Maximum search results per page.
    search_results: usize => crate::native_union::Rule::Text,
    /// Maximum characters in one evidence excerpt.
    excerpt_characters: usize => crate::native_union::Rule::Text,
    /// Maximum lines in one source excerpt.
    source_lines: usize => crate::native_union::Rule::Text,
    /// Maximum child entries per namespace in an overview.
    namespace_entries: usize => crate::native_union::Rule::Text,
    /// Largest verification snippet accepted.
    verification_input_bytes: usize => crate::native_union::Rule::Text,
    /// Seconds to wait inline before returning a job receipt.
    inline_wait_seconds: u64 => crate::native_union::Rule::Text,
    /// Maximum bounded wait in `job_control`.
    max_job_wait_seconds: u64 => crate::native_union::Rule::Text,
    /// Concurrent expensive build/probe workers.
    expensive_worker_concurrency: usize => crate::native_union::Rule::Text,
    /// Warm language-server sessions kept alive.
    warm_lsp_sessions: usize => crate::native_union::Rule::Text,
    /// Largest NDJSON-RPC frame the daemon will accept (§2.1).
    rpc_message_bytes: usize => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Execution-profile policy (§10).
#[serde(default)]
pub struct Policy {
    /// Profiles a caller may select from.
    ///
    /// A caller selects from what configuration has enabled; it never grants itself permission.
    /// `runtime` is never a default — it is an explicitly enabled local profile.
    enabled_profiles: Vec<String> => crate::native_union::Rule::Sequence,
    /// Whether `build` requires an operational sandbox.
    require_sandbox_for_build: bool => crate::native_union::Rule::Text,
    /// Whether `runtime` requires an operational sandbox.
    require_sandbox_for_runtime: bool => crate::native_union::Rule::Text,
    /// Whether executed code may reach the network.
    execution_network: bool => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Freshness policy (§3.3): mutable lookups get finite TTLs; immutable artifacts are reused by
/// digest.
#[serde(default)]
pub struct FreshnessConfig {
    /// How long a registry answer is trusted under `cache_ok`.
    registry_ttl_seconds: u64 => crate::native_union::Rule::Text,
    /// How long a mutable documentation page is trusted under `cache_ok`.
    mutable_docs_ttl_seconds: u64 => crate::native_union::Rule::Text,
    /// How long an "unavailable" answer is remembered. Shorter than the positive TTLs on
    /// purpose: a timeout is not a permanent absence (§8.4).
    negative_cache_ttl_seconds: u64 => crate::native_union::Rule::Text,
    /// Whether a "latest" question always revalidates against the registry.
    latest_requires_revalidation: bool => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Per-ecosystem producer settings.
#[serde(default)]
pub struct Producers {
    /// GitHub REST base for public immutable source acquisition.
    github_api_url: String => crate::native_union::Rule::Text,
    /// Rust producers.
    rust: RustProducers => crate::native_union::Rule::Text,
    /// Static Python distribution and worker settings.
    python: PythonProducers => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Python producers. The worker executable belongs to the service installation.
#[serde(default)]
pub struct PythonProducers {
    /// PyPI JSON API base, including /pypi.
    pypi_url: String => crate::native_union::Rule::Text,
    /// Simple API base for artifact corroboration.
    simple_url: String => crate::native_union::Rule::Text,
    /// Service-owned Python interpreter containing enrichment_worker and pinned Griffe.
    worker_python: PathBuf => crate::native_union::Rule::Text,
    /// Total worker deadline.
    worker_timeout_seconds: u64 => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Rust producer settings (`[producers.rust]`).
#[serde(default)]
pub struct RustProducers {
    /// Download hosted rustdoc JSON before any local compilation (§4.1).
    prefer_hosted_rustdoc_json: bool => crate::native_union::Rule::Text,
    /// Never explore all feature combinations by default (§4.3).
    all_features_by_default: bool => crate::native_union::Rule::Text,
    /// The Rust semantic engine.
    lsp: String => crate::native_union::Rule::Text,
    /// Base URL of the sparse registry index. Its host becomes a trusted endpoint.
    crates_io_index_url: String => crate::native_union::Rule::Text,
    /// Base URL of the registry web API. Its host becomes a trusted endpoint.
    crates_io_api_url: String => crate::native_union::Rule::Text,
    /// Base URL of the documentation host. Its host becomes a trusted endpoint.
    docs_rs_url: String => crate::native_union::Rule::Text,
    /// The identifying `User-Agent` sent with every request; crates.io's policy requires one.
    user_agent: String => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Outbound network limits (§10). Not in the frozen example; added for Phase 1.
#[serde(default)]
pub struct Network {
    /// Largest response body accepted before decompression.
    max_download_bytes: u64 => crate::native_union::Rule::Text,
    /// Largest decompressed payload accepted.
    max_decompressed_bytes: u64 => crate::native_union::Rule::Text,
    /// Redirect hops followed, each re-checked against policy.
    max_redirects: u32 => crate::native_union::Rule::Text,
    /// Total deadline for one HTTP request.
    request_timeout_seconds: u64 => crate::native_union::Rule::Text,
    /// Deadline for one durable acquisition. On expiry cancel producer work and settle cleanup;
    /// a publication already admitted commits before the terminal job result is reported.
    acquisition_timeout_seconds: u64 => crate::native_union::Rule::Text,
}
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
        let mut config: Self = parse(&text).map_err(|err| ConfigError::Parse {
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
    fn retained_defaults_match_frozen_provenance_without_loading_a_retired_config() {
        let frozen: toml::Value = toml::from_str(FROZEN_EXAMPLE).unwrap();
        let defaults = serde_json::to_value(Config::default()).unwrap();
        for section in ["limits", "policy", "freshness"] {
            for (name, actual) in defaults[section].as_object().unwrap() {
                assert_eq!(
                    actual,
                    &serde_json::to_value(&frozen[section][name]).unwrap(),
                    "{section}.{name}"
                );
            }
        }
        for name in [
            "prefer_hosted_rustdoc_json",
            "all_features_by_default",
            "lsp",
        ] {
            assert_eq!(
                defaults["producers"]["rust"][name],
                serde_json::to_value(&frozen["producers"]["rust"][name]).unwrap()
            );
        }
        assert!(
            !Config::default()
                .policy
                .enabled_profiles
                .contains(&"runtime".into())
        );
        assert!(
            parse::<Config>(FROZEN_EXAMPLE).is_err(),
            "retired options cannot be silently accepted"
        );
    }

    #[test]
    fn generated_config_records_preserve_values_and_exclude_only_load_origin() {
        use crate::native_union::NativeStruct;
        let mut config = Config::default();
        let id = crate::native_key::Key::OperationPolicy
            .record(&config)
            .unwrap();
        config.source = Source::File("/tmp/operator.toml".into());
        assert_eq!(
            id,
            crate::native_key::Key::OperationPolicy
                .record(&config)
                .unwrap()
        );
        let batch = Config::batch(&[config]).unwrap();
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert_eq!(Config::decode(rows.row(0)).unwrap(), Config::default());
        for source in ["unknown = true", "[network]\nunknown = true"] {
            assert!(parse::<Config>(source).is_err());
        }
        parse::<Config>(include_str!("../../../config/service.dev.toml")).unwrap();
    }

    #[test]
    fn a_configured_value_overrides_the_default() {
        let config: Config = parse(
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
        let config: Config = parse(
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
        let config: Config = parse("[policy]\nenabled_profiles = []\n").expect("parses");
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
            parse(include_str!("../../../config/service.workstation.toml")).unwrap();
        let resource = config.execution.resources().unwrap();
        assert_eq!(resource.cpu_quota_micros, 3_200_000);
        assert_eq!(resource.memory_bytes, 32 * 1024 * 1024 * 1024);
        assert_eq!(resource.scratch_bytes, 16 * 1024 * 1024 * 1024);
        assert_eq!(resource.pids, 2048);
        assert_eq!(resource.swap_bytes, 0);
        for field in ["cpus", "memory_mib", "scratch_mib", "pids"] {
            let config: Execution = parse(&format!("{field} = 0")).unwrap();
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

crate::native_struct! {
/// Service-owned rootless execution backend. Immutable image IDs are operator admitted.
#[serde(default)]
pub struct Execution {
    storage_root: Option<PathBuf> => crate::native_union::Rule::Text,
    python_image: Option<String> => crate::native_union::Rule::Text,
    rust_image: Option<String> => crate::native_union::Rule::Text,
    deadline_seconds: u64 => crate::native_union::Rule::Text,
    output_bytes: usize => crate::native_union::Rule::Text,
    /// CPU bandwidth in logical CPUs, not affinity or a guaranteed scheduling allocation.
    cpus: u32 => crate::native_union::Rule::Text,
    memory_mib: u64 => crate::native_union::Rule::Text,
    /// Hard writable tmpfs ceiling, additionally limited by the effective memory bound.
    scratch_mib: u64 => crate::native_union::Rule::Text,
    /// First-party executor binary, mounted read-only and bound into qualification.
    executor_path: Option<PathBuf> => crate::native_union::Rule::Text,
    pids: u32 => crate::native_union::Rule::Text,
    queue_limit: usize => crate::native_union::Rule::Text,
    /// The container broker executable. Absent means the packaged `/usr/bin/podman`.
    ///
    /// Configurable because an operator may install Podman elsewhere, and because a cleanup
    /// failure is otherwise unreachable from a test: the F3 oracle points this at a wrapper
    /// that fails `rm` a bounded number of times.
    broker_path: Option<PathBuf> => crate::native_union::Rule::Text,
    /// How long the cleanup supervisor keeps retrying removal before the owned container is
    /// declared unresolved. Its worker permit is held for the whole window.
    cleanup_deadline_seconds: u64 => crate::native_union::Rule::Text,
    /// Aggregate ceiling on writable capsule storage, in MiB.
    capsule_budget_mib: u64 => crate::native_union::Rule::Text,
    /// Idle seconds before a warm session is evicted and its container removed.
    lsp_idle_seconds: u64 => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Requested cgroup and writable-workspace limits. Capacities, not eager allocations.
pub struct ExecutionResources {
    cpu_quota_micros: u64 => crate::native_union::Rule::Text,
    cpu_period_micros: u64 => crate::native_union::Rule::Text,
    memory_bytes: u64 => crate::native_union::Rule::Text,
    swap_bytes: u64 => crate::native_union::Rule::Text,
    scratch_bytes: u64 => crate::native_union::Rule::Text,
    pids: u32 => crate::native_union::Rule::Text,
}
}
