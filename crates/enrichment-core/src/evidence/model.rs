//! The normalized evidence records (blueprint §6.1): symbols, relationships, fragments,
//! availability and the snapshot manifest.
//!
//! Identities are content-derived and package-qualified (§3.1): a symbol is named by its
//! canonical public path, a definition by the path it is defined at. Rustdoc's item IDs are
//! kept only as `producer_local_id`, never as cross-release identity.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::EvidenceKind;
use crate::canonical;
use crate::identity::{ContextId, EnvironmentId, ReleaseId, SnapshotId};
use crate::producer::ProducerRun;
use crate::wire::EvidenceClass;

/// What kind of item a symbol is.
///
/// The values are, in order: `module`, `struct`, `union`, `enum`, `variant`, `struct_field`,
/// `trait`, `trait_alias`, `type_alias`, `function`, `method`, `constant`, `static`, `macro`,
/// `proc_macro`, `assoc_type`, `assoc_const`, `primitive`, `extern_crate`, `import`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum SymbolKind {
    Module,
    Struct,
    Union,
    Enum,
    Variant,
    StructField,
    Trait,
    TraitAlias,
    TypeAlias,
    Function,
    Method,
    Constant,
    Static,
    Macro,
    ProcMacro,
    AssocType,
    AssocConst,
    Primitive,
    ExternCrate,
    Import,
}

impl SymbolKind {
    /// The wire spelling.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Struct => "struct",
            Self::Union => "union",
            Self::Enum => "enum",
            Self::Variant => "variant",
            Self::StructField => "struct_field",
            Self::Trait => "trait",
            Self::TraitAlias => "trait_alias",
            Self::TypeAlias => "type_alias",
            Self::Function => "function",
            Self::Method => "method",
            Self::Constant => "constant",
            Self::Static => "static",
            Self::Macro => "macro",
            Self::ProcMacro => "proc_macro",
            Self::AssocType => "assoc_type",
            Self::AssocConst => "assoc_const",
            Self::Primitive => "primitive",
            Self::ExternCrate => "extern_crate",
            Self::Import => "import",
        }
    }

    /// Parse the wire spelling.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        [
            Self::Module,
            Self::Struct,
            Self::Union,
            Self::Enum,
            Self::Variant,
            Self::StructField,
            Self::Trait,
            Self::TraitAlias,
            Self::TypeAlias,
            Self::Function,
            Self::Method,
            Self::Constant,
            Self::Static,
            Self::Macro,
            Self::ProcMacro,
            Self::AssocType,
            Self::AssocConst,
            Self::Primitive,
            Self::ExternCrate,
            Self::Import,
        ]
        .into_iter()
        .find(|k| k.as_str() == text)
    }
}

/// A deprecation notice as the producer recorded it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Deprecated {
    /// The `since` value, if given.
    pub since: Option<String>,
    /// The note, if given.
    pub note: Option<String>,
}

/// One public symbol at one public path (§6.1).
///
/// A re-export is a symbol whose `path` differs from its `definition_path`; both share one
/// `definition_id`, which is how an overview counts capabilities once (gate R07).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Symbol {
    /// Identity of this public path: hash of crate, path, kind and (for trait-impl members)
    /// the trait.
    pub symbol_id: String,
    /// Identity of the definition this path names.
    pub definition_id: String,
    /// Canonical public path, e.g. `enr_fixture::inner::Widget`.
    pub path: String,
    /// The last path segment.
    pub name: String,
    /// What kind of item this is.
    pub kind: SymbolKind,
    /// The containing path, when any.
    pub parent_path: Option<String>,
    /// Rendered signature, when the producer rendered one.
    pub signature: Option<String>,
    /// First paragraph of the documentation, bounded.
    pub doc_summary: Option<String>,
    /// Full documentation text.
    pub docs: Option<String>,
    /// Deprecation, when marked.
    pub deprecated: Option<Deprecated>,
    /// Source file as the producer recorded it.
    pub span_file: Option<String>,
    /// 1-based source line as the producer recorded it.
    pub span_line: Option<u32>,
    /// Whether this path is a re-export of a definition elsewhere.
    pub is_reexport: bool,
    /// The path the item is defined at.
    pub definition_path: String,
    /// The crate the definition belongs to.
    pub defined_in_crate: String,
    /// The producer's own item identifier -- local, never a cross-release identity.
    pub producer_local_id: u32,
    /// `cfg`-shaped attribute strings the producer preserved, as declared hints only. Never
    /// a feature predicate: items compiled out are absent, not annotated (§4.3).
    pub cfg_hints: Vec<String>,
}

impl Symbol {
    /// Derive a symbol identity.
    #[must_use]
    pub fn symbol_id_for(
        crate_name: &str,
        path: &str,
        kind: SymbolKind,
        qualifier: Option<&str>,
    ) -> String {
        canonical::short_id(
            "sym",
            &serde_json::json!({
                "crate": crate_name, "path": path, "kind": kind, "qualifier": qualifier
            }),
        )
    }

    /// Derive a definition identity.
    #[must_use]
    pub fn definition_id_for(
        crate_name: &str,
        definition_path: &str,
        kind: SymbolKind,
        qualifier: Option<&str>,
    ) -> String {
        canonical::short_id(
            "def",
            &serde_json::json!({
                "crate": crate_name, "path": definition_path, "kind": kind, "qualifier": qualifier
            }),
        )
    }
}

/// Typed relations between symbols (§6.1).
///
/// The values are, in order: `reexports`, `implements`, `member_of`, `documents`, `returns`,
/// `accepts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RelationKind {
    Reexports,
    Implements,
    MemberOf,
    Documents,
    Returns,
    Accepts,
}

impl RelationKind {
    /// The wire spelling.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reexports => "reexports",
            Self::Implements => "implements",
            Self::MemberOf => "member_of",
            Self::Documents => "documents",
            Self::Returns => "returns",
            Self::Accepts => "accepts",
        }
    }

    /// Parse the wire spelling.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        [
            Self::Reexports,
            Self::Implements,
            Self::MemberOf,
            Self::Documents,
            Self::Returns,
            Self::Accepts,
        ]
        .into_iter()
        .find(|k| k.as_str() == text)
    }
}

/// One typed edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Relationship {
    /// Content-derived identity.
    pub relationship_id: String,
    /// The source symbol.
    pub source_id: String,
    /// The source path, for display without a join.
    pub source_path: String,
    /// The target symbol or definition, when it is in this snapshot.
    pub target_id: Option<String>,
    /// The target path as written, even when the target is external.
    pub target_path: String,
    /// The relation.
    pub relation: RelationKind,
    /// A qualifier such as `blanket`, `auto` or `trait_impl`.
    pub detail: Option<String>,
    /// Which producer emitted it.
    pub producer: String,
}

impl Relationship {
    /// Build an edge with a content-derived identity.
    #[must_use]
    pub fn new(
        source_id: &str,
        source_path: &str,
        target_id: Option<&str>,
        target_path: &str,
        relation: RelationKind,
        detail: Option<&str>,
        producer: &str,
    ) -> Self {
        let relationship_id = canonical::short_id(
            "rel8",
            &serde_json::json!({
                "source": source_id, "target": target_id, "target_path": target_path,
                "relation": relation, "detail": detail
            }),
        );
        Self {
            relationship_id,
            source_id: source_id.to_owned(),
            source_path: source_path.to_owned(),
            target_id: target_id.map(str::to_owned),
            target_path: target_path.to_owned(),
            relation,
            detail: detail.map(str::to_owned),
            producer: producer.to_owned(),
        }
    }
}

/// What kind of text a fragment is.
///
/// The values are, in order: `api_signature`, `doc_text`, `feature_definition`,
/// `readme_section`, `changelog_section`, `example`, `source_excerpt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum FragmentKind {
    ApiSignature,
    DocText,
    FeatureDefinition,
    ReadmeSection,
    ChangelogSection,
    Example,
    SourceExcerpt,
}

impl FragmentKind {
    /// The wire spelling.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApiSignature => "api_signature",
            Self::DocText => "doc_text",
            Self::FeatureDefinition => "feature_definition",
            Self::ReadmeSection => "readme_section",
            Self::ChangelogSection => "changelog_section",
            Self::Example => "example",
            Self::SourceExcerpt => "source_excerpt",
        }
    }

    /// Parse the wire spelling.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        [
            Self::ApiSignature,
            Self::DocText,
            Self::FeatureDefinition,
            Self::ReadmeSection,
            Self::ChangelogSection,
            Self::Example,
            Self::SourceExcerpt,
        ]
        .into_iter()
        .find(|k| k.as_str() == text)
    }

    /// The `search_evidence` kind family this fragment belongs to.
    #[must_use]
    pub fn family(self) -> &'static str {
        match self {
            Self::ApiSignature => "api",
            Self::DocText => "docs",
            Self::FeatureDefinition => "features",
            Self::ReadmeSection => "docs",
            Self::ChangelogSection => "release_notes",
            Self::Example => "examples",
            Self::SourceExcerpt => "source",
        }
    }
}

/// A bounded extract with its locator and epistemic class (§6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceFragment {
    /// Content-derived identity.
    pub fragment_id: String,
    /// What kind of text this is.
    pub kind: FragmentKind,
    /// What it is about: a symbol path, a feature name, a heading.
    pub subject: String,
    /// The artifact it was read from.
    pub artifact_id: String,
    /// Position within the artifact, as a JSON object.
    pub locator: serde_json::Map<String, serde_json::Value>,
    /// The text.
    pub text: String,
    /// Which epistemic class it belongs to.
    pub evidence_class: EvidenceClass,
    /// Which producer emitted it.
    pub producer: String,
    /// The producer's exact version.
    pub producer_version: String,
}

impl EvidenceFragment {
    /// Build a fragment with a content-derived identity.
    #[must_use]
    pub fn new(
        kind: FragmentKind,
        subject: &str,
        artifact_id: &str,
        locator: serde_json::Value,
        text: String,
        evidence_class: EvidenceClass,
        producer: &str,
        producer_version: &str,
    ) -> Self {
        let fragment_id = canonical::short_id(
            "frag",
            &serde_json::json!({
                "kind": kind, "subject": subject, "artifact": artifact_id, "locator": locator
            }),
        );
        Self {
            fragment_id,
            kind,
            subject: subject.to_owned(),
            artifact_id: artifact_id.to_owned(),
            locator: locator.as_object().cloned().unwrap_or_default(),
            text,
            evidence_class,
            producer: producer.to_owned(),
            producer_version: producer_version.to_owned(),
        }
    }
}

/// The configuration a documentation build was observed under (§4.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ObservedConfiguration {
    /// Features the docs build enabled, as the manifest declared them.
    pub features: Vec<String>,
    /// Whether all features were enabled.
    pub all_features: bool,
    /// Whether default features were disabled.
    pub no_default_features: bool,
    /// The target the JSON itself declares.
    pub target: String,
    /// The rustdoc JSON format version.
    pub format_version: u32,
    /// Where the configuration was read from.
    pub source: String,
}

/// What a caller declared about its own environment, for comparison.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RequestedConfiguration {
    /// Features the project enables, when declared.
    pub features: Option<Vec<String>>,
    /// Whether default features are enabled, when declared.
    pub default_features: Option<bool>,
    /// The project's target, when declared.
    pub target: Option<String>,
}

/// Availability is several separate states, never one boolean (§4.3).
///
/// The values are, in order: `documented_available`, `project_availability_unverified`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum AvailabilityStatus {
    DocumentedAvailable,
    ProjectAvailabilityUnverified,
}

/// What can and cannot be said about a symbol's availability to a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Availability {
    /// The strongest honest claim.
    pub status: AvailabilityStatus,
    /// The documented build's configuration.
    pub observed_configuration: ObservedConfiguration,
    /// The caller's declared configuration.
    pub requested_configuration: RequestedConfiguration,
    /// Why the status is what it is, in prose.
    pub notes: Vec<String>,
}

impl Availability {
    /// Compare an observed documentation build with a declared project configuration.
    ///
    /// Presence in the documented build establishes `documented_available`. It never
    /// establishes project availability: a target difference, a feature difference, or an
    /// undeclared environment each leave the project claim unverified, and no per-symbol
    /// feature predicate is invented from documentation annotations.
    #[must_use]
    pub fn assess(
        observed: ObservedConfiguration,
        requested: RequestedConfiguration,
        cfg_hints: &[String],
    ) -> Self {
        let mut notes = Vec::new();
        if let Some(target) = &requested.target
            && target != &observed.target
        {
            notes.push(format!(
                "The documentation was built for {}; the project targets {target}. Target-gated \
                 items may differ.",
                observed.target
            ));
        }
        if observed.all_features {
            notes.push(
                "The documentation build enabled all features; a project with a narrower \
                 feature set may not have every documented item."
                    .to_owned(),
            );
        }
        if let Some(features) = &requested.features {
            let extra: Vec<&String> = observed
                .features
                .iter()
                .filter(|f| !features.contains(f))
                .collect();
            if !extra.is_empty() {
                notes.push(format!(
                    "The documentation build enabled features the project does not: {}.",
                    extra
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        if requested.features.is_none() && requested.target.is_none() {
            notes.push(
                "No project environment was declared, so availability in the project is \
                 unverified."
                    .to_owned(),
            );
        }
        if !cfg_hints.is_empty() {
            notes.push(format!(
                "The item carries declared cfg hints ({}); these are documentation \
                 annotations, not a verified feature predicate.",
                cfg_hints.join("; ")
            ));
        }
        Self {
            status: AvailabilityStatus::ProjectAvailabilityUnverified,
            observed_configuration: observed,
            requested_configuration: requested,
            notes,
        }
    }
}

/// A table stored in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TableRef {
    /// File name within the snapshot directory.
    pub file: String,
    /// Row count at publication.
    pub rows: u64,
}

/// Counts an overview and a manifest report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SnapshotCounts {
    /// Public paths.
    pub symbols: u64,
    /// Distinct definitions.
    pub definitions: u64,
    /// Paths that re-export a definition reachable elsewhere.
    pub reexports: u64,
    /// Re-exports whose target is outside this crate.
    pub unresolved_reexports: u64,
    /// Typed edges.
    pub relationships: u64,
    /// Fragments.
    pub fragments: u64,
    /// Items the producer saw in total, including those not surfaced as symbols.
    pub producer_items: u64,
}

/// The immutable description of one snapshot (§6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SnapshotManifest {
    /// The snapshot's identity.
    pub snapshot_id: SnapshotId,
    /// Wire schema version at publication.
    pub schema_version: String,
    /// Normalizer version at publication.
    pub normalizer_version: String,
    /// The context this snapshot serves.
    pub context_id: ContextId,
    /// The release.
    pub release_id: ReleaseId,
    /// The environment.
    pub environment_id: EnvironmentId,
    /// The crate name as rustdoc knows it.
    pub crate_name: String,
    /// The crate version the JSON declared.
    pub crate_version: Option<String>,
    /// Input artifact digests by role.
    pub inputs: BTreeMap<String, String>,
    /// Producer identities by name.
    pub producers: BTreeMap<String, String>,
    /// Every producer run that fed this snapshot.
    pub producer_runs: Vec<ProducerRun>,
    /// Tables and their row counts.
    pub tables: BTreeMap<String, TableRef>,
    /// Counts.
    pub counts: SnapshotCounts,
    /// The documentation build's configuration.
    pub observed_configuration: ObservedConfiguration,
    /// Evidence kinds this snapshot contains.
    pub indexed: Vec<EvidenceKind>,
    /// Evidence kinds that were expected and are absent.
    pub missing: Vec<EvidenceKind>,
    /// Publication time, RFC 3339. Provenance only.
    pub published_at: String,
}
