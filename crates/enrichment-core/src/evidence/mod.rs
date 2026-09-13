//! The canonical evidence model (blueprint §6): a small typed model, not an ontology.
//!
//! Stable enums and typed relationships cover observed evidence; free text is for excerpts and
//! explanations, never for essential machine state. Everything here is Rust-owned; the Python
//! boundary sees it only through the generated schemas.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::canonical;

pub mod model;

pub use model::{
    Availability, AvailabilityStatus, Deprecated, EvidenceFragment, FragmentKind,
    ObservedConfiguration, RelationKind, Relationship, RequestedConfiguration, SnapshotCounts,
    SnapshotManifest, Symbol, SymbolKind, TableRef,
};

/// The kinds of evidence a producer can require or yield, and that `coverage.indexed` and
/// `coverage.missing` name.
///
/// The values are, in order: `registry_metadata`, `crate_source`,
/// `documentation_build_config`, `hosted_rustdoc_json`, `public_api`, `documentation`,
/// `examples`, `release_notes`, `source_excerpts`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum EvidenceKind {
    RegistryMetadata,
    CrateSource,
    DocumentationBuildConfig,
    HostedRustdocJson,
    PublicApi,
    Documentation,
    Examples,
    ReleaseNotes,
    SourceExcerpts,
}

impl EvidenceKind {
    /// The `coverage` spelling of this kind.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RegistryMetadata => "registry_metadata",
            Self::CrateSource => "crate_source",
            Self::DocumentationBuildConfig => "documentation_build_config",
            Self::HostedRustdocJson => "hosted_rustdoc_json",
            Self::PublicApi => "public_api",
            Self::Documentation => "documentation",
            Self::Examples => "examples",
            Self::ReleaseNotes => "release_notes",
            Self::SourceExcerpts => "source_excerpts",
        }
    }
}

/// Why an expected kind of evidence is absent. Each reason is a distinct fact (§10): a
/// network failure, a missing upstream document, an unsupported format and a policy refusal
/// call for different next actions.
///
/// The values are, in order: `hosted_json_missing`, `hosted_json_unsupported`,
/// `upstream_unavailable`, `policy_denied`, `extraction_failed`, `not_attempted`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum GapReason {
    HostedJsonMissing,
    HostedJsonUnsupported,
    UpstreamUnavailable,
    PolicyDenied,
    ExtractionFailed,
    NotAttempted,
}

/// One piece of expected evidence that is absent, with why and what would supply it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Gap {
    /// What is missing.
    pub kind: EvidenceKind,
    /// Why.
    pub reason: GapReason,
    /// Human-readable detail, e.g. the format version that was refused.
    pub detail: String,
    /// The producer and profile that would supply it, when one exists.
    pub planned_fallback: Option<PlannedFallback>,
}

/// A producer that could supply missing evidence, and whether policy currently allows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PlannedFallback {
    /// Producer name.
    pub producer: String,
    /// The execution profile it needs.
    pub profile: String,
    /// Whether configuration currently enables that profile.
    pub enabled: bool,
    /// What a caller or operator would do to make it run.
    pub next_action: String,
}

/// What kind of upstream object an artifact is.
///
/// The values are, in order: `registry_index_entry`, `registry_version_metadata`,
/// `crate_tarball`, `rustdoc_json`, `cargo_manifest`, `readme`, `changelog`, `source_file`,
/// `other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ArtifactKind {
    RegistryIndexEntry,
    RegistryVersionMetadata,
    CrateTarball,
    RustdocJson,
    CargoManifest,
    Readme,
    Changelog,
    SourceFile,
    Other,
}

/// An immutable, content-addressed artifact (§6.1).
///
/// The digest is the identity; everything else is provenance. Two retrievals of identical
/// bytes share one artifact, and the first retrieval's provenance is the one kept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Artifact {
    /// `art_<32 hex>`, derived from the digest; the handle callers pass to `read_artifact`.
    pub artifact_id: String,
    /// Full SHA-256 of the stored bytes, lower-case hex.
    pub sha256: String,
    /// Media type of the stored bytes (after any transport decompression).
    pub media_type: String,
    /// Size of the stored bytes.
    pub size_bytes: u64,
    /// What the artifact is.
    pub kind: ArtifactKind,
    /// Where it was requested from.
    pub source_uri: String,
    /// Where it was actually served from after redirects, when different.
    pub final_url: Option<String>,
    /// When it was retrieved, RFC 3339. Provenance only; never part of any identity.
    pub retrieved_at: String,
    /// HTTP validator, when the server supplied one.
    pub etag: Option<String>,
    /// HTTP validator, when the server supplied one.
    pub last_modified: Option<String>,
    /// Transport compression that was removed before storage, e.g. `zstd`.
    pub compression: Option<String>,
}

/// The number of hex digits of the digest kept in an artifact identity.
pub const ARTIFACT_ID_HEX_DIGITS: usize = 32;

/// Derive the artifact handle from a full SHA-256 hex digest.
#[must_use]
pub fn artifact_id_for(sha256_hex: &str) -> String {
    format!(
        "art_{}",
        &sha256_hex[..ARTIFACT_ID_HEX_DIGITS.min(sha256_hex.len())]
    )
}

/// Whether a string is a well-formed artifact handle.
#[must_use]
pub fn is_artifact_id(value: &str) -> bool {
    value.strip_prefix("art_").is_some_and(|hex| {
        hex.len() == ARTIFACT_ID_HEX_DIGITS && hex.bytes().all(|b| b.is_ascii_hexdigit())
    })
}

impl Artifact {
    /// Describe bytes that are about to be stored.
    ///
    /// `retrieved_at` is supplied by the caller so the record can carry the real retrieval time
    /// without this module reading a clock, which keeps it trivially testable.
    #[must_use]
    pub fn describe(
        bytes: &[u8],
        kind: ArtifactKind,
        media_type: &str,
        source_uri: &str,
        retrieved_at: &str,
    ) -> Self {
        let sha256 = canonical::sha256_hex(bytes);
        Self {
            artifact_id: artifact_id_for(&sha256),
            sha256,
            media_type: media_type.to_owned(),
            size_bytes: bytes.len() as u64,
            kind,
            source_uri: source_uri.to_owned(),
            final_url: None,
            retrieved_at: retrieved_at.to_owned(),
            etag: None,
            last_modified: None,
            compression: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_artifact_id_is_derived_from_the_digest() {
        let artifact = Artifact::describe(b"abc", ArtifactKind::Other, "text/plain", "x://y", "t");
        assert_eq!(
            artifact.sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(artifact.artifact_id, "art_ba7816bf8f01cfea414140de5dae2223");
        assert!(is_artifact_id(&artifact.artifact_id));
        assert!(!is_artifact_id("art_short"));
        assert!(!is_artifact_id("/etc/passwd"));
    }
}
