//! Bounded ownership of final lease-release writes on the existing native executor.
//! This is a lifecycle mechanism; Delta and native plans remain release authority.
use datafusion::error::{DataFusionError, Result};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tokio::task::JoinSet;

const MAX_OWNERS: usize = 1024;

#[derive(Default)]
pub(crate) struct Releases {
    state: Mutex<State>,
    closing: tokio::sync::Mutex<()>,
}
#[derive(Default)]
struct State {
    closed: bool,
    draining: bool,
    reserved: usize,
    failures: usize,
    tasks: JoinSet<Result<()>>,
}

/// A release task is admitted before the physical owner exists. Holding a slot
/// guarantees that queue pressure cannot discard that owner's eventual release.
#[derive(Debug)]
pub struct ReleaseSlot {
    owner: Arc<Releases>,
}
impl std::fmt::Debug for Releases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Releases").finish_non_exhaustive()
    }
}
impl Drop for ReleaseSlot {
    fn drop(&mut self) {
        if let Ok(mut state) = self.owner.state.lock() {
            state.reserved -= 1;
        }
    }
}
impl Releases {
    pub(crate) fn reserve(self: &Arc<Self>) -> Result<ReleaseSlot> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| error("retention release ownership poisoned"))?;
        if state.closed || state.draining || state.reserved >= MAX_OWNERS {
            return Err(error(
                "retention release admission closed or at its owner bound",
            ));
        }
        state.reserved += 1;
        Ok(ReleaseSlot {
            owner: self.clone(),
        })
    }

    pub(crate) fn submit(
        self: &Arc<Self>,
        slot: ReleaseSlot,
        executor: &tokio::runtime::Handle,
        work: impl Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        if !Arc::ptr_eq(self, &slot.owner) {
            return Err(error("retention release slot belongs to another runtime"));
        }
        let mut state = self
            .state
            .lock()
            .map_err(|_| error("retention release ownership poisoned"))?;
        while let Some(completed) = state.tasks.try_join_next() {
            if !matches!(completed, Ok(Ok(()))) {
                state.failures = state.failures.saturating_add(1);
            }
        }
        if state.closed {
            state.failures = state.failures.saturating_add(1);
            return Err(error(
                "retention release queue closed; durable protection remains",
            ));
        }
        state.tasks.spawn_on(
            async move {
                let _slot = slot;
                work.await
            },
            executor,
        );
        Ok(())
    }

    /// Join submitted releases and their descendants, leaving live owners and admission open.
    /// The caller must first join/drop every producer of bytes it is about to seal. This
    /// barrier proves only submitted work, not the exit of an arbitrary still-live owner.
    pub(crate) async fn flush(&self) -> Result<()> {
        self.drain(false).await
    }

    /// Join the release tree to quiescence before closing telemetry/storage. A release
    /// can drop another retained owner, so admission closes only at the atomic empty
    /// observation, after all admitted parents and their descendants have completed.
    pub(crate) async fn close(&self) -> Result<()> {
        self.drain(true).await
    }

    async fn drain(&self, close: bool) -> Result<()> {
        let _closing = self.closing.lock().await;
        let failures = futures::future::poll_fn(|cx| -> std::task::Poll<Result<usize>> {
            let mut state = self
                .state
                .lock()
                .map_err(|_| error("retention release ownership poisoned"))?;
            state.draining |= close;
            // Keep every handle and observed failure in shared ownership while waiting.
            // Cancelling a closer must not abort releases or make the next close appear empty.
            loop {
                match state.tasks.poll_join_next(cx) {
                    std::task::Poll::Pending => return std::task::Poll::Pending,
                    std::task::Poll::Ready(Some(completed)) => {
                        if !matches!(completed, Ok(Ok(()))) {
                            state.failures = state.failures.saturating_add(1);
                        }
                    }
                    std::task::Poll::Ready(None) => {
                        if close && state.reserved != 0 {
                            return std::task::Poll::Ready(Err(error(&format!(
                                "{} physical owners still hold reserved releases",
                                state.reserved
                            ))));
                        }
                        state.closed |= close;
                        return std::task::Poll::Ready(Ok(state.failures));
                    }
                }
            }
        })
        .await?;
        if failures != 0 {
            return Err(error(&format!(
                "{failures} retention releases require durable reconciliation"
            )));
        }
        Ok(())
    }
}

fn error(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_release_flush_joins_writes_without_revoking_live_owners() -> Result<()> {
        let releases = Arc::new(Releases::default());
        let live = releases.reserve()?;
        let (complete, waiting) = tokio::sync::oneshot::channel();
        releases.submit(
            releases.reserve()?,
            &tokio::runtime::Handle::current(),
            async move {
                waiting.await.map_err(|_| error("fixture signal"))?;
                Ok(())
            },
        )?;
        let mut flush = Box::pin(releases.flush());
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(10), &mut flush)
                .await
                .is_err()
        );
        complete.send(()).expect("pending writer");
        flush.await?;
        let after = releases.reserve()?;
        drop((live, after));
        releases.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_release_capacity_is_reserved_before_admission() -> Result<()> {
        let releases = Arc::new(Releases::default());
        let mut owners = (0..MAX_OWNERS)
            .map(|_| releases.reserve())
            .collect::<Result<Vec<_>>>()?;
        assert!(releases.reserve().is_err());
        let slot = owners.pop().unwrap();
        releases.submit(slot, &tokio::runtime::Handle::current(), async { Ok(()) })?;
        assert!(
            releases
                .close()
                .await
                .unwrap_err()
                .to_string()
                .contains("physical owners")
        );
        // Draining refuses new physical owners while every already reserved release
        // remains admissible, including after a close that found surviving readers.
        assert!(releases.reserve().is_err());
        let slot = owners.pop().unwrap();
        releases.submit(slot, &tokio::runtime::Handle::current(), async { Ok(()) })?;
        drop(owners);
        releases.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_release_close_joins_descendants_created_during_drain() -> Result<()> {
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        };
        let releases = Arc::new(Releases::default());
        let (ready, start) = tokio::sync::oneshot::channel();
        let nested = releases.clone();
        let child_slot = releases.reserve()?;
        let completed = Arc::new(AtomicBool::new(false));
        let witness = completed.clone();
        releases.submit(
            releases.reserve()?,
            &tokio::runtime::Handle::current(),
            async move {
                start.await.map_err(|_| error("parent release lost"))?;
                nested.submit(child_slot, &tokio::runtime::Handle::current(), async move {
                    witness.store(true, Ordering::Release);
                    Ok(())
                })
            },
        )?;
        let mut closing = Box::pin(releases.close());
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(10), &mut closing)
                .await
                .is_err()
        );
        ready.send(()).expect("parent remains physically owned");
        closing.await?;
        assert!(completed.load(Ordering::Acquire));
        releases.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn cancelled_close_preserves_release_work_and_failures() -> Result<()> {
        let releases = Arc::new(Releases::default());
        let (release, wait) = tokio::sync::oneshot::channel();
        releases.submit(
            releases.reserve()?,
            &tokio::runtime::Handle::current(),
            async move {
                wait.await.map_err(|_| error("release driver was lost"))?;
                Err(error("durable release failure"))
            },
        )?;
        let mut close = Box::pin(releases.close());
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(25), &mut close)
                .await
                .is_err()
        );
        drop(close);
        release
            .send(())
            .expect("release task must survive a cancelled closer");
        let first = releases.close().await.unwrap_err().to_string();
        assert!(first.contains("1 retention releases require durable reconciliation"));
        assert_eq!(releases.close().await.unwrap_err().to_string(), first);
        Ok(())
    }
}
