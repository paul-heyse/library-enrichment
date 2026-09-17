//! Typed catalog records. Visibility is determined by one committed catalog generation.

use crate::identity::{ContextId, SnapshotId};
use crate::native_union::Rule;
use crate::producer::ProducerRun;
use serde::{Deserialize, Serialize};

crate::native_struct! {
/// A derived comparison is owned by its request and exact input pair, never a producer run.
pub struct ComparisonPublication {
    job_id: String => Rule::NonEmpty,
    request_digest: String => Rule::Sha256,
    before_context_id: ContextId => Rule::Text,
    before_snapshot_id: SnapshotId => Rule::Text,
    after_context_id: ContextId => Rule::Text,
    after_snapshot_id: SnapshotId => Rule::Text,
    state: crate::wire::JobState => Rule::Text,
    delivery: super::Artifact => Rule::Text,
}
}

impl ComparisonPublication {
    pub fn validate(&self) -> Result<(), String> {
        if !valid_job_id(&self.job_id)
            || !valid_digest(&self.request_digest)
            || !matches!(
                self.state,
                crate::wire::JobState::Succeeded | crate::wire::JobState::Partial
            )
        {
            return Err("invalid comparison publication identity or outcome".into());
        }
        validate_delivery(&self.delivery)
    }
}

fn valid_job_id(value: &str) -> bool {
    value
        .strip_prefix("job_")
        .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()))
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_delivery(delivery: &super::Artifact) -> Result<(), String> {
    if !valid_digest(&delivery.sha256)
        || delivery.artifact_id != super::artifact_id_for(&delivery.sha256)
        || delivery.size_bytes == 0
        || delivery.size_bytes > 32 * 1024 * 1024
        || delivery.kind != super::ArtifactKind::Other
        || delivery.media_type != crate::operation::results::MEDIA_TYPE
        || delivery.source_uri != crate::operation::results::JOB_URI
        || delivery.final_url.is_some()
        || delivery.etag.is_some()
        || delivery.last_modified.is_some()
        || delivery.compression.is_some()
    {
        return Err("invalid committed job delivery descriptor".into());
    }
    Ok(())
}

crate::native_struct! {
/// A committed job result is recovered through native catalog records, never by rerunning it.
pub struct JobPublication {
    job_id: String => Rule::NonEmpty,
    context_id: ContextId => Rule::Text,
    snapshot_id: SnapshotId => Rule::Text,
    kind: PublishedJobKind => Rule::Text,
    state: crate::wire::JobState => Rule::Text,
    attempt_id: String => Rule::NonEmpty,
    result_artifact_ids: Vec<String> => Rule::Set,
    /// Complete bounded presentation, admitted before this catalog publication.
    /// It is not a producer input and does not enter evidence identity.
    delivery: super::Artifact => Rule::Text,
}
}

crate::native_vocabulary! {
pub enum PublishedJobKind { Verify = "verify", Inspect = "inspect", Resolve = "resolve" }
}

impl JobPublication {
    /// # Errors
    /// Only a service job, a terminal outcome and a bounded result closure may be published.
    pub fn validate(&self) -> Result<(), String> {
        if !valid_job_id(&self.job_id)
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
        validate_delivery(&self.delivery)?;
        let mut ids = self.result_artifact_ids.clone();
        ids.sort();
        ids.dedup();
        if ids.len() != self.result_artifact_ids.len() {
            return Err("duplicate job result artifacts".into());
        }
        Ok(())
    }
}

crate::native_struct! {
/// An immutable snapshot manifest admitted for a particular context.
pub struct SnapshotEntry {
    snapshot_id: SnapshotId => crate::native_union::Rule::Text,
    context_id: ContextId => crate::native_union::Rule::Text,
    publication: super::snapshot::EvidenceManifest => crate::native_union::Rule::Text,
}
}

impl SnapshotEntry {
    /// # Errors
    /// Reject invalid physical identity rather than accepting an unverifiable reference.
    pub fn validate(&self) -> Result<(), String> {
        self.publication.validate()?;
        if self.snapshot_id != self.publication.snapshot_id
            || self.context_id != self.publication.context_id
        {
            return Err("invalid snapshot publication scope".into());
        }
        Ok(())
    }
}

crate::native_struct! {
/// Current selection is an ordered catalog fact, separate from snapshot identity.
pub struct SnapshotSelection {
    context_id: ContextId => Rule::Text,
    snapshot_id: SnapshotId => Rule::Text,
    generation: u64 => Rule::Text,
}
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

crate::native_struct! {
pub(crate) struct SnapshotAttemptIdentity {
    snapshot_id: SnapshotId => Rule::Text,
    attempt_id: String => Rule::NonEmpty,
}
}

impl SnapshotAttempt {
    #[must_use]
    pub fn association_id(&self) -> String {
        crate::native_key::Key::SnapshotAttempt
            .record(&SnapshotAttemptIdentity {
                snapshot_id: self.snapshot_id.clone(),
                attempt_id: self.run.attempt_id.clone(),
            })
            .expect("typed snapshot attempt identity")
    }
}
