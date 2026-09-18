//! Request shapes shared by the daemon's RPC methods and the MCP adapter's tools.
//!
//! The adapter validates and forwards; the core decides. Defaults live here so both sides
//! agree on what an omitted field means (blueprint §7.1: "The service must not interpret every
//! lookup as an upgrade request").

use schemars::JsonSchema;

use crate::identity::{Ecosystem, ResearchMode};

pub mod resources;

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
    features: Option<Vec<String>> => crate::native_union::Rule::Set,
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
    extras: Option<Vec<String>> => crate::native_union::Rule::Set,
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
}

crate::native_struct! {
/// What `library_overview` asks for.
pub struct OverviewRequest {
    /// Independently paged library-level feature/docs/note/example discovery; None uses bounded defaults.
    #[serde(default)]
    discovery: Option<Vec<crate::wire::research::DiscoverySelection>> => crate::native_union::Rule::SequenceBounds { min: 0, max: crate::wire::research::DiscoveryKind::VALUES.len() as u64 },
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    /// Narrow to one module subtree.
    #[serde(default)]
    area: Option<String> => crate::native_union::Rule::Text,
    /// Cap on child entries per namespace; the configured limit bounds it.
    #[serde(default)]
    max_items: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1, max: u64::MAX },
    /// Advisory byte budget; the configured inline budget bounds it.
    #[serde(default)]
    max_bytes: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1024, max: u64::MAX },
}
}

crate::native_struct! {
/// What `search_evidence` asks for.
pub struct SearchRequest {
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    /// The query.
    #[schemars(length(min = 1))]
    query: String => crate::native_union::Rule::NonEmpty,
    /// Evidence families: `api`, `docs`, `examples`, `release_notes`, `features`. Source requires inspection.
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
    max_items: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1, max: u64::MAX },
    /// Byte budget for the page; the configured inline budget bounds it.
    #[serde(default)]
    max_bytes: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1024, max: u64::MAX },
}
}

crate::native_struct! {
/// How much `inspect_symbol` retrieves.
///
/// What `inspect_symbol` asks for.
pub struct InspectRequest {
    /// The context from `resolve_library`.
    #[schemars(length(min = 1))]
    context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    /// A specific snapshot; the context's current one when omitted.
    #[serde(default)]
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
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
    max_bytes: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1024, max: u64::MAX },
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
    methods: Vec<crate::evidence::execution::SemanticMethod> => crate::native_union::Rule::SequenceBounds { min: 0, max: crate::evidence::execution::SemanticMethod::VALUES.len() as u64 },
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
    max_bytes: Option<usize> => crate::native_union::Rule::UnsignedRange { min: 1024, max: u64::MAX },
}
}

crate::native_struct! {
/// Compare a pinned snapshot pair, or explicitly acquire a version pair first.
#[derive(Default)]
#[serde(default)]
pub struct CompareRequest {
    /// Continue alternatives within one changed key, independently of the changed-key page.
    alternative_cursor: Option<String> => crate::native_union::Rule::Text,
    before_context_id: Option<crate::identity::ContextId> => crate::native_union::Rule::Text,
    after_context_id: Option<crate::identity::ContextId> => crate::native_union::Rule::Text,
    before_snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    after_snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
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

crate::native_struct! {
/// What the snapshot-manifest resource asks for.
pub struct ManifestRequest {
    /// The snapshot to describe.
    #[schemars(length(min = 1))]
    snapshot_id: crate::identity::SnapshotId => crate::native_union::Rule::Text,
}
}

macro_rules! durable_arguments {
    (@collect [$($variant:ident($input:ty) = $tag:literal,)*];) => {
        crate::native_union! { pub enum Arguments {
            $($variant = $tag { request: $input => crate::native_union::Rule::Text }),*
        } }
        crate::native_vocabulary! {
            #[derive(Hash, PartialOrd, Ord)]
            pub enum CommandKind { $($variant = $tag),* }
        }
        impl Arguments {
            pub fn command_kind(&self) -> CommandKind {
                match self { $(Self::$variant { .. } => CommandKind::$variant),* }
            }
        }
        impl From<Arguments> for ResearchRequest {
            fn from(arguments: Arguments) -> Self {
                match arguments { $(Arguments::$variant { request } => Self::$variant(request)),* }
            }
        }
    };
    (@collect [$($done:tt)*]; $variant:ident($input:ty) = $tag:literal => Durable; $($rest:tt)*) => {
        durable_arguments! { @collect [$($done)* $variant($input) = $tag,]; $($rest)* }
    };
    (@collect [$($done:tt)*]; $variant:ident($input:ty) = $tag:literal => Immediate; $($rest:tt)*) => {
        durable_arguments! { @collect [$($done)*]; $($rest)* }
    };
}
macro_rules! request_budget {
    (Bytes, $request:ident) => {
        $request.max_bytes
    };
    (None, $request:ident) => {{
        let _ = $request;
        None
    }};
}
macro_rules! output_page {
    (Paged, $value:ident) => {
        Some(&mut $value.page)
    };
    (None, $value:ident) => {{
        let _ = $value;
        None
    }};
}
macro_rules! research_operations {
    ($($variant:ident($input:ty) = $name:literal => ($result:ident($output:ty),$rpc:literal,$description:literal,$effect:ident,$published:literal,$internal:literal,$durability:ident,$budget:ident,$response:ident,$page:ident)),* $(,)?) => {
        crate::native_payload! { @tag "method", "params";
            /// Every operation shares its native field, wire and MCP binding declaration.
            #[derive(JsonSchema)]
            pub enum ResearchRequest { $($variant($input) = $name),* }
        }
        crate::native_vocabulary! { pub enum Operation { $($variant = $internal),* } }
        crate::native_payload! { @untagged "tool";
            /// Native payloads derive from the same operation declaration as their requests.
            #[derive(JsonSchema)]
            #[schemars(rename = "LibraryEnrichmentToolData")]
            pub enum ToolData {
                Empty(crate::wire::data::EmptyData) = "empty",
                AdapterStatus(Box<crate::wire::data::LocalStatus>) = "adapter_status",
                $($result(Box<$output>) = $name),*
            }
        }
        $(impl From<$output> for ToolData {
            fn from(value: $output) -> Self { Self::$result(Box::new(value)) }
        })*
        impl ToolData {
            pub fn page_mut(&mut self) -> Option<&mut crate::wire::Page> {
                match self {
                    $(Self::$result(value) => output_page!($page, value),)*
                    Self::Empty(_) | Self::AdapterStatus(_) => None,
                }
            }
        }
        pub fn register_output_schemas(generator: &mut schemars::SchemaGenerator) {
            $(let _ = generator.subschema_for::<$output>();)*
        }

        $(impl From<$input> for ResearchRequest {
            fn from(request: $input) -> Self { Self::$variant(request) }
        })*
        impl ResearchRequest {
            /// One generated RPC decoder, including the non-tool manifest resource.
            pub fn from_rpc(method: &str, params: serde_json::Value) -> Option<Result<Self,serde_json::Error>> {
                match method { $($rpc => Some(serde_json::from_value(params).map(Self::$variant)),)* _ => None }
            }
            pub fn operation(&self) -> Operation {
                match self { $(Self::$variant(_) => Operation::$variant),* }
            }
            pub fn requested_budget(&self) -> Option<usize> {
                match self { $(Self::$variant(request) => request_budget!($budget, request)),* }
            }
        }
        impl Operation {
            pub fn rpc(self) -> &'static str { match self { $(Self::$variant => $rpc),* } }
            /// Request/result fields and native codec revisions jointly invalidate continuations.
            pub fn contract_id(self) -> &'static str {
                match self { $(Self::$variant => {
                    static CONTRACT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
                        let schema = arrow::datatypes::Schema::new(vec![
                            crate::native_union::field::<$input>(concat!($internal,"_request"), crate::native_union::Rule::Text),
                            crate::native_union::field::<$output>(concat!($internal,"_result"), crate::native_union::Rule::Text),
                        ]);
                        crate::native_contract::Manifest::new(&schema, &schema)
                            .and_then(|manifest| crate::operation::Contract::new(Operation::$variant.definition(), manifest).identity())
                            .expect("finite operation contract declaration")
                    });
                    CONTRACT.as_str()
                }),* }
            }
            pub fn definition(self) -> crate::operation::Definition {
                match self { $(Self::$variant => crate::operation::Definition {
                    operation: self, name: $name.into(), rpc: $rpc.into(), description: $description.into(),
                    effect: crate::wire::bindings::Effect::$effect, published: $published,
                    durability: crate::operation::Durability::$durability,
                    response: crate::operation::ResponsePolicy::$response,
                }),* }
            }
        }
        pub fn operation_definitions() -> Vec<crate::operation::Definition> {
            vec![$(Operation::$variant.definition()),*]
        }
        durable_arguments! { @collect []; $($variant($input) = $internal => $durability;)* }
        /// Generated transport catalog, emitted with the request schema.
        pub fn operation_bindings() -> Vec<serde_json::Value> {
            vec![$(crate::wire::bindings::binding::<$input,$output>(Operation::$variant.definition())),*]
        }
    };
}
research_operations! {
    Resolve(ResolveRequest) = "resolve_library" => (ResolveLibrary(crate::wire::data::ResolveData),"library.resolve","Establish exact identity and environment before research.",Acquire,true,"resolve",Durable,None,Research,None),
    Overview(OverviewRequest) = "library_overview" => (LibraryOverview(crate::wire::data::OverviewData),"library.overview","Inspect published library capabilities, coverage and environment.",Read,true,"overview",Immediate,Bytes,Research,None),
    Search(SearchRequest) = "search_evidence" => (SearchEvidence(crate::wire::data::SearchData),"evidence.search","Find bounded evidence with native ranking and provenance.",Read,true,"search",Immediate,Bytes,Research,Paged),
    Inspect(InspectRequest) = "inspect_symbol" => (InspectSymbol(crate::wire::data::InspectData),"symbol.inspect","Inspect a symbol and requested aspects; execution requires explicit intent and native authorization.",Execute,true,"inspect",Durable,Bytes,Research,None),
    Compare(CompareRequest) = "compare_releases" => (CompareReleases(crate::wire::data::CompareData),"library.compare","Compare exact evidence scopes with explicit coverage and environment confounders.",Acquire,true,"compare",Durable,Bytes,Research,Paged),
    Verify(crate::execution::VerifyRequest) = "verify_usage" => (VerifyUsage(crate::execution::VerificationData),"usage.verify","Test a proposed usage in an authorized isolated environment.",Execute,true,"verify",Durable,Bytes,Verification,None),
    ReadArtifact(ReadArtifactRequest) = "read_artifact" => (ReadArtifact(crate::wire::data::ArtifactSliceData),"artifact.read","Read bounded immutable content or an independently retained result section.",Read,true,"read_artifact",Immediate,Bytes,Research,Paged),
    Job(crate::execution::JobRequest) = "job_control" => (JobControl(crate::execution::JobData),"job.control","Observe, wait for or cancel your interest in durable work.",Execute,true,"job",Immediate,Bytes,Job,None),
    ServiceStatus(StatusRequest) = "service_status" => (ServiceStatus(crate::wire::status::StatusData),"service.status","Inspect readiness and capabilities without indexing.",Read,true,"service_status",Immediate,None,Status,None),
    SnapshotManifest(ManifestRequest) = "snapshot_manifest" => (SnapshotManifest(crate::wire::data::ManifestData),"snapshot.manifest","Read an exact published snapshot manifest.",Read,false,"snapshot_manifest",Immediate,None,Research,None),
}

/// The request schema also carries the generated transport catalog, outside its validation keywords.
pub fn request_schema() -> serde_json::Value {
    let mut schema = schemars::generate::SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<ResearchRequest>()
        .to_value();
    let operations = operation_bindings();
    schema["x-enrichment-resources"] = serde_json::Value::Array(resources::bindings(&operations));
    schema["x-enrichment-operations"] = serde_json::Value::Array(operations);
    schema["x-enrichment-mcp-delivery"] = crate::mcp_delivery::contract();
    crate::native_wire::schema(schema)
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
    fn inspection_options_have_a_closed_transport_vocabulary() {
        let defaults: InspectionOptions = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(defaults.intent, InspectionIntent::Retained);
        for unknown in [
            serde_json::json!({"methods":["arbitrary/method"]}),
            serde_json::json!({"host_path":"/tmp/source.py"}),
        ] {
            assert!(serde_json::from_value::<InspectionOptions>(unknown).is_err());
        }
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
    fn unknown_fields_are_rejected_and_omitted_ones_default() {
        let parsed: ResolveRequest =
            serde_json::from_str(r#"{"name":"serde"}"#).expect("minimal request parses");
        assert_eq!(parsed.freshness, FreshnessMode::CacheOk);
        assert_eq!(parsed.ecosystem, Ecosystem::Rust);
        assert!(serde_json::from_str::<ResolveRequest>(r#"{"name":"x","upgrade":true}"#).is_err());
        let inspect: InspectRequest =
            serde_json::from_value(serde_json::json!({"context_id":format!("ctx_{}", "a".repeat(64)),"symbol_path":"a::b"})).expect("parses");
        assert_eq!(inspect.selection, crate::wire::ResearchSelection::Default);
        assert!(
            serde_json::from_str::<SearchRequest>(r#"{"context_id":"c","query":"q","sort":1}"#)
                .is_err()
        );
    }
}
