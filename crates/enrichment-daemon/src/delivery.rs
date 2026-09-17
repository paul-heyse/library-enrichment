//! One bounded encoding and immutable overflow contract for replies and durable journals.
use crate::envelope;
use enrichment_core::{
    canonical,
    evidence::Artifact,
    wire::{DeliveryDescriptor, Envelope, RecoveryAction},
};
use enrichment_store::BlobStore;
use std::io;

pub(crate) const MAX_RESULT_BYTES: usize = enrichment_core::operation::results::MAX_BYTES as usize;
pub(crate) const JOURNAL_BYTES: usize = 1024 * 1024;

pub(crate) use enrichment_store::result_delivery::MinimumBudget;

/// The minimum-budget response is itself a complete, bounded error. Its short mandatory
/// fields avoid repeating an arbitrarily large failed result or its explanatory text.
pub(crate) fn budget_failure(
    requested: Option<usize>,
    effective: usize,
    minimum: usize,
) -> Envelope {
    let mut result = envelope::error(
        enrichment_core::wire::ErrorCode::BudgetExceeded,
        "Response cap is too small",
        "Increase max_bytes to the required minimum.",
        false,
    );
    result.coverage.scope = "request".into();
    result.coverage.limitations.clear();
    result.delivery.set_limits(requested, effective);
    let diagnostic = &mut result.error_mut().expect("typed error").diagnostic;
    diagnostic.stage = "result_delivery".into();
    diagnostic.rule = Some("encoded_response_bytes".into());
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

pub(crate) async fn encode(
    blobs: &BlobStore,
    catalog: &enrichment_store::control::ControlStore,
    runtime: &enrichment_store::runtime::QueryRuntime,
    mut result: Envelope,
    inline: usize,
    requested: Option<usize>,
    profile: enrichment_core::mcp_delivery::DeliveryProfile,
) -> io::Result<Envelope> {
    if !(1024..=MAX_RESULT_BYTES).contains(&inline) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid delivery budget",
        ));
    }
    result.delivery.set_limits(requested, inline);
    // Reject before to_value/canonicalization. JSON escaping is included in this bound.
    size(&result, MAX_RESULT_BYTES)?;
    catalog
        .retain_artifacts(
            runtime,
            &result
                .artifacts
                .iter()
                .map(|handle| handle.receipt.clone())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(io::Error::other)?;
    if enrichment_store::result_delivery::inline(runtime, &result, inline, profile.clone())
        .await
        .map_err(io::Error::other)?
    {
        return Ok(result);
    }
    let (artifact, index) =
        if let DeliveryDescriptor::Artifact { artifact_id, .. } = &result.delivery {
            let artifact = catalog
                .pin()
                .await
                .map_err(io::Error::other)?
                .artifact(runtime, artifact_id)
                .await
                .map_err(io::Error::other)?
                .ok_or_else(|| io::Error::other("retained result missing"))?;
            let owned = blobs.clone();
            let descriptor = artifact.clone();
            let (index, _) = runtime
                .blocking(move || {
                    owned.read_result_sections(&descriptor, &["status"], MAX_RESULT_BYTES as u64)
                })
                .await
                .map_err(io::Error::other)??;
            (artifact, index)
        } else {
            let owned = blobs.clone();
            let input = result.clone();
            let output = runtime
                .blocking(move || store_result(&owned, &input, "service:bounded-result/4"))
                .await
                .map_err(io::Error::other)??;
            catalog
                .retain_result(runtime, blobs, &output.0)
                .await
                .map_err(io::Error::other)?;
            output
        };
    enrichment_store::result_delivery::retained(
        runtime,
        index.record.header.clone(),
        &artifact,
        &index,
        enrichment_store::result_delivery::DeliveryOptions {
            inline,
            requested,
            request_id: result.request_id,
            profile,
        },
    )
    .await
}

fn store_result(
    blobs: &BlobStore,
    result: &Envelope,
    uri: &str,
) -> io::Result<(Artifact, enrichment_store::result::Index)> {
    enrichment_store::result::store(blobs, result, uri)
}

pub(crate) async fn prepare_comparison(
    blobs: &BlobStore,
    runtime: &enrichment_store::runtime::QueryRuntime,
    result: Envelope,
) -> io::Result<(Artifact, Envelope)> {
    size(&result, MAX_RESULT_BYTES)?;
    let request_id = result.request_id.clone();
    let owned = blobs.clone();
    let (artifact, index) = runtime
        .blocking(move || store_result(&owned, &result, enrichment_store::result::JOB_URI))
        .await
        .map_err(io::Error::other)??;
    let bounded = enrichment_store::result_delivery::retained(
        runtime,
        index.record.header.clone(),
        &artifact,
        &index,
        enrichment_store::result_delivery::DeliveryOptions {
            inline: JOURNAL_BYTES,
            requested: None,
            request_id,
            profile: Default::default(),
        },
    )
    .await?;
    Ok((artifact, bounded))
}

/// Recover only already admitted bytes. Large replies use a descriptor directly, so recovery
/// cannot require a fresh artifact write. The caller validates publication and evidence closure.
pub(crate) async fn recover_job(
    blobs: &BlobStore,
    runtime: &enrichment_store::runtime::QueryRuntime,
    catalog: &enrichment_store::control::ControlSnapshot,
    publication: &enrichment_core::evidence::catalog::JobPublication,
) -> io::Result<Envelope> {
    recover_result(blobs, runtime, catalog, &publication.delivery).await
}

pub(crate) async fn recover_result(
    blobs: &BlobStore,
    runtime: &enrichment_store::runtime::QueryRuntime,
    catalog: &enrichment_store::control::ControlSnapshot,
    artifact: &Artifact,
) -> io::Result<Envelope> {
    let declared = catalog
        .retained_result(runtime, &artifact.artifact_id)
        .await
        .map_err(io::Error::other)?;
    let owned = blobs.clone();
    let descriptor = artifact.clone();
    let (index, _) = runtime
        .blocking(move || owned.read_result_sections(&descriptor, &[], MAX_RESULT_BYTES as u64))
        .await
        .map_err(io::Error::other)??;
    let record = catalog
        .result_record(&declared)
        .await
        .map_err(io::Error::other)?;
    if index.record != record {
        return Err(io::Error::other(
            "retained native record differs from immutable result",
        ));
    }
    enrichment_store::result_delivery::retained(
        runtime,
        record.header,
        artifact,
        &index,
        enrichment_store::result_delivery::DeliveryOptions {
            inline: JOURNAL_BYTES,
            requested: None,
            request_id: envelope::new_request_id(),
            profile: Default::default(),
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::wire::ErrorCode;

    #[tokio::test]
    async fn delivery_preserves_outcome_and_scope_and_indexes_independent_sections() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let runtime = enrichment_store::runtime::QueryRuntime::new(
            &dir.path().join("spill"),
            Default::default(),
        )
        .unwrap();
        let catalog =
            enrichment_store::control::ControlStore::open(dir.path(), runtime.clone()).unwrap();
        let mut result = envelope::ok(
            "retained comparison",
            envelope::fixture_payload(&"é😀\\\"\n".repeat(2000)),
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
        let expected_data = serde_json::to_value(&result.data).unwrap();
        result.delivery.set_limits(Some(8192), 8192);
        let reply = encode(&blobs, &catalog, &runtime, result, 8192, Some(8192))
            .await
            .unwrap();
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
                .any(|s| s.name == enrichment_core::wire::research::ResultSectionName::Data)
        );
        let artifact = catalog
            .pin()
            .await
            .unwrap()
            .artifact(&runtime, artifact_id)
            .await
            .unwrap()
            .unwrap();
        let mut file = blobs.capture(&artifact, MAX_RESULT_BYTES as u64).unwrap();
        let (index, base) =
            enrichment_store::result::index(&mut file, artifact.size_bytes).unwrap();
        let actual: serde_json::Value = enrichment_store::result::read_section(
            &mut file,
            &index,
            base,
            "data",
            MAX_RESULT_BYTES as u64,
        )
        .unwrap();
        assert_eq!(actual, expected_data);
        let actual: enrichment_core::wire::Coverage =
            enrichment_store::result::read_section(&mut file, &index, base, "coverage", 8192)
                .unwrap();
        assert_eq!(actual, coverage);
        let admitted = blobs.read_delivery(&artifact, "req_admitted").unwrap();
        assert_eq!(admitted.status(), reply.status());
        assert_eq!(admitted.coverage, reply.coverage);
        assert_eq!(serde_json::to_value(admitted.data).unwrap(), expected_data);
    }
    #[tokio::test]
    async fn overflow_delivery_recovers_from_read_only_bytes_without_regeneration() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let context =
            enrichment_core::identity::ContextId::try_from(format!("ctx_{}", "0".repeat(64)))
                .unwrap();
        let snapshot =
            enrichment_core::identity::SnapshotId::try_from(format!("snap_{}", "1".repeat(64)))
                .unwrap();
        let mut answer = envelope::ok(
            "complete result",
            envelope::fixture_payload(&"🌎\"\n".repeat(180_000)),
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
        let runtime = enrichment_store::runtime::QueryRuntime::new(
            &dir.path().join("spill"),
            Default::default(),
        )
        .unwrap();
        let catalog =
            enrichment_store::control::ControlStore::open(dir.path(), runtime.clone()).unwrap();
        catalog
            .retain_result(&runtime, &blobs, &artifact)
            .await
            .unwrap();
        let pin = catalog.pin().await.unwrap();
        // Enrollment and byte verification precede read-only recovery.
        std::fs::remove_dir(blobs.root().join(".staging")).unwrap();
        let readonly = BlobStore::read_only(dir.path()).unwrap();
        let reply = recover_job(&readonly, &runtime, &pin, &publication)
            .await
            .unwrap();
        assert_eq!(
            serde_json::to_value(&reply.delivery).unwrap()["artifact_id"],
            artifact.artifact_id
        );
        assert_eq!(reply.context_id, answer.context_id);
        assert_eq!(reply.snapshot_id, answer.snapshot_id);
        assert!(size(&reply, JOURNAL_BYTES).is_ok());
        assert!(!blobs.root().join(".staging").exists());
        let decoded = readonly.read_delivery(&artifact, "req_readonly").unwrap();
        assert_eq!(decoded.data, answer.data);
    }

    #[tokio::test]
    async fn escaped_results_are_bounded_before_copy_and_reuse_one_artifact() {
        let dir = tempfile::tempdir().unwrap();
        let blobs = BlobStore::open(dir.path()).unwrap();
        let runtime = enrichment_store::runtime::QueryRuntime::new(
            &dir.path().join("spill"),
            Default::default(),
        )
        .unwrap();
        let catalog =
            enrichment_store::control::ControlStore::open(dir.path(), runtime.clone()).unwrap();
        let mut answer = envelope::error(
            ErrorCode::UnsupportedFormat,
            "fixture",
            "read evidence",
            false,
        );
        answer.data = envelope::fixture_payload(&"🌎\n\"\\".repeat(1000));
        let size_bytes = size(&answer, MAX_RESULT_BYTES).unwrap();
        assert!(size(&answer, size_bytes - 1).is_err());
        assert_eq!(
            size(&answer, size_bytes).unwrap(),
            serde_json::to_vec(&answer).unwrap().len()
        );
        let first = encode(&blobs, &catalog, &runtime, answer.clone(), 4096, Some(4096))
            .await
            .unwrap();
        answer.request_id = envelope::new_request_id();
        let second = encode(&blobs, &catalog, &runtime, answer, 4096, Some(4096))
            .await
            .unwrap();
        assert_eq!(
            serde_json::to_value(&first.delivery).unwrap()["artifact_id"],
            serde_json::to_value(&second.delivery).unwrap()["artifact_id"]
        );
        assert_ne!(first.request_id, second.request_id);
        assert!(size(&second, 4096).is_ok());
        let artifact = catalog
            .pin()
            .await
            .unwrap()
            .artifact(
                &runtime,
                serde_json::to_value(&first.delivery).unwrap()["artifact_id"]
                    .as_str()
                    .unwrap(),
            )
            .await
            .unwrap()
            .unwrap();
        let decoded = blobs.read_delivery(&artifact, "req_check").unwrap();
        assert_eq!(
            decoded.data,
            envelope::fixture_payload(&"🌎\n\"\\".repeat(1000))
        );
    }
}
