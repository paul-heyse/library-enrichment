//! What the retrieval operations share: opening a context's snapshot, building evidence
//! entries and artifact handles, and the byte budget.

use enrichment_core::evidence::{Artifact, EvidenceFragment};
use enrichment_core::identity::{Context, ContextId, Environment, Release, SnapshotId};
use enrichment_core::wire::{ArtifactHandle, Envelope, ErrorCode, Evidence, SourceVersionMatch};
use enrichment_store::SnapshotReader;

use crate::envelope;
use crate::service::Service;

/// A context with its published snapshot open for reading.
pub struct Opened {
    /// The context.
    pub context: Context,
    /// Its environment.
    pub environment: Environment,
    /// The release.
    pub release: Release,
    /// The snapshot being read.
    pub snapshot_id: SnapshotId,
    /// The reader.
    pub reader: SnapshotReader,
}

/// Open the snapshot a request names, or the context's current one.
///
/// # Errors
///
/// Returns a complete error envelope: an unknown context, an unpublished snapshot, or an
/// unreadable one each name a next action. A context whose resolution found no usable rustdoc
/// JSON has no snapshot, and the error says so rather than pretending an empty index.
pub async fn open_context(
    service: &Service,
    context_id: &str,
    snapshot_id: Option<&str>,
) -> Result<Opened, Box<Envelope>> {
    let Ok(id) = ContextId::try_from(context_id.to_owned()) else {
        return Err(Box::new(envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!("`{context_id}` is not a context identity"),
            "Pass the `context_id` returned by `resolve_library`.",
            false,
        )));
    };
    let (context, environment) = match service.catalog.context(&id) {
        Ok(Some(pair)) => pair,
        Ok(None) => {
            return Err(Box::new(envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("context {context_id} is not known to this service"),
                "Call `resolve_library` first; contexts are minted there.",
                false,
            )));
        }
        Err(err) => return Err(Box::new(store_error(&err))),
    };
    let release = match service.catalog.release(&context.release_id) {
        Ok(Some(release)) => release,
        Ok(None) => {
            return Err(Box::new(envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("release record for {context_id} is missing"),
                "Resolve the release again with freshness=revalidate.",
                true,
            )));
        }
        Err(err) => return Err(Box::new(store_error(&err))),
    };

    let snapshot_id = match snapshot_id {
        Some(requested) => match SnapshotId::try_from(requested.to_owned()) {
            Ok(id) => id,
            Err(_) => {
                return Err(Box::new(envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    format!("`{requested}` is not a snapshot identity"),
                    "Pass a `snapshot_id` from an earlier result, or omit it.",
                    false,
                )));
            }
        },
        None => match service.catalog.current_snapshot(&id) {
            Ok(Some(current)) => current,
            Ok(None) => {
                return Err(Box::new(envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    format!(
                        "no snapshot is published for {context_id}: its resolution found no \
                         rustdoc JSON this build can read"
                    ),
                    "Read the `gaps` in the `resolve_library` result; the planned fallback \
                     names what would supply the API. Registry and source evidence are still \
                     available through `read_artifact`.",
                    false,
                )));
            }
            Err(err) => return Err(Box::new(store_error(&err))),
        },
    };

    let reader = match SnapshotReader::open(&service.paths, &snapshot_id).await {
        Ok(reader) => reader,
        Err(enrichment_store::QueryError::NotPublished(_)) => {
            return Err(Box::new(envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("snapshot {snapshot_id} is not published"),
                "Omit `snapshot_id` to read the context's current snapshot.",
                false,
            )));
        }
        Err(err) => {
            return Err(Box::new(envelope::error(
                ErrorCode::ExtractionFailed,
                format!("snapshot {snapshot_id} could not be opened: {err}"),
                "Resolve the release again to republish; if it persists, report it with the \
                 daemon log.",
                true,
            )));
        }
    };
    if reader.manifest().context_id != id {
        return Err(Box::new(envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!("snapshot {snapshot_id} belongs to a different context"),
            "Use a snapshot returned for this context, or omit `snapshot_id`.",
            false,
        )));
    }

    Ok(Opened {
        context,
        environment,
        release,
        snapshot_id,
        reader,
    })
}

/// A store failure as an envelope.
#[must_use]
pub fn store_error(err: &std::io::Error) -> Envelope {
    envelope::error(
        ErrorCode::ArtifactUnavailable,
        format!("service state could not be read: {err}"),
        "Check that the service data directory is readable; see `library-enrichmentd status`.",
        true,
    )
}

/// A query failure as an envelope.
#[must_use]
pub fn query_error(err: &enrichment_store::QueryError) -> Envelope {
    envelope::error(
        ErrorCode::ExtractionFailed,
        format!("snapshot query failed: {err}"),
        "Retry; if it persists, resolve the release again to republish the snapshot.",
        true,
    )
}

/// The URI an artifact is cited by.
#[must_use]
pub fn artifact_uri_for(artifact_id: &str) -> String {
    format!("library-evidence://artifacts/{artifact_id}")
}

/// The readable handle for an artifact record.
#[must_use]
pub fn handle_for(artifact: &Artifact, description: String) -> Option<ArtifactHandle> {
    envelope::artifact_uri(&format!("artifacts/{}", artifact.artifact_id))
        .ok()
        .map(|uri| ArtifactHandle {
            artifact_id: artifact.artifact_id.clone(),
            uri,
            media_type: artifact.media_type.clone(),
            description,
        })
}

/// Cite a fragment as an evidence entry, with a bounded excerpt.
#[must_use]
pub fn evidence_from_fragment(
    service: &Service,
    fragment: &EvidenceFragment,
    source_version_match: SourceVersionMatch,
    excerpt_chars: usize,
) -> Evidence {
    let source_uri = service
        .blobs
        .find(&fragment.artifact_id)
        .ok()
        .flatten()
        .map(|a| a.source_uri)
        .unwrap_or_else(|| artifact_uri_for(&fragment.artifact_id));
    Evidence {
        evidence_id: format!(
            "ev_{}",
            &fragment.fragment_id[5.min(fragment.fragment_id.len())..]
        ),
        evidence_class: fragment.evidence_class,
        subject: fragment.subject.clone(),
        artifact_id: fragment.artifact_id.clone(),
        source_uri,
        locator: fragment.locator.clone(),
        source_version_match,
        producer: fragment.producer.clone(),
        producer_version: fragment.producer_version.clone(),
        excerpt: truncate(&fragment.text, excerpt_chars),
    }
}

/// Cut text to `max_chars` characters, marking the cut.
#[must_use]
pub fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max_chars.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// The byte budget for one inline result: the caller's request bounded by configuration.
#[must_use]
pub fn byte_budget(service: &Service, requested: Option<usize>) -> usize {
    let configured = service.config.limits.inline_result_bytes;
    requested.map_or(configured, |r| r.clamp(1024, configured))
}

/// Serialized size of a value, for budget accounting.
#[must_use]
pub fn json_size<T: serde::Serialize>(value: &T) -> usize {
    serde_json::to_vec(value).map_or(0, |v| v.len())
}

/// A JSON object from any serializable payload.
#[must_use]
pub fn to_object<T: serde::Serialize>(value: &T) -> enrichment_core::wire::JsonObject {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
}
