//! One concrete comparison job owns its two acquisition interests; no alternative resolver.
use super::{common, compare, resolve, resolve_job, verify};
use crate::{envelope, jobs, service::Service};
use enrichment_core::{
    canonical,
    request::CompareRequest,
    wire::{Envelope, ErrorCode, JobState, Outcome},
};
use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub(super) async fn submit(service: &Service, request: CompareRequest) -> Envelope {
    let key = canonical::digest_hex(&serde_json::json!(["comparison-job/1", request]));
    let (record, token, new) = match service
        .jobs
        .submit(key, jobs::JobSpec::Compare(request.clone()))
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "comparison_job"),
    };
    if new {
        let owned = service.clone();
        let id = record.job_id.clone();
        tokio::spawn(async move {
            if let Err(error) = owned
                .repository
                .runtime
                .job_operation(
                    id.clone(),
                    owned.operation_descriptor("library.compare", &request),
                    std::time::Duration::from_secs(
                        owned.config.network.acquisition_timeout_seconds,
                    ),
                    run(&owned, &id, request),
                )
                .await
            {
                eprintln!("comparison job {id}: {error}");
                let recovery = async {
                    let record = owned.jobs.get(&id)?;
                    if let Some((state, result)) =
                        recover(&owned.repository, &owned.blobs, &record).await?
                    {
                        owned.jobs.finish(&id, state, result)
                    } else {
                        owned.jobs.fail_unfinished(&id, &error)
                    }
                }
                .await;
                if let Err(recovery) = recovery {
                    eprintln!("comparison job {id} terminal recovery: {recovery}");
                }
            }
        });
    }
    match verify::wait(
        service,
        &record.job_id,
        service.config.limits.inline_wait_seconds.min(10),
    )
    .await
    {
        Ok(record) if jobs::terminal(record.state) => record
            .result
            .unwrap_or_else(|| failure("terminal comparison lacks a result")),
        Ok(record) => verify::pending(record.data(Some(token))),
        Err(error) => common::operation_error(&error, "comparison_job"),
    }
}

async fn run(service: &Service, id: &str, request: CompareRequest) -> io::Result<()> {
    let cancel = service.jobs.cancellation(id)?;
    if !service.jobs.start(id)? {
        return service.jobs.finish(
            id,
            JobState::Cancelled,
            failure("comparison cancelled before acquisition"),
        );
    }
    let digest = canonical::digest_hex(&serde_json::json!(["comparison-request/1", request]));
    let mut result = execute(service, request, &cancel, id, &digest).await;
    // Catalog visibility wins a cancellation racing the commit. Reuse admitted bytes.
    if let Some((state, committed)) =
        recover(&service.repository, &service.blobs, &service.jobs.get(id)?).await?
    {
        return service.jobs.finish(id, state, committed);
    }
    let state = if cancel.load(Ordering::Acquire) {
        result = failure("comparison cancelled before terminal delivery");
        JobState::Cancelled
    } else {
        match result.outcome() {
            Some(Outcome::Ok { .. }) => JobState::Succeeded,
            Some(Outcome::Partial { .. }) => JobState::Partial,
            _ => JobState::Failed,
        }
    };
    service.jobs.finish(id, state, result)
}

async fn execute(
    service: &Service,
    mut request: CompareRequest,
    cancel: &AtomicBool,
    id: &str,
    digest: &str,
) -> Envelope {
    let prerequisites = match request.resolutions() {
        Ok(value) => value,
        Err(error) => return failure(error),
    };
    let mut contexts = Vec::with_capacity(2);
    for prerequisite in prerequisites {
        if cancel.load(Ordering::Acquire) {
            return failure("comparison cancelled");
        }
        let version = prerequisite.version.as_deref().unwrap_or_default();
        let context = if let Some(retained) =
            resolve::replay_recorded(service, &prerequisite, version).await
        {
            if !matches!(
                retained.outcome(),
                Some(Outcome::Ok { .. } | Outcome::Partial { .. })
            ) {
                return retained;
            }
            let Some(context) = retained.context_id else {
                return failure("retained acquisition lacks context identity");
            };
            context
        } else {
            let (child, token, _) = match resolve_job::subscribe(service, prerequisite) {
                Ok(value) => value,
                Err(error) => return common::operation_error(&error, "comparison_job"),
            };
            match child_context(service, &child.job_id, &token, cancel).await {
                Ok(Ok(context)) => context,
                Ok(Err(result)) => return *result,
                Err(error) => return common::operation_error(&error, "comparison_job"),
            }
        };
        contexts.push(context);
    }
    if cancel.load(Ordering::Acquire) {
        return failure("comparison cancelled");
    }
    request.before_context_id = Some(contexts.remove(0));
    request.after_context_id = Some(contexts.remove(0));
    request.ecosystem = None;
    request.name = None;
    request.from_version = None;
    request.to_version = None;
    // The read pins one catalog generation only after both acquisitions are complete.
    tokio::select! {
        result = compare::read_job(service, request, id, digest) => result,
        () = resolve_job::cancelled(cancel) => failure("comparison cancelled during native query"),
    }
}

/// A restart verifies both input snapshots and indexed result bytes, without rerunning a query.
pub(super) async fn recover(
    repository: &enrichment_store::repository::EvidenceRepository,
    blobs: &enrichment_store::BlobStore,
    record: &jobs::JobRecord,
) -> io::Result<Option<(JobState, Envelope)>> {
    let jobs::JobSpec::Compare(request) = &record.specification else {
        return Err(io::Error::other(
            "comparison publication has a different journal kind",
        ));
    };
    let fail = |error| io::Error::other(error);
    let catalog = repository.catalog.pin().await.map_err(fail)?;
    let Some(publication) = catalog
        .comparison_publication(&repository.runtime, &record.job_id)
        .await
        .map_err(fail)?
    else {
        return Ok(None);
    };
    let digest = canonical::digest_hex(&serde_json::json!(["comparison-request/1", request]));
    if publication.request_digest != digest {
        return Err(io::Error::other(
            "comparison publication request differs from journal",
        ));
    }
    let before = repository
        .open_snapshot(catalog.clone(), &publication.before_snapshot_id)
        .await
        .map_err(fail)?;
    let after = repository
        .open_snapshot(catalog, &publication.after_snapshot_id)
        .await
        .map_err(fail)?;
    if before.manifest.context_id != publication.before_context_id
        || after.manifest.context_id != publication.after_context_id
    {
        return Err(io::Error::other(
            "comparison publication input ownership differs",
        ));
    }
    repository
        .validate_comparison_delivery(&publication)
        .await
        .map_err(fail)?;
    let result = crate::delivery::recover_result(blobs, &publication.delivery)?;
    Ok(Some((publication.state, result)))
}

async fn child_context(
    service: &Service,
    id: &str,
    token: &str,
    cancel: &AtomicBool,
) -> io::Result<Result<String, Box<Envelope>>> {
    let mut detached = false;
    loop {
        let mut record = service.jobs.get(id)?;
        if cancel.load(Ordering::Acquire) && !detached {
            record = service.jobs.cancel(id, token)?;
            detached = true;
        }
        if detached && !record.interests.is_empty() {
            return Ok(Err(Box::new(failure(
                "comparison cancelled; another subscriber retains acquisition",
            ))));
        }
        if jobs::terminal(record.state) {
            if detached {
                return Ok(Err(Box::new(failure(
                    "comparison cancelled; acquisition cleanup settled",
                ))));
            }
            let result = record
                .result
                .ok_or_else(|| io::Error::other("terminal acquisition lacks result"))?;
            if matches!(record.state, JobState::Succeeded | JobState::Partial) {
                // A large successful resolve can have a BUDGET_EXCEEDED delivery envelope.
                // Its durable terminal state and preserved context identify the prerequisite;
                // comparison does not need to deserialize the unrelated full resolve payload.
                let context = result.context_id.ok_or_else(|| {
                    io::Error::other("completed acquisition lacks context identity")
                })?;
                return Ok(Ok(context));
            }
            return Ok(Err(Box::new(result)));
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

fn failure(message: impl Into<String>) -> Envelope {
    envelope::error(
        ErrorCode::ArtifactUnavailable,
        message,
        "Inspect the comparison job and retry explicitly when its prerequisites are available.",
        false,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{config::Config, request::ResolveRequest, wire::Coverage};
    use enrichment_store::StatePaths;
    fn service(root: &std::path::Path) -> Service {
        Service::open(
            Config::default(),
            StatePaths::explicit(root.join("cache"), root.join("data")),
        )
        .unwrap()
    }
    fn specification() -> jobs::JobSpec {
        jobs::JobSpec::Resolve(ResolveRequest {
            name: "enr-fixture".into(),
            version: Some("0.1.0".into()),
            ..Default::default()
        })
    }
    #[tokio::test]
    async fn comparison_cancellation_preserves_another_subscriber() {
        let root = tempfile::tempdir().unwrap();
        let service = service(root.path());
        let (child, comparison_interest, _) = service
            .jobs
            .submit("shared".into(), specification())
            .unwrap();
        let (_, surviving_interest, _) = service
            .jobs
            .submit("shared".into(), specification())
            .unwrap();
        assert!(service.jobs.start(&child.job_id).unwrap());
        let cancelled = AtomicBool::new(true);
        assert!(
            child_context(&service, &child.job_id, &comparison_interest, &cancelled)
                .await
                .unwrap()
                .is_err()
        );
        let record = service.jobs.get(&child.job_id).unwrap();
        assert_eq!(record.state, JobState::Running);
        assert_eq!(record.interests, [surviving_interest].into());
        assert!(
            !service
                .jobs
                .cancellation(&child.job_id)
                .unwrap()
                .load(Ordering::Acquire)
        );
        service
            .jobs
            .finish(
                &child.job_id,
                JobState::Failed,
                failure("test owner completed"),
            )
            .unwrap();
    }
    #[tokio::test]
    async fn comparison_uses_terminal_context_when_resolution_delivery_overflows() {
        let root = tempfile::tempdir().unwrap();
        let service = service(root.path());
        let (child, token, _) = service
            .jobs
            .submit("large".into(), specification())
            .unwrap();
        service.jobs.start(&child.job_id).unwrap();
        let mut result = envelope::ok(
            "resolved",
            common::to_object(&serde_json::json!({"text":"\\🌎".repeat(200_000)})),
            Coverage {
                details: None,
                assessments: Vec::new(),
                scope: "test exact acquisition".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: vec![],
            },
        );
        result.context_id = Some("ctx_exact".into());
        result.snapshot_id = Some("snap_exact".into());
        service
            .jobs
            .finish(&child.job_id, JobState::Succeeded, result)
            .unwrap();
        let record = service.jobs.get(&child.job_id).unwrap();
        assert!(matches!(
            record.result.unwrap().outcome(),
            Some(Outcome::Ok { .. })
        ));
        assert_eq!(
            child_context(&service, &child.job_id, &token, &AtomicBool::new(false))
                .await
                .unwrap()
                .unwrap(),
            "ctx_exact"
        );
    }
}
