//! Bounded ownership of final lease-release writes on the existing native executor.
//! This is a lifecycle mechanism; Delta and native plans remain release authority.
use datafusion::error::{DataFusionError, Result};
use std::{future::Future, sync::Mutex};
use tokio::task::JoinSet;

const MAX_PENDING: usize = 64;

#[derive(Default)]
pub(crate) struct Releases {
    state: Mutex<State>,
    closing: tokio::sync::Mutex<()>,
}
#[derive(Default)]
struct State {
    closed: bool,
    failures: usize,
    tasks: JoinSet<Result<()>>,
}

impl Releases {
    pub(crate) fn submit(
        &self,
        executor: &tokio::runtime::Handle,
        work: impl Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| error("retention release ownership poisoned"))?;
        while let Some(completed) = state.tasks.try_join_next() {
            if !matches!(completed, Ok(Ok(()))) {
                state.failures = state.failures.saturating_add(1);
            }
        }
        if state.closed || state.tasks.len() >= MAX_PENDING {
            state.failures = state.failures.saturating_add(1);
            return Err(error(
                "retention release queue closed or full; durable protection remains",
            ));
        }
        state.tasks.spawn_on(work, executor);
        Ok(())
    }

    /// Close admission, then await every admitted release before closing telemetry/storage.
    pub(crate) async fn close(&self) -> Result<()> {
        let _closing = self.closing.lock().await;
        let failures = futures::future::poll_fn(|cx| -> std::task::Poll<Result<usize>> {
            let mut state = self
                .state
                .lock()
                .map_err(|_| error("retention release ownership poisoned"))?;
            state.closed = true;
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
    async fn cancelled_close_preserves_release_work_and_failures() -> Result<()> {
        let releases = Releases::default();
        let (release, wait) = tokio::sync::oneshot::channel();
        releases.submit(&tokio::runtime::Handle::current(), async move {
            wait.await.map_err(|_| error("release driver was lost"))?;
            Err(error("durable release failure"))
        })?;
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
