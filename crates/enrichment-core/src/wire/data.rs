//! Typed `data` payloads for the research tools (blueprint §7).
//!
//! The frozen envelope leaves `data` as a free-form object. These types give each tool's
//! payload a schema of its own, emitted alongside the envelope schema so the adapter can
//! validate what it forwards and the Python DTOs can be generated rather than hand-written
//! (§6.3). The envelope's `data` field itself stays a `JsonObject`; a payload is serialized
//! into it.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::evidence::{
    Artifact, Availability, EvidenceFragment, FragmentKind, Gap, ObservedConfiguration,
    SnapshotCounts, Symbol, SymbolKind,
};
use crate::identity::{Context, Environment, Release};
use crate::producer::ProducerRun;
use crate::producer::docsrs::DocsRsMetadata;
use crate::producer::source::SourceExcerpt;
use crate::registry::UpstreamCheck;

/// The file the tool payload schemas are emitted to.
pub const TOOL_DATA_SCHEMA_FILE: &str = "tool-data.schema.json";

/// What docs.rs had for the resolved release.
///
/// The values are, in order: `available`, `missing`, `unsupported`, `not_attempted`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum HostedJsonState {
    Available,
    Missing,
    Unsupported,
    NotAttempted,
}

/// The hosted rustdoc JSON facet of a resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HostedJsonReport {
    /// What was found.
    pub state: HostedJsonState,
    /// The format version the payload declared, when one was read.
    pub format_version: Option<u32>,
    /// The format versions this build can interpret.
    pub supported_formats: Vec<u32>,
    /// The target the JSON was requested for.
    pub target: String,
    /// The URL that was asked.
    pub url: String,
    /// `crate_version` as the JSON itself declares it, when available.
    pub declared_crate_version: Option<String>,
}

/// What the published snapshot holds, summarized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SnapshotSummary {
    /// The snapshot identity.
    pub snapshot_id: String,
    /// Normalizer version that produced it.
    pub normalizer_version: String,
    /// Counts.
    pub counts: SnapshotCounts,
    /// Publication time.
    pub published_at: String,
}

/// The `resolve_library` payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ResolveData {
    /// The release that was resolved. Its version is the one asked for, never an upgrade.
    pub release: Release,
    /// The environment the context binds, with its resolution status.
    pub environment: Environment,
    /// The context identity every later call takes.
    pub context: Context,
    /// What the registry said about newer releases, kept apart from the resolution.
    pub upstream: Option<UpstreamCheck>,
    /// The maintainer's docs.rs build configuration, read from the crate manifest.
    pub observed_configuration: Option<DocsRsMetadata>,
    /// Hosted rustdoc JSON availability.
    pub hosted_rustdoc_json: Option<HostedJsonReport>,
    /// Distribution metadata and file inventory, only for Python.
    #[serde(default)]
    pub python: Option<crate::producer::python::Distribution>,
    /// The snapshot published from this resolution, when normalization succeeded.
    pub snapshot: Option<SnapshotSummary>,
    /// Every artifact this resolution stored or reused.
    pub artifacts: Vec<Artifact>,
    /// Expected evidence that is absent, each with a reason and a planned fallback.
    pub gaps: Vec<Gap>,
    /// Provenance for each producer that ran.
    pub producer_runs: Vec<ProducerRun>,
    /// Whether this answer was replayed from a recorded resolution rather than fetched.
    pub answered_from_cache: bool,
}

/// One child in a namespace sample.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OverviewChild {
    /// Public path.
    pub path: String,
    /// Kind.
    pub kind: SymbolKind,
    /// Summary, when documented.
    pub doc_summary: Option<String>,
    /// Whether this path re-exports a definition elsewhere.
    pub is_reexport: bool,
    /// Whether deprecated.
    pub deprecated: bool,
}

/// A namespace facet in an overview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NamespaceFacet {
    /// The module path.
    pub path: String,
    /// First paragraph of the module docs.
    pub doc_summary: Option<String>,
    /// Distinct definitions directly under this module, by kind.
    pub counts_by_kind: BTreeMap<String, u64>,
    /// A bounded sample of direct children, definitions counted once.
    pub children: Vec<OverviewChild>,
    /// Children beyond the sample.
    pub truncated_children: u64,
}

/// The `library_overview` payload (§7.1: a tree and facets, never a symbol dump).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OverviewData {
    /// The crate's root module name.
    pub crate_name: String,
    /// The crate version the documentation declares.
    pub crate_version: Option<String>,
    /// The snapshot read.
    pub snapshot: SnapshotSummary,
    /// The documentation build's configuration.
    pub observed_configuration: Option<ObservedConfiguration>,
    /// The subtree the overview was narrowed to, when any.
    pub area: Option<String>,
    /// Distinct definitions by kind across the area.
    pub definitions_by_kind: BTreeMap<String, u64>,
    /// Namespaces, root first.
    pub namespaces: Vec<NamespaceFacet>,
    /// Namespaces beyond the returned set.
    pub truncated_namespaces: u64,
    /// Library-level retained fragments, preserving independently sourced alternatives.
    pub discovery: Vec<DiscoveryFacet>,
    /// Paths that re-export a definition reachable elsewhere.
    pub reexports: u64,
    /// Re-exports whose target is outside this crate.
    pub unresolved_reexports: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryFacet {
    pub kind: crate::wire::research::DiscoveryKind,
    pub state: super::research::AspectState,
    pub reason: Option<String>,
    pub diagnostic: Option<super::research::Diagnostic>,
    pub items: Vec<FragmentProjection>,
    pub page: Option<super::Page>,
}

/// What kind of thing a search hit is.
///
/// The values are, in order: `symbol`, `fragment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum HitKind {
    Symbol,
    Fragment,
}

/// One scoring factor that fired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScoreFactor {
    /// Factor name.
    pub name: String,
    /// Points contributed.
    pub points: u32,
}

/// One search hit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchHit {
    /// Symbol or fragment.
    pub hit: HitKind,
    /// Total score.
    pub score: u32,
    /// The factors behind the score.
    pub factors: Vec<ScoreFactor>,
    /// The evidence entry this hit is cited by.
    pub evidence_id: String,
    /// Symbol path, for symbol hits.
    pub path: Option<String>,
    /// Symbol kind, for symbol hits.
    pub symbol_kind: Option<SymbolKind>,
    /// Rendered signature, for symbol hits.
    pub signature: Option<String>,
    /// Other public paths to the same definition.
    pub also_at: Vec<String>,
    /// Whether the symbol is deprecated.
    pub deprecated: bool,
    /// Fragment kind, for fragment hits.
    pub fragment_kind: Option<FragmentKind>,
    /// Fragment subject, for fragment hits.
    pub subject: Option<String>,
    /// Bounded excerpt.
    pub excerpt: String,
}

/// The `search_evidence` payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchData {
    /// Bounded page over this payload, not over the whole research envelope.
    pub page: super::job::Page,
    /// The query as asked.
    pub query: String,
    /// The tokens it was split into.
    pub tokens: Vec<String>,
    /// The evidence families searched.
    pub kinds: Vec<String>,
    /// Namespace subtree that was searched, when requested.
    #[serde(default)]
    pub area: Option<String>,
    /// The hits on this page.
    pub hits: Vec<SearchHit>,
    /// The scoring legend, in rank order.
    pub scoring: Vec<ScoreFactor>,
    /// Which sources were searched.
    pub searched: Vec<String>,
    /// Page offset.
    pub offset: u64,
}

/// A selected projection of an admitted API observation, never a replacement stored fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApiObservationProjection {
    /// Identity of the complete admitted fact, not a hash of this query projection.
    pub observation_id: String,
    pub subject: crate::evidence::relational::SubjectRef,
    pub origin: crate::evidence::relational::ApiOrigin,
    pub environment_id: String,
    pub payload: crate::evidence::relational::ApiPayload,
    pub source: crate::evidence::relational::FactSource,
    /// False means docs were omitted by projection; it does not assert absent documentation.
    pub docs_included: bool,
}

/// One exact definition a caller can select when a public path is ambiguous.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InspectionCandidate {
    pub path: String,
    pub definition_id: String,
    pub kind: SymbolKind,
    pub qualifier: Option<String>,
}

/// A requested projection of a retained fragment; its identity still names the full fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FragmentProjection {
    pub fragment: EvidenceFragment,
    pub text_complete: bool,
    /// A complete-text request for this selection when text was projected.
    pub complete: Option<super::research::RecoveryAction>,
}

/// Bounded inspection payload with independently qualified observation projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InspectData {
    /// Lexical direct children, independently paged; this does not establish typed membership.
    pub children: Vec<InspectionCandidate>,
    /// Bindings reached by qualified member_of relationships, independently paged.
    pub members: Vec<InspectionCandidate>,
    /// Independent aspect states and continuations.
    pub aspect_outcomes: Vec<super::research::AspectOutcome>,
    /// The symbol, when one was selected. Its `docs` are bounded here.
    pub symbol: Option<Symbol>,
    /// Whether `docs` was cut to fit.
    pub docs_truncated: bool,
    /// Other public paths to the same definition.
    pub also_at: Vec<String>,
    /// Distinct selectable definitions; public paths alone may collide across kinds.
    pub candidates: Vec<InspectionCandidate>,
    /// The aspects actually returned.
    pub aspects: Vec<String>,
    /// What can and cannot be said about availability.
    pub availability: Option<Availability>,
    /// Typed edges touching the symbol.
    pub relationships: Vec<crate::evidence::relational::RelationshipObservation>,
    /// Qualified projections; complete immutable observations remain in the snapshot.
    pub observations: Vec<ApiObservationProjection>,
    /// Fragments about the symbol, bounded.
    pub fragments: Vec<FragmentProjection>,
    /// Source excerpt at `source` depth.
    pub source: Option<SourceExcerpt>,
    /// Retained execution facts selected natively by symbol/document and exact scope.
    pub execution_observations: Vec<crate::evidence::execution::ExecutionObservation>,
    /// Actual attempt attribution for an execution job result.
    pub producer_runs: Vec<crate::producer::ProducerRun>,
}

/// How a slice is encoded.
///
/// The values are, in order: `utf8`, `base64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum SliceEncoding {
    Utf8,
    Base64,
}

/// The `read_artifact` payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSliceData {
    /// Bounded page over this payload, not over the whole research envelope.
    pub page: super::job::Page,
    /// The artifact record.
    pub artifact: Artifact,
    /// How `content` is encoded.
    pub encoding: SliceEncoding,
    /// First byte offset of the slice.
    pub start: u64,
    /// One past the last byte offset of the slice.
    pub end: u64,
    /// Total artifact size.
    pub total: u64,
    /// The slice.
    pub content: String,
    /// SHA-256 of the slice bytes.
    pub content_digest: String,
    /// Bytes after `end`.
    pub remaining: u64,
    /// The section that was selected, when one was.
    pub section: Option<String>,
}

/// The snapshot manifest resource: what a snapshot contains.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ManifestData {
    /// The immutable manifest as published.
    pub manifest: crate::evidence::snapshot::EvidenceManifest,
    /// Whether this snapshot is the context's current one.
    pub is_current: bool,
    /// Operational attribution from the catalog generation pinned by this request.
    pub producer_runs: Vec<crate::producer::ProducerRun>,
    pub catalog_generation: u64,
}

/// Identity of one side of an immutable comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComparisonSide {
    pub coverage: super::Coverage,
    pub context_id: String,
    pub snapshot_id: String,
    pub release: Release,
    pub environment: Environment,
}

/// A changed environment/configuration field, separate from release changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationDifference {
    pub field: String,
    pub before: serde_json::Value,
    pub after: serde_json::Value,
}

/// An observed diff with explicit completeness and environment confounders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CompareData {
    /// Bounded page over this payload, not over the whole research envelope.
    pub page: super::job::Page,
    pub before: ComparisonSide,
    pub after: ComparisonSide,
    pub comparable: bool,
    pub same_release: bool,
    pub configuration_differences: Vec<ConfigurationDifference>,
    pub confounders: Vec<String>,
    pub scopes: Vec<crate::compare::Scope>,
    pub api_complete: bool,
    pub total_changes: u64,
    pub changes: Vec<crate::compare::Change>,
    pub offset: u64,
}

/// Every tool payload, for schema emission. Never sent on the wire as a union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "tool", rename_all = "snake_case")]
#[schemars(rename = "LibraryEnrichmentToolData")]
pub enum ToolData {
    VerifyUsage(Box<crate::execution::VerificationData>),
    JobControl(Box<crate::execution::JobData>),
    /// `compare_releases`.
    CompareReleases(Box<CompareData>),
    /// `resolve_library`.
    ResolveLibrary(Box<ResolveData>),
    /// `library_overview`.
    LibraryOverview(Box<OverviewData>),
    /// `search_evidence`.
    SearchEvidence(Box<SearchData>),
    /// `inspect_symbol`.
    InspectSymbol(Box<InspectData>),
    /// `read_artifact`.
    ReadArtifact(Box<ArtifactSliceData>),
    /// The snapshot manifest resource.
    SnapshotManifest(Box<ManifestData>),
    /// `service_status`.
    ServiceStatus(Box<super::status::StatusData>),
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
            "Symbol",
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
