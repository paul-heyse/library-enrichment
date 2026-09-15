//! The shared service state one daemon process owns: configuration, resolved state roots, the
//! policy-bound HTTP client, the blob store and the catalog.
//!
//! Built once in the binary after the roots are resolved, then shared by every connection.
//! Tests build one over a temporary directory; nothing here reads `LIBENR_*` itself.

use std::io;

use enrichment_core::clock;
use enrichment_core::config::Config;
use enrichment_core::policy::FetchPolicy;
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
    /// Bind exact request/configuration identities to the native operation before queueing.
    /// This identifies the effective daemon policy; producer admission still enforces it.
    pub fn operation_descriptor(
        &self,
        method: &str,
        request: &impl serde::Serialize,
    ) -> enrichment_store::runtime::OperationDescriptor {
        enrichment_store::runtime::OperationDescriptor {
            method: method.into(),
            request_digest: enrichment_core::canonical::digest_hex(&serde_json::json!([
                "research-operation/2",
                method,
                request
            ])),
            policy_digest: enrichment_core::canonical::digest_hex(&serde_json::json!([
                "effective-operation-policy/2",
                self.config,
            ])),
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
        let runtime = QueryRuntime::new(
            &paths.cache_root.join("query-spill"),
            QueryLimits {
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
                row_groups: arrow.row_groups,
            },
            AdmissionLimits {
                record_bytes: arrow.record_bytes,
                file_bytes: arrow.file_bytes,
                batch_rows: arrow.batch_rows,
                batch_bytes: arrow.batch_bytes,
                cached_snapshots: arrow.cached_snapshots,
                table_rows: arrow.table_rows,
                deadline: std::time::Duration::from_secs(arrow.admission_deadline_seconds),
                ..AdmissionLimits::default()
            },
        )
        .map_err(|e| store_err(&paths.data_root)(io::Error::other(e.to_string())))?;
        let metrics = crate::metrics::Metrics::new();
        let fetcher = Fetcher::new(FetchPolicy::from_config(&config))?
            .with_cache(
                paths.cache_root.join("http"),
                config.freshness.negative_cache_ttl_seconds,
            )
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
        // A private startup runtime can perform native catalog recovery even when this sync
        // constructor is called from a Tokio worker. No library producer is replayed here.
        let jobs = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()?;
                    crate::jobs::Jobs::open_recover(
                        &paths.data_root,
                        permits,
                        config.execution.queue_limit,
                        |record| {
                            runtime.block_on(crate::ops::verify::recover(
                                &repository,
                                &blobs,
                                record,
                            ))
                        },
                    )
                })
                .join()
                .map_err(|_| std::io::Error::other("job recovery worker panicked"))?
        })
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
