//! The shared service state one daemon process owns: configuration, resolved state roots, the
//! policy-bound HTTP client, the blob store and the catalog.
//!
//! Built once in the binary after the roots are resolved, then shared by every connection.
//! Tests build one over a temporary directory; nothing here reads `LIBENR_*` itself.

use std::io;

use enrichment_core::clock;
use enrichment_core::config::Config;
use enrichment_store::{
    BlobStore, StatePaths,
    admission::AdmissionLimits,
    dataset::WriteLimits,
    repository::EvidenceRepository,
    runtime::{QueryLimits, QueryRuntime},
};

use crate::fetch::{FetchError, Fetcher};

/// Everything a request handler needs.
#[derive(Clone)]
pub struct Service {
    /// Held for the lifetime of all daemon workers; recovery cannot race a live writer.
    pub writer_lock: std::sync::Arc<std::fs::File>,
    /// Execution ownership is scoped to the cache root, independently of catalog ownership.
    pub cache_writer_lock: std::sync::Arc<std::fs::File>,
    /// Durable core-owned verification jobs.
    pub jobs: std::sync::Arc<crate::jobs::Jobs>,
    /// Owns container cleanup until absence is confirmed, and quarantines admission until then.
    pub execution: std::sync::Arc<crate::execution::cleanup::Supervisor>,
    /// Warm language-server sessions, keyed by server, image and capsule digest.
    pub lsp: std::sync::Arc<crate::lsp::Manager>,
    /// One producer run per distinct extraction, however many callers ask for it (§8.2).
    pub single_flight: std::sync::Arc<crate::single_flight::SingleFlight>,
    /// Operational counters for this process (§14.3), published in `service.status`.
    pub metrics: std::sync::Arc<crate::metrics::Metrics>,
    /// The configuration in force.
    pub config: Config,
    /// Resolved cache and data roots.
    pub paths: StatePaths,
    /// The bounded HTTP client.
    pub fetcher: Fetcher,
    /// Content-addressed artifacts.
    pub blobs: BlobStore,
    /// Releases, contexts and current-snapshot pointers.
    pub repository: EvidenceRepository,
    /// When this service started, RFC 3339.
    pub started_at: String,
    /// The same instant as `started_at`, monotonic, for `health.uptime_seconds`.
    pub started_instant: std::time::Instant,
    /// True only after the pinned static worker completed a valid job in this process.
    pub python_worker_qualified: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
            .field("paths", &self.paths)
            .finish_non_exhaustive()
    }
}

/// Why the service could not be assembled.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    /// A state directory could not be created.
    #[error("cannot open service state under {path}: {source}")]
    Store {
        /// The root at fault.
        path: String,
        /// The I/O failure.
        source: io::Error,
    },
    /// The HTTP client could not be built.
    #[error(transparent)]
    Fetch(#[from] FetchError),
}

impl Service {
    /// Cancel work and await both physical exit and durable reconciliation before reopening roots.
    /// A retained terminal result alone is not evidence that the owned driver has exited.
    pub async fn shutdown(&self) -> io::Result<()> {
        let seconds = self
            .config
            .network
            .request_timeout_seconds
            .max(self.config.execution.deadline_seconds)
            .saturating_add(self.config.execution.cleanup_deadline_seconds)
            .saturating_add(self.config.arrow.query_deadline_seconds);
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(seconds);
        self.jobs.request_shutdown().await?;
        self.lsp.shutdown(deadline).await?;
        loop {
            let reconciled = self.jobs.reconcile_finished().await;
            let counts = self.jobs.counts().await;
            if reconciled.is_ok()
                && matches!(counts, Ok((0, 0)))
                && !self.jobs.has_owned_work()
                && self.execution.is_idle()
            {
                self.repository
                    .runtime
                    .close_diagnostics()
                    .await
                    .map_err(io::Error::other)?;
                return Ok(());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(io::Error::other(format!(
                    "shutdown could not reconcile physical ownership: reconciliation={reconciled:?}, counts={counts:?}, containers={:?}",
                    self.execution.outstanding()
                )));
            }
            // The owned handles remain installed across transient persistence failures.
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        }
    }

    /// Bind exact request/configuration identities to the native operation before queueing.
    /// This identifies the effective daemon policy; producer admission still enforces it.
    pub fn operation_descriptor(
        &self,
        method: &str,
        request: &impl serde::Serialize,
    ) -> enrichment_core::telemetry::OperationDescriptor {
        enrichment_core::telemetry::OperationDescriptor {
            method: method.into(),
            request_digest: enrichment_core::canonical::digest_hex(&serde_json::json!([
                "research-operation/2",
                method,
                request
            ])),
            policy_digest: enrichment_core::native_key::Key::OperationPolicy
                .value(&self.config)
                .expect("validated typed effective configuration"),
        }
    }
    /// Assemble the service over explicit roots.
    ///
    /// # Errors
    ///
    /// Fails if a state directory cannot be created or the HTTP client cannot be built.
    pub fn open(config: Config, paths: StatePaths) -> Result<Self, ServiceError> {
        // A policy the service cannot honour is refused here, before any state is opened.
        config
            .policy
            .validate()
            .map_err(|message| ServiceError::Store {
                path: paths.data_root.display().to_string(),
                source: io::Error::other(message),
            })?;
        let store_err = |path: &std::path::Path| {
            let path = path.display().to_string();
            move |source| ServiceError::Store { path, source }
        };
        enrichment_store::state::initialize(&paths).map_err(store_err(&paths.data_root))?;
        let writer_lock = enrichment_store::state::exclusive(&paths.data_root, ".daemon.lock")
            .map_err(|e| {
                store_err(&paths.data_root)(std::io::Error::other(format!(
                    "another writer owns this data root: {e}"
                )))
            })?;
        enrichment_store::state::recover_staging(&paths).map_err(store_err(&paths.data_root))?;
        let cache_writer_lock =
            enrichment_store::state::exclusive(&paths.cache_root, ".execution-owner.lock")
                .map_err(|e| {
                    store_err(&paths.cache_root)(std::io::Error::other(format!(
                        "another writer owns this cache root: {e}"
                    )))
                })?;
        let blobs = BlobStore::open(&paths.data_root).map_err(store_err(&paths.data_root))?;
        crate::execution::ownership::validate_for_state(&paths, &config.execution)
            .map_err(store_err(&paths.cache_root))?;
        let arrow = &config.arrow;
        let runtime = QueryRuntime::with_diagnostics(
            &paths.cache_root.join("query-spill"),
            &paths.data_root.join("diagnostics"),
            QueryLimits {
                native: arrow.native.clone(),
                memory_bytes: arrow.memory_bytes,
                spill_bytes: arrow.spill_bytes,
                metadata_cache_bytes: arrow.metadata_cache_bytes,
                batch_rows: arrow.batch_rows,
                partitions: arrow.partitions,
                concurrency: arrow.concurrency,
                result_rows: arrow.result_rows,
                result_bytes: arrow.result_bytes,
                deadline: std::time::Duration::from_secs(arrow.query_deadline_seconds),
            },
        )
        .map_err(|e| store_err(&paths.cache_root)(io::Error::other(e.to_string())))?;
        let repository = EvidenceRepository::new(
            paths.clone(),
            runtime,
            WriteLimits {
                record_bytes: arrow.record_bytes,
                batch_rows: arrow.batch_rows,
                batch_bytes: arrow.batch_bytes,
                file_bytes: arrow.file_bytes,
                table_rows: arrow.table_rows,
            },
            AdmissionLimits {
                record_bytes: arrow.record_bytes,
                table_rows: arrow.table_rows,
                deadline: std::time::Duration::from_secs(arrow.admission_deadline_seconds),
            },
        )
        .map_err(|e| store_err(&paths.data_root)(io::Error::other(e.to_string())))?;
        let metrics = crate::metrics::Metrics::new();
        let fetcher = Fetcher::new(&config, &repository.runtime, &paths.data_root)?
            // The fetcher is the only code that opens a socket, so it is the only place that
            // can tell a cache hit from a revalidation from a download (§14.3).
            .with_metrics(std::sync::Arc::clone(&metrics));
        // One permit set covers running work and unresolved cleanup, so the two cannot
        // together exceed the configured concurrency.
        let concurrency = config.limits.expensive_worker_concurrency.clamp(1, 16);
        let permits = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
        let execution = crate::execution::cleanup::Supervisor::new(
            std::sync::Arc::clone(&permits),
            config.execution.cleanup_deadline_seconds,
            concurrency,
        );
        // Reconcile owned containers before the journal turns interrupted work terminal, so a
        // job is never reported finished while its container may still be running.
        crate::execution::Runner::new(
            &config.execution,
            &paths.cache_root,
            std::sync::Arc::clone(&execution),
        )
        .and_then(|runner| runner.recover_owned())
        .map_err(store_err(&paths.cache_root))?;
        crate::execution::budget::recover_orphans(&paths.cache_root)
            .map_err(store_err(&paths.cache_root))?;
        let recovery_repository = repository.clone();
        let recovery_blobs = blobs.clone();
        let recovery_execution = std::sync::Arc::clone(&execution);
        let job_config = std::sync::Arc::new(config.clone());
        let job_cache = paths.cache_root.clone();
        let jobs = repository
            .runtime
            .bootstrap(async move {
                crate::jobs::Jobs::open_recover(
                enrichment_store::control_jobs::JobStore::new(
                    recovery_repository.catalog.clone(),
                    recovery_repository.runtime.clone(),
                    format!("daemon_{}", uuid::Uuid::new_v4().simple()),
                    job_config,
                ),
                job_cache,
                recovery_blobs.clone(),
                permits,
                recovery_execution,
                move |record| {
                    let repository = recovery_repository.clone();
                    let blobs = recovery_blobs.clone();
                    async move {
                        crate::ops::verify::recover(&repository, &blobs, &record).await
                    }
                },
            ).await
            })
            .map_err(|error| store_err(&paths.data_root)(io::Error::other(error)))?
            .map_err(store_err(&paths.data_root))?;
        Ok(Self {
            writer_lock: std::sync::Arc::new(writer_lock),
            cache_writer_lock: std::sync::Arc::new(cache_writer_lock),
            jobs: std::sync::Arc::new(jobs),
            execution,
            // `limits.warm_lsp_sessions` is the key the frozen example configuration
            // documents (blueprint §7.3 budgets two), so it is the key that governs.
            single_flight: crate::single_flight::SingleFlight::new(),
            metrics,
            lsp: crate::lsp::Manager::new(
                config.limits.warm_lsp_sessions.min(concurrency),
                config.execution.lsp_idle_seconds,
            ),
            config,
            paths,
            fetcher,
            blobs,
            repository,
            started_at: clock::now_rfc3339(),
            started_instant: std::time::Instant::now(),
            python_worker_qualified: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// Whether the data root is present and writable, which is what `cache_ready` reports.
    #[must_use]
    pub fn cache_ready(&self) -> bool {
        let probe = self.paths.data_root.join(".write-probe");
        let ok = std::fs::write(&probe, b"").is_ok();
        let _ = std::fs::remove_file(&probe);
        ok
    }
}
