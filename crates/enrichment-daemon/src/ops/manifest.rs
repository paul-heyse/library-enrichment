//! `snapshot.manifest`: the resource behind `library-evidence://snapshots/{id}/manifest`.
//!
//! A manifest is the immutable description of one snapshot (§6.1): what was normalized, from
//! which artifact digests, by which producers, and what is indexed or missing. It is read from
//! the published directory, never reconstructed, so the resource answers the same bytes for
//! the life of the snapshot.

use enrichment_core::identity::SnapshotId;
use enrichment_core::wire::data::ManifestData;
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Freshness, SourceVersionMatch};
use serde::Deserialize;

use super::common;
use crate::envelope::{self, Research};
use crate::service::Service;

/// Parameters of `snapshot.manifest`.
#[derive(Debug, Clone, Deserialize)]
pub struct ManifestRequest {
    /// The snapshot to describe.
    pub snapshot_id: String,
}

/// Read a published manifest.
pub fn manifest(service: &Service, request: ManifestRequest) -> Envelope {
    let Ok(id) = SnapshotId::try_from(request.snapshot_id.trim().to_owned()) else {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!("`{}` is not a snapshot identity", request.snapshot_id),
            "Pass the `snapshot_id` a `resolve_library` result returned.",
            false,
        );
    };
    let manifest = match enrichment_store::snapshot::read_manifest(&service.paths, &id) {
        Ok(Some(manifest)) => manifest,
        Ok(None) => {
            return envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("snapshot {id} is not published on this service"),
                "Resolve the release again; a snapshot is published when hosted rustdoc JSON \
                 normalizes.",
                false,
            );
        }
        Err(err) => return common::store_error(&err),
    };
    let is_current = service
        .catalog
        .current_snapshot(&manifest.context_id)
        .ok()
        .flatten()
        .is_some_and(|current| current == id);
    let data = ManifestData {
        manifest: manifest.clone(),
        is_current,
    };
    Research {
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
        coverage: Coverage {
            scope: format!("the manifest of snapshot {id}"),
            indexed: manifest
                .indexed
                .iter()
                .map(|k| k.as_str().to_owned())
                .collect(),
            missing: manifest
                .missing
                .iter()
                .map(|k| k.as_str().to_owned())
                .collect(),
            limitations: Vec::new(),
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
    }
    .ok()
}
