//! In-process ownership of container cleanup.
//!
//! The native physical-owner control record recovers a leaked container on the *next*
//! daemon start. That is the cross-restart boundary, and it is not immediate cleanup: between
//! the failure and the restart the container keeps running, and the job that owned it has
//! already returned its worker permit.
//!
//! This module closes that window. Every container is registered here before it is created and
//! stays registered until its absence is independently confirmed. Two things follow:
//!
//! * A removal that has begun and not finished retains its original worker permit while it retries, so
//!   accumulating half-removed containers cannot exceed the concurrency bound.
//! * Once a removal is abandoned, or once removals occupy the whole bound, new execution
//!   admission is quarantined rather than queued behind a boundary nobody is watching.
//!
//! A container that is merely *running* is neither of those. It is a busy service, and the
//! durable job queue is what absorbs that -- see [`Stage`].
//!
//! Dropping an execution future -- a cancelled task, a panicking worker, a runtime shutdown --
//! goes through [`ContainerGuard`]'s `Drop`, which schedules the same retry. `kill_on_drop`
//! reaps the Podman *client*; only removal reaps the container.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::Runner;

/// Whether new execution work may be admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admission {
    /// Every owned container is accounted for.
    Open,
    /// Cleanup is unresolved; explain it rather than queueing behind it.
    Quarantined {
        /// How many owned containers are outstanding.
        outstanding: usize,
        /// The concrete reason, for the caller's `next_action`.
        detail: String,
    },
}

/// What an owned container is currently doing.
///
/// "Running" and "not yet removed" are different facts and they deserve different answers. A
/// container doing the work it was created for is a busy service; a container whose removal is
/// being retried is a boundary that is not holding. Quarantining on the first would refuse work
/// the durable job queue exists to absorb, and would report "cleanup is unresolved" about a
/// service that is merely occupied.
#[derive(Debug, PartialEq, Eq)]
enum Stage {
    /// Executing, bounded by its own deadline. Nothing is wrong.
    Running,
    /// Removal has begun and absence is not yet confirmed.
    Removing,
    /// Removal was retried until the deadline and gave up. Only a restart resolves this.
    Abandoned(String),
}

/// One owned container whose absence is not yet confirmed.
#[derive(Debug)]
struct Outstanding {
    since: Instant,
    stage: Stage,
    // The same admitted permit, held even after retry exhaustion until reconciliation.
    _lease: Option<Arc<Lease>>,
}

/// One execution admission shared across preparation, execution and container cleanup.
#[derive(Debug)]
pub struct Lease {
    _permit: tokio::sync::OwnedSemaphorePermit,
    active: Mutex<Option<String>>,
    retained_locks: Mutex<Vec<std::fs::File>>,
}

impl Lease {
    /// Keep a retained capsule locked through warm use and any unresolved cleanup.
    pub(crate) fn hold_capsule_lock(&self, path: &std::path::Path) -> std::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        file.try_lock().map_err(|error| {
            std::io::Error::other(format!(
                "retained capsule is in use or awaiting cleanup: {error}",
            ))
        })?;
        self.retained_locks
            .lock()
            .map_err(|_| std::io::Error::other("capsule ownership lock poisoned"))?
            .push(file);
        Ok(())
    }

    pub(crate) fn active_container(&self) -> bool {
        // Poisoned ownership is unconfirmed, never permission to delete a live mount.
        self.active
            .lock()
            .map(|active| active.is_some())
            .unwrap_or(true)
    }
}

/// Tracks every owned container until its absence is confirmed.
#[derive(Debug)]
pub struct Supervisor {
    outstanding: Mutex<BTreeMap<String, Outstanding>>,
    permits: Arc<tokio::sync::Semaphore>,
    deadline: Duration,
    concurrency: usize,
    changed: tokio::sync::watch::Sender<u64>,
}

impl Supervisor {
    /// Capacity not currently held by execution or unconfirmed cleanup.
    #[must_use]
    pub fn available_permits(&self) -> usize {
        self.permits.available_permits()
    }

    /// Build a supervisor over the worker semaphore jobs already share.
    #[must_use]
    pub fn new(
        permits: Arc<tokio::sync::Semaphore>,
        cleanup_deadline_seconds: u64,
        concurrency: usize,
    ) -> Arc<Self> {
        Arc::new(Self {
            outstanding: Mutex::new(BTreeMap::new()),
            permits,
            deadline: Duration::from_secs(cleanup_deadline_seconds.clamp(5, 3600)),
            concurrency: concurrency.clamp(1, 16),
            changed: tokio::sync::watch::channel(0).0,
        })
    }

    /// Obtain one permit, refusing quarantine before and after waiting for capacity.
    pub async fn lease(&self) -> std::io::Result<Arc<Lease>> {
        let mut changed = self.changed.subscribe();
        loop {
            if let Admission::Quarantined { detail, .. } = self.admission() {
                return Err(std::io::Error::other(detail));
            }
            tokio::select! {
                permit = Arc::clone(&self.permits).acquire_owned() => {
                    let permit = permit.map_err(std::io::Error::other)?;
                    if let Admission::Quarantined { detail, .. } = self.admission() {
                        return Err(std::io::Error::other(detail));
                    }
                    return Ok(Arc::new(Lease { _permit: permit, active: Mutex::new(None), retained_locks: Mutex::new(Vec::new()) }));
                },
                _ = changed.changed() => {},
            }
        }
    }

    /// Wait for this lease's container to be absent, or name an abandoned cleanup obligation.
    /// A terminal job cannot imply completed cancellation before this succeeds.
    pub async fn wait_for_cleanup(&self, lease: &Lease) -> std::io::Result<()> {
        let mut changed = self.changed.subscribe();
        loop {
            {
                let outstanding = self
                    .outstanding
                    .lock()
                    .map_err(|_| std::io::Error::other("cleanup ownership lock poisoned"))?;
                let active = lease
                    .active
                    .lock()
                    .map_err(|_| std::io::Error::other("execution lease poisoned"))?;
                let Some(name) = active.as_ref() else {
                    return Ok(());
                };
                match outstanding.get(name) {
                    Some(Outstanding {
                        stage: Stage::Abandoned(reason),
                        ..
                    }) => {
                        return Err(std::io::Error::other(format!(
                            "container {name} cleanup is unconfirmed: {reason}; execution remains quarantined",
                        )));
                    }
                    Some(_) => {}
                    None => {
                        return Err(std::io::Error::other(
                            "active container lost cleanup ownership",
                        ));
                    }
                }
            }
            changed.changed().await.map_err(std::io::Error::other)?;
        }
    }

    /// May new execution work be admitted right now?
    #[must_use]
    pub fn admission(&self) -> Admission {
        let Ok(outstanding) = self.outstanding.lock() else {
            return Admission::Quarantined {
                outstanding: 0,
                detail: "the execution cleanup supervisor is poisoned; restart the daemon so \
                         startup reconciliation can confirm every owned container is absent"
                    .to_owned(),
            };
        };
        let abandoned = outstanding
            .iter()
            .find_map(|(name, entry)| match &entry.stage {
                Stage::Abandoned(reason) => Some((name.clone(), reason.clone())),
                _ => None,
            });
        if let Some((name, reason)) = abandoned {
            return Admission::Quarantined {
                outstanding: outstanding.len(),
                detail: format!(
                    "cleanup of owned container {name} was abandoned after {reason}; inspect the \
                     owned container records under the service execution root and restart the \
                     daemon so startup reconciliation can confirm absence"
                ),
            };
        }
        let removing = outstanding
            .values()
            .filter(|entry| entry.stage == Stage::Removing)
            .count();
        if removing.saturating_add(1) > self.concurrency {
            return Admission::Quarantined {
                outstanding: removing,
                detail: format!(
                    "{removing} owned container(s) are still being removed, which is the whole \
                     configured worker concurrency; retry once cleanup completes"
                ),
            };
        }
        Admission::Open
    }

    /// Owned containers whose absence is not yet confirmed.
    #[must_use]
    pub fn outstanding(&self) -> Vec<String> {
        self.outstanding
            .lock()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// True when every owned container this process created has been confirmed absent.
    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.outstanding
            .lock()
            .map(|o| o.is_empty())
            .unwrap_or(false)
    }

    fn register(&self, name: &str, lease: Option<Arc<Lease>>) -> std::io::Result<()> {
        let mut outstanding = self
            .outstanding
            .lock()
            .map_err(|_| std::io::Error::other("cleanup ownership lock poisoned"))?;
        // Serialize registration against abandoning cleanup: an already acquired permit is
        // not authorization to start another container after the boundary failed.
        if outstanding
            .values()
            .any(|entry| matches!(entry.stage, Stage::Abandoned(_)))
        {
            return Err(std::io::Error::other("execution admission is quarantined"));
        }
        if outstanding.contains_key(name) {
            return Err(std::io::Error::other("duplicate container ownership"));
        }
        if let Some(lease) = &lease {
            let mut active = lease
                .active
                .lock()
                .map_err(|_| std::io::Error::other("execution lease poisoned"))?;
            if active.is_some() {
                return Err(std::io::Error::other(
                    "execution lease already owns a container; cleanup must confirm absence before another stage",
                ));
            }
            *active = Some(name.to_owned());
        }
        outstanding.insert(
            name.to_owned(),
            Outstanding {
                since: Instant::now(),
                stage: Stage::Running,
                _lease: lease,
            },
        );
        Ok(())
    }

    /// Removal has started. From here the clock that matters is the cleanup deadline, not how
    /// long the container has been doing its job.
    fn begin_removal(&self, name: &str) {
        if let Ok(mut outstanding) = self.outstanding.lock()
            && let Some(entry) = outstanding.get_mut(name)
            && entry.stage == Stage::Running
        {
            entry.since = Instant::now();
            entry.stage = Stage::Removing;
        }
    }

    fn resolve(&self, name: &str) {
        if let Ok(mut outstanding) = self.outstanding.lock() {
            if let Some(entry) = outstanding.remove(name)
                && let Some(lease) = entry._lease
                && let Ok(mut active) = lease.active.lock()
            {
                *active = None;
            }
            self.changed
                .send_modify(|version| *version = version.wrapping_add(1));
        }
    }

    fn abandon(&self, name: &str, reason: String) {
        if let Ok(mut outstanding) = self.outstanding.lock()
            && let Some(entry) = outstanding.get_mut(name)
        {
            entry.stage = Stage::Abandoned(reason);
            self.changed
                .send_modify(|version| *version = version.wrapping_add(1));
        }
    }

    fn elapsed(&self, name: &str) -> Duration {
        self.outstanding
            .lock()
            .ok()
            .and_then(|o| o.get(name).map(|e| e.since.elapsed()))
            .unwrap_or_default()
    }
}

/// Holds ownership of one container until its absence is confirmed.
///
/// Constructed immediately after the durable owner record is written and before the container
/// exists, so there is no window in which a container is running with nothing watching it.
#[derive(Debug)]
pub struct ContainerGuard {
    name: String,
    runner: Runner,
    supervisor: Arc<Supervisor>,
    resolved: bool,
}

impl ContainerGuard {
    /// Register a container the caller is about to create.
    pub fn register(
        runner: Runner,
        supervisor: Arc<Supervisor>,
        name: String,
    ) -> std::io::Result<Self> {
        let lease = runner
            .lease
            .clone()
            .ok_or_else(|| std::io::Error::other("container requires an execution lease"))?;
        supervisor.register(&name, Some(lease))?;
        Ok(Self {
            name,
            runner,
            supervisor,
            resolved: false,
        })
    }

    /// Remove the container now. Returns whether absence was confirmed.
    ///
    /// A failure is not propagated: the guard keeps ownership and its `Drop` schedules the
    /// retry, so the caller can report its own outcome without losing the cleanup obligation.
    pub async fn remove_now(&mut self) -> bool {
        if self.resolved {
            return true;
        }
        self.supervisor.begin_removal(&self.name);
        match self.runner.remove(&self.name).await {
            Ok(()) => {
                self.resolved = true;
                self.supervisor.resolve(&self.name);
                true
            }
            Err(err) => {
                eprintln!(
                    "library-enrichmentd: cleanup of {} did not confirm absence: {err}",
                    self.name
                );
                false
            }
        }
    }

    /// Consume ownership and wait through supervised retries before reporting settlement.
    pub async fn settle(mut self) -> std::io::Result<()> {
        if self.remove_now().await {
            return Ok(());
        }
        let supervisor = self.supervisor.clone();
        let lease = self
            .runner
            .lease
            .clone()
            .ok_or_else(|| std::io::Error::other("cleanup lost its lease"))?;
        drop(self);
        supervisor.wait_for_cleanup(&lease).await
    }
}

impl Drop for ContainerGuard {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        let name = std::mem::take(&mut self.name);
        let runner = self.runner.clone();
        let supervisor = Arc::clone(&self.supervisor);
        // Ownership already contains the original execution lease. Never acquire a second
        // permit: the caller may hold the last one while this guard is being dropped.
        supervisor.begin_removal(&name);
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            // No runtime to retry on. The durable owner record is still the recovery path, so
            // say so instead of implying the container was removed.
            supervisor.abandon(
                &name,
                "no async runtime was available for in-process retry".to_owned(),
            );
            return;
        };
        handle.spawn(async move {
            let deadline = supervisor.deadline;
            let mut backoff = Duration::from_millis(250);
            loop {
                if runner.remove(&name).await.is_ok() {
                    supervisor.resolve(&name);
                    return;
                }
                if supervisor.elapsed(&name) >= deadline {
                    supervisor.abandon(&name, format!("{} seconds of retries", deadline.as_secs()));
                    eprintln!(
                        "library-enrichmentd: abandoning cleanup of owned container {name}; new \
                         execution admission is quarantined until a restart reconciles it"
                    );
                    return;
                }
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(5));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn supervisor(concurrency: usize) -> Arc<Supervisor> {
        Supervisor::new(
            Arc::new(tokio::sync::Semaphore::new(concurrency)),
            30,
            concurrency,
        )
    }

    #[tokio::test]
    async fn a_cleanup_waiter_does_not_finish_while_the_container_is_still_owned() {
        let supervisor = supervisor(1);
        let lease = supervisor.lease().await.expect("lease");
        supervisor
            .register("owned", Some(lease.clone()))
            .expect("registered");
        supervisor.begin_removal("owned");
        assert!(
            tokio::time::timeout(
                Duration::from_millis(10),
                supervisor.wait_for_cleanup(&lease)
            )
            .await
            .is_err()
        );
        supervisor.resolve("owned");
        supervisor
            .wait_for_cleanup(&lease)
            .await
            .expect("absence confirmed");
    }

    #[tokio::test]
    async fn abandoned_cleanup_is_a_failure_and_retains_its_workspace() {
        let supervisor = supervisor(1);
        let lease = supervisor.lease().await.expect("lease");
        let dir = tempfile::tempdir().expect("scratch root");
        let scratch = dir.path().join("capsule");
        std::fs::create_dir(&scratch).expect("capsule");
        let runner = Runner::new(
            &enrichment_core::config::Execution::default(),
            dir.path(),
            supervisor.clone(),
            crate::execution::test_ownership(dir.path()),
        )
        .expect("runner")
        .using_lease(lease.clone());
        supervisor
            .register("owned", Some(lease.clone()))
            .expect("registered");
        runner.discard_workspace(scratch.clone());
        tokio::task::yield_now().await;
        assert!(scratch.exists(), "an active mount cannot be removed");
        supervisor.abandon("owned", "injected retry exhaustion".into());
        assert!(supervisor.wait_for_cleanup(&lease).await.is_err());
        tokio::task::yield_now().await;
        assert!(
            scratch.exists(),
            "failed cleanup retains scratch for reconciliation"
        );
        assert_eq!(supervisor.available_permits(), 0);
        supervisor.resolve("owned");
    }

    #[tokio::test]
    async fn cleanup_keeps_the_original_lease_and_wakes_queued_work_on_quarantine() {
        let supervisor = supervisor(1);
        let lease = supervisor.lease().await.expect("admitted");
        supervisor
            .register("owned", Some(Arc::clone(&lease)))
            .expect("register");
        drop(lease);
        assert_eq!(supervisor.permits.available_permits(), 0);
        supervisor.begin_removal("owned");
        // Removing the last container already quarantines admission; increasing the bound in
        // the other test below exercises retry exhaustion below that bound as well.
        assert!(supervisor.lease().await.is_err());
        supervisor.abandon("owned", "retry deadline".into());
        assert_eq!(
            supervisor.permits.available_permits(),
            0,
            "abandonment never frees an unsafe slot"
        );
        supervisor.resolve("owned");
        assert_eq!(supervisor.permits.available_permits(), 1);
        assert!(supervisor.lease().await.is_ok());
    }

    #[tokio::test]
    async fn admission_waiter_is_released_when_cleanup_is_abandoned() {
        let supervisor = supervisor(2);
        let first = supervisor.lease().await.expect("first");
        let second = supervisor.lease().await.expect("second");
        supervisor.register("owned", Some(first)).expect("register");
        let waiting = {
            let supervisor = Arc::clone(&supervisor);
            tokio::spawn(async move { supervisor.lease().await })
        };
        tokio::task::yield_now().await;
        assert!(!waiting.is_finished());
        supervisor.begin_removal("owned");
        supervisor.abandon("owned", "retry deadline".into());
        assert!(
            tokio::time::timeout(Duration::from_secs(1), waiting)
                .await
                .expect("quarantine wakes waiter")
                .expect("task")
                .is_err()
        );
        assert_eq!(supervisor.permits.available_permits(), 0);
        drop(second);
        assert_eq!(supervisor.permits.available_permits(), 1);
        assert!(supervisor.lease().await.is_err());
        supervisor.resolve("owned");
        assert_eq!(supervisor.permits.available_permits(), 2);
    }

    #[test]
    fn a_running_container_is_busy_work_not_unresolved_cleanup() {
        // The distinction this guards: with a concurrency of one, a single *running* probe must
        // not make the next submission read as "cleanup is unresolved". Busy is the durable job
        // queue's problem; unresolved cleanup is a boundary that is not holding.
        let supervisor = supervisor(1);
        supervisor.register("libenr-a", None).expect("register");
        assert_eq!(supervisor.admission(), Admission::Open);
        assert!(
            !supervisor.is_idle(),
            "it is still owned, so shutdown must still wait for it"
        );

        supervisor.begin_removal("libenr-a");
        assert!(matches!(
            supervisor.admission(),
            Admission::Quarantined { outstanding: 1, .. }
        ));
        supervisor.resolve("libenr-a");
        assert_eq!(supervisor.admission(), Admission::Open);
        assert!(supervisor.is_idle());
    }

    #[test]
    fn an_abandoned_cleanup_quarantines_admission_below_the_concurrency_bound() {
        // The count alone would not trip quarantine here: 1 outstanding of 4 workers. An
        // abandoned removal must quarantine anyway, because nothing is watching that container.
        let supervisor = supervisor(4);
        supervisor.register("libenr-b", None).expect("register");
        supervisor.begin_removal("libenr-b");
        assert_eq!(supervisor.admission(), Admission::Open);
        supervisor.abandon("libenr-b", "30 seconds of retries".to_owned());
        let Admission::Quarantined { detail, .. } = supervisor.admission() else {
            panic!("an abandoned cleanup must quarantine admission");
        };
        assert!(detail.contains("libenr-b"), "{detail}");
        assert!(detail.contains("restart"), "{detail}");
        assert_eq!(supervisor.outstanding(), vec!["libenr-b".to_owned()]);
    }
}
