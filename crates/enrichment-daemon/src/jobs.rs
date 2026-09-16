//! Native Delta job coordination; in-memory handles only signal owned running effects.
use enrichment_core::{
    execution::{JobData, VerifyRequest},
    wire::{Envelope, JobState},
};
use enrichment_store::{
    BlobStore,
    control_jobs::{Arguments, JobStore, Resolution},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

/// Closed durable operation inputs; no generic workflow or arbitrary command payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "operation",
    content = "request",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum JobSpec {
    Verify(VerifyRequest),
    Inspect(enrichment_core::request::InspectRequest),
    Resolve(enrichment_core::request::ResolveRequest),
    Compare(enrichment_core::request::CompareRequest),
}
/// Immutable inputs selected by an acquisition before its publication is admitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionStage {
    pub release_id: String,
    pub environment_id: String,
    pub context_id: String,
    pub attempt_id: String,
    pub input_artifact_ids: Vec<String>,
    pub result_artifact_id: String,
}
impl ResolutionStage {
    fn validate(&self) -> io::Result<()> {
        if self.release_id.is_empty()
            || self.environment_id.is_empty()
            || self.context_id.is_empty()
            || self.attempt_id.is_empty()
            || self.input_artifact_ids.is_empty()
            || self.input_artifact_ids.len() > 8192
            || !self.input_artifact_ids.contains(&self.result_artifact_id)
            || self
                .input_artifact_ids
                .iter()
                .any(|id| !enrichment_core::evidence::is_artifact_id(id))
            || self.input_artifact_ids.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(io::Error::other(
                "invalid exact resolution publication stage",
            ));
        }
        Ok(())
    }
}
impl From<VerifyRequest> for JobSpec {
    fn from(r: VerifyRequest) -> Self {
        Self::Verify(r)
    }
}
impl JobSpec {
    fn validate(&self) -> io::Result<()> {
        let pinned = |context: &str, snapshot: &Option<String>| {
            if context.is_empty() || snapshot.as_ref().is_none_or(|s| s.is_empty()) {
                Err(io::Error::other(
                    "durable inspection/verification requires a pinned context and snapshot",
                ))
            } else {
                Ok(())
            }
        };
        match self {
            Self::Verify(r) => pinned(&r.context_id, &r.snapshot_id),
            Self::Inspect(r) => {
                pinned(&r.context_id, &r.snapshot_id)?;
                let options = r.execution.as_ref().ok_or_else(|| {
                    io::Error::other("durable inspection requires explicit execution intent")
                })?;
                if options.intent == enrichment_core::request::InspectionIntent::Retained {
                    return Err(io::Error::other("retained reads are not producer jobs"));
                }
                options.validate(32768).map_err(io::Error::other)
            }
            Self::Resolve(r) => r.validate().map_err(io::Error::other),
            Self::Compare(r) => r.resolutions().map(|_| ()).map_err(io::Error::other),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobRecord {
    pub job_id: String,
    pub key: String,
    pub specification: JobSpec,
    pub state: JobState,
    pub stage: String,
    pub interests: BTreeSet<String>,
    pub detached_interests: BTreeSet<String>,
    pub submitted_at: String,
    pub updated_at: String,
    pub result: Option<Envelope>,
    pub resolution: Option<ResolutionStage>,
}
impl JobRecord {
    pub fn data(&self, token: Option<String>) -> JobData {
        JobData {
            job_id: self.job_id.clone(),
            state: self.state,
            stage: self.stage.clone(),
            interest_token: token,
            active_interests: self.interests.len(),
            submitted_at: self.submitted_at.clone(),
            updated_at: self.updated_at.clone(),
            result: self.result.as_ref().map(Into::into),
        }
    }
}

pub fn terminal(state: JobState) -> bool {
    matches!(
        state,
        JobState::Succeeded | JobState::Partial | JobState::Failed | JobState::Cancelled
    )
}

struct OwnedHandle {
    kind: enrichment_store::native_effect::CommandKind,
    cancel: Arc<AtomicBool>,
    fence: Option<u64>,
    heartbeat: Option<Heartbeat>,
    task: Option<datafusion::common::runtime::SpawnedTask<()>>,
    physically_finished: bool,
}
/// This task can renew ownership but cannot start an effect. Dropping its physical owner
/// aborts it; losing its native grant signals the same cancellation channel as the client.
struct Heartbeat(tokio::task::JoinHandle<()>);
impl Drop for Heartbeat {
    fn drop(&mut self) {
        self.0.abort();
    }
}
struct CancelOnExit(Arc<AtomicBool>);
impl Drop for CancelOnExit {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}
pub struct Jobs {
    native: JobStore,
    cache_root: std::path::PathBuf,
    blobs: BlobStore,
    execution: Arc<crate::execution::cleanup::Supervisor>,
    handles: Mutex<BTreeMap<String, OwnedHandle>>,
    pub permits: Arc<tokio::sync::Semaphore>,
}
impl Jobs {
    pub async fn open_recover<F, Fut>(
        native: JobStore,
        cache_root: std::path::PathBuf,
        blobs: BlobStore,
        permits: Arc<tokio::sync::Semaphore>,
        execution: Arc<crate::execution::cleanup::Supervisor>,
        mut recover: F,
    ) -> io::Result<Self>
    where
        F: FnMut(JobRecord) -> Fut + Send,
        Fut: std::future::Future<Output = io::Result<Option<(JobState, Envelope)>>> + Send,
    {
        let jobs = Self {
            native,
            cache_root,
            blobs,
            execution,
            handles: Mutex::new(BTreeMap::new()),
            permits,
        };
        let pin = jobs.native.pin().await.map_err(io::Error::other)?;
        for state in jobs.native.active(&pin).await.map_err(io::Error::other)? {
            let record = jobs.get(&state.job_id).await?;
            let (state,result)=recover(record.clone()).await?.unwrap_or_else(||(JobState::Failed,crate::envelope::error(
                enrichment_core::wire::ErrorCode::VerificationFailed,
                "The daemon restarted before a terminal outcome was committed.",
                "Physical ownership has been reconciled. Inspect retained evidence before resubmitting.",false)));
            let artifact = jobs.store_result(result).await?;
            jobs.native
                .recover_settled(&record.job_id, state, &artifact)
                .await
                .map_err(io::Error::other)?;
        }
        if jobs.execution.is_idle() {
            jobs.native
                .confirm_terminal_cleanup()
                .await
                .map_err(io::Error::other)?;
        }
        Ok(jobs)
    }
    pub async fn submit(
        &self,
        request: impl Into<JobSpec>,
    ) -> io::Result<(JobRecord, String, bool)> {
        self.reconcile_finished().await?;
        let specification = request.into();
        specification.validate()?;
        let kind = match &specification {
            JobSpec::Resolve(_) => enrichment_store::native_effect::CommandKind::Resolve,
            JobSpec::Compare(_) => enrichment_store::native_effect::CommandKind::Compare,
            JobSpec::Inspect(_) => enrichment_store::native_effect::CommandKind::Inspect,
            JobSpec::Verify(_) => enrichment_store::native_effect::CommandKind::Verify,
        };
        let mut arguments = Arguments::default();
        match specification {
            JobSpec::Verify(request) => arguments.verify = Some(request),
            JobSpec::Inspect(request) => arguments.inspect = Some(request),
            JobSpec::Resolve(request) => arguments.resolve = Some(request),
            JobSpec::Compare(request) => arguments.compare = Some(request),
        }
        let interest = format!("interest_{}", uuid::Uuid::new_v4().simple());
        let (id, fresh) = self
            .native
            .submit(
                self.native
                    .command_input(arguments)
                    .await
                    .map_err(io::Error::other)?,
                interest.clone(),
            )
            .await
            .map_err(io::Error::other)?;
        if fresh {
            self.handles
                .lock()
                .map_err(|_| io::Error::other("owned handle lock"))?
                .insert(
                    id.clone(),
                    OwnedHandle {
                        kind,
                        cancel: Arc::new(AtomicBool::new(false)),
                        fence: None,
                        heartbeat: None,
                        task: None,
                        physically_finished: false,
                    },
                );
        }
        Ok((self.get(&id).await?, interest, fresh))
    }
    pub async fn get(&self, id: &str) -> io::Result<JobRecord> {
        let pin = self.native.pin().await.map_err(io::Error::other)?;
        let command = self
            .native
            .command(&pin, id)
            .await
            .map_err(io::Error::other)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "unknown native job"))?;
        let state = self
            .native
            .transition(&pin, id)
            .await
            .map_err(io::Error::other)?
            .ok_or_else(|| io::Error::other("job transition absent"))?;
        let arguments = command.arguments;
        let specification = match (
            arguments.verify,
            arguments.inspect,
            arguments.resolve,
            arguments.compare,
        ) {
            (Some(value), None, None, None) => JobSpec::Verify(value),
            (None, Some(value), None, None) => JobSpec::Inspect(value),
            (None, None, Some(value), None) => JobSpec::Resolve(value),
            (None, None, None, Some(value)) => JobSpec::Compare(value),
            _ => return Err(io::Error::other("invalid typed native command union")),
        };
        let interests = self
            .native
            .interests(&pin, id)
            .await
            .map_err(io::Error::other)?;
        let result = state
            .result
            .as_ref()
            .map(|artifact| {
                // Publication may settle the job atomically with its complete result.
                // Project the retained descriptor from that selected artifact as well as
                // from ordinary finish; never expose a complete stored envelope as inline.
                crate::delivery::recover_result(&self.blobs, artifact)
            })
            .transpose()?;
        Ok(JobRecord {
            job_id: id.into(),
            key: command.job_key,
            specification,
            state: state.state,
            stage: state.stage,
            interests: interests
                .iter()
                .filter(|i| i.attached)
                .map(|i| i.interest_id.clone())
                .collect(),
            detached_interests: interests
                .iter()
                .filter(|i| !i.attached)
                .map(|i| i.interest_id.clone())
                .collect(),
            submitted_at: command.submitted_at,
            updated_at: state.updated_at,
            result,
            resolution: state.resolution.map(|value| ResolutionStage {
                release_id: value.release_id,
                environment_id: value.environment_id,
                context_id: value.context_id,
                attempt_id: value.attempt_id,
                input_artifact_ids: value.input_artifact_ids,
                result_artifact_id: value.result_artifact_id,
            }),
        })
    }
    pub async fn counts(&self) -> io::Result<(usize, usize)> {
        self.native.counts().await.map_err(io::Error::other)
    }
    pub async fn request_shutdown(&self) -> io::Result<()> {
        self.native
            .request_shutdown()
            .await
            .map_err(io::Error::other)?;
        for handle in self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?
            .values()
        {
            handle.cancel.store(true, Ordering::Release);
        }
        Ok(())
    }
    pub fn cancellation(&self, id: &str) -> io::Result<Arc<AtomicBool>> {
        self.handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?
            .get(id)
            .map(|handle| Arc::clone(&handle.cancel))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no local effect owner"))
    }
    pub async fn cancel(&self, id: &str, token: &str) -> io::Result<JobRecord> {
        self.native
            .cancel(id, token)
            .await
            .map_err(io::Error::other)?;
        let record = self.get(id).await?;
        if record.state == JobState::CancelRequested
            && let Ok(cancel) = self.cancellation(id)
        {
            cancel.store(true, Ordering::Release);
        }
        Ok(record)
    }
    pub async fn start(&self, id: &str) -> io::Result<bool> {
        let execution = self.native.config().execution.clone();
        let cache = self.cache_root.clone();
        let cleanup = match self.execution.admission() {
            crate::execution::cleanup::Admission::Quarantined { detail, .. } => Some(detail),
            crate::execution::cleanup::Admission::Open => None,
        };
        let captured = self
            .native
            .runtime()
            .blocking(move || crate::execution::admission::capture(&execution, &cache, cleanup))
            .await
            .map_err(io::Error::other)?;
        let policy = enrichment_store::execution_policy::Policy::bind(
            self.native.runtime(),
            self.native.config(),
            captured,
        )
        .await
        .map_err(io::Error::other)?;
        let period = self.native.renewal_period().map_err(io::Error::other)?;
        let attempt = format!("attempt_{}", uuid::Uuid::new_v4().simple());
        // No expiry grants a replacement owner. Physical cleanup and the persisted fence do.
        if !self
            .native
            .start(id, &attempt, &policy)
            .await
            .map_err(io::Error::other)?
        {
            return Ok(false);
        }
        let pin = self.native.pin().await.map_err(io::Error::other)?;
        let claim = self
            .native
            .claim(&pin, id)
            .await
            .map_err(io::Error::other)?
            .ok_or_else(|| io::Error::other("claim disappeared"))?;
        {
            let mut handles = self
                .handles
                .lock()
                .map_err(|_| io::Error::other("owned handle lock"))?;
            handles
                .get_mut(id)
                .ok_or_else(|| io::Error::other("local ownership handle absent"))?
                .fence = Some(claim.fence);
        }
        let grant = self
            .native
            .grant(id, claim.fence)
            .await
            .map_err(io::Error::other)?;
        enrichment_store::native_effect::bind_claim(grant).map_err(io::Error::other)?;
        let mut handles = self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?;
        let handle = handles
            .get_mut(id)
            .ok_or_else(|| io::Error::other("local ownership handle absent"))?;
        handle.fence = Some(claim.fence);
        let cancellation = CancelOnExit(Arc::clone(&handle.cancel));
        let native = self.native.clone();
        let id = id.to_owned();
        handle.heartbeat = Some(Heartbeat(tokio::spawn(async move {
            let _cancellation = cancellation;
            loop {
                tokio::time::sleep(period).await;
                if !matches!(native.renew(&id, claim.fence).await, Ok(true)) {
                    break;
                }
            }
        })));
        Ok(true)
    }
    pub fn publication_fence(
        &self,
        id: &str,
    ) -> io::Result<enrichment_store::control::PublicationFence> {
        Ok(enrichment_store::control::PublicationFence {
            owner: self.native.owner().into(),
            fence: self
                .fence(id)?
                .ok_or_else(|| io::Error::other("job has no native claim"))?,
        })
    }
    fn fence(&self, id: &str) -> io::Result<Option<u64>> {
        Ok(self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?
            .get(id)
            .ok_or_else(|| io::Error::other("unknown effect owner"))?
            .fence)
    }
    pub async fn pin_resolution(&self, id: &str, stage: ResolutionStage) -> io::Result<()> {
        stage.validate()?;
        self.native
            .pin_resolution(
                id,
                self.fence(id)?
                    .ok_or_else(|| io::Error::other("acquisition has no claim"))?,
                Resolution {
                    release_id: stage.release_id,
                    environment_id: stage.environment_id,
                    context_id: stage.context_id,
                    attempt_id: stage.attempt_id,
                    input_artifact_ids: stage.input_artifact_ids,
                    result_artifact_id: stage.result_artifact_id,
                },
            )
            .await
            .map_err(io::Error::other)
    }
    async fn store_result(
        &self,
        result: Envelope,
    ) -> io::Result<enrichment_core::evidence::Artifact> {
        self.native
            .retain_delivery(self.blobs.clone(), result)
            .await
            .map_err(io::Error::other)
    }
    pub async fn finish(&self, id: &str, state: JobState, result: Envelope) -> io::Result<()> {
        self.native
            .settle_result(id, self.fence(id)?, state, || self.store_result(result))
            .await
            .map_err(io::Error::other)?;
        Ok(())
    }

    /// Own the driver until it has returned, independently of its terminal publication.
    /// QueryRuntime supplies the native executor and DataFusion's abort-on-drop task handle.
    pub fn spawn(
        self: &Arc<Self>,
        runtime: &enrichment_store::runtime::QueryRuntime,
        id: String,
        work: impl std::future::Future<Output = ()> + Send + 'static,
    ) -> io::Result<()> {
        use futures::FutureExt;
        let mut handles = self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?;
        let handle = handles
            .get_mut(&id)
            .ok_or_else(|| io::Error::other("effect has no local owner"))?;
        if handle.task.is_some() || handle.physically_finished {
            return Err(io::Error::other("effect driver already assigned"));
        }
        let jobs = Arc::clone(self);
        let kind = handle.kind;
        let command_runtime = runtime.clone();
        let work = Box::pin(work);
        handle.task = Some(runtime.spawn(async move {
            // The future and all of its local guards are dropped before cleanup is observed.
            let completion =
                std::panic::AssertUnwindSafe(command_runtime.command(id.clone(), kind, work))
                    .catch_unwind()
                    .await;
            if !matches!(completion, Ok(Ok(()))) {
                eprintln!("native job {id}: driver failed; retained state requires reconciliation");
            }
            if let Err(error) = jobs.settle_returned(&id).await {
                // Retain the completed physical handle so reconciliation can retry the durable
                // settlement. Failure to commit a result must not erase observed physical exit.
                eprintln!("native job {id}: terminal reconciliation: {error}");
            }
            if let Ok(mut handles) = jobs.handles.lock()
                && let Some(handle) = handles.get_mut(&id)
            {
                handle.physically_finished = true;
                handle.heartbeat.take();
            }
            if let Err(error) = jobs.reconcile_finished().await {
                eprintln!("native job {id}: cleanup reconciliation: {error}");
            }
        }));
        Ok(())
    }

    async fn settle_returned(&self, id: &str) -> io::Result<()> {
        self.native
            .settle_unfinished(id, self.fence(id)?, || {
                self.store_result(crate::envelope::error(
                    enrichment_core::wire::ErrorCode::VerificationFailed,
                    "The owned driver exited before retaining a terminal result.",
                    "Inspect the recorded operation diagnostics before explicitly retrying.",
                    false,
                ))
            })
            .await
            .map_err(io::Error::other)
    }

    pub fn has_owned_work(&self) -> bool {
        self.handles
            .lock()
            .map_or(true, |handles| !handles.is_empty())
    }

    pub async fn reconcile_finished(&self) -> io::Result<()> {
        if !self.execution.is_idle() {
            return Ok(());
        }
        let ids = self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?
            .iter()
            .filter(|(_, handle)| handle.physically_finished)
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        for id in &ids {
            self.settle_returned(id).await?;
        }
        self.native
            .confirm_cleanup(&ids)
            .await
            .map_err(io::Error::other)?;
        let mut handles = self
            .handles
            .lock()
            .map_err(|_| io::Error::other("owned handle lock"))?;
        for id in ids {
            handles.remove(&id);
        }
        Ok(())
    }
    pub async fn fail_unfinished(&self, id: &str, error: &io::Error) -> io::Result<()> {
        if terminal(self.get(id).await?.state) {
            return Ok(());
        }
        self.finish(
            id,
            JobState::Failed,
            crate::envelope::error(
                enrichment_core::wire::ErrorCode::ArtifactUnavailable,
                format!("Owned job failed: {error}"),
                "Inspect retained results and cleanup before explicitly retrying.",
                false,
            ),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn terminal_publication_does_not_release_a_live_driver() {
        let root = tempfile::tempdir().unwrap();
        let service = crate::service::Service::open(
            enrichment_core::config::Config::default(),
            enrichment_store::StatePaths::explicit(
                root.path().join("cache"),
                root.path().join("data"),
            ),
        )
        .unwrap();
        let (record, _, _) = service
            .jobs
            .submit(JobSpec::Resolve(enrichment_core::request::ResolveRequest {
                name: "fixture".into(),
                version: Some("1.0.0".into()),
                ..Default::default()
            }))
            .await
            .unwrap();
        let id = record.job_id.clone();
        let jobs = Arc::clone(&service.jobs);
        let (terminal_sent, terminal_seen) = tokio::sync::oneshot::channel();
        let (release, exit) = tokio::sync::oneshot::channel();
        service
            .jobs
            .spawn(&service.repository.runtime, id.clone(), async move {
                assert!(jobs.start(&id).await.unwrap());
                jobs.finish(
                    &id,
                    JobState::Succeeded,
                    crate::envelope::ok(
                        "Native driver result retained",
                        serde_json::Map::new(),
                        enrichment_core::wire::Coverage {
                            details: None,
                            assessments: vec![],
                            scope: "driver ownership".into(),
                            indexed: Default::default(),
                            missing: Default::default(),
                            limitations: vec![],
                        },
                    ),
                )
                .await
                .unwrap();
                terminal_sent.send(()).unwrap();
                exit.await.unwrap();
            })
            .unwrap();
        terminal_seen.await.unwrap();
        assert!(service.jobs.has_owned_work());
        let record = service.jobs.get(&record.job_id).await.unwrap();
        assert_eq!(record.state, JobState::Succeeded);
        assert!(matches!(
            record.result.unwrap().delivery,
            enrichment_core::wire::DeliveryDescriptor::Artifact { .. }
        ));
        let pin = service.jobs.native.pin().await.unwrap();
        assert_eq!(
            service
                .jobs
                .native
                .claim(&pin, &record.job_id)
                .await
                .unwrap()
                .unwrap()
                .cleanup_state,
            "unresolved"
        );
        service.jobs.reconcile_finished().await.unwrap();
        assert!(
            service.jobs.has_owned_work(),
            "a terminal outcome does not prove physical exit"
        );
        release.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(30), async {
            while service.jobs.has_owned_work() {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        let pin = service.jobs.native.pin().await.unwrap();
        assert_eq!(
            service
                .jobs
                .native
                .claim(&pin, &record.job_id)
                .await
                .unwrap()
                .unwrap()
                .cleanup_state,
            "settled"
        );
    }
}
