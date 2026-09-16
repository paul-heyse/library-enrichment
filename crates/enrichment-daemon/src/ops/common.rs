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
        .map_err(|e| Box::new(operation_error(&e, "context_selection")))?;
    open_context_at(service, catalog, context_id, snapshot_id).await
}

/// Bind every part of a multi-snapshot operation to the same coherent catalog generation.
pub async fn open_context_at(
    service: &Service,
    catalog: std::sync::Arc<enrichment_store::control::ControlSnapshot>,
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
        .map_err(|e| Box::new(operation_error(&e, "context_selection")))?
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
        .map_err(|e| Box::new(operation_error(&e, "context_selection")))?
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
            .map_err(|e| Box::new(operation_error(&e, "context_selection")))?
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
pub fn operation_error(err: &(impl std::error::Error + 'static), stage: &str) -> Envelope {
    let Some(mut diagnostic) = enrichment_store::query_failure::diagnostic_from_error(err) else {
        let mut result = envelope::error(
            ErrorCode::InternalError,
            format!("service operation failed: {err}"),
            "Report the request correlation and native failure; no automatic retry is established.",
            false,
        );
        result.error_mut().expect("typed error").diagnostic.stage = stage.into();
        return result;
    };
    if diagnostic.rule.is_none() {
        diagnostic.stage = stage.into();
    }
    diagnostic_envelope(err, diagnostic)
}

/// Native failures preserve their origin and state whether repeating a request is useful.
#[must_use]
pub fn query_error(err: &enrichment_store::QueryError) -> Envelope {
    let diagnostic = err.diagnostic();
    diagnostic_envelope(err, diagnostic)
}

fn diagnostic_envelope(
    err: &impl std::fmt::Display,
    diagnostic: enrichment_core::wire::Diagnostic,
) -> Envelope {
    use enrichment_core::wire::{DiagnosticCause, RecoveryAction};
    let code = match diagnostic.cause {
        DiagnosticCause::Capacity | DiagnosticCause::Deadline => ErrorCode::BudgetExceeded,
        DiagnosticCause::NotFound => ErrorCode::ArtifactUnavailable,
        DiagnosticCause::InvalidInput => ErrorCode::UnsupportedFormat,
        DiagnosticCause::Unsupported => ErrorCode::UnsupportedCapability,
        _ => ErrorCode::QueryFailed,
    };
    let next = match &diagnostic.actions[0] {
        RecoveryAction::ChangeRequest { reason }
        | RecoveryAction::OperatorSetup { reason }
        | RecoveryAction::ReportDefect { reason } => reason.as_str(),
        _ => "Inspect the structured recovery action.",
    };
    let mut result = envelope::error(code, format!("native operation failed: {err}"), next, false);
    result.error_mut().expect("error outcome").diagnostic = diagnostic;
    result
}

#[must_use]
pub fn job_error(error: std::io::Error, job_id: &str) -> Envelope {
    let mut result = query_error(&error.into());
    let detail = result.error_mut().expect("error outcome");
    detail.diagnostic.stage = "job_lookup".into();
    detail.diagnostic.affected_ids.push(job_id.into());
    if detail.diagnostic.cause == enrichment_core::wire::DiagnosticCause::NotFound {
        detail.next_action = "Use the job_id returned by the original submission. An unknown ID is not a storage-permission failure.".into();
        detail.diagnostic.actions = vec![enrichment_core::wire::RecoveryAction::ChangeRequest {
            reason: detail.next_action.clone(),
        }];
    }
    result
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
            receipt: artifact.clone(),
            uri,
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
pub async fn enforce_budget(
    service: &Service,
    result: Envelope,
    requested: Option<usize>,
) -> Envelope {
    let result = match enrichment_store::runtime::charge_result(json_size(&result)) {
        Ok(()) => result,
        Err(error) => query_error(&error.into()),
    };
    crate::delivery::encode(
        &service.blobs,
        &service.repository.catalog,
        &service.repository.runtime,
        result,
        byte_budget(service, requested),
        requested,
    )
    .await
    .unwrap_or_else(|error| {
        if let Some(minimum) = error
            .get_ref()
            .and_then(|e| e.downcast_ref::<crate::delivery::MinimumBudget>())
        {
            return crate::delivery::budget_failure(
                requested,
                byte_budget(service, requested),
                minimum.minimum,
            );
        }
        let mut failure = query_error(&error.into());
        failure.error_mut().expect("error outcome").diagnostic.stage = "result_delivery".into();
        failure
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::wire::{DiagnosticCause, RecoveryAction};

    #[test]
    fn operation_failures_preserve_io_causes_and_stage_without_blanket_retry() {
        for (kind, cause) in [
            (std::io::ErrorKind::NotFound, DiagnosticCause::NotFound),
            (
                std::io::ErrorKind::PermissionDenied,
                DiagnosticCause::PermissionDenied,
            ),
            (
                std::io::ErrorKind::InvalidData,
                DiagnosticCause::CorruptState,
            ),
            (std::io::ErrorKind::StorageFull, DiagnosticCause::Capacity),
        ] {
            let result =
                operation_error(&std::io::Error::new(kind, "fixture origin"), "journal_read");
            let error = result.error().unwrap();
            assert_eq!(error.diagnostic.cause, cause);
            assert_eq!(error.diagnostic.stage, "journal_read");
            assert!(!error.retryable);
            if cause == DiagnosticCause::NotFound {
                assert!(matches!(
                    error.diagnostic.actions[0],
                    RecoveryAction::ChangeRequest { .. }
                ));
            } else if cause == DiagnosticCause::PermissionDenied {
                assert!(matches!(
                    error.diagnostic.actions[0],
                    RecoveryAction::OperatorSetup { .. }
                ));
            } else if cause == DiagnosticCause::CorruptState {
                assert!(matches!(
                    error.diagnostic.actions[0],
                    RecoveryAction::ReportDefect { .. }
                ));
            }
        }
    }
}
