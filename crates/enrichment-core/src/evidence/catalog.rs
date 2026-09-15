//! Typed catalog records. Visibility is determined by one committed catalog generation.

use crate::identity::{ContextId, SnapshotId};
use crate::producer::ProducerRun;
use serde::{Deserialize, Serialize};

/// A committed job result is recovered through native catalog records, never by rerunning it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobPublication {
    pub job_id: String,
    pub context_id: ContextId,
    pub snapshot_id: SnapshotId,
    pub kind: PublishedJobKind,
    pub state: crate::wire::JobState,
    pub attempt_id: String,
    pub result_artifact_ids: Vec<String>,
    /// Complete bounded presentation, admitted before this catalog publication.
    /// It is not a producer input and does not enter evidence identity.
    pub delivery: super::Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishedJobKind {
    Verify,
    Inspect,
    Resolve,
}

impl JobPublication {
    /// # Errors
    /// Only a service job, a terminal outcome and a bounded result closure may be published.
    pub fn validate(&self) -> Result<(), String> {
        if !self
            .job_id
            .strip_prefix("job_")
            .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()))
            || !matches!(
                self.state,
                crate::wire::JobState::Succeeded
                    | crate::wire::JobState::Partial
                    | crate::wire::JobState::Failed
                    | crate::wire::JobState::Cancelled
            )
            || self.attempt_id.is_empty()
            || self.result_artifact_ids.is_empty()
            || self.result_artifact_ids.len() > 64
            || self
                .result_artifact_ids
                .iter()
                .any(|s| !super::is_artifact_id(s))
        {
            return Err("invalid durable job publication".into());
        }
        let delivery = &self.delivery;
        if delivery.sha256.len() != 64
            || !delivery
                .sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || delivery.artifact_id != super::artifact_id_for(&delivery.sha256)
            || delivery.size_bytes == 0
            || delivery.size_bytes > 32 * 1024 * 1024
            || delivery.kind != super::ArtifactKind::Other
            || delivery.media_type != "application/json"
            || delivery.source_uri != "service:job-delivery/1"
            || delivery.retrieved_at.is_empty()
            || delivery.final_url.is_some()
            || delivery.etag.is_some()
            || delivery.last_modified.is_some()
            || delivery.compression.is_some()
        {
            return Err("invalid committed job delivery descriptor".into());
        }
        let mut ids = self.result_artifact_ids.clone();
        ids.sort();
        ids.dedup();
        if ids.len() != self.result_artifact_ids.len() {
            return Err("duplicate job result artifacts".into());
        }
        Ok(())
    }
}

/// An immutable snapshot manifest admitted for a particular context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotEntry {
    pub snapshot_id: SnapshotId,
    pub context_id: ContextId,
    pub manifest_digest: String,
    pub manifest_bytes: u64,
}

impl SnapshotEntry {
    /// # Errors
    /// Reject invalid physical identity rather than accepting an unverifiable reference.
    pub fn validate(&self) -> Result<(), String> {
        if self.manifest_digest.len() != 64
            || !self
                .manifest_digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.manifest_bytes == 0
        {
            return Err("invalid snapshot manifest identity".into());
        }
        Ok(())
    }
}

/// Current selection is an ordered catalog fact, not part of snapshot semantic identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotSelection {
    pub context_id: ContextId,
    pub snapshot_id: SnapshotId,
    pub generation: u64,
}

/// A real acquisition attempt can be associated with already retained semantic evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotAttempt {
    pub snapshot_id: SnapshotId,
    pub run: ProducerRun,
    /// Exact acquisition descriptors from this attempt, including receipt clocks and HTTP
    /// validators. These do not enter semantic snapshot or observation identity.
    pub artifacts: Vec<super::Artifact>,
}

impl SnapshotAttempt {
    #[must_use]
    pub fn association_id(&self) -> String {
        format!(
            "association_{}",
            crate::canonical::digest_hex(&serde_json::json!([
                "snapshot-attempt/1",
                self.snapshot_id,
                self.run.attempt_id,
            ]))
        )
    }
}
