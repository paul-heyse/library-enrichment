//! Concrete acquisition jobs: immutable selection before publication, no replay on restart.
use super::{common, resolve, verify};
use crate::{
    envelope,
    jobs::{self, Resolution},
    service::Service,
};
use enrichment_core::{
    canonical,
    evidence::{ArtifactKind, catalog::PublishedJobKind, snapshot::SnapshotMetadata},
    identity::{Ecosystem, ResearchMode},
    request::ResolveRequest,
    wire::{Envelope, ErrorCode, JobState, Outcome},
};
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
    pub committed: std::sync::OnceLock<(JobState, enrichment_core::identity::SnapshotId, Envelope)>,
    pub id: enrichment_core::identity::JobId,
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
    let (record, token, new) = match subscribe(service, request).await {
        Ok(v) => v,
        Err(e) => return common::operation_error(&e, "resolve_job"),
    };
    delivery(service, record, token, new).await
}

/// Register interest before waiting, so a concrete parent operation owns cancellation even
/// when its request disconnects before the first pending response.
pub(super) async fn subscribe(
    service: &Service,
    request: ResolveRequest,
) -> io::Result<(jobs::JobRecord, enrichment_core::identity::InterestId, bool)> {
    let (record, token, new) = service
        .jobs
        .submit(jobs::Arguments::Resolve {
            request: request.clone(),
        })
        .await?;
    service.single_flight.submitted(new);
    if new {
        let service = service.clone();
        let id = record.job_id;
        let jobs = std::sync::Arc::clone(&service.jobs);
        let runtime = service.repository.runtime.clone();
        jobs.spawn(&runtime, id, async move {
            if let Err(e) = service
                .repository
                .runtime
                .job_operation(
                    id.to_string(),
                    service.operation_descriptor(&request.clone().into()),
                    std::time::Duration::from_secs(
                        service.config.network.acquisition_timeout_seconds,
                    ),
                    run(&service, &id, request),
                )
                .await
            {
                // Control errors and unresolved cleanup remain visible; never report completion.
                eprintln!("acquisition job {id}: {e}");
            }
        })?;
    }
    Ok((record, token, new))
}

async fn delivery(
    service: &Service,
    record: jobs::JobRecord,
    token: enrichment_core::identity::InterestId,
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
                return common::operation_error(
                    &io::Error::other("terminal acquisition lacks its result"),
                    "resolve_job",
                );
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
        Err(e) => common::operation_error(&e, "resolve_job"),
    }
}
async fn run(
    service: &Service,
    id: &enrichment_core::identity::JobId,
    request: ResolveRequest,
) -> io::Result<()> {
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
            return service.jobs.finish(id, state, failure(detail)).await;
        }
    };
    if !service.jobs.start(id).await? {
        return service
            .jobs
            .finish(
                id,
                JobState::Cancelled,
                failure("acquisition cancelled while queued"),
            )
            .await;
    }
    let _timing = service.single_flight.running();
    let work = Work {
        committed: std::sync::OnceLock::new(),
        id: *id,
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
        return service.jobs.finish(id, *state, result.clone()).await;
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
    service.jobs.finish(id, state, result).await
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
    job_id: enrichment_core::identity::JobId,
    request: ResolveRequest,
    stage: Resolution,
    normalization: enrichment_core::producer::ProducerRun,
}

/// Add a semantic output descriptor and actual attempt receipt before native normalization.
pub(super) fn prepare(
    acq: &mut resolve::Acquisition<'_>,
    metadata: &SnapshotMetadata,
) -> Result<Option<(Resolution, enrichment_store::repository::JobCompletion)>, String> {
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
    let stage = Resolution {
        release_id: metadata.release.release_id.clone(),
        environment_id: metadata.environment.environment_id.clone(),
        context_id: metadata.context.context_id.clone(),
        attempt_id: run.attempt_id,
        input_artifact_ids,
        result_artifact_id: result.artifact_id.clone(),
    };
    let receipt = Receipt {
        format: "resolution-attempt/2".into(),
        job_id: work.id,
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
            "producer:resolution-attempt/2",
            None,
        )
        .map_err(|e| e.to_string())?;
    acq.runs
        .last_mut()
        .ok_or("normalization attempt disappeared")?
        .log = Some(log.artifact_id);
    let mut template = acq
        .delivery_template
        .clone()
        .ok_or("resolution delivery template missing before publication")?;
    let enrichment_core::wire::data::ToolData::ResolveLibrary(data) = &mut template.data else {
        return Err("resolution requires a native resolve payload".into());
    };
    data.artifacts = acq.artifacts.clone();
    data.producer_runs = acq.runs.clone();
    data.gaps = acq.gaps.clone();
    for artifact in &acq.artifacts {
        if !template
            .artifacts
            .iter()
            .any(|handle| handle.receipt.artifact_id == artifact.artifact_id)
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
    let native_result =
        enrichment_core::operation::results::ResultRecord::from_envelope(&template)?;
    Ok(Some((
        stage.clone(),
        enrichment_store::repository::JobCompletion {
            publication_fence: acq
                .service
                .jobs
                .publication_fence(&work.id)
                .map_err(|e| e.to_string())?,
            job_id: work.id,
            kind: PublishedJobKind::Resolve,

            attempt_id: stage.attempt_id,
            result_artifact_ids: vec![result.artifact_id],
            result: native_result,
        },
    )))
}
