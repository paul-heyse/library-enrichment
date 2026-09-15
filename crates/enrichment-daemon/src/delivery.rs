//! One bounded encoding and immutable overflow contract for replies and durable journals.
use crate::{envelope, ops::common};
use enrichment_core::{
    canonical, clock,
    evidence::{Artifact, ArtifactKind},
    wire::{Envelope, ErrorCode},
};
use enrichment_store::BlobStore;
use std::io;

pub(crate) const MAX_RESULT_BYTES: usize = 32 * 1024 * 1024;
pub(crate) const JOURNAL_BYTES: usize = 1024 * 1024;

/// Count escaped UTF-8 JSON bytes without constructing another result-sized allocation.
pub(crate) fn size(value: &impl serde::Serialize, limit: usize) -> io::Result<usize> {
    canonical::serialized_size(value, limit)
}

pub(crate) fn encode(
    blobs: &BlobStore,
    mut result: Envelope,
    inline: usize,
    previews: bool,
) -> io::Result<Envelope> {
    if !(1024..=MAX_RESULT_BYTES).contains(&inline) {
        return Err(io::Error::other("invalid delivery budget"));
    }
    // Reject before to_value/canonicalization. JSON escaping is included in this bound.
    let mut bytes = size(&result, MAX_RESULT_BYTES)?;
    while previews && bytes > inline {
        let Some(entry) = result
            .evidence
            .iter_mut()
            .filter(|e| e.excerpt.chars().count() > 80)
            .max_by_key(|e| e.excerpt.len())
        else {
            break;
        };
        entry.excerpt =
            common::truncate(&entry.excerpt, (entry.excerpt.chars().count() / 2).max(80));
        bytes = size(&result, MAX_RESULT_BYTES)?;
    }
    if bytes <= inline {
        return Ok(result);
    }
    let artifact = store_result(blobs, &result, "service:bounded-result")?;
    overflow(&artifact, result, bytes, inline)
}

fn store_result(blobs: &BlobStore, result: &Envelope, uri: &str) -> io::Result<Artifact> {
    size(result, MAX_RESULT_BYTES)?;
    let mut document = serde_json::to_value(result)?;
    document
        .as_object_mut()
        .ok_or_else(|| io::Error::other("non-object result"))?
        .remove("request_id");
    let bytes = serde_json::to_vec(&canonical::canonicalize(document))?;
    Ok(blobs
        .put(&bytes, |_| {
            Artifact::describe(
                &bytes,
                ArtifactKind::Other,
                "application/json",
                uri,
                &clock::now_rfc3339(),
            )
        })?
        .acquired)
}

fn overflow(
    artifact: &Artifact,
    result: Envelope,
    bytes: usize,
    inline: usize,
) -> io::Result<Envelope> {
    let id = artifact.artifact_id.clone();
    let mut bounded = envelope::error(
        ErrorCode::BudgetExceeded,
        "The complete answer is stored because it exceeds the inline budget.",
        "Read data.result_artifact_id with read_artifact and follow its cursor.",
        false,
    );
    bounded.request_id = result.request_id;
    bounded.context_id = result.context_id;
    bounded.snapshot_id = result.snapshot_id;
    bounded.freshness = result.freshness;
    bounded.data = common::to_object(
        &serde_json::json!({"result_artifact_id":id,"omitted_response_bytes":bytes}),
    );
    bounded.pagination.returned = 0;
    bounded.pagination.truncated = true;
    bounded.artifacts = common::handle_for(
        artifact,
        "Complete answer excluding transport request identity".into(),
    )
    .into_iter()
    .collect();
    if size(&bounded, MAX_RESULT_BYTES)? > inline {
        bounded.artifacts.clear();
    }
    if size(&bounded, MAX_RESULT_BYTES)? > inline {
        bounded.summary = "Answer stored; read data.result_artifact_id.".into();
        bounded.data.remove("omitted_response_bytes");
    }
    if size(&bounded, MAX_RESULT_BYTES)? > inline {
        return Err(io::Error::other(
            "delivery identity envelope exceeds inline budget",
        ));
    }
    Ok(bounded)
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
    render: impl Fn(&enrichment_core::evidence::snapshot::EvidenceManifest) -> io::Result<Envelope>
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
        std::sync::Arc::new(move |manifest| {
            let result = render(manifest)?;
            let bytes = size(&result, MAX_RESULT_BYTES)?;
            let artifact = store_result(&blobs, &result, "service:job-delivery/1")?;
            let bounded = if bytes <= JOURNAL_BYTES {
                result
            } else {
                overflow(&artifact, result, bytes, JOURNAL_BYTES)?
            };
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
    if publication.delivery.size_bytes + 128 <= JOURNAL_BYTES as u64 {
        return blobs.read_delivery(&publication.delivery, envelope::new_request_id().as_str());
    }
    // Read only the bounded header, hashing the same stream. Large data fields are skipped.
    #[derive(serde::Deserialize)]
    struct Header {
        context_id: String,
        snapshot_id: String,
        freshness: enrichment_core::wire::Freshness,
    }
    let header: Header = blobs.read_json(&publication.delivery, MAX_RESULT_BYTES as u64)?;
    let mut result = envelope::error(
        ErrorCode::BudgetExceeded,
        "stored complete answer",
        "read artifact",
        false,
    );
    result.context_id = Some(header.context_id);
    result.snapshot_id = Some(header.snapshot_id);
    result.freshness = header.freshness;
    overflow(
        &publication.delivery,
        result,
        publication.delivery.size_bytes as usize,
        JOURNAL_BYTES,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
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
                scope: "bounded delivery fixture".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: vec![],
            },
        );
        answer.context_id = Some(context.to_string());
        answer.snapshot_id = Some(snapshot.to_string());
        let artifact = store_result(&blobs, &answer, "service:job-delivery/1").unwrap();
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
        assert_eq!(reply.data["result_artifact_id"], artifact.artifact_id);
        assert_eq!(reply.context_id, answer.context_id);
        assert_eq!(reply.snapshot_id, answer.snapshot_id);
        assert!(size(&reply, JOURNAL_BYTES).is_ok());
        assert!(!blobs.root().join(".staging").exists());
        let decoded: serde_json::Value = readonly
            .read_json(&artifact, MAX_RESULT_BYTES as u64)
            .unwrap();
        assert_eq!(decoded["data"], serde_json::to_value(answer.data).unwrap());
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
        let first = encode(&blobs, answer.clone(), 1024, false).unwrap();
        answer.request_id = envelope::new_request_id();
        let second = encode(&blobs, answer, 1024, false).unwrap();
        assert_eq!(
            first.data["result_artifact_id"],
            second.data["result_artifact_id"]
        );
        assert_ne!(first.request_id, second.request_id);
        assert!(size(&second, 1024).is_ok());
        let artifact = blobs
            .find(first.data["result_artifact_id"].as_str().unwrap())
            .unwrap()
            .unwrap();
        let decoded: serde_json::Value =
            serde_json::from_slice(&blobs.read(&artifact.sha256).unwrap()).unwrap();
        assert_eq!(decoded["data"]["text"], "🌎\n\"\\".repeat(1000));
        assert!(decoded.get("request_id").is_none());
    }
}
