//! Propagate the owned operation into DataFusion's native async and blocking task boundary.
use datafusion::common::{
    DataFusionError, Result,
    runtime::{JoinSetTracer, set_join_set_tracer},
};
use futures::future::BoxFuture;
use std::{any::Any, sync::OnceLock};

struct NativeContext;
impl JoinSetTracer for NativeContext {
    fn trace_future(
        &self,
        work: BoxFuture<'static, Box<dyn Any + Send>>,
    ) -> BoxFuture<'static, Box<dyn Any + Send>> {
        let operation = crate::runtime::capture_operation();
        let effects = crate::native_effect::capture();
        Box::pin(operation.scope(effects.scope(work)))
    }

    fn trace_block(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>,
    ) -> Box<dyn FnOnce() -> Box<dyn Any + Send> + Send> {
        let operation = crate::runtime::capture_operation();
        let effects = crate::native_effect::capture();
        Box::new(move || operation.run(|| effects.run(work)))
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
