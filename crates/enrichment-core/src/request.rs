//! Request shapes shared by the daemon's RPC methods and the MCP adapter's tools.
//!
//! The adapter validates and forwards; the core decides. Defaults live here so both sides
//! agree on what an omitted field means (blueprint §7.1: "The service must not interpret every
//! lookup as an upgrade request").

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::identity::{Ecosystem, ResearchMode};

/// The freshness options (§3.3).
///
/// The values are, in order: `cache_ok`, `revalidate`, `offline`. `cache_ok` reuses a recorded
/// exact resolution without age expiry; `revalidate` consults the registry; `offline` never opens
/// a socket and fails clearly when nothing is recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum FreshnessMode {
    #[default]
    CacheOk,
    Revalidate,
    Offline,
}

/// What `resolve_library` asks for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ResolveRequest {
    /// Which ecosystem.
    pub ecosystem: Ecosystem,
    /// Package name as the caller spelled it.
    pub name: String,
    /// Exact version. Omitted only for an explicit upstream question.
    pub version: Option<String>,
    /// Canonical public GitHub repository URL for revision mode.
    pub repository: Option<String>,
    /// Full immutable commit SHA; exclusive with version.
    pub revision: Option<String>,
    /// Explicit package root within the repository; omitted means root.
    pub package_subdir: Option<String>,
    /// Research mode; defaults to `project` when a version is given and `upstream` otherwise.
    pub mode: Option<ResearchMode>,
    /// Features the caller's project enables, when known.
    pub features: Option<Vec<String>>,
    /// Whether the caller's project enables default features, when known.
    pub default_features: Option<bool>,
    /// The caller's target triple, when known.
    pub target: Option<String>,
    /// Exact analyzed Python interpreter version; never inferred from the worker.
    pub python_version: Option<String>,
    /// Explicit Python extras, distinct from Rust feature selection.
    pub extras: Option<Vec<String>>,
    /// Freshness policy for this call.
    pub freshness: FreshnessMode,
    /// Whether a prerelease may satisfy an unversioned request.
    pub allow_prerelease: bool,
    /// Whether a yanked release may be resolved.
    pub allow_yanked: bool,
    /// Whether the caller accepts a local rustdoc build when hosted JSON is unusable.
    ///
    /// Hosted docs.rs JSON comes first, always (§4.1). This opts into the §4.4 fallback, which
    /// compiles third-party code on a dated nightly inside a capsule and can take minutes. It is
    /// a request, not a grant: the `build` profile must also be enabled and its images qualified,
    /// or the answer is `POLICY_DENIED` with the setup action.
    pub allow_local_build: bool,
}

impl Default for ResolveRequest {
    fn default() -> Self {
        Self {
            ecosystem: Ecosystem::Rust,
            name: String::new(),
            version: None,
            repository: None,
            revision: None,
            package_subdir: None,
            mode: None,
            features: None,
            default_features: None,
            target: None,
            python_version: None,
            extras: None,
            freshness: FreshnessMode::CacheOk,
            allow_prerelease: false,
            allow_yanked: false,
            allow_local_build: false,
        }
    }
}

impl ResolveRequest {
    /// The mode in force: the caller's, or `project` with a version and `upstream` without.
    #[must_use]
    pub fn effective_mode(&self) -> ResearchMode {
        self.mode.unwrap_or(if self.version.is_some() {
            ResearchMode::Project
        } else {
            ResearchMode::Upstream
        })
    }

    /// Whether the caller declared anything about its environment.
    #[must_use]
    pub fn declares_environment(&self) -> bool {
        self.features.is_some() || self.default_features.is_some() || self.target.is_some()
    }

    /// Reject requests that cannot be acted on, with a message a caller can fix.
    ///
    /// # Errors
    ///
    /// Returns the problem in prose.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("`name` must not be empty".to_owned());
        }
        if self.name.chars().any(|c| {
            !(c.is_ascii_alphanumeric()
                || c == '-'
                || c == '_'
                || (self.ecosystem == Ecosystem::Python && c == '.'))
        }) {
            return Err(format!(
                "`{}` is not a package name: only ASCII letters, digits, `-` and `_` are allowed",
                self.name
            ));
        }
        if self.effective_mode() == ResearchMode::Revision {
            if self.version.is_some() {
                return Err("version and revision forms are exclusive".into());
            }
            crate::producer::revision::Revision::from_request(self)?;
        } else if self.repository.is_some()
            || self.revision.is_some()
            || self.package_subdir.is_some()
        {
            return Err("repository/revision/package_subdir require mode=revision".into());
        }
        if let Some(version) = &self.version {
            let valid = match self.ecosystem {
                Ecosystem::Rust => semver::Version::parse(version).is_ok(),
                Ecosystem::Python => version.parse::<pep440_rs::Version>().is_ok(),
            };
            if !valid {
                return Err(format!(
                    "`{version}` is not an exact version for {:?}",
                    self.ecosystem
                ));
            }
        }
        match self.ecosystem {
            Ecosystem::Rust if self.python_version.is_some() || self.extras.is_some() => {
                return Err("Python interpreter/extras do not describe a Rust environment".into());
            }
            Ecosystem::Python if self.features.is_some() || self.default_features.is_some() => {
                return Err(
                    "Use extras for Python; Rust features/default_features are not applicable"
                        .into(),
                );
            }
            _ => {}
        }
        if let Some(version) = &self.python_version {
            let parsed = version
                .parse::<pep440_rs::Version>()
                .map_err(|e| e.to_string())?;
            if parsed.release().len() < 2 || parsed.any_prerelease() {
                return Err("python_version must specify a stable major.minor[.patch]".into());
            }
        }
        Ok(())
    }
}

/// What `library_overview` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct OverviewRequest {
    /// Independently paged library-level feature/docs/note/example discovery; None uses bounded defaults.
    pub discovery: Option<Vec<crate::wire::research::DiscoverySelection>>,
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// Narrow to one module subtree.
    pub area: Option<String>,
    /// Cap on child entries per namespace; the configured limit bounds it.
    pub max_items: Option<usize>,
    /// Advisory byte budget; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// What `search_evidence` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct SearchRequest {
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// The query.
    pub query: String,
    /// Evidence families: `api`, `docs`, `examples`, `release_notes`, `features`, `source`.
    pub kinds: Option<Vec<String>>,
    /// Restrict symbol/fragment subjects to this namespace subtree.
    pub area: Option<String>,
    /// Continue a previous page.
    pub cursor: Option<String>,
    /// Page size; the configured limit bounds it.
    pub max_items: Option<usize>,
    /// Byte budget for the page; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// How much `inspect_symbol` retrieves.
///
/// What `inspect_symbol` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct InspectRequest {
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// A qualified path, or a bare name when unambiguous.
    pub symbol_path: String,
    /// Select one definition at this path, using an inspection candidate or search result.
    pub definition_id: Option<String>,
    /// One bounded preset or explicit independently paged aspects.
    pub selection: crate::wire::ResearchSelection,
    /// Byte budget; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
    /// Retained execution selection and explicit execution intent (ADR-0026).
    pub execution: Option<InspectionOptions>,
}

/// Reading retained evidence has no execution effects or qualification prerequisite.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InspectionIntent {
    #[default]
    Retained,
    ExecuteOnMiss,
    Rerun,
}

/// A finite inspection request, never a shell command or arbitrary LSP method.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct InspectionOptions {
    pub intent: InspectionIntent,
    /// Required only for execution: build for semantics, runtime for runtime objects.
    pub profile: Option<crate::policy::ExecutionProfile>,
    /// Omitted uses the selected symbol's generated consumer.
    pub snippet: Option<String>,
    /// Advanced override; zero-based UTF-8 byte position at a character boundary.
    pub position: Option<crate::evidence::execution::Utf8Position>,
    /// Empty selects hover, definition, references and diagnostics for semantic inspection.
    pub methods: Vec<crate::evidence::execution::SemanticMethod>,
    /// Explicit Python selection. Import and introspection hooks may execute in the runtime capsule.
    pub runtime: Option<RuntimeSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSelection {
    pub module: String,
    pub attributes: Vec<String>,
}

impl InspectionOptions {
    pub fn validate(&self, input_bound: usize) -> Result<(), String> {
        if self.methods.len() > 5
            || self
                .methods
                .iter()
                .enumerate()
                .any(|(i, m)| self.methods[..i].contains(m))
        {
            return Err("select at most five distinct semantic methods".into());
        }
        if let Some(text) = &self.snippet {
            if text.trim().is_empty() || text.len() > input_bound {
                return Err("consumer snippet is empty or exceeds its byte budget".into());
            }
            if let Some(position) = self.position {
                position.validate(text)?;
            }
        } else if self.position.is_some() {
            return Err("an explicit position requires its exact consumer snippet".into());
        }
        if let Some(runtime) = &self.runtime {
            let identifier = |s: &str| {
                let mut chars = s.chars();
                chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
                    && chars.all(|c| c == '_' || c.is_alphanumeric())
            };
            if runtime.module.len() > 512
                || runtime.attributes.len() > 32
                || !runtime.module.split('.').all(identifier)
                || runtime
                    .attributes
                    .iter()
                    .any(|s| s.len() > 512 || !identifier(s))
                || self.snippet.is_some()
                || !self.methods.is_empty()
            {
                return Err("runtime inspection needs a bounded import/attribute selection, without semantic query fields".into());
            }
        }
        if self.intent != InspectionIntent::Retained {
            let expected = if self.runtime.is_some() {
                crate::policy::ExecutionProfile::Runtime
            } else {
                crate::policy::ExecutionProfile::Build
            };
            if self.profile != Some(expected) {
                return Err(format!(
                    "this execution requires the explicit {expected:?} profile"
                ));
            }
        }
        Ok(())
    }
}

/// What `read_artifact` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ReadArtifactRequest {
    /// A service-issued artifact handle.
    pub artifact_id: String,
    /// One typed result projection or Markdown heading, instead of the complete artifact.
    pub section: Option<crate::wire::research::ArtifactSection>,
    /// Continue a previous read.
    pub cursor: Option<String>,
    /// Byte budget for this slice; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// Compare a pinned snapshot pair, or explicitly acquire a version pair first.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct CompareRequest {
    /// Continue alternatives within one changed key, independently of the changed-key page.
    pub alternative_cursor: Option<String>,
    pub before_context_id: Option<String>,
    pub after_context_id: Option<String>,
    pub before_snapshot_id: Option<String>,
    pub after_snapshot_id: Option<String>,
    pub ecosystem: Option<Ecosystem>,
    pub name: Option<String>,
    pub from_version: Option<String>,
    pub to_version: Option<String>,
    pub scopes: Option<Vec<crate::compare::Scope>>,
    pub cursor: Option<String>,
    pub max_items: Option<usize>,
    pub max_bytes: Option<usize>,
}

impl CompareRequest {
    /// Closed acquisition prerequisites for the version convenience form.
    pub fn resolutions(&self) -> Result<[ResolveRequest; 2], String> {
        if self.before_context_id.is_some()
            || self.after_context_id.is_some()
            || self.before_snapshot_id.is_some()
            || self.after_snapshot_id.is_some()
        {
            return Err("version comparison cannot contain context or snapshot IDs".into());
        }
        let ecosystem = self
            .ecosystem
            .ok_or("version comparison needs an ecosystem")?;
        let name = self
            .name
            .as_ref()
            .ok_or("version comparison needs a name")?;
        let from = self
            .from_version
            .as_ref()
            .ok_or("version comparison needs from_version")?;
        let to = self
            .to_version
            .as_ref()
            .ok_or("version comparison needs to_version")?;
        let requests = [from, to].map(|version| ResolveRequest {
            ecosystem,
            name: name.clone(),
            version: Some(version.clone()),
            ..Default::default()
        });
        for request in &requests {
            request.validate()?;
        }
        Ok(requests)
    }
}

/// What the snapshot-manifest resource asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ManifestRequest {
    /// The snapshot to describe.
    pub snapshot_id: String,
}

/// Authoritative request schemas for implemented research methods.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "method", content = "params")]
pub enum ResearchRequest {
    #[serde(rename = "service_status")]
    ServiceStatus(StatusRequest),
    #[serde(rename = "verify_usage")]
    Verify(crate::execution::VerifyRequest),
    #[serde(rename = "job_control")]
    Job(crate::execution::JobRequest),
    #[serde(rename = "compare_releases")]
    Compare(CompareRequest),
    #[serde(rename = "resolve_library")]
    Resolve(ResolveRequest),
    #[serde(rename = "library_overview")]
    Overview(OverviewRequest),
    #[serde(rename = "search_evidence")]
    Search(SearchRequest),
    #[serde(rename = "inspect_symbol")]
    Inspect(InspectRequest),
    #[serde(rename = "read_artifact")]
    ReadArtifact(ReadArtifactRequest),
    #[serde(rename = "snapshot_manifest")]
    SnapshotManifest(ManifestRequest),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct StatusRequest {
    pub component: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspection_options_bind_execution_intent_and_utf8_positions() {
        let defaults: InspectionOptions = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(defaults.intent, InspectionIntent::Retained);
        defaults.validate(32768).unwrap();
        let mut execute = defaults.clone();
        execute.intent = InspectionIntent::ExecuteOnMiss;
        assert!(execute.validate(32768).is_err());
        execute.profile = Some(crate::policy::ExecutionProfile::Build);
        execute.snippet = Some("# 😀\nvalue()".into());
        execute.position = Some(crate::evidence::execution::Utf8Position { line: 0, byte: 3 });
        assert!(execute.validate(32768).is_err());
        execute.position = Some(crate::evidence::execution::Utf8Position { line: 1, byte: 0 });
        execute.validate(32768).unwrap();
        assert!(
            serde_json::from_value::<InspectionOptions>(
                serde_json::json!({"methods":["arbitrary/method"]})
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<InspectionOptions>(
                serde_json::json!({"host_path":"/tmp/source.py"})
            )
            .is_err()
        );
        let runtime: InspectionOptions = serde_json::from_value(serde_json::json!({"intent":"rerun","profile":"runtime", "runtime":{"module":"fixture","attributes":["f"]}})).unwrap();
        runtime.validate(32768).unwrap();
    }

    #[test]
    fn the_mode_defaults_from_the_presence_of_a_version() {
        let mut request = ResolveRequest {
            name: "serde".into(),
            version: Some("1.0.0".into()),
            ..ResolveRequest::default()
        };
        assert_eq!(request.effective_mode(), ResearchMode::Project);
        request.version = None;
        assert_eq!(request.effective_mode(), ResearchMode::Upstream);
        request.mode = Some(ResearchMode::Revision);
        assert_eq!(request.effective_mode(), ResearchMode::Revision);
    }

    #[test]
    fn bad_requests_are_rejected_with_a_reason() {
        assert!(ResolveRequest::default().validate().is_err());
        let bad_name = ResolveRequest {
            name: "../etc".into(),
            ..ResolveRequest::default()
        };
        assert!(bad_name.validate().is_err());
        let bad_version = ResolveRequest {
            name: "serde".into(),
            version: Some("latest".into()),
            ..ResolveRequest::default()
        };
        assert!(bad_version.validate().is_err());
        let ok = ResolveRequest {
            name: "serde_json".into(),
            version: Some("1.0.0".into()),
            ..ResolveRequest::default()
        };
        assert_eq!(ok.validate(), Ok(()));
    }

    #[test]
    fn unknown_fields_are_rejected_and_omitted_ones_default() {
        let parsed: ResolveRequest =
            serde_json::from_str(r#"{"name":"serde"}"#).expect("minimal request parses");
        assert_eq!(parsed.freshness, FreshnessMode::CacheOk);
        assert_eq!(parsed.ecosystem, Ecosystem::Rust);
        assert!(serde_json::from_str::<ResolveRequest>(r#"{"name":"x","upgrade":true}"#).is_err());
        let inspect: InspectRequest =
            serde_json::from_str(r#"{"context_id":"ctx_x","symbol_path":"a::b"}"#).expect("parses");
        assert_eq!(inspect.selection, crate::wire::ResearchSelection::Default);
        assert!(
            serde_json::from_str::<SearchRequest>(r#"{"context_id":"c","query":"q","sort":1}"#)
                .is_err()
        );
    }
}
