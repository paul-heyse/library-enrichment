//! `snapshot.manifest`: the resource behind `library-evidence://snapshots/{id}/manifest`.
//!
//! The captured control transaction owns metadata and the exact Delta table/cohort vector.
//! Change explanations use its predecessor selection, never CDF commit timestamps.

use enrichment_core::identity::SnapshotId;
use enrichment_core::wire::data::ManifestData;
use enrichment_core::wire::{Envelope, ErrorCode, Freshness, SourceVersionMatch};

use super::common;
use crate::envelope::{self, Research};
use crate::service::Service;

pub use enrichment_core::request::ManifestRequest;

/// Read a published manifest.
pub async fn manifest(service: &Service, request: ManifestRequest) -> Envelope {
    let Ok(id) = SnapshotId::try_from(request.snapshot_id.trim().to_owned()) else {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!("`{}` is not a snapshot identity", request.snapshot_id),
            "Pass the `snapshot_id` a `resolve_library` result returned.",
            false,
        );
    };
    let catalog = match service.repository.catalog.pin().await {
        Ok(value) => value,
        Err(e) => return common::operation_error(&e, "manifest_read"),
    };
    let reader =
        match enrichment_store::SnapshotReader::open(&service.repository, catalog.clone(), &id)
            .await
        {
            Ok(value) => value,
            Err(e) => return common::query_error(&e),
        };
    let manifest = reader.manifest().clone();
    let is_current = match catalog
        .current(&service.repository.runtime, &manifest.context_id)
        .await
    {
        Ok(value) => value.as_ref() == Some(&id),
        Err(e) => return common::operation_error(&e, "manifest_read"),
    };
    let previous = match catalog.previous(&service.repository.runtime, &id).await {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "manifest_changes"),
    };
    let publication_changes = if let Some(before_id) = &previous {
        let before = match catalog
            .snapshot(&service.repository.runtime, before_id)
            .await
        {
            Ok(Some(value)) => value.publication,
            Ok(None) => {
                return envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    "Prior publication is unavailable",
                    "Reacquire the context.",
                    false,
                );
            }
            Err(error) => return common::operation_error(&error, "manifest_changes"),
        };
        match service
            .repository
            .publication_changes(&before, &manifest)
            .await
        {
            Ok(value) => value,
            Err(error) => return common::operation_error(&error, "manifest_changes"),
        }
    } else {
        Vec::new()
    };
    let data = ManifestData {
        previous_snapshot_id: previous.map(|id| id.to_string()),
        publication_changes,
        manifest: manifest.clone(),
        is_current,
        producer_runs: match reader.producer_runs().await {
            Ok(value) => value,
            Err(e) => return common::query_error(&e),
        },
        control_version: catalog.generation(),
    };
    let research = Research {
        summary: format!(
            "Snapshot {id} of {} {}: {} definitions, {} fragments, published {}.",
            manifest.crate_name,
            manifest
                .crate_version
                .as_deref()
                .unwrap_or("(unknown version)"),
            manifest.counts.definitions,
            manifest.counts.fragments,
            manifest.published_at
        ),
        data: common::to_object(&data),
        coverage: match reader.assess_acquisition().await {
            Ok(value) => value,
            Err(error) => return common::query_error(&error),
        },
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match: SourceVersionMatch::Exact,
            latest_verified: false,
        },
        context_id: Some(manifest.context_id.to_string()),
        snapshot_id: Some(id.to_string()),
        evidence: Vec::new(),
        artifacts: Vec::new(),
    };
    if research.coverage.complete() {
        research.ok()
    } else {
        research.partial()
    }
}
