//! The canonical evidence model (blueprint §6): a small typed model, not an ontology.
//!
//! Stable enums and typed relationships cover observed evidence; free text is for excerpts and
//! explanations, never for essential machine state. Everything here is Rust-owned; the Python
//! boundary sees it only through the generated schemas.

use crate::canonical;

pub mod catalog;
pub mod document;
pub mod execution;
pub mod ingest;
pub mod metadata;
pub mod model;
pub mod path;
pub mod relational;
pub mod snapshot;
pub mod text;

pub use relational::TextFragment;

pub use model::{
    Availability, AvailabilityStatus, Deprecated, FragmentKind, ObservedConfiguration,
    RelationKind, RequestedConfiguration, SnapshotCounts, SymbolHeader, SymbolKind,
};

crate::native_vocabulary! {
/// The kinds of evidence a producer can require or yield, and that `coverage.indexed` and
/// `coverage.missing` name.
///
/// The values are, in order: `registry_metadata`, `crate_source`,
/// `documentation_build_config`, `hosted_rustdoc_json`, `public_api`, `documentation`,
/// `examples`, `release_notes`, `source_excerpts`.
    #[derive(Hash, PartialOrd, Ord)]
    #[schemars(inline)]
    pub enum EvidenceKind {
        RegistryMetadata = "registry_metadata",
        CrateSource = "crate_source",
        DocumentationBuildConfig = "documentation_build_config",
        HostedRustdocJson = "hosted_rustdoc_json",
        PublicApi = "public_api",
        Documentation = "documentation",
        Examples = "examples",
        ReleaseNotes = "release_notes",
        SourceExcerpts = "source_excerpts",
        DistributionSource = "distribution_source",
        Stubs = "stubs",
        Inventory = "inventory",
        RuntimeApi = "runtime_api",
        SemanticQueries = "semantic_queries",
        UsageProbes = "usage_probes",
    }
}

crate::native_vocabulary! {
/// Why an expected kind of evidence is absent. Each reason is a distinct fact (§10): a
/// network failure, a missing upstream document, an unsupported format and a policy refusal
/// call for different next actions.
///
/// The values are, in order: `hosted_json_missing`, `hosted_json_unsupported`,
/// `upstream_unavailable`, `policy_denied`, `extraction_failed`, `not_attempted`.
    #[derive(Hash)]
    #[schemars(inline)]
    pub enum GapReason {
        HostedJsonMissing = "hosted_json_missing",
        HostedJsonUnsupported = "hosted_json_unsupported",
        UpstreamUnavailable = "upstream_unavailable",
        PolicyDenied = "policy_denied",
        ExtractionFailed = "extraction_failed",
        NotAttempted = "not_attempted",
    }
}

crate::native_struct! {
/// One piece of expected evidence that is absent, with why and what would supply it.
    pub struct Gap {
        kind: EvidenceKind => crate::native_union::Rule::Text,
        reason: GapReason => crate::native_union::Rule::Text,
        detail: String => crate::native_union::Rule::Text,
        planned_fallback: Option<PlannedFallback> => crate::native_union::Rule::Text,
    }
}
crate::native_struct! {
    pub struct PlannedFallback {
        producer: String => crate::native_union::Rule::NonEmpty,
        profile: crate::policy::ExecutionProfile => crate::native_union::Rule::Text,
        enabled: bool => crate::native_union::Rule::Text,
        next_action: String => crate::native_union::Rule::NonEmpty,
    }
}

crate::native_vocabulary! {
/// What kind of upstream object an artifact is.
///
/// The values are, in order: `registry_index_entry`, `registry_version_metadata`,
/// `crate_tarball`, `rustdoc_json`, `cargo_manifest`, `readme`, `changelog`, `source_file`,
/// `other`.
#[derive(Hash)]
#[schemars(inline)]
pub enum ArtifactKind {
    RegistryIndexEntry = "registry_index_entry",
    RegistryVersionMetadata = "registry_version_metadata",
    CrateTarball = "crate_tarball",
    RustdocJson = "rustdoc_json",
    CargoManifest = "cargo_manifest",
    Readme = "readme",
    Changelog = "changelog",
    SourceFile = "source_file",
    Other = "other",
}
}

crate::native_struct! {
/// An immutable, content-addressed artifact (§6.1).
///
/// The digest identifies immutable bytes. Every acquisition keeps its own exact receipt,
/// including source locator and retrieval time, in the native Delta catalog. Evidence resolves
/// provenance through its selected attempt; byte delivery resolves the complete digest.
pub struct Artifact {
    /// `art_<64 hex>`, derived from the complete digest; used by `read_artifact`.
    artifact_id: String => crate::native_union::Rule::ArtifactIdentity { digest: "sha256".into() },
    /// Full SHA-256 of the stored bytes, lower-case hex.
    sha256: String => crate::native_union::Rule::Sha256,
    /// Media type of the stored bytes (after any transport decompression).
    media_type: String => crate::native_union::Rule::NonEmpty,
    /// Size of the stored bytes.
    size_bytes: u64 => crate::native_union::Rule::Text,
    /// What the artifact is.
    kind: ArtifactKind => crate::native_union::Rule::Text,
    /// Where it was requested from.
    source_uri: String => crate::native_union::Rule::NonEmpty,
    /// Where it was actually served from after redirects, when different.
    final_url: Option<String> => crate::native_union::Rule::Text,
    /// When it was retrieved, RFC 3339. Provenance only; never part of any identity.
    retrieved_at: crate::native_time::AcquisitionTime => crate::native_union::Rule::Text,
    /// HTTP validator, when the server supplied one.
    etag: Option<String> => crate::native_union::Rule::Text,
    /// HTTP validator, when the server supplied one.
    last_modified: Option<String> => crate::native_union::Rule::Text,
    /// Transport compression that was removed before storage, e.g. `zstd`.
    compression: Option<String> => crate::native_union::Rule::Text,
}
}

/// The number of hex digits of the digest kept in an artifact identity.
pub const ARTIFACT_ID_HEX_DIGITS: usize = 64;

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
        retrieved_at: crate::native_time::AcquisitionTime,
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
            retrieved_at,
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
        let artifact = Artifact::describe(
            b"abc",
            ArtifactKind::Other,
            "text/plain",
            "x://y",
            crate::native_time::AcquisitionTime::from_micros(1).unwrap(),
        );
        assert_eq!(
            artifact.sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            artifact.artifact_id,
            "art_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(is_artifact_id(&artifact.artifact_id));
        assert!(!is_artifact_id("art_short"));
        assert!(!is_artifact_id("/etc/passwd"));
    }
}

pub mod arrow_model;

pub mod declarations;
