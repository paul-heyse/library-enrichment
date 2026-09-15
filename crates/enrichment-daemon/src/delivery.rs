//! One bounded encoding and immutable overflow contract for replies and durable journals.
use crate::{envelope, ops::common};
use enrichment_core::{
    canonical,
    evidence::Artifact,
    wire::{DeliveryDescriptor, DeliveryLimits, Envelope, RecoveryAction},
};
use enrichment_store::BlobStore;
use std::io;

pub(crate) const MAX_RESULT_BYTES: usize = 32 * 1024 * 1024;
pub(crate) const JOURNAL_BYTES: usize = 1024 * 1024;

#[derive(Debug, thiserror::Error)]
#[error("delivery requires at least {minimum} encoded bytes")]
pub(crate) struct MinimumBudget {
    pub minimum: usize,
}

/// The minimum-budget response is itself a complete, bounded error. Its short mandatory
/// fields avoid repeating an arbitrarily large failed result or its explanatory text.
pub(crate) fn budget_failure(
    requested: Option<usize>,
    effective: usize,
    minimum: usize,
) -> Envelope {
    let mut result = envelope::error(
        enrichment_core::wire::ErrorCode::BudgetExceeded,
        "Envelope cap is too small",
        "Increase max_bytes to the required minimum.",
        false,
    );
    result.coverage.scope = "request".into();
    result.coverage.limitations.clear();
    result.delivery.set_limits(requested, effective);
    let diagnostic = &mut result.error_mut().expect("typed error").diagnostic;
    diagnostic.stage = "result_delivery".into();
    diagnostic.rule = Some("encoded_envelope_bytes".into());
    diagnostic.observed = Some(minimum as u64);
    diagnostic.allowed = Some(effective as u64);
    diagnostic.actions = vec![RecoveryAction::ChangeRequest {
        reason: format!(
            "Set max_bytes to at least {minimum}; the service cap must also permit it."
        ),
    }];
    result
}

/// Count escaped UTF-8 JSON bytes without constructing another result-sized allocation.
pub(crate) fn size(value: &impl serde::Serialize, limit: usize) -> io::Result<usize> {
    canonical::serialized_size(value, limit)
}

pub(crate) fn encode(
    blobs: &BlobStore,
    mut result: Envelope,
    inline: usize,
    requested: Option<usize>,
) -> io::Result<Envelope> {
    if !(1024..=MAX_RESULT_BYTES).contains(&inline) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid delivery budget",
        ));
    }
    result.delivery.set_limits(requested, inline);
    // Reject before to_value/canonicalization. JSON escaping is included in this bound.
    let bytes = size(&result, MAX_RESULT_BYTES)?;
    if bytes <= inline {
        return Ok(result);
    }
    let (artifact, index) =
        if let DeliveryDescriptor::Artifact { artifact_id, .. } = &result.delivery {
            let artifact = blobs
                .find(artifact_id)?
                .ok_or_else(|| io::Error::other("retained result missing"))?;
            let (index, _) =
                blobs.read_result_sections(&artifact, &["status"], MAX_RESULT_BYTES as u64)?;
            (artifact, index)
        } else {
            store_result(blobs, &result, "service:bounded-result/2")?
        };
    overflow(&artifact, &index, result, bytes, inline)
}

pub(crate) fn terminal(blobs: &BlobStore, result: Envelope) -> io::Result<Envelope> {
    if let DeliveryDescriptor::Artifact { artifact_id, .. } = &result.delivery {
        let artifact = blobs
            .find(artifact_id)?
            .ok_or_else(|| io::Error::other("terminal delivery artifact missing"))?;
        blobs.result_dependencies(&artifact)?;
        // Verify the root itself even when it has no dependent artifacts.
        let _ = blobs.read_result_sections(&artifact, &["status"], MAX_RESULT_BYTES as u64)?;
        return Ok(result);
    }
    let bytes = size(&result, MAX_RESULT_BYTES)?;
    let (artifact, index) = store_result(blobs, &result, "service:terminal-result/2")?;
    overflow(&artifact, &index, result, bytes, JOURNAL_BYTES)
}

fn store_result(
    blobs: &BlobStore,
    result: &Envelope,
    uri: &str,
) -> io::Result<(Artifact, enrichment_store::result::Index)> {
    enrichment_store::result::store(blobs, result, uri)
}

fn overflow(
    artifact: &Artifact,
    index: &enrichment_store::result::Index,
    result: Envelope,
    bytes: usize,
    inline: usize,
) -> io::Result<Envelope> {
    let id = artifact.artifact_id.clone();
    use enrichment_core::wire::research::ResultSectionName;
    let sections = [
        ResultSectionName::Coverage,
        ResultSectionName::Signature,
        ResultSectionName::Changes,
        ResultSectionName::Aspects,
        ResultSectionName::Data,
    ]
    .into_iter()
    .filter(|name| index.sections.contains_key(name.as_str()))
    .collect();
    let mut bounded = result;
    bounded.data.clear();
    bounded.evidence.clear();
    let requested = match &bounded.delivery {
        DeliveryDescriptor::Inline { limits } | DeliveryDescriptor::Artifact { limits, .. } => {
            limits.requested_max_bytes
        }
    };
    bounded.delivery = DeliveryDescriptor::retained(
        id,
        sections,
        DeliveryLimits {
            requested_max_bytes: requested,
            effective_max_bytes: Some(inline),
        },
    );
    let _ = bytes;
    bounded.artifacts = common::handle_for(
        artifact,
        "Indexed complete result; request identity is a fixed retained placeholder".into(),
    )
    .into_iter()
    .collect();
    if size(&bounded, MAX_RESULT_BYTES)? > inline {
        bounded.artifacts.clear();
    }
    if size(&bounded, MAX_RESULT_BYTES)? > inline && !bounded.coverage.limitations.is_empty() {
        bounded.coverage.limitations.clear();
        bounded.coverage.details = Some(RecoveryAction::ReadArtifact {
            artifact_id: artifact.artifact_id.clone(),
            section: Some(enrichment_core::wire::research::ArtifactSection::Result {
                name: enrichment_core::wire::research::ResultSectionName::Coverage,
            }),
            cursor: None,
        });
    }
    let minimum = size(&bounded, MAX_RESULT_BYTES)?;
    if minimum > inline {
        return Err(io::Error::new(
            io::ErrorKind::OutOfMemory,
            MinimumBudget { minimum },
        ));
    }
    Ok(bounded)
}

pub(crate) fn prepare_comparison(
    blobs: &BlobStore,
    result: Envelope,
) -> io::Result<(Artifact, Envelope)> {
    let bytes = size(&result, MAX_RESULT_BYTES)?;
    let (artifact, index) = store_result(blobs, &result, enrichment_store::result::JOB_URI)?;
    let bounded = overflow(&artifact, &index, result, bytes, JOURNAL_BYTES)?;
    Ok((artifact, bounded))
}

/// A prepared journal envelope is selected by the same final candidate identity as the
/// catalog commit. Rebase replaces this bounded slot; reading it after commit performs no I/O.
pub(crate) struct DeliverySlot(std::sync::Arc<std::sync::Mutex<Option<Envelope>>>);
impl DeliverySlot {
    pub fn get(&self, snapshot: &str) -> io::Result<Envelope> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("delivery slot poisoned"))?
            .as_ref()
            .filter(|result| result.snapshot_id.as_deref() == Some(snapshot))
            .cloned()
            .ok_or_else(|| io::Error::other("prepared delivery does not match committed candidate"))
    }
}
pub(crate) fn prepare_job(
    blobs: BlobStore,
    render: impl Fn(
        &enrichment_core::evidence::snapshot::EvidenceManifest,
        &enrichment_core::wire::Coverage,
    ) -> io::Result<Envelope>
    + Send
    + Sync
    + 'static,
) -> (
    enrichment_store::repository::JobDeliveryFactory,
    DeliverySlot,
) {
    let slot = std::sync::Arc::new(std::sync::Mutex::new(None));
    let output = std::sync::Arc::clone(&slot);
    let factory: enrichment_store::repository::JobDeliveryFactory =
        std::sync::Arc::new(move |manifest, coverage| {
            let result = render(manifest, coverage)?;
            let bytes = size(&result, MAX_RESULT_BYTES)?;
            let (artifact, index) =
                store_result(&blobs, &result, enrichment_store::result::JOB_URI)?;
            let bounded = overflow(&artifact, &index, result, bytes, JOURNAL_BYTES)?;
            *output
                .lock()
                .map_err(|_| io::Error::other("delivery slot poisoned"))? = Some(bounded);
            Ok(artifact)
        });
    (factory, DeliverySlot(slot))
}

/// Recover only already admitted bytes. Large replies use a descriptor directly, so recovery
/// cannot require a fresh artifact write. The caller validates publication and evidence closure.
pub(crate) fn recover_job(
    blobs: &BlobStore,
    publication: &enrichment_core::evidence::catalog::JobPublication,
) -> io::Result<Envelope> {
    recover_result(blobs, &publication.delivery)
}

pub(crate) fn recover_result(blobs: &BlobStore, artifact: &Artifact) -> io::Result<Envelope> {
    let (index, mut fields) = blobs.read_result_sections(
        artifact,
        &[
            "status",
            "job",
            "error",
            "summary",
            "context_id",
            "snapshot_id",
            "coverage",
            "freshness",
        ],
        MAX_RESULT_BYTES as u64,
    )?;
    fn field<T: serde::de::DeserializeOwned>(
        fields: &mut std::collections::BTreeMap<String, serde_json::Value>,
        name: &str,
    ) -> io::Result<T> {
        serde_json::from_value(
            fields
                .remove(name)
                .ok_or_else(|| io::Error::other("missing recovery field"))?,
        )
        .map_err(Into::into)
    }
    let status = field(&mut fields, "status")?;
    let job = field(&mut fields, "job")?;
    let error = field(&mut fields, "error")?;
    use enrichment_core::wire::{Outcome, Status};
    let outcome = match (status, job, error) {
        (Status::Ok, job, None) => Outcome::Ok { job },
        (Status::Partial, job, None) => Outcome::Partial { job },
        (Status::Pending, Some(job), None) => Outcome::Pending { job },
        (Status::Error, job, Some(error)) => Outcome::Error { job, error },
        _ => return Err(io::Error::other("invalid committed research outcome")),
    };
    let result = Envelope::new(
        enrichment_core::wire::EnvelopeBody {
            request_id: envelope::new_request_id(),
            summary: field(&mut fields, "summary")?,
            context_id: field(&mut fields, "context_id")?,
            snapshot_id: field(&mut fields, "snapshot_id")?,
            data: enrichment_core::wire::JsonObject::new(),
            coverage: field(&mut fields, "coverage")?,
            freshness: field(&mut fields, "freshness")?,
            evidence: Vec::new(),
            artifacts: Vec::new(),
            delivery: DeliveryDescriptor::default(),
        },
        outcome,
    );
    overflow(
        artifact,
        &index,
        result,
        artifact.size_bytes as usize,
        JOURNAL_BYTES,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::wire::ErrorCode;

    #[test]
    fn delivery_preserves_outcome_and_scope_and_indexes_independent_sections() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let mut result = envelope::ok(
            "retained comparison",
            common::to_object(&serde_json::json!({
                "changes": [{"before": "é😀\\\"\n".repeat(2000), "after": "changed"}],
                "observations": [{"payload": {"signature": "pub fn selected()"}}],
                "aspect_outcomes": [],
            })),
            enrichment_core::wire::Coverage {
                details: None,
                assessments: Vec::new(),
                scope: "requested documentation".into(),
                indexed: ["documentation".into()].into_iter().collect(),
                missing: ["release_notes".into()].into_iter().collect(),
                limitations: vec!["notes have no qualified observation".into()],
            },
        )
        .into_partial();
        let coverage = result.coverage.clone();
        let expected_changes = result.data["changes"].clone();
        result.delivery.set_limits(Some(8192), 8192);
        let reply = encode(&blobs, result, 8192, Some(8192)).unwrap();
        assert_eq!(reply.status(), enrichment_core::wire::Status::Partial);
        assert_eq!(reply.coverage, coverage);
        assert!(reply.data.is_empty());
        let DeliveryDescriptor::Artifact {
            artifact_id,
            sections,
            ..
        } = &reply.delivery
        else {
            panic!("stored result")
        };
        assert!(
            sections
                .iter()
                .any(|s| s.name == enrichment_core::wire::research::ResultSectionName::Changes)
        );
        let artifact = blobs.find(artifact_id).unwrap().unwrap();
        let mut file = blobs.capture(&artifact, MAX_RESULT_BYTES as u64).unwrap();
        let (index, base) =
            enrichment_store::result::index(&mut file, artifact.size_bytes).unwrap();
        let actual: serde_json::Value = enrichment_store::result::read_section(
            &mut file,
            &index,
            base,
            "changes",
            MAX_RESULT_BYTES as u64,
        )
        .unwrap();
        assert_eq!(actual, expected_changes);
        let actual: enrichment_core::wire::Coverage =
            enrichment_store::result::read_section(&mut file, &index, base, "coverage", 8192)
                .unwrap();
        assert_eq!(actual, coverage);
        let admitted = blobs.read_delivery(&artifact, "req_admitted").unwrap();
        assert_eq!(admitted.status(), reply.status());
        assert_eq!(admitted.coverage, reply.coverage);
        assert_eq!(admitted.data["changes"], expected_changes);
    }
    #[test]
    fn overflow_delivery_recovers_from_read_only_bytes_without_regeneration() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let context =
            enrichment_core::identity::ContextId::try_from("ctx_0123456789abcdef".to_owned())
                .unwrap();
        let snapshot =
            enrichment_core::identity::SnapshotId::try_from("snap_0123456789abcdef".to_owned())
                .unwrap();
        let mut answer = envelope::ok(
            "complete result",
            common::to_object(&serde_json::json!({"text":"🌎\"\n".repeat(180_000)})),
            enrichment_core::wire::Coverage {
                details: None,
                assessments: Vec::new(),
                scope: "bounded delivery fixture".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: vec![],
            },
        );
        answer.context_id = Some(context.to_string());
        answer.snapshot_id = Some(snapshot.to_string());
        let (artifact, _) =
            store_result(&blobs, &answer, enrichment_store::result::JOB_URI).unwrap();
        assert!(artifact.size_bytes > JOURNAL_BYTES as u64);
        let publication = enrichment_core::evidence::catalog::JobPublication {
            job_id: format!("job_{}", "a".repeat(32)),
            context_id: context,
            snapshot_id: snapshot,
            kind: enrichment_core::evidence::catalog::PublishedJobKind::Resolve,
            state: enrichment_core::wire::JobState::Succeeded,
            attempt_id: "attempt_fixture".into(),
            result_artifact_ids: vec![artifact.artifact_id.clone()],
            delivery: artifact.clone(),
        };
        // A reconstruction path calling put/capture would need this directory and fail or
        // recreate it. Recovery reads the admitted document through the verified JSON stream.
        std::fs::remove_dir(blobs.root().join(".staging")).unwrap();
        let readonly = BlobStore::read_only(dir.path()).unwrap();
        let reply = recover_job(&readonly, &publication).unwrap();
        assert_eq!(
            serde_json::to_value(&reply.delivery).unwrap()["artifact_id"],
            artifact.artifact_id
        );
        assert_eq!(reply.context_id, answer.context_id);
        assert_eq!(reply.snapshot_id, answer.snapshot_id);
        assert!(size(&reply, JOURNAL_BYTES).is_ok());
        assert!(!blobs.root().join(".staging").exists());
        let decoded: serde_json::Value = readonly
            .read_json(&artifact, MAX_RESULT_BYTES as u64)
            .unwrap();
        assert_eq!(
            decoded["result"]["data"],
            serde_json::to_value(answer.data).unwrap()
        );
    }

    #[test]
    fn escaped_results_are_bounded_before_copy_and_reuse_one_artifact() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let mut answer = envelope::error(
            ErrorCode::UnsupportedFormat,
            "fixture",
            "read evidence",
            false,
        );
        answer
            .data
            .insert("text".into(), serde_json::json!("🌎\n\"\\".repeat(1000)));
        let size_bytes = size(&answer, MAX_RESULT_BYTES).unwrap();
        assert!(size(&answer, size_bytes - 1).is_err());
        assert_eq!(
            size(&answer, size_bytes).unwrap(),
            serde_json::to_vec(&answer).unwrap().len()
        );
        let first = encode(&blobs, answer.clone(), 4096, Some(4096)).unwrap();
        answer.request_id = envelope::new_request_id();
        let second = encode(&blobs, answer, 4096, Some(4096)).unwrap();
        assert_eq!(
            serde_json::to_value(&first.delivery).unwrap()["artifact_id"],
            serde_json::to_value(&second.delivery).unwrap()["artifact_id"]
        );
        assert_ne!(first.request_id, second.request_id);
        assert!(size(&second, 4096).is_ok());
        let artifact = blobs
            .find(
                serde_json::to_value(&first.delivery).unwrap()["artifact_id"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap()
            .unwrap();
        let decoded: serde_json::Value =
            serde_json::from_slice(&blobs.read(&artifact.sha256).unwrap()).unwrap();
        assert_eq!(decoded["result"]["data"]["text"], "🌎\n\"\\".repeat(1000));
        assert_eq!(decoded["result"]["request_id"], "req_retained");
    }
}
