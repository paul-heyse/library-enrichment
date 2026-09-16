//! Warm language-server sessions, owned by the daemon.
//!
//! Blueprint §9.2: "The daemon owns warm sessions keyed by language server/version and capsule
//! digest." Starting `ty` or `rust-analyzer` costs seconds; answering a second question about
//! the same capsule should not pay that again. §7.3 budgets **two** warm sessions, evicted by
//! memory budget and idle policy, and that is the default here.
//!
//! The counters matter as much as the reuse. Gate C17 -- "inspection does not start an LSP
//! unnecessarily" -- is only checkable if something counts starts, so [`Manager::metrics`]
//! reports starts and reuses and `service.status` publishes them. A signature read that quietly
//! started a server would show up there as a number that moved.

pub mod client;
mod diagnostics;
pub mod document;
pub mod framing;
mod notifications;
pub mod settings;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use client::Session;
use settings::Server;

/// What identifies one warm session.
///
/// Capsule digest is in the key because a session's answers are only meaningful for the
/// environment it was started against: same server, different dependencies, different truth.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionKey {
    /// Which server.
    pub server: &'static str,
    /// The image it runs in.
    pub image_id: String,
    /// The content digest of the capsule it was started against.
    pub capsule_digest: String,
}

/// Session starts and reuses, for `service.status` and gate C17.
///
/// The shape is the core's, because it is published in `service_status` (§6.3).
pub use enrichment_core::wire::status::LspMetrics as Metrics;

struct Warm {
    session: Session,
    last_used: Instant,
}

/// The daemon's warm sessions.
pub struct Manager {
    sessions: Mutex<HashMap<SessionKey, Warm>>,
    counters: std::sync::Mutex<Metrics>,
    limit: usize,
    idle: Duration,
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager")
            .field("limit", &self.limit)
            .field("idle", &self.idle)
            .field("metrics", &self.metrics())
            .finish()
    }
}

impl Manager {
    /// Build a manager from the configured budget.
    #[must_use]
    pub fn new(limit: usize, idle_seconds: u64) -> Arc<Self> {
        Arc::new(Self {
            sessions: Mutex::new(HashMap::new()),
            counters: std::sync::Mutex::new(Metrics::default()),
            limit: limit.clamp(1, 8),
            idle: Duration::from_secs(idle_seconds.clamp(10, 3600)),
        })
    }

    /// Starts, reuses, evictions and the current warm count.
    #[must_use]
    pub fn metrics(&self) -> Metrics {
        self.counters.lock().map(|m| *m).unwrap_or_default()
    }

    /// Run `query` against a warm session, starting one only if there is none.
    ///
    /// `start` is called only on a miss, so a caller that must not start a server can tell
    /// whether one would have been started by reading [`Metrics::started`] before and after --
    /// which is exactly what C17 does.
    ///
    /// # Errors
    ///
    /// Propagates whatever `start` or `query` returns.
    pub async fn with_session<T, S, Fut, F>(
        &self,
        key: SessionKey,
        cancel: Arc<std::sync::atomic::AtomicBool>,
        supervisor: &crate::execution::cleanup::Supervisor,
        start: S,
        query: F,
    ) -> std::io::Result<T>
    where
        S: FnOnce() -> Fut + Send,
        Fut: Future<Output = std::io::Result<Session>> + Send,
        F: for<'a> FnOnce(&'a mut Session) -> QueryFuture<'a, T> + Send,
    {
        let mut sessions = tokio::select! {
            biased;
            () = cancelled(&cancel) => return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "cancelled before session admission")),
            lock = self.sessions.lock() => lock,
        };
        let warm = match sessions.remove(&key) {
            Some(mut warm) => {
                if let Err(error) = warm.session.admit_current_command().await {
                    warm.session.stop().await?;
                    return Err(error);
                }
                self.bump(|m| m.reused += 1);
                warm
            }
            None => {
                // Evict before starting, so the budget is a ceiling rather than a suggestion.
                self.evict_until(&mut sessions, self.limit.saturating_sub(1))
                    .await?;
                while supervisor.available_permits() == 0 && !sessions.is_empty() {
                    let target = sessions.len() - 1;
                    self.evict_until(&mut sessions, target).await?;
                }
                self.record_warm(&sessions);
                self.bump(|m| m.started += 1);
                Warm {
                    session: start().await?,
                    last_used: Instant::now(),
                }
            }
        };
        let mut warm = warm;
        warm.session.set_cancellation(cancel);
        let result = query(&mut warm.session).await;
        match result {
            Ok(value) => {
                warm.last_used = Instant::now();
                sessions.insert(key, warm);
                self.record_warm(&sessions);
                Ok(value)
            }
            Err(err) => {
                // A session that failed a query may be in an unknown protocol state. Ending it
                // is cheaper than reasoning about what it still believes.
                self.record_warm(&sessions);
                drop(sessions);
                warm.session.stop().await?;
                self.bump(|m| m.evicted += 1);
                Err(err)
            }
        }
    }

    /// Evict every session idle longer than the configured window.
    pub async fn evict_idle(&self) {
        let mut sessions = self.sessions.lock().await;
        let stale: Vec<SessionKey> = sessions
            .iter()
            .filter(|(_, warm)| warm.last_used.elapsed() >= self.idle)
            .map(|(key, _)| key.clone())
            .collect();
        for key in stale {
            if let Some(warm) = sessions.remove(&key) {
                if let Err(error) = warm.session.stop().await {
                    eprintln!("idle language-server cleanup: {error}");
                }
                self.bump(|m| m.evicted += 1);
            }
        }
        self.record_warm(&sessions);
    }

    /// Evict idle sessions and reserve the freed capacity before another warm start can take it.
    /// Cancellation also interrupts waiting for an active query or a busy worker.
    pub async fn execution_lease(
        &self,
        supervisor: &crate::execution::cleanup::Supervisor,
        cancel: &std::sync::atomic::AtomicBool,
    ) -> std::io::Result<Option<Arc<crate::execution::cleanup::Lease>>> {
        let acquire = async {
            let mut sessions = self.sessions.lock().await;
            while supervisor.available_permits() == 0 && !sessions.is_empty() {
                let target = sessions.len() - 1;
                self.evict_until(&mut sessions, target).await?;
            }
            self.record_warm(&sessions);
            let lease = supervisor.lease().await?;
            drop(sessions);
            Ok(Some(lease))
        };
        tokio::select! {
            biased;
            () = async {
                while !cancel.load(std::sync::atomic::Ordering::Acquire) {
                    tokio::time::sleep(Duration::from_millis(25)).await;
                }
            } => Ok(None),
            result = acquire => result,
        }
    }

    /// Stop every warm session and remove its container. Called on shutdown.
    pub async fn shutdown(&self, deadline: tokio::time::Instant) -> std::io::Result<()> {
        tokio::time::timeout_at(deadline, async {
            let mut sessions = self.sessions.lock().await;
            let keys: Vec<SessionKey> = sessions.keys().cloned().collect();
            for key in keys {
                if let Some(warm) = sessions.remove(&key) {
                    warm.session.stop().await?;
                }
            }
            self.record_warm(&sessions);
            Ok(())
        }).await.map_err(|_| std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "language-server shutdown deadline exceeded; owned containers remain subject to cleanup/recovery",
        ))?
    }

    async fn evict_until(
        &self,
        sessions: &mut HashMap<SessionKey, Warm>,
        target: usize,
    ) -> std::io::Result<()> {
        while sessions.len() > target {
            // Least recently used: the one whose answers are least likely to be wanted next.
            let Some(oldest) = sessions
                .iter()
                .min_by_key(|(_, warm)| warm.last_used)
                .map(|(key, _)| key.clone())
            else {
                return Ok(());
            };
            if let Some(warm) = sessions.remove(&oldest) {
                warm.session.stop().await?;
                self.bump(|m| m.evicted += 1);
            }
        }
        Ok(())
    }

    fn bump(&self, change: impl FnOnce(&mut Metrics)) {
        if let Ok(mut counters) = self.counters.lock() {
            change(&mut counters);
        }
    }

    fn record_warm(&self, sessions: &HashMap<SessionKey, Warm>) {
        let warm = sessions.len() as u64;
        self.bump(|m| m.warm = warm);
    }
}

/// Cancellation is shared by independently subscribed durable jobs.
pub async fn cancelled(cancel: &std::sync::atomic::AtomicBool) {
    while !cancel.load(std::sync::atomic::Ordering::Acquire) {
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

/// A query's future, borrowing the session it runs against.
///
/// Boxed because a query holds `&mut Session` across awaits, and an `impl Future` return in a
/// generic closure bound cannot express that borrow. One allocation per language-server query is
/// not worth an unreadable signature.
pub type QueryFuture<'a, T> =
    std::pin::Pin<Box<dyn Future<Output = std::io::Result<T>> + Send + 'a>>;

/// The server that handles an ecosystem.
#[must_use]
pub fn server_for(ecosystem: enrichment_core::identity::Ecosystem) -> Server {
    match ecosystem {
        enrichment_core::identity::Ecosystem::Python => Server::Ty,
        enrichment_core::identity::Ecosystem::Rust => Server::RustAnalyzer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_budget_is_clamped_rather_than_trusted() {
        // A configuration of zero warm sessions would make every query start and stop a server,
        // which is worse than the default rather than stricter than it.
        assert_eq!(Manager::new(0, 0).limit, 1);
        assert_eq!(Manager::new(99, 0).limit, 8);
        assert_eq!(Manager::new(2, 5).idle, Duration::from_secs(10));
        assert_eq!(Manager::new(2, 300).idle, Duration::from_secs(300));
    }

    #[test]
    fn a_key_separates_capsules_as_well_as_servers() {
        // Same server, different dependency closure: different truth, so a different session.
        let key = |digest: &str| SessionKey {
            server: "ty",
            image_id: "sha256:aa".into(),
            capsule_digest: digest.into(),
        };
        assert_ne!(key("one"), key("two"));
        assert_eq!(key("one"), key("one"));
    }

    #[test]
    fn metrics_start_at_zero_so_a_start_is_visible() {
        assert_eq!(Manager::new(2, 60).metrics(), Metrics::default());
    }

    #[tokio::test]
    async fn shutdown_deadline_includes_waiting_for_an_active_query() {
        let manager = Manager::new(1, 60);
        let _query = manager.sessions.lock().await;
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            manager.shutdown(tokio::time::Instant::now() + Duration::from_millis(10)),
        )
        .await
        .expect("outer watchdog")
        .expect_err("manager remains occupied");
        assert_eq!(result.kind(), std::io::ErrorKind::TimedOut);
    }

    #[tokio::test]
    async fn cancelled_admission_finishes_without_waiting_for_unrelated_capacity() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let permits = Arc::new(tokio::sync::Semaphore::new(1));
        let supervisor = crate::execution::cleanup::Supervisor::new(permits, 5, 1);
        let occupied = supervisor.lease().await.expect("first job admitted");
        let manager = Manager::new(1, 60);
        let cancel = Arc::new(AtomicBool::new(false));
        let waiting = tokio::spawn({
            let manager = manager.clone();
            let supervisor = supervisor.clone();
            let cancel = cancel.clone();
            async move { manager.execution_lease(&supervisor, &cancel).await }
        });
        tokio::task::yield_now().await;
        cancel.store(true, Ordering::Release);
        let result = tokio::time::timeout(Duration::from_secs(1), waiting)
            .await
            .expect("cancelled waiter finishes with capacity still occupied")
            .expect("task")
            .expect("admission");
        assert!(result.is_none());
        assert_eq!(supervisor.available_permits(), 0);
        drop(occupied);
        assert_eq!(supervisor.available_permits(), 1);
    }

    #[tokio::test]
    async fn cancelled_admission_does_not_wait_for_the_session_manager_lock() {
        use std::sync::atomic::AtomicBool;
        let supervisor = crate::execution::cleanup::Supervisor::new(
            Arc::new(tokio::sync::Semaphore::new(1)),
            5,
            1,
        );
        let manager = Manager::new(1, 60);
        let _active_query = manager.sessions.lock().await;
        let cancelled = AtomicBool::new(true);
        assert!(
            tokio::time::timeout(
                Duration::from_secs(1),
                manager.execution_lease(&supervisor, &cancelled),
            )
            .await
            .expect("cancelled without waiting for query")
            .expect("admission")
            .is_none()
        );
        assert_eq!(supervisor.available_permits(), 1);
    }
}
