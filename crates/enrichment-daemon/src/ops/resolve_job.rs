//! Concrete acquisition jobs: immutable selection before publication, no replay on restart.
use super::{common, resolve, verify};
use crate::{
    envelope,
    jobs::{self, ResolutionStage},
    service::Service,
};
use enrichment_core::{
    canonical,
    evidence::{ArtifactKind, catalog::PublishedJobKind, snapshot::SnapshotMetadata},
    identity::{Ecosystem, ResearchMode},
    request::ResolveRequest,
    wire::{Envelope, ErrorCode, JobState, Outcome},
};
use enrichment_store::{BlobStore, SnapshotReader, repository::EvidenceRepository};
use serde::{Deserialize, Serialize};
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub(super) struct Work {
    pub committed: std::sync::OnceLock<(JobState, String, Envelope)>,
    pub id: String,
    pub request: ResolveRequest,
    pub cancel: Arc<AtomicBool>,
    pub lease: Arc<crate::execution::cleanup::Lease>,
}
impl Work {
    pub fn check(&self) -> io::Result<()> {
        if self.cancel.load(Ordering::Acquire) {
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "acquisition cancelled before publication",
            ))
        } else {
            Ok(())
        }
    }
}
pub(super) async fn cancelled(cancel: &AtomicBool) {
    while !cancel.load(Ordering::Acquire) {
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

pub(super) async fn submit(service: &Service, request: ResolveRequest) -> Envelope {
    let (record, token, new) = match subscribe(service, request) {
        Ok(v) => v,
        Err(e) => return common::store_error(&e),
    };
    delivery(service, record, token, new).await
}

/// Register interest before waiting, so a concrete parent operation owns cancellation even
/// when its request disconnects before the first pending response.
pub(super) fn subscribe(
    service: &Service,
    request: ResolveRequest,
) -> io::Result<(jobs::JobRecord, String, bool)> {
    let key = resolve::acquisition_key(service, &request);
    let (record, token, new) = service
        .jobs
        .submit(key, jobs::JobSpec::Resolve(request.clone()))?;
    service.single_flight.submitted(new);
    if new {
        let service = service.clone();
        let id = record.job_id.clone();
        tokio::spawn(async move {
            if let Err(e) = run(&service, &id, request).await {
                // Journal errors and unresolved cleanup remain visible; never report completion.
                eprintln!("acquisition job {id}: {e}");
            }
        });
    }
    Ok((record, token, new))
}

async fn delivery(
    service: &Service,
    record: jobs::JobRecord,
    token: String,
    new: bool,
) -> Envelope {
    match verify::wait(
        service,
        &record.job_id,
        service.config.limits.inline_wait_seconds.min(10),
    )
    .await
    {
        Ok(record) if jobs::terminal(record.state) => {
            let Some(mut result) = record.result else {
                return common::store_error(&io::Error::other(
                    "terminal acquisition lacks its result",
                ));
            };
            if !new {
                result
                    .coverage
                    .limitations
                    .push(resolve::SHARED_RUN_LIMITATION.into());
            }
            result
        }
        Ok(record) => {
            let mut result = verify::pending(record.data(Some(token)));
            if !new {
                result
                    .coverage
                    .limitations
                    .push(resolve::SHARED_RUN_LIMITATION.into());
            }
            result
        }
        Err(e) => common::store_error(&e),
    }
}
async fn run(service: &Service, id: &str, request: ResolveRequest) -> io::Result<()> {
    let cancel = service.jobs.cancellation(id)?;
    let lease = match service
        .lsp
        .execution_lease(&service.execution, &cancel)
        .await
    {
        Ok(Some(v)) => v,
        result => {
            let (state, detail) = match result {
                Ok(None) => (
                    JobState::Cancelled,
                    "cancelled while waiting for acquisition capacity".to_owned(),
                ),
                Err(e) => (
                    JobState::Failed,
                    format!("acquisition admission failed: {e}"),
                ),
                Ok(Some(_)) => unreachable!(),
            };
            return service.jobs.finish(id, state, failure(detail));
        }
    };
    if !service.jobs.start(id)? {
        return service.jobs.finish(
            id,
            JobState::Cancelled,
            failure("acquisition cancelled while queued"),
        );
    }
    let _timing = service.single_flight.running();
    let work = Work {
        committed: std::sync::OnceLock::new(),
        id: id.into(),
        request: request.clone(),
        cancel,
        lease,
    };
    let acquisition = async {
        if request.effective_mode() == ResearchMode::Revision {
            super::revision::resolve(service, request.clone(), &work).await
        } else if request.ecosystem == Ecosystem::Python {
            super::python::acquire(service, &request, &work).await
        } else {
            resolve::acquire(service, &request, &work).await
        }
    };
    tokio::pin!(acquisition);
    let mut timed_out = false;
    let result = tokio::select! {
        result = &mut acquisition => result,
        () = tokio::time::sleep(Duration::from_secs(service.config.network.acquisition_timeout_seconds)) => {
            timed_out = true;
            work.cancel.store(true, Ordering::Release);
            // Keep ownership until the bounded fetch/worker/container paths settle. Dropping the
            // publication future here could lose the success response after the catalog commit.
            acquisition.await
        }
    };
    service.execution.wait_for_cleanup(&work.lease).await?;
    // A catalog commit wins cancellation that arrived after publication admission.
    if let Some((state, _, result)) = work.committed.get() {
        return service.jobs.finish(id, *state, result.clone());
    }
    let (state, result) = if timed_out {
        (
            JobState::Failed,
            failure("acquisition deadline exceeded; producer cleanup settled"),
        )
    } else if work.cancel.load(Ordering::Acquire) {
        (
            JobState::Cancelled,
            failure("acquisition cancelled; producer cleanup settled"),
        )
    } else {
        let state = match result.outcome() {
            Some(Outcome::Ok { .. }) => JobState::Succeeded,
            Some(Outcome::Partial { .. }) => JobState::Partial,
            _ => JobState::Failed,
        };
        (state, result)
    };
    service.jobs.finish(id, state, result)
}
fn failure(detail: impl Into<String>) -> Envelope {
    envelope::error(
        ErrorCode::ExtractionFailed,
        detail,
        "Inspect the recorded job and prerequisites; resubmit explicitly if acquisition is still needed.",
        false,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolutionOutput {
    format: String,
    metadata: SnapshotMetadata,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    format: String,
    job_id: String,
    request: ResolveRequest,
    stage: ResolutionStage,
    normalization: enrichment_core::producer::ProducerRun,
}

/// Add a semantic output descriptor and actual attempt receipt before native normalization.
pub(super) fn prepare(
    acq: &mut resolve::Acquisition<'_>,
    metadata: &SnapshotMetadata,
) -> Result<
    Option<(
        ResolutionStage,
        enrichment_store::repository::JobCompletion,
        crate::delivery::DeliverySlot,
    )>,
    String,
> {
    let Some(work) = acq.work else {
        return Ok(None);
    };
    work.check().map_err(|e| e.to_string())?;
    let value = ResolutionOutput {
        format: "resolution-result/1".into(),
        metadata: metadata.clone(),
    };
    let bytes =
        canonical::to_canonical_string(&serde_json::to_value(value).map_err(|e| e.to_string())?);
    let result = acq
        .store_bytes(
            bytes.as_bytes(),
            ArtifactKind::Other,
            "application/json",
            "producer:resolution-result/1",
            None,
        )
        .map_err(|e| e.to_string())?;
    let run = acq
        .runs
        .last_mut()
        .ok_or("resolution has no normalization attempt")?;
    if run.log.is_some() {
        return Err("normalization already has an unexpected attempt log".into());
    }
    run.inputs
        .insert("resolution-result".into(), result.sha256.clone());
    let mut input_artifact_ids: Vec<_> = acq
        .artifacts
        .iter()
        .filter(|a| run.inputs.values().any(|d| d == &a.sha256))
        .map(|a| a.artifact_id.clone())
        .collect();
    input_artifact_ids.sort();
    input_artifact_ids.dedup();
    let stage = ResolutionStage {
        release_id: metadata.release.release_id.to_string(),
        environment_id: metadata.environment.environment_id.to_string(),
        context_id: metadata.context.context_id.to_string(),
        attempt_id: run.attempt_id.clone(),
        input_artifact_ids,
        result_artifact_id: result.artifact_id.clone(),
    };
    let receipt = Receipt {
        format: "resolution-attempt/1".into(),
        job_id: work.id.clone(),
        request: work.request.clone(),
        stage: stage.clone(),
        normalization: run.clone(),
    };
    let bytes = serde_json::to_vec(&receipt).map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 {
        return Err("resolution attempt exceeds reconstruction bound".into());
    }
    let log = acq
        .store_bytes(
            &bytes,
            ArtifactKind::Other,
            "application/json",
            "producer:resolution-attempt/1",
            None,
        )
        .map_err(|e| e.to_string())?;
    acq.runs
        .last_mut()
        .ok_or("normalization attempt disappeared")?
        .log = Some(log.artifact_id);
    let state = if acq.gaps.is_empty() {
        JobState::Succeeded
    } else {
        JobState::Partial
    };
    let mut template = acq
        .delivery_template
        .clone()
        .ok_or("resolution delivery template missing before publication")?;
    // The payload's metadata was assembled by the actual acquisition path. Late provenance is
    // filled once, before candidate rendering; the snapshot summary is bound by the coordinator.
    for (key, rows) in [
        (
            "artifacts",
            serde_json::to_value(&acq.artifacts).map_err(|e| e.to_string())?,
        ),
        (
            "producer_runs",
            serde_json::to_value(&acq.runs).map_err(|e| e.to_string())?,
        ),
    ] {
        let existing = template
            .data
            .entry(key)
            .or_insert_with(|| serde_json::json!([]));
        let entries = existing
            .as_array_mut()
            .ok_or("resolution provenance must be an array")?;
        for row in rows
            .as_array()
            .ok_or("resolution provenance serializer returned a non-array")?
        {
            if key == "producer_runs"
                && let Some(existing) = entries
                    .iter_mut()
                    .find(|existing| existing["attempt_id"] == row["attempt_id"])
            {
                // The template predates the final config/input/log fields for this attempt.
                // It is the same attempt, so replace its provisional presentation instead of
                // returning two contradictory records with one identity.
                *existing = row.clone();
            } else if !entries.contains(row) {
                entries.push(row.clone());
            }
        }
    }
    template.data.insert(
        "gaps".into(),
        serde_json::to_value(&acq.gaps).map_err(|e| e.to_string())?,
    );
    for artifact in &acq.artifacts {
        if !template
            .artifacts
            .iter()
            .any(|handle| handle.artifact_id == artifact.artifact_id)
            && let Some(handle) = common::handle_for(
                artifact,
                "Acquisition input, result or attempt receipt".into(),
            )
        {
            template.artifacts.push(handle);
        }
    }
    crate::delivery::size(&template, crate::delivery::MAX_RESULT_BYTES)
        .map_err(|e| e.to_string())?;
    let (prepare_delivery, delivery) = crate::delivery::prepare_job(
        acq.service.blobs.clone(),
        move |manifest| {
            let mut result = template.clone();
            result.context_id = Some(manifest.context_id.to_string());
            result.snapshot_id = Some(manifest.snapshot_id.to_string());
            result.summary = format!(
                "Published {} for {} {}; indexed {} definitions and {} evidence fragments. See coverage and gaps for qualified limits.",
                manifest.snapshot_id,
                manifest.crate_name,
                manifest
                    .crate_version
                    .as_deref()
                    .unwrap_or("selected revision"),
                manifest.counts.definitions,
                manifest.counts.fragments,
            );
            result.data.insert(
                "snapshot".into(),
                serde_json::to_value(enrichment_core::wire::data::SnapshotSummary {
                    snapshot_id: manifest.snapshot_id.to_string(),
                    normalizer_version: manifest.normalizer_version.clone(),
                    counts: manifest.counts.clone(),
                    published_at: manifest.published_at.clone(),
                })?,
            );
            result
                .coverage
                .indexed
                .extend(manifest.indexed.iter().map(|kind| kind.as_str().to_owned()));
            result
                .coverage
                .missing
                .extend(manifest.missing.iter().map(|kind| kind.as_str().to_owned()));
            Ok(Envelope::new(
                enrichment_core::wire::EnvelopeBody {
                    request_id: result.request_id,
                    summary: result.summary,
                    context_id: result.context_id,
                    snapshot_id: result.snapshot_id,
                    data: result.data,
                    coverage: result.coverage,
                    freshness: result.freshness,
                    evidence: result.evidence,
                    artifacts: result.artifacts,
                    pagination: result.pagination,
                },
                if state == JobState::Partial {
                    Outcome::Partial { job: None }
                } else {
                    Outcome::Ok { job: None }
                },
            ))
        },
    );
    Ok(Some((
        stage.clone(),
        enrichment_store::repository::JobCompletion {
            job_id: work.id.clone(),
            kind: PublishedJobKind::Resolve,
            state,
            attempt_id: stage.attempt_id,
            result_artifact_ids: vec![result.artifact_id],
            prepare_delivery,
        },
        delivery,
    )))
}

/// Recover only a committed exact result, without fetching or starting producers.
pub(super) async fn recover(
    repository: &EvidenceRepository,
    blobs: &BlobStore,
    record: &jobs::JobRecord,
) -> io::Result<Option<(JobState, Envelope)>> {
    let error = |e: enrichment_store::QueryError| io::Error::other(e.to_string());
    let catalog = repository
        .catalog
        .pin()
        .await
        .map_err(|e| io::Error::other(e.to_string()))?;
    let Some(publication) = catalog
        .job_publication(&repository.runtime, &record.job_id)
        .await
        .map_err(|e| io::Error::other(e.to_string()))?
    else {
        return Ok(None);
    };
    let jobs::JobSpec::Resolve(request) = &record.specification else {
        return Err(io::Error::other(
            "resolution publication has a different operation",
        ));
    };
    let stage = record
        .resolution
        .as_ref()
        .ok_or_else(|| io::Error::other("committed resolution lacks pinned exact inputs"))?;
    if publication.kind != PublishedJobKind::Resolve
        || publication.attempt_id != stage.attempt_id
        || publication.context_id.as_str() != stage.context_id
        || publication.result_artifact_ids != [stage.result_artifact_id.clone()]
    {
        return Err(io::Error::other(
            "committed resolution disagrees with its exact stage",
        ));
    }
    let reader = SnapshotReader::open(repository, catalog, &publication.snapshot_id)
        .await
        .map_err(error)?;
    let manifest = reader.manifest();
    if manifest.metadata.release_id.as_str() != stage.release_id
        || manifest.metadata.environment_id.as_str() != stage.environment_id
    {
        return Err(io::Error::other(
            "resolution identity disagrees with catalog",
        ));
    }
    let attempt = reader.attempt(&stage.attempt_id).await.map_err(error)?;
    let log = attempt
        .artifacts
        .iter()
        .find(|a| Some(&a.artifact_id) == attempt.run.log.as_ref())
        .ok_or_else(|| io::Error::other("resolution attempt log missing"))?;
    if log.size_bytes > 1024 * 1024 {
        return Err(io::Error::other(
            "resolution log exceeds reconstruction bound",
        ));
    }
    let receipt: Receipt = blobs.read_json(log, 1024 * 1024)?;
    let mut expected_run = attempt.run.clone();
    expected_run.log = None;
    if receipt.format != "resolution-attempt/1"
        || receipt.job_id != record.job_id
        || receipt.request != *request
        || receipt.stage != *stage
        || receipt.normalization != expected_run
    {
        return Err(io::Error::other(
            "resolution receipt differs from journal or producing attempt",
        ));
    }
    let result = attempt
        .artifacts
        .iter()
        .find(|a| a.artifact_id == stage.result_artifact_id)
        .ok_or_else(|| io::Error::other("resolution output missing"))?;
    if result.size_bytes > 1024 * 1024 {
        return Err(io::Error::other(
            "resolution output exceeds reconstruction bound",
        ));
    }
    let output: ResolutionOutput = blobs.read_json(result, 1024 * 1024)?;
    let bytes = canonical::to_canonical_string(&serde_json::to_value(&output)?);
    if canonical::sha256_hex(bytes.as_bytes()) != result.sha256
        || output.format != "resolution-result/1"
        || output.metadata.context.context_id != manifest.metadata.context_id
        || output.metadata.environment.environment_id != manifest.metadata.environment_id
        || output.metadata.release.release_id != manifest.metadata.release_id
    {
        return Err(io::Error::other(
            "resolution output differs from its canonical exact identity",
        ));
    }
    let mut inputs: Vec<_> = attempt
        .artifacts
        .iter()
        .filter(|a| attempt.run.inputs.values().any(|d| d == &a.sha256))
        .map(|a| a.artifact_id.clone())
        .collect();
    inputs.sort();
    inputs.dedup();
    if inputs != stage.input_artifact_ids {
        return Err(io::Error::other(
            "resolution input closure differs from its pinned stage",
        ));
    }
    output.metadata.validate().map_err(io::Error::other)?;
    repository
        .validate_delivery(&publication)
        .await
        .map_err(|e| io::Error::other(e.to_string()))?;
    Ok(Some((
        publication.state,
        crate::delivery::recover_job(blobs, &publication)?,
    )))
}
