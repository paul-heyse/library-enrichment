//! Coverage, freshness, evidence fragments and artifact handles (blueprint §6.2, §7.2).

use std::collections::BTreeSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::JsonObject;
use super::ids::ArtifactUri;

/// What was actually looked at, and what was not.
///
/// `ok` means successful within this scope, never complete knowledge of a library. An empty
/// `search_evidence` result with `indexed: ["public_api"]` is a valid negative answer; the same
/// result with `missing: ["public_api"]` is an evidence gap. The two must never collapse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    /// What the result claims to cover, in prose.
    pub scope: String,
    /// Evidence kinds that were successfully indexed.
    pub indexed: BTreeSet<String>,
    /// Evidence kinds that were expected but are absent.
    pub missing: BTreeSet<String>,
    /// Known reasons this result may not generalize.
    pub limitations: Vec<String>,
}

/// How well the evidence's source version matches the version that was asked about.
///
/// The values are, in order: `exact`, `compatible_claimed`, `mismatched`, `unknown`.
/// `compatible_claimed` is a claim by the source, not a verified fact.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum SourceVersionMatch {
    Exact,
    CompatibleClaimed,
    Mismatched,
    Unknown,
}

/// Registry freshness. Blueprint §3.3: freshness is not one timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Freshness {
    /// When the registry was last consulted, or `null` if it was not consulted at all.
    #[schemars(extend("format" = "date-time"))]
    pub registry_checked_at: Option<String>,
    /// How the evidence's source version relates to the requested one.
    pub source_version_match: SourceVersionMatch,
    /// Whether "this is the latest release" was actually revalidated. A cache hit alone is not
    /// evidence that a release is still latest.
    pub latest_verified: bool,
}

/// The six epistemic classes (blueprint §6.2).
///
/// These are categories, not a confidence scale. An API signature can be `compiler_derived`
/// while "this replaces our orchestration layer" is `agent_inferred`, and the two never merge.
///
/// The values are, in order: `declared`, `statically_extracted`, `compiler_derived`,
/// `typechecker_observed`, `runtime_observed`, `agent_inferred`.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum EvidenceClass {
    Declared,
    StaticallyExtracted,
    CompilerDerived,
    TypecheckerObserved,
    RuntimeObserved,
    AgentInferred,
}

/// One supporting fact, with the provenance needed to check it.
///
/// Blueprint §7.2: every entry carries an ID, class, subject, exact locator, source-version
/// match and producer, plus a compact excerpt -- not an unexplained numeric confidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    /// Stable handle for citation.
    pub evidence_id: String,
    /// Which epistemic class this fact belongs to.
    pub evidence_class: EvidenceClass,
    /// What the fact is about -- a symbol path, a setting, a release.
    pub subject: String,
    /// The artifact this was read out of.
    pub artifact_id: String,
    /// Where the artifact came from.
    #[schemars(extend("format" = "uri"))]
    pub source_uri: String,
    /// Position within the artifact. Specialized as a discriminated union per producer.
    pub locator: JsonObject,
    /// How the source's version relates to the requested one.
    pub source_version_match: SourceVersionMatch,
    /// Which producer emitted this.
    pub producer: String,
    /// The exact producer version, for reproducibility.
    pub producer_version: String,
    /// A compact supporting quote.
    pub excerpt: String,
}

/// A pointer to a bounded, readable artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactHandle {
    /// Stable handle, usable with `read_artifact`.
    pub artifact_id: String,
    /// Always a `library-evidence://` URI -- enforced by [`ArtifactUri`], not only by the schema.
    pub uri: ArtifactUri,
    /// The artifact's media type.
    pub media_type: String,
    /// What the artifact contains, so a caller can decide whether to read it.
    pub description: String,
}
