//! Propagate the owned operation into DataFusion's native async and blocking task boundary.
use datafusion::common::{
    DataFusionError, Result,
    runtime::{JoinSetTracer, set_join_set_tracer},
};
use futures::future::BoxFuture;
use std::{any::Any, sync::OnceLock};
use tokio_util::task::TaskTracker;
use tracing::{Instrument, instrument::WithSubscriber};

tokio::task_local! { static TASKS: Option<TaskTracker>; }

/// Runtime ownership is distinct from operation/effect authority. Root drivers establish
/// this scope; DataFusion children keep a token through physical exit, including callbacks
/// that cannot be cancelled after spawn_blocking has started.
#[derive(Clone)]
pub(crate) struct PhysicalContext(Option<TaskTracker>);
impl PhysicalContext {
    pub(crate) fn owned(tasks: TaskTracker) -> Self {
        Self(Some(tasks))
    }
    pub(crate) fn detached() -> Self {
        Self(None)
    }
    pub(crate) async fn scope<T>(self, work: impl Future<Output = T>) -> T {
        TASKS.scope(self.0, work).await
    }
    pub(crate) fn run<T>(self, work: impl FnOnce() -> T) -> T {
        TASKS.sync_scope(self.0, work)
    }
}

fn physical_context() -> PhysicalContext {
    PhysicalContext(TASKS.try_with(Clone::clone).ok().flatten())
}

struct NativeContext;
impl JoinSetTracer for NativeContext {
    fn trace_future(
        &self,
        work: BoxFuture<'static, Box<dyn Any + Send>>,
    ) -> BoxFuture<'static, Box<dyn Any + Send>> {
        let operation = crate::runtime::capture_operation();
        let admission = crate::runtime::capture_query_admission();
        let effects = crate::native_effect::capture();
        let physical = physical_context();
        let tasks = physical.0.clone();
        let work = physical
            .scope(admission.scope(operation.scope(effects.scope(work))))
            .instrument(tracing::Span::current())
            .with_current_subscriber();
        match tasks {
            Some(tasks) => Box::pin(tasks.track_future(work)),
            None => Box::pin(work),
        }
    }

    fn trace_block(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>,
    ) -> Box<dyn FnOnce() -> Box<dyn Any + Send> + Send> {
        let operation = crate::runtime::capture_operation();
        let admission = crate::runtime::capture_query_admission();
        let effects = crate::native_effect::capture();
        let dispatch = tracing::dispatcher::get_default(Clone::clone);
        let span = tracing::Span::current();
        let physical = physical_context();
        let token = physical.0.as_ref().map(TaskTracker::token);
        Box::new(move || {
            let _token = token;
            tracing::dispatcher::with_default(&dispatch, || {
                let _span = span.enter();
                physical.run(|| admission.run(|| operation.run(|| effects.run(work))))
            })
        })
    }
}

pub(crate) fn install() -> Result<()> {
    static CONTEXT: NativeContext = NativeContext;
    static INSTALLATION: OnceLock<std::result::Result<(), String>> = OnceLock::new();
    INSTALLATION
        .get_or_init(|| set_join_set_tracer(&CONTEXT).map_err(|error| error.to_string()))
        .as_ref()
        .copied()
        .map_err(|error| DataFusionError::Configuration(format!("native task context: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn native_child_tasks_keep_operation_identity_and_expired_budget() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        runtime
            .job_operation(
                "native-child-owner".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "fixture.native_task_context".into(),
                    request_digest: "a".repeat(64),
                    policy_digest: "b".repeat(64),
                },
                std::time::Duration::ZERO,
                async {
                    let mut children = datafusion::common::runtime::JoinSet::new();
                    let child_runtime = runtime.clone();
                    children.spawn(async move {
                        let id = crate::runtime::operation_id();
                        let failure = child_runtime
                            .execute(child_runtime.session().read_empty()?)
                            .await;
                        assert!(
                            failure.is_err(),
                            "a native task must not reset the expired owner budget"
                        );
                        Ok::<_, DataFusionError>(id)
                    });
                    let id = children.join_next().await.unwrap().unwrap()?;
                    assert_eq!(id.as_deref(), Some("native-child-owner"));
                    children.spawn_blocking(|| Ok(crate::runtime::operation_id()));
                    let id = children.join_next().await.unwrap().unwrap()?;
                    assert_eq!(id.as_deref(), Some("native-child-owner"));
                    Ok::<_, DataFusionError>(())
                },
            )
            .await?;
        assert_eq!(crate::runtime::operation_id(), None);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
