//! Typed `data` payloads for the research tools (blueprint §7).
//!
//! The frozen envelope leaves `data` as a free-form object. These types give each tool's
//! payload a schema of its own, emitted alongside the envelope schema so the adapter can
//! validate what it forwards and the Python DTOs can be generated rather than hand-written
//! (§6.3). The envelope's `data` field itself stays a `JsonObject`; a payload is serialized
//! into it.

use std::collections::BTreeMap;

use schemars::JsonSchema;

use crate::evidence::{
    Artifact, Availability, FragmentKind, Gap, ObservedConfiguration, SnapshotCounts, SymbolHeader,
    SymbolKind, TextFragment,
};
use crate::identity::{Context, Environment, Release};
use crate::producer::ProducerRun;
use crate::producer::docsrs::DocsRsMetadata;
use crate::producer::source::SourceExcerpt;
use crate::registry::UpstreamCheck;

/// The file the tool payload schemas are emitted to.
pub const TOOL_DATA_SCHEMA_FILE: &str = "tool-data.schema.json";

crate::native_vocabulary! {
/// What docs.rs had for the resolved release.
///
/// The values are, in order: `available`, `missing`, `unsupported`, `not_attempted`.
#[schemars(inline)]
pub enum HostedJsonState {
    Available = "available",
    Missing = "missing",
    Unsupported = "unsupported",
    NotAttempted = "not_attempted",
}
}

crate::native_struct! {
/// The hosted rustdoc JSON facet of a resolution.
pub struct HostedJsonReport {
    /// What was found.
    state: HostedJsonState => crate::native_union::Rule::Text,
    /// The format version the payload declared, when one was read.
    format_version: Option<u32> => crate::native_union::Rule::Text,
    /// The format versions this build can interpret.
    supported_formats: Vec<u32> => crate::native_union::Rule::Sequence,
    /// The target the JSON was requested for.
    target: String => crate::native_union::Rule::Text,
    /// The URL that was asked.
    url: String => crate::native_union::Rule::Text,
    /// `crate_version` as the JSON itself declares it, when available.
    declared_crate_version: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// What the published snapshot holds, summarized.
pub struct SnapshotSummary {
    /// The snapshot identity.
    snapshot_id: String => crate::native_union::Rule::Text,
    /// Normalizer version that produced it.
    normalizer_version: String => crate::native_union::Rule::Text,
    /// Counts.
    counts: SnapshotCounts => crate::native_union::Rule::Text,
    /// Publication time.
    published_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// The `resolve_library` payload.
pub struct ResolveData {
    /// The release that was resolved. Its version is the one asked for, never an upgrade.
    release: Release => crate::native_union::Rule::Text,
    /// The environment the context binds, with its resolution status.
    environment: Environment => crate::native_union::Rule::Text,
    /// The context identity every later call takes.
    context: Context => crate::native_union::Rule::Text,
    /// What the registry said about newer releases, kept apart from the resolution.
    upstream: Option<UpstreamCheck> => crate::native_union::Rule::Text,
    /// The maintainer's docs.rs build configuration, read from the crate manifest.
    observed_configuration: Option<DocsRsMetadata> => crate::native_union::Rule::Text,
    /// Hosted rustdoc JSON availability.
    hosted_rustdoc_json: Option<HostedJsonReport> => crate::native_union::Rule::Text,
    /// Distribution metadata and file inventory, only for Python.
    #[serde(default)]
    python: Option<crate::producer::python::Distribution> => crate::native_union::Rule::Text,
    /// The snapshot published from this resolution, when normalization succeeded.
    snapshot: Option<SnapshotSummary> => crate::native_union::Rule::Text,
    /// Every artifact this resolution stored or reused.
    artifacts: Vec<Artifact> => crate::native_union::Rule::Sequence,
    /// Expected evidence that is absent, each with a reason and a planned fallback.
    gaps: Vec<Gap> => crate::native_union::Rule::Sequence,
    /// Provenance for each producer that ran.
    producer_runs: Vec<ProducerRun> => crate::native_union::Rule::Sequence,
    /// Whether this answer was replayed from a recorded resolution rather than fetched.
    answered_from_cache: bool => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// One child in a namespace sample.
pub struct OverviewChild {
    /// Public path.
    path: String => crate::native_union::Rule::Text,
    /// Kind.
    kind: SymbolKind => crate::native_union::Rule::Text,
    /// Summary, when documented.
    doc_summary: Option<String> => crate::native_union::Rule::Text,
    /// Whether this path re-exports a definition elsewhere.
    is_reexport: bool => crate::native_union::Rule::Text,
    /// Whether deprecated.
    deprecated: bool => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// A namespace facet in an overview.
pub struct NamespaceFacet {
    /// The module path.
    path: String => crate::native_union::Rule::Text,
    /// First paragraph of the module docs.
    doc_summary: Option<String> => crate::native_union::Rule::Text,
    /// Distinct definitions directly under this module, by kind.
    counts_by_kind: BTreeMap<String, u64> => crate::native_union::Rule::Map,
    /// A bounded sample of direct children, definitions counted once.
    children: Vec<OverviewChild> => crate::native_union::Rule::Sequence,
    /// Children beyond the sample.
    truncated_children: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// The `library_overview` payload (§7.1: a tree and facets, never a symbol dump).
pub struct OverviewData {
    /// The crate's root module name.
    crate_name: String => crate::native_union::Rule::Text,
    /// The crate version the documentation declares.
    crate_version: Option<String> => crate::native_union::Rule::Text,
    /// The snapshot read.
    snapshot: SnapshotSummary => crate::native_union::Rule::Text,
    /// The documentation build's configuration.
    observed_configuration: Option<ObservedConfiguration> => crate::native_union::Rule::Text,
    /// The subtree the overview was narrowed to, when any.
    area: Option<String> => crate::native_union::Rule::Text,
    /// Distinct definitions by kind across the area.
    definitions_by_kind: BTreeMap<String, u64> => crate::native_union::Rule::Map,
    /// Namespaces, root first.
    namespaces: Vec<NamespaceFacet> => crate::native_union::Rule::Sequence,
    /// Namespaces beyond the returned set.
    truncated_namespaces: u64 => crate::native_union::Rule::Text,
    /// Library-level retained fragments, preserving independently sourced alternatives.
    discovery: Vec<DiscoveryFacet> => crate::native_union::Rule::Sequence,
    /// Paths that re-export a definition reachable elsewhere.
    reexports: u64 => crate::native_union::Rule::Text,
    /// Re-exports whose target is outside this crate.
    unresolved_reexports: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct DiscoveryFacet {
    kind: crate::wire::research::DiscoveryKind => crate::native_union::Rule::Text,
    state: super::research::AspectState => crate::native_union::Rule::Text,
    reason: Option<String> => crate::native_union::Rule::Text,
    diagnostic: Option<super::research::Diagnostic> => crate::native_union::Rule::Text,
    items: Vec<FragmentProjection> => crate::native_union::Rule::Sequence,
    page: Option<super::Page> => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! {
/// What kind of thing a search hit is.
///
/// The values are, in order: `symbol`, `fragment`.
#[schemars(inline)]
pub enum HitKind {
    Symbol = "symbol",
    Fragment = "fragment",
}
}

crate::native_struct! {
/// One scoring factor that fired.
pub struct ScoreFactor {
    /// Factor name.
    name: String => crate::native_union::Rule::Text,
    /// Points contributed.
    points: u32 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// One search hit.
pub struct SearchHit {
    /// Symbol or fragment.
    hit: HitKind => crate::native_union::Rule::Text,
    /// Total score.
    score: u32 => crate::native_union::Rule::Text,
    /// The factors behind the score.
    factors: Vec<ScoreFactor> => crate::native_union::Rule::Sequence,
    /// The evidence entry this hit is cited by.
    evidence_id: String => crate::native_union::Rule::Text,
    /// Symbol path, for symbol hits.
    path: Option<String> => crate::native_union::Rule::Text,
    /// Symbol kind, for symbol hits.
    symbol_kind: Option<SymbolKind> => crate::native_union::Rule::Text,
    /// Rendered signature, for symbol hits.
    signature: Option<String> => crate::native_union::Rule::Text,
    /// Other public paths to the same definition.
    also_at: Vec<String> => crate::native_union::Rule::Sequence,
    /// Whether the symbol is deprecated.
    deprecated: bool => crate::native_union::Rule::Text,
    /// Fragment kind, for fragment hits.
    fragment_kind: Option<FragmentKind> => crate::native_union::Rule::Text,
    /// Fragment subject, for fragment hits.
    subject: Option<String> => crate::native_union::Rule::Text,
    /// Bounded excerpt.
    excerpt: String => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// The `search_evidence` payload.
pub struct SearchData {
    /// Bounded page over this payload, not over the whole research envelope.
    page: super::job::Page => crate::native_union::Rule::Text,
    /// The query as asked.
    query: String => crate::native_union::Rule::Text,
    /// The tokens it was split into.
    tokens: Vec<String> => crate::native_union::Rule::Sequence,
    /// The evidence families searched.
    kinds: Vec<String> => crate::native_union::Rule::Sequence,
    /// Namespace subtree that was searched, when requested.
    #[serde(default)]
    area: Option<String> => crate::native_union::Rule::Text,
    /// The hits on this page.
    hits: Vec<SearchHit> => crate::native_union::Rule::Sequence,
    /// The scoring legend, in rank order.
    scoring: Vec<ScoreFactor> => crate::native_union::Rule::Sequence,
    /// Which sources were searched.
    searched: Vec<String> => crate::native_union::Rule::Sequence,
    /// Page offset.
    offset: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// A selected projection of an admitted API observation, never a replacement stored fact.
pub struct ApiObservationProjection {
    /// Identity of the complete admitted fact, not a hash of this query projection.
    observation_id: String => crate::native_union::Rule::Text,
    subject: crate::evidence::relational::SubjectRef => crate::native_union::Rule::Text,
    origin: crate::evidence::relational::ApiOrigin => crate::native_union::Rule::Text,
    environment_id: String => crate::native_union::Rule::Text,
    payload: crate::evidence::relational::ApiPayload => crate::native_union::Rule::Text,
    source: crate::evidence::relational::FactSource => crate::native_union::Rule::Text,
    /// False means docs were omitted by projection; it does not assert absent documentation.
    docs_included: bool => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// One exact definition a caller can select when a public path is ambiguous.
pub struct InspectionCandidate {
    path: String => crate::native_union::Rule::Text,
    definition_id: String => crate::native_union::Rule::Text,
    kind: SymbolKind => crate::native_union::Rule::Text,
    qualifier: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// A requested projection of a retained fragment; its identity still names the full fact.
pub struct FragmentProjection {
    fragment: TextFragment => crate::native_union::Rule::Text,
    text_complete: bool => crate::native_union::Rule::Text,
    /// A complete-text request for this selection when text was projected.
    complete: Option<super::research::RecoveryAction> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Bounded inspection payload with independently qualified observation projections.
pub struct InspectData {
    /// Lexical direct children, independently paged; this does not establish typed membership.
    children: Vec<InspectionCandidate> => crate::native_union::Rule::Sequence,
    /// Bindings reached by qualified member_of relationships, independently paged.
    members: Vec<InspectionCandidate> => crate::native_union::Rule::Sequence,
    /// Independent aspect states and continuations.
    aspect_outcomes: Vec<super::research::AspectOutcome> => crate::native_union::Rule::Section("aspects".into()),
    /// The symbol, when one was selected. Its `docs` are bounded here.
    symbol: Option<SymbolHeader> => crate::native_union::Rule::Text,
    /// Whether `docs` was cut to fit.
    docs_truncated: bool => crate::native_union::Rule::Text,
    /// Other public paths to the same definition.
    also_at: Vec<String> => crate::native_union::Rule::Sequence,
    /// Distinct selectable definitions; public paths alone may collide across kinds.
    candidates: Vec<InspectionCandidate> => crate::native_union::Rule::Sequence,
    /// The aspects actually returned.
    aspects: Vec<String> => crate::native_union::Rule::Sequence,
    /// What can and cannot be said about availability.
    availability: Option<Availability> => crate::native_union::Rule::Text,
    /// Typed edges touching the symbol.
    relationships: Vec<crate::evidence::relational::RelationshipObservation> => crate::native_union::Rule::Sequence,
    /// Qualified projections; complete immutable observations remain in the snapshot.
    observations: Vec<ApiObservationProjection> => crate::native_union::Rule::Section("signature".into()),
    /// Fragments about the symbol, bounded.
    fragments: Vec<FragmentProjection> => crate::native_union::Rule::Sequence,
    /// Source excerpt at `source` depth.
    source: Option<SourceExcerpt> => crate::native_union::Rule::Text,
    /// Retained execution facts selected natively by symbol/document and exact scope.
    execution_observations: Vec<crate::evidence::execution::ExecutionObservation> => crate::native_union::Rule::Sequence,
    /// Actual attempt attribution for an execution job result.
    producer_runs: Vec<crate::producer::ProducerRun> => crate::native_union::Rule::Sequence,
}
}

crate::native_vocabulary! {
/// How a slice is encoded.
///
/// The values are, in order: `utf8`, `base64`.
#[schemars(inline)]
pub enum SliceEncoding {
    Utf8 = "utf8",
    Base64 = "base64",
}
}

crate::native_struct! {
/// The `read_artifact` payload.
pub struct ArtifactSliceData {
    /// Bounded page over this payload, not over the whole research envelope.
    page: super::job::Page => crate::native_union::Rule::Text,
    /// The artifact record.
    artifact: Artifact => crate::native_union::Rule::Text,
    /// How `content` is encoded.
    encoding: SliceEncoding => crate::native_union::Rule::Text,
    /// First byte offset of the slice.
    start: u64 => crate::native_union::Rule::Text,
    /// One past the last byte offset of the slice.
    end: u64 => crate::native_union::Rule::Text,
    /// Total artifact size.
    total: u64 => crate::native_union::Rule::Text,
    /// The slice.
    content: String => crate::native_union::Rule::Text,
    /// SHA-256 of the slice bytes.
    content_digest: String => crate::native_union::Rule::Text,
    /// Bytes after `end`.
    remaining: u64 => crate::native_union::Rule::Text,
    /// The section that was selected, when one was.
    section: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// The snapshot manifest resource: what a snapshot contains.
pub struct ManifestData {
    /// The immutable manifest as published.
    manifest: crate::evidence::snapshot::EvidenceManifest => crate::native_union::Rule::Text,
    /// Whether this snapshot is the context's current one.
    is_current: bool => crate::native_union::Rule::Text,
    /// Operational attribution from the catalog generation pinned by this request.
    producer_runs: Vec<crate::producer::ProducerRun> => crate::native_union::Rule::Sequence,
    control_version: u64 => crate::native_union::Rule::Text,
    previous_snapshot_id: Option<String> => crate::native_union::Rule::Text,
    publication_changes: Vec<RelationChanges> => crate::native_union::Rule::Sequence,
}
}

crate::native_struct! {
/// Semantic row changes between publication cohorts, derived from bounded native CDF inputs.
pub struct RelationChanges {
    relation: String => crate::native_union::Rule::Text,
    inserted: u64 => crate::native_union::Rule::Text,
    removed: u64 => crate::native_union::Rule::Text,
    updated: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Identity of one side of an immutable comparison.
pub struct ComparisonSide {
    coverage: super::Coverage => crate::native_union::Rule::Text,
    context_id: String => crate::native_union::Rule::Text,
    snapshot_id: String => crate::native_union::Rule::Text,
    release: Release => crate::native_union::Rule::Text,
    environment: Environment => crate::native_union::Rule::Text,
}
}

crate::native_union! { @tag "field";
/// A typed environment/configuration difference, separate from release changes.
pub enum ConfigurationDifference {
    Toolchain = "toolchain" {
        before: Option<String> => crate::native_union::Rule::Text,
        after: Option<String> => crate::native_union::Rule::Text,
    },
    Target = "target" {
        before: Option<String> => crate::native_union::Rule::Text,
        after: Option<String> => crate::native_union::Rule::Text,
    },
    Features = "features" {
        before: Vec<String> => crate::native_union::Rule::Set,
        after: Vec<String> => crate::native_union::Rule::Set,
    },
    FeaturesKnown = "features_known" {
        before: bool => crate::native_union::Rule::Text,
        after: bool => crate::native_union::Rule::Text,
    },
    DefaultFeatures = "default_features" {
        before: Option<bool> => crate::native_union::Rule::Text,
        after: Option<bool> => crate::native_union::Rule::Text,
    },
    LockDigest = "lock_digest" {
        before: Option<String> => crate::native_union::Rule::Text,
        after: Option<String> => crate::native_union::Rule::Text,
    },
    Resolution = "resolution" {
        before: crate::identity::EnvironmentResolution => crate::native_union::Rule::Text,
        after: crate::identity::EnvironmentResolution => crate::native_union::Rule::Text,
    },
    ObservedConfiguration = "observed_configuration" {
        before: Option<ObservedConfiguration> => crate::native_union::Rule::Text,
        after: Option<ObservedConfiguration> => crate::native_union::Rule::Text,
    },
}
}

crate::native_struct! {
/// An observed diff with explicit completeness and environment confounders.
pub struct CompareData {
    /// Bounded page over this payload, not over the whole research envelope.
    page: super::job::Page => crate::native_union::Rule::Text,
    before: ComparisonSide => crate::native_union::Rule::Text,
    after: ComparisonSide => crate::native_union::Rule::Text,
    comparable: bool => crate::native_union::Rule::Text,
    same_release: bool => crate::native_union::Rule::Text,
    configuration_differences: Vec<ConfigurationDifference> => crate::native_union::Rule::Sequence,
    confounders: Vec<String> => crate::native_union::Rule::Sequence,
    scopes: Vec<crate::compare::Scope> => crate::native_union::Rule::Sequence,
    api_complete: bool => crate::native_union::Rule::Text,
    total_changes: u64 => crate::native_union::Rule::Text,
    changes: Vec<crate::compare::Change> => crate::native_union::Rule::Section("changes".into()),
    offset: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// An explicit absence of inline tool data; never an untyped dictionary.
#[derive(Default)]
pub struct EmptyData {}
}
crate::native_struct! {
/// Adapter-observed daemon availability; it establishes no library evidence.
pub struct AdapterDaemonStatus {
    available: bool => crate::native_union::Rule::Text,
    detail: String => crate::native_union::Rule::Text,
}
}
crate::native_struct! {
pub struct AdapterStatus {
    available: bool => crate::native_union::Rule::Text,
    tools: Vec<String> => crate::native_union::Rule::Set,
}
}
crate::native_struct! {
pub struct LocalStatus {
    daemon: AdapterDaemonStatus => crate::native_union::Rule::Text,
    adapter: AdapterStatus => crate::native_union::Rule::Text,
}
}

crate::native_payload! { @untagged "tool";
/// Finite native tool payloads. The transport emits the selected record without its native tag.
#[derive(JsonSchema)]
#[schemars(rename = "LibraryEnrichmentToolData")]
pub enum ToolData {
    Empty(EmptyData) = "empty",
    AdapterStatus(Box<LocalStatus>) = "adapter_status",
    VerifyUsage(Box<crate::execution::VerificationData>) = "verify_usage",
    JobControl(Box<crate::execution::JobData>) = "job_control",
    CompareReleases(Box<CompareData>) = "compare_releases",
    ResolveLibrary(Box<ResolveData>) = "resolve_library",
    LibraryOverview(Box<OverviewData>) = "library_overview",
    SearchEvidence(Box<SearchData>) = "search_evidence",
    InspectSymbol(Box<InspectData>) = "inspect_symbol",
    ReadArtifact(Box<ArtifactSliceData>) = "read_artifact",
    SnapshotManifest(Box<ManifestData>) = "snapshot_manifest",
    ServiceStatus(Box<super::status::StatusData>) = "service_status",
}
}
impl Default for ToolData {
    fn default() -> Self {
        Self::Empty(EmptyData {})
    }
}
impl ToolData {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }
    pub fn set_page(&mut self, page: super::Page) {
        match self {
            Self::CompareReleases(data) => data.page = page,
            Self::SearchEvidence(data) => data.page = page,
            Self::ReadArtifact(data) => data.page = page,
            _ => panic!("pagination requires a declared paged payload"),
        }
    }
}
macro_rules! payload_from {
    ($($ty:ty => $variant:ident),* $(,)?) => { $(
        impl From<$ty> for ToolData { fn from(value: $ty) -> Self { Self::$variant(Box::new(value)) } }
    )* };
}
payload_from! {
    crate::execution::VerificationData => VerifyUsage,
    crate::execution::JobData => JobControl,
    CompareData => CompareReleases,
    ResolveData => ResolveLibrary,
    OverviewData => LibraryOverview,
    SearchData => SearchEvidence,
    InspectData => InspectSymbol,
    ArtifactSliceData => ReadArtifact,
    ManifestData => SnapshotManifest,
    super::status::StatusData => ServiceStatus,
}

/// The tool payload schema, canonicalized like the envelope schema.
#[must_use]
pub fn tool_data_schema() -> serde_json::Value {
    let settings = schemars::generate::SchemaSettings::draft2020_12().for_serialize();
    let mut generator = settings.into_generator();
    // Tagged-union emission may inline closed payloads to accommodate its own tool tag.
    // Explicitly retain the untagged domain DTO definitions for adapter composition.
    let _ = generator.subschema_for::<ResolveData>();
    let _ = generator.subschema_for::<OverviewData>();
    let _ = generator.subschema_for::<SearchData>();
    let _ = generator.subschema_for::<InspectData>();
    let _ = generator.subschema_for::<CompareData>();
    let _ = generator.subschema_for::<ArtifactSliceData>();
    let _ = generator.subschema_for::<ManifestData>();
    let _ = generator.subschema_for::<crate::execution::JobData>();
    let _ = generator.subschema_for::<crate::execution::VerificationData>();
    let schema = generator.into_root_schema_for::<ToolData>();
    let mut value = serde_json::to_value(schema).unwrap_or_default();
    if let Some(object) = value.as_object_mut() {
        object.insert(
            "$id".to_owned(),
            serde_json::Value::String(
                "https://library-enrichment.local/schemas/tool-data.schema.json".to_owned(),
            ),
        );
    }
    crate::canonical::canonicalize(value)
}

/// The tool payload schema as the exact bytes `emit-schemas` writes.
#[must_use]
pub fn tool_data_schema_json() -> String {
    let mut json = serde_json::to_string_pretty(&tool_data_schema())
        .expect("a generated schema is always serializable");
    json.push('\n');
    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tool_data_schema_names_every_payload() {
        let schema = tool_data_schema();
        let defs = schema["$defs"].as_object().expect("$defs");
        for name in [
            "ResolveData",
            "OverviewData",
            "SearchData",
            "InspectData",
            "ArtifactSliceData",
            "SymbolHeader",
            "Artifact",
        ] {
            assert!(defs.contains_key(name), "{name} missing from $defs");
        }
        assert_eq!(schema["title"], "LibraryEnrichmentToolData");
    }

    #[test]
    fn schema_emission_is_deterministic() {
        assert_eq!(tool_data_schema_json(), tool_data_schema_json());
    }
}
