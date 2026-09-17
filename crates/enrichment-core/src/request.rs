//! Request shapes shared by the daemon's RPC methods and the MCP adapter's tools.
//!
//! The adapter validates and forwards; the core decides. Defaults live here so both sides
//! agree on what an omitted field means (blueprint §7.1: "The service must not interpret every
//! lookup as an upgrade request").

use schemars::JsonSchema;

use crate::identity::{Ecosystem, ResearchMode};

crate::native_vocabulary! {
/// The freshness options (§3.3).
///
/// The values are, in order: `cache_ok`, `revalidate`, `offline`. `cache_ok` reuses a recorded
/// exact resolution without age expiry; `revalidate` consults the registry; `offline` never opens
/// a socket and fails clearly when nothing is recorded.
#[derive(Hash,Default)]
#[schemars(inline)]
pub enum FreshnessMode {
    #[default]
    CacheOk = "cache_ok",
    Revalidate = "revalidate",
    Offline = "offline",
}
}

crate::native_struct! {
/// What `resolve_library` asks for.
pub struct ResolveRequest {
    /// Which ecosystem.
    ecosystem: Ecosystem => crate::native_union::Rule::Text,
    /// Package name as the caller spelled it.
    #[schemars(length(min = 1))]
    name: String => crate::native_union::Rule::NonEmpty,
    /// Exact version. Omitted only for an explicit upstream question.
    #[serde(default)]
    version: Option<String> => crate::native_union::Rule::Text,
    /// Canonical public GitHub repository URL for revision mode.
    #[serde(default)]
    repository: Option<String> => crate::native_union::Rule::Text,
    /// Full immutable commit SHA; exclusive with version.
    #[serde(default)]
    revision: Option<String> => crate::native_union::Rule::Text,
    /// Explicit package root within the repository; omitted means root.
    #[serde(default)]
    package_subdir: Option<String> => crate::native_union::Rule::Text,
    /// Research mode; defaults to `project` when a version is given and `upstream` otherwise.
    #[serde(default)]
    mode: Option<ResearchMode> => crate::native_union::Rule::Text,
    /// Features the caller's project enables, when known.
    #[serde(default)]
    features: Option<Vec<String>> => crate::native_union::Rule::Sequence,
    /// Whether the caller's project enables default features, when known.
    #[serde(default)]
    default_features: Option<bool> => crate::native_union::Rule::Text,
    /// The caller's target triple, when known.
    #[serde(default)]
    target: Option<String> => crate::native_union::Rule::Text,
    /// Exact analyzed Python interpreter version; never inferred from the worker.
    #[serde(default)]
    python_version: Option<String> => crate::native_union::Rule::Text,
    /// Explicit Python extras, distinct from Rust feature selection.
    #[serde(default)]
    extras: Option<Vec<String>> => crate::native_union::Rule::Sequence,
    /// Freshness policy for this call.
    #[serde(default)]
    freshness: FreshnessMode => crate::native_union::Rule::Text,
    /// Whether a prerelease may satisfy an unversioned request.
    #[serde(default)]
    allow_prerelease: bool => crate::native_union::Rule::Text,
    /// Whether a yanked release may be resolved.
    #[serde(default)]
    allow_yanked: bool => crate::native_union::Rule::Text,
    /// Whether the caller accepts a local rustdoc build when hosted JSON is unusable.
    ///
    /// Hosted docs.rs JSON comes first, always (§4.1). This opts into the §4.4 fallback, which
    /// compiles third-party code on a dated nightly inside a capsule and can take minutes. It is
    /// a request, not a grant: the `build` profile must also be enabled and its images qualified,
    /// or the answer is `POLICY_DENIED` with the setup action.
    #[serde(default)]
    allow_local_build: bool => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// What `library_overview` asks for.
#[derive(Default)]
pub struct OverviewRequest {
    /// Independently paged library-level feature/docs/note/example discovery; None uses bounded defaults.
    #[serde(default)]
    discovery: Option<Vec<crate::wire::research::DiscoverySelection>> => crate::native_union::Rule::Text,
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: String => crate::native_union::Rule::NonEmpty,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<String> => crate::native_union::Rule::Text,
    /// Narrow to one module subtree.
    #[serde(default)]
    area: Option<String> => crate::native_union::Rule::Text,
    /// Cap on child entries per namespace; the configured limit bounds it.
    #[serde(default)]
    #[schemars(range(min = 1))]
    max_items: Option<usize> => crate::native_union::Rule::Text,
    /// Advisory byte budget; the configured inline budget bounds it.
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// What `search_evidence` asks for.
#[derive(Default)]
pub struct SearchRequest {
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: String => crate::native_union::Rule::NonEmpty,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<String> => crate::native_union::Rule::Text,
    /// The query.
    #[schemars(length(min = 1))]
    query: String => crate::native_union::Rule::NonEmpty,
    /// Evidence families: `api`, `docs`, `examples`, `release_notes`, `features`, `source`.
    #[serde(default)]
    kinds: Option<Vec<String>> => crate::native_union::Rule::Text,
    /// Restrict symbol/fragment subjects to this namespace subtree.
    #[serde(default)]
    area: Option<String> => crate::native_union::Rule::Text,
    /// Continue a previous page.
    #[serde(default)]
    cursor: Option<String> => crate::native_union::Rule::Text,
    /// Page size; the configured limit bounds it.
    #[serde(default)]
    #[schemars(range(min = 1))]
    max_items: Option<usize> => crate::native_union::Rule::Text,
    /// Byte budget for the page; the configured inline budget bounds it.
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// How much `inspect_symbol` retrieves.
///
/// What `inspect_symbol` asks for.
#[derive(Default)]
pub struct InspectRequest {
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: String => crate::native_union::Rule::NonEmpty,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<String> => crate::native_union::Rule::Text,
    /// A qualified path, or a bare name when unambiguous.
    #[schemars(length(min = 1))]
    symbol_path: String => crate::native_union::Rule::NonEmpty,
    /// Select one definition at this path, using an inspection candidate or search result.
    #[serde(default)]
    definition_id: Option<String> => crate::native_union::Rule::Text,
    /// One bounded preset or explicit independently paged aspects.
    #[serde(default)]
    selection: crate::wire::ResearchSelection => crate::native_union::Rule::Text,
    /// Byte budget; the configured inline budget bounds it.
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
    /// Retained execution selection and explicit execution intent (ADR-0026).
    #[serde(default)]
    execution: Option<InspectionOptions> => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! {
/// Reading retained evidence has no execution effects or qualification prerequisite.
#[derive(Default)]
pub enum InspectionIntent {
    #[default]
    Retained = "retained",
    ExecuteOnMiss = "execute_on_miss",
    Rerun = "rerun",
}
}

crate::native_struct! {
/// A finite inspection request, never a shell command or arbitrary LSP method.
#[derive(Default)]
#[serde(default)]
pub struct InspectionOptions {
    intent: InspectionIntent => crate::native_union::Rule::Text,
    /// Required only for execution: build for semantics, runtime for runtime objects.
    profile: Option<crate::policy::ExecutionProfile> => crate::native_union::Rule::Text,
    /// Omitted uses the selected symbol's generated consumer.
    snippet: Option<String> => crate::native_union::Rule::Text,
    /// Advanced override; zero-based UTF-8 byte position at a character boundary.
    position: Option<crate::evidence::execution::Utf8Position> => crate::native_union::Rule::Text,
    /// Empty selects hover, definition, references and diagnostics for semantic inspection.
    methods: Vec<crate::evidence::execution::SemanticMethod> => crate::native_union::Rule::Sequence,
    /// Explicit Python selection. Import and introspection hooks may execute in the runtime capsule.
    runtime: Option<RuntimeSelection> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct RuntimeSelection {
    module: String => crate::native_union::Rule::Text,
    attributes: Vec<String> => crate::native_union::Rule::Sequence,
}
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

crate::native_struct! {
/// What `read_artifact` asks for.
#[derive(Default)]
pub struct ReadArtifactRequest {
    /// A service-issued artifact handle.
    #[schemars(length(min = 1))]
    artifact_id: String => crate::native_union::Rule::NonEmpty,
    /// One typed result projection or Markdown heading, instead of the complete artifact.
    #[serde(default)]
    section: Option<crate::wire::research::ArtifactSection> => crate::native_union::Rule::Text,
    /// Continue a previous read.
    #[serde(default)]
    cursor: Option<String> => crate::native_union::Rule::Text,
    /// Byte budget for this slice; the configured inline budget bounds it.
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Compare a pinned snapshot pair, or explicitly acquire a version pair first.
#[derive(Default)]
#[serde(default)]
pub struct CompareRequest {
    /// Continue alternatives within one changed key, independently of the changed-key page.
    alternative_cursor: Option<String> => crate::native_union::Rule::Text,
    before_context_id: Option<String> => crate::native_union::Rule::Text,
    after_context_id: Option<String> => crate::native_union::Rule::Text,
    before_snapshot_id: Option<String> => crate::native_union::Rule::Text,
    after_snapshot_id: Option<String> => crate::native_union::Rule::Text,
    ecosystem: Option<Ecosystem> => crate::native_union::Rule::Text,
    name: Option<String> => crate::native_union::Rule::Text,
    from_version: Option<String> => crate::native_union::Rule::Text,
    to_version: Option<String> => crate::native_union::Rule::Text,
    scopes: Option<Vec<crate::compare::Scope>> => crate::native_union::Rule::Sequence,
    cursor: Option<String> => crate::native_union::Rule::Text,
    max_items: Option<usize> => crate::native_union::Rule::Text,
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// What the snapshot-manifest resource asks for.
#[derive(Default)]
pub struct ManifestRequest {
    /// The snapshot to describe.
    #[schemars(length(min = 1))]
    snapshot_id: String => crate::native_union::Rule::NonEmpty,
}
}

macro_rules! research_operations {
    ($($variant:ident($input:ty) = $name:literal => ($output:ty,$rpc:literal,$description:literal,$effect:ident,$published:literal)),* $(,)?) => {
        crate::native_payload! { @tag "method", "params";
            /// Every operation shares its native field, wire and MCP binding declaration.
            #[derive(JsonSchema)]
            pub enum ResearchRequest { $($variant($input) = $name),* }
        }
        /// Generated transport catalog, emitted with the request schema.
        pub fn operation_bindings() -> Vec<serde_json::Value> {
            vec![$(crate::wire::bindings::binding::<$input,$output>(
                $name,$rpc,$description,crate::wire::bindings::Effect::$effect,$published
            )),*]
        }
    };
}
research_operations! {
    Resolve(ResolveRequest) = "resolve_library" => (crate::wire::data::ResolveData,"library.resolve","Establish exact identity and environment before research.",Acquire,true),
    Overview(OverviewRequest) = "library_overview" => (crate::wire::data::OverviewData,"library.overview","Inspect published library capabilities, coverage and environment.",Read,true),
    Search(SearchRequest) = "search_evidence" => (crate::wire::data::SearchData,"evidence.search","Find bounded evidence with native ranking and provenance.",Read,true),
    Inspect(InspectRequest) = "inspect_symbol" => (crate::wire::data::InspectData,"symbol.inspect","Inspect a symbol and requested aspects; execution requires explicit intent and native authorization.",Execute,true),
    Compare(CompareRequest) = "compare_releases" => (crate::wire::data::CompareData,"library.compare","Compare exact evidence scopes with explicit coverage and environment confounders.",Acquire,true),
    Verify(crate::execution::VerifyRequest) = "verify_usage" => (crate::execution::VerificationData,"usage.verify","Test a proposed usage in an authorized isolated environment.",Execute,true),
    ReadArtifact(ReadArtifactRequest) = "read_artifact" => (crate::wire::data::ArtifactSliceData,"artifact.read","Read bounded immutable content or an independently retained result section.",Read,true),
    Job(crate::execution::JobRequest) = "job_control" => (crate::execution::JobData,"job.control","Observe, wait for or cancel your interest in durable work.",Execute,true),
    ServiceStatus(StatusRequest) = "service_status" => (crate::wire::status::StatusData,"service.status","Inspect readiness and capabilities without indexing.",Read,true),
    SnapshotManifest(ManifestRequest) = "snapshot_manifest" => (crate::wire::data::ManifestData,"snapshot.manifest","Read an exact published snapshot manifest.",Read,false),
}

/// The request schema also carries the generated transport catalog, outside its validation keywords.
pub fn request_schema() -> serde_json::Value {
    let mut schema = schemars::generate::SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<ResearchRequest>()
        .to_value();
    schema["x-enrichment-operations"] = serde_json::Value::Array(operation_bindings());
    schema
}

crate::native_struct! {
#[derive(Default)]
#[serde(default)]
pub struct StatusRequest {
    component: Option<String> => crate::native_union::Rule::Text,
}
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
