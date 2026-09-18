//! One concrete comparison job owns its two acquisition interests; no alternative resolver.
use super::{common, compare, resolve, resolve_job, verify};
use crate::{envelope, jobs, service::Service};
use enrichment_core::{
    request::CompareRequest,
    wire::{Envelope, ErrorCode, JobState, Outcome},
};
use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub(super) async fn submit(service: &Service, request: CompareRequest) -> Envelope {
    let (record, token, new) = match service
        .jobs
        .submit(jobs::Arguments::Compare {
            request: request.clone(),
        })
        .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "comparison_job"),
    };
    if new {
        let owned = service.clone();
        let id = record.job_id;
        let jobs = std::sync::Arc::clone(&owned.jobs);
        let runtime = owned.repository.runtime.clone();
        if let Err(error) = jobs.spawn(&runtime, id, async move {
            if let Err(error) = owned
                .repository
                .runtime
                .job_operation(
                    id.to_string(),
                    owned.operation_descriptor(&request.clone().into()),
                    std::time::Duration::from_secs(
                        owned.config.network.acquisition_timeout_seconds,
                    ),
                    run(&owned, &id, request),
                )
                .await
            {
                eprintln!("comparison job {id}: {error}");
                let recovery = async {
                    let record = owned.jobs.get(&id).await?;
                    if let Some((state, result)) =
                        recover(&owned.repository, &owned.blobs, &record).await?
                    {
                        owned.jobs.finish(&id, state, result).await
                    } else {
                        owned.jobs.fail_unfinished(&id, &error).await
                    }
                }
                .await;
                if let Err(recovery) = recovery {
                    eprintln!("comparison job {id} terminal recovery: {recovery}");
                }
            }
        }) {
            return common::operation_error(&error, "native_effect_owner");
        }
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

async fn run(
    service: &Service,
    id: &enrichment_core::identity::JobId,
    request: CompareRequest,
) -> io::Result<()> {
    let cancel = service.jobs.cancellation(id)?;
    if !service.jobs.start(id).await? {
        return service
            .jobs
            .finish(
                id,
                JobState::Cancelled,
                failure("comparison cancelled before acquisition"),
            )
            .await;
    }
    let digest = enrichment_core::native_key::Key::ResearchInvocation
        .hex_digest(
            &enrichment_core::operation::identities::ResearchInvocation {
                request: request.clone().into(),
            },
        )
        .expect("declared comparison request identity");
    let mut result = execute(service, request, &cancel, id, &digest).await;
    // Catalog visibility wins a cancellation racing the commit. Reuse admitted bytes.
    if let Some((state, committed)) = recover(
        &service.repository,
        &service.blobs,
        &service.jobs.get(id).await?,
    )
    .await?
    {
        return service.jobs.finish(id, state, committed).await;
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
    service.jobs.finish(id, state, result).await
}

async fn execute(
    service: &Service,
    mut request: CompareRequest,
    cancel: &AtomicBool,
    id: &enrichment_core::identity::JobId,
    digest: &str,
) -> Envelope {
    let prerequisites = match enrichment_store::research_selection::comparison_prerequisites(
        &service.repository.runtime,
        &request,
        service.config.limits.verification_input_bytes,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return failure(error.to_string()),
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
            let (child, token, _) = match resolve_job::subscribe(service, prerequisite).await {
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
    let Some(recovered) =
        enrichment_store::job_recovery::comparison(repository, blobs, &record.snapshot)
            .await
            .map_err(io::Error::other)?
    else {
        return Ok(None);
    };
    let result = crate::delivery::recover_result(
        blobs,
        &repository.runtime,
        &recovered.catalog,
        &recovered.publication.delivery,
    )
    .await?;
    Ok(Some((recovered.publication.state, result)))
}

async fn child_context(
    service: &Service,
    id: &enrichment_core::identity::JobId,
    token: &enrichment_core::identity::InterestId,
    cancel: &AtomicBool,
) -> io::Result<Result<enrichment_core::identity::ContextId, Box<Envelope>>> {
    let mut detached = false;
    loop {
        let mut record = service.jobs.get(id).await?;
        if cancel.load(Ordering::Acquire) && !detached {
            record = service.jobs.cancel(id, token).await?;
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
            if matches!(
                record.snapshot.state,
                JobState::Succeeded | JobState::Partial
            ) {
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
    fn specification() -> jobs::Arguments {
        jobs::Arguments::Resolve {
            request: ResolveRequest {
                name: "enr-fixture".into(),
                version: Some("0.1.0".into()),
                ..Default::default()
            },
        }
    }
    #[tokio::test]
    async fn comparison_cancellation_preserves_another_subscriber() {
        let root = tempfile::tempdir().unwrap();
        let service = service(root.path());
        let (child, comparison_interest, _) = service.jobs.submit(specification()).await.unwrap();
        let (_, surviving_interest, _) = service.jobs.submit(specification()).await.unwrap();
        assert!(service.jobs.start(&child.job_id).await.unwrap());
        let cancelled = AtomicBool::new(true);
        assert!(
            child_context(&service, &child.job_id, &comparison_interest, &cancelled)
                .await
                .unwrap()
                .is_err()
        );
        let record = service.jobs.get(&child.job_id).await.unwrap();
        assert_eq!(record.state, JobState::Running);
        assert_eq!(record.interests, vec![surviving_interest]);
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
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn comparison_uses_terminal_context_when_resolution_delivery_overflows() {
        let root = tempfile::tempdir().unwrap();
        let service = service(root.path());
        let (child, token, _) = service.jobs.submit(specification()).await.unwrap();
        service.jobs.start(&child.job_id).await.unwrap();
        let mut result = envelope::ok(
            "resolved",
            envelope::fixture_payload(&"\\🌎".repeat(200_000)),
            Coverage {
                details: None,
                assessments: Vec::new(),
                scope: "test exact acquisition".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: vec![],
            },
        );
        result.context_id = Some(format!("ctx_{}", "a".repeat(64)).try_into().unwrap());
        result.snapshot_id = Some(format!("snap_{}", "b".repeat(64)).try_into().unwrap());
        service
            .jobs
            .finish(&child.job_id, JobState::Succeeded, result)
            .await
            .unwrap();
        let record = service.jobs.get(&child.job_id).await.unwrap();
        assert!(matches!(
            record.result.unwrap().outcome(),
            Some(Outcome::Ok { .. })
        ));
        assert_eq!(
            child_context(&service, &child.job_id, &token, &AtomicBool::new(false))
                .await
                .unwrap()
                .unwrap(),
            enrichment_core::identity::ContextId::try_from(format!("ctx_{}", "a".repeat(64)))
                .unwrap()
        );
    }
}
