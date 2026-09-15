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
    let catalog = service
        .repository
        .catalog
        .pin()
        .await
        .map_err(|e| Box::new(store_error(&e)))?;
    open_context_at(service, catalog, context_id, snapshot_id).await
}

/// Bind every part of a multi-snapshot operation to the same coherent catalog generation.
pub async fn open_context_at(
    service: &Service,
    catalog: std::sync::Arc<enrichment_store::catalog_generation::PinnedCatalog>,
    context_id: &str,
    snapshot_id: Option<&str>,
) -> Result<Opened, Box<Envelope>> {
    let id = ContextId::try_from(context_id.to_owned()).map_err(|_| {
        Box::new(envelope::error(
            ErrorCode::ArtifactUnavailable,
            "Invalid context identity",
            "Use the context_id returned by resolve_library.",
            false,
        ))
    })?;
    let runtime = &service.repository.runtime;
    let (context, environment) = catalog
        .context(runtime, &id)
        .await
        .map_err(|e| Box::new(store_error(&e)))?
        .ok_or_else(|| {
            Box::new(envelope::error(
                ErrorCode::ArtifactUnavailable,
                "Context is not in the committed catalog",
                "Resolve this library and environment first.",
                false,
            ))
        })?;
    let release = catalog
        .release(runtime, &context.release_id)
        .await
        .map_err(|e| Box::new(store_error(&e)))?
        .ok_or_else(|| {
            Box::new(envelope::error(
                ErrorCode::ExtractionFailed,
                "Context release is missing",
                "Check service storage integrity.",
                false,
            ))
        })?;
    let snapshot_id = match snapshot_id {
        Some(value) => SnapshotId::try_from(value.to_owned()).map_err(|_| {
            Box::new(envelope::error(
                ErrorCode::ArtifactUnavailable,
                "Invalid snapshot identity",
                "Use a snapshot_id returned by this service.",
                false,
            ))
        })?,
        None => catalog
            .current(runtime, &id)
            .await
            .map_err(|e| Box::new(store_error(&e)))?
            .ok_or_else(|| {
                Box::new(envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    "No snapshot is selected for this context",
                    "Resolve this library and read its declared gaps.",
                    false,
                ))
            })?,
    };
    let reader = SnapshotReader::open(&service.repository, catalog, &snapshot_id)
        .await
        .map_err(|e| Box::new(query_error(&e)))?;
    if reader.manifest().context_id != id {
        return Err(Box::new(envelope::error(
            ErrorCode::ArtifactUnavailable,
            "Snapshot belongs to a different context",
            "Use a snapshot returned for this exact context.",
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
pub fn store_error(err: &impl std::fmt::Display) -> Envelope {
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
        if err.is_budget() {
            ErrorCode::BudgetExceeded
        } else if matches!(err, enrichment_store::QueryError::NotPublished(_)) {
            ErrorCode::ArtifactUnavailable
        } else {
            ErrorCode::ExtractionFailed
        },
        format!("snapshot query failed: {err}"),
        "Refine the requested scope or inspect the configured query budgets and storage integrity.",
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
    _service: &Service,
    fragment: &EvidenceFragment,
    source_version_match: SourceVersionMatch,
    excerpt_chars: usize,
) -> Evidence {
    let source_uri = fragment
        .source_uri
        .clone()
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
        source_version_match: if fragment.source_uri.is_none() {
            SourceVersionMatch::Unknown
        } else {
            fragment
                .source_version_match
                .unwrap_or(source_version_match)
        },
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
    let configured = service.config.limits.inline_result_bytes.max(1024);
    requested.map_or(configured, |r| r.clamp(1024, configured))
}

/// Serialized size of a value, for budget accounting.
#[must_use]
pub fn json_size<T: serde::Serialize>(value: &T) -> usize {
    crate::delivery::size(value, crate::delivery::MAX_RESULT_BYTES).unwrap_or(usize::MAX)
}

/// A JSON object from any serializable payload.
#[must_use]
pub fn to_object<T: serde::Serialize>(value: &T) -> enrichment_core::wire::JsonObject {
    let serde_json::Value::Object(object) =
        serde_json::to_value(value).expect("Rust wire payload must serialize")
    else {
        panic!("tool data must be a JSON object");
    };
    object
}

/// Enforce the complete serialized envelope budget. Oversized answers remain available as
/// immutable JSON artifacts; only the per-call request ID is excluded from reusable content.
pub fn enforce_budget(service: &Service, result: Envelope, requested: Option<usize>) -> Envelope {
    crate::delivery::encode(
        &service.blobs,
        result,
        byte_budget(service, requested),
        true,
    )
    .unwrap_or_else(|error| {
        envelope::error(
            ErrorCode::BudgetExceeded,
            format!("Answer delivery failed: {error}"),
            "Inspect service storage and request a bounded evidence selection.",
            false,
        )
    })
}
