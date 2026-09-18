//! Delta kernel I/O on a separately scheduled lane of the owned DataFusion runtime.
//! The kernel still supplies parsing, replay, scans and its bounded I/O pipeline.
//! This adapter carries physical runtime ownership and operation/effect context.
use std::{future::Future, num::NonZeroUsize, sync::Arc};

use async_trait::async_trait;
use bytes::Bytes;
use delta_kernel::{DeltaResult, Engine};
use delta_kernel_default_engine::{DefaultEngineBuilder, executor::TaskExecutor};
use deltalake::{
    DeltaResult as TableResult,
    kernel::{Version, transaction::TransactionError},
    logstore::{CommitOrBytes, LogStore, LogStoreConfig, LogStoreRef},
};
use futures::future::BoxFuture;
use object_store::{ObjectStore, path::Path};
use tokio::runtime::{EnterGuard, Handle, RuntimeFlavor};
use tracing::instrument::WithSubscriber;
use uuid::Uuid;

use crate::runtime::QueryRuntime;

#[derive(Clone)]
struct KernelTasks {
    executor: Arc<crate::runtime::NativeExecutor>,
    handle: Handle,
}
impl TaskExecutor for KernelTasks {
    type Guard<'a> = EnterGuard<'a>;

    fn block_on<T>(&self, work: T) -> T::Output
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let work = self
            .executor
            .track(self.executor.physical_context().scope(work));
        let task = self.executor.spawn_on(self.executor.io_handle(), work);
        let wait = || {
            futures::executor::block_on(task.join_unwind())
                .expect("owned kernel executor exited before synchronous work completed")
        };
        // A transport can be current-thread Tokio; work always runs on the owned
        // I/O executor. Synchronous callers and their awaited filesystem work have
        // different blocking pools, including at query/blocking concurrency one.
        if Handle::try_current()
            .is_ok_and(|handle| handle.runtime_flavor() == RuntimeFlavor::MultiThread)
        {
            tokio::task::block_in_place(wait)
        } else {
            wait()
        }
    }

    fn spawn<F>(&self, work: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let executor = self.executor.clone();
        let operation = crate::runtime::capture_operation();
        let effects = crate::native_effect::capture();
        let input = crate::task_context::InputContext::capture();
        // The kernel's stream/channel cancellation owns termination. The runtime tracker
        // waits for physical exit without retaining a completed task's output or handle.
        let physical = self.executor.physical_context();
        self.executor.io_handle().spawn(
            self.executor
                .track(async move {
                    let _executor = executor;
                    physical
                        .scope(input.scope(operation.scope(effects.scope(Box::pin(work)))))
                        .await;
                })
                .with_current_subscriber(),
        );
    }

    fn spawn_blocking<T, R>(&self, work: T) -> BoxFuture<'_, DeltaResult<R>>
    where
        T: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // Enter the owned scope before invoking DataFusion's blocking task hook; its
        // token survives dropping this returned future while the callback is running.
        let task = self
            .executor
            .physical_context()
            .run(|| self.executor.spawn_blocking(work));
        Box::pin(async move { task.await.map_err(delta_kernel::Error::join_failure) })
    }

    fn enter(&self) -> Self::Guard<'_> {
        self.handle.enter()
    }
}

struct OwnedLogStore {
    inner: LogStoreRef,
    neutral_engine: Arc<dyn Engine>,
    tasks: Arc<KernelTasks>,
    batch_rows: NonZeroUsize,
    buffers: NonZeroUsize,
}

pub(crate) fn bind(inner: LogStoreRef, runtime: QueryRuntime) -> LogStoreRef {
    let (batch_rows, buffers) = runtime.kernel_io_shape();
    let tasks = Arc::new(KernelTasks {
        handle: runtime.executor_handle(),
        executor: runtime.executor_owner(),
    });
    let batch_rows = NonZeroUsize::new(batch_rows).expect("validated native batch size");
    let buffers = NonZeroUsize::new(buffers).expect("validated native partition count");
    let neutral_engine = Arc::new(
        DefaultEngineBuilder::new(inner.root_object_store(None))
            .with_task_executor(tasks.clone())
            .with_batch_size(batch_rows)
            .with_buffer_size(buffers)
            .build(),
    );
    Arc::new(OwnedLogStore {
        inner,
        neutral_engine,
        tasks,
        batch_rows,
        buffers,
    })
}

/// Native providers construct a DataFusionEngine from their TaskContext, independently
/// of LogStore::engine. Give that path the same owned I/O handler and exact root backend.
pub(crate) fn session_engine(
    runtime: &QueryRuntime,
) -> datafusion::error::Result<deltalake::delta_datafusion::engine::KernelIoEngine> {
    let (batch_rows, buffers) = runtime.kernel_io_shape();
    let store = runtime
        .session()
        .runtime_env()
        .object_store(datafusion::execution::object_store::ObjectStoreUrl::local_filesystem())?;
    Ok(deltalake::delta_datafusion::engine::KernelIoEngine(
        Arc::new(
            DefaultEngineBuilder::new(store)
                .with_task_executor(Arc::new(KernelTasks {
                    handle: runtime.executor_handle(),
                    executor: runtime.executor_owner(),
                }))
                .with_batch_size(
                    NonZeroUsize::new(batch_rows).expect("validated native batch size"),
                )
                .with_buffer_size(
                    NonZeroUsize::new(buffers).expect("validated native partition count"),
                )
                .build(),
        ),
    ))
}

#[async_trait]
impl LogStore for OwnedLogStore {
    fn name(&self) -> String {
        self.inner.name()
    }
    async fn refresh(&self) -> TableResult<()> {
        self.inner.refresh().await
    }
    async fn read_commit_entry(&self, version: Version) -> TableResult<Option<Bytes>> {
        self.inner.read_commit_entry(version).await
    }
    async fn write_commit_entry(
        &self,
        version: Version,
        commit: CommitOrBytes,
        operation: Uuid,
    ) -> Result<(), TransactionError> {
        self.inner
            .write_commit_entry(version, commit, operation)
            .await
    }
    async fn abort_commit_entry(
        &self,
        version: Version,
        commit: CommitOrBytes,
        operation: Uuid,
    ) -> Result<(), TransactionError> {
        self.inner
            .abort_commit_entry(version, commit, operation)
            .await
    }
    async fn get_latest_version(&self, start_version: Version) -> TableResult<Version> {
        // The native local implementation calls `engine` internally. Forwarding this
        // method to the inner store would silently re-create its default executor.
        deltalake::logstore::get_latest_version(self, start_version).await
    }
    fn object_store(&self, operation: Option<Uuid>) -> Arc<dyn ObjectStore> {
        self.inner.object_store(operation)
    }
    fn root_object_store(&self, operation: Option<Uuid>) -> Arc<dyn ObjectStore> {
        self.inner.root_object_store(operation)
    }
    fn engine(&self, operation: Option<Uuid>) -> Arc<dyn Engine> {
        if operation.is_none() {
            return self.neutral_engine.clone();
        }
        Arc::new(
            DefaultEngineBuilder::new(self.inner.root_object_store(operation))
                .with_task_executor(self.tasks.clone())
                .with_batch_size(self.batch_rows)
                .with_buffer_size(self.buffers)
                .build(),
        )
    }
    fn to_uri(&self, location: &Path) -> String {
        self.inner.to_uri(location)
    }
    fn root_url(&self) -> &url::Url {
        self.inner.root_url()
    }
    fn log_path(&self) -> &Path {
        self.inner.log_path()
    }
    fn transaction_url(&self, operation: Option<Uuid>) -> TableResult<url::Url> {
        self.inner.transaction_url(operation)
    }
    async fn is_delta_table_location(&self) -> TableResult<bool> {
        self.inner.is_delta_table_location().await
    }
    fn config(&self) -> &LogStoreConfig {
        self.inner.config()
    }
    fn object_store_url(&self) -> datafusion::execution::object_store::ObjectStoreUrl {
        self.inner.object_store_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::error::Result;

    #[tokio::test]
    async fn shutdown_waits_for_cancelled_blocking_work_and_kernel_descendants() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let tasks = KernelTasks {
            handle: runtime.executor_handle(),
            executor: runtime.executor_owner(),
        };
        let (parent_release, parent_wait) = tokio::sync::oneshot::channel();
        let (child_release, child_wait) = tokio::sync::oneshot::channel();
        let (child_started, child_running) = tokio::sync::oneshot::channel();
        let (child_dropped, child_gone) = tokio::sync::oneshot::channel();
        struct OnDrop(Option<tokio::sync::oneshot::Sender<()>>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                if let Some(send) = self.0.take() {
                    let _ = send.send(());
                }
            }
        }
        let nested = tasks.clone();
        tasks.spawn(async move {
            let _ = parent_wait.await;
            // Spawn a child after shutdown has already started. Parent ownership must
            // bridge this transition without a closed-and-empty interval.
            nested.spawn(async move {
                let _drop = OnDrop(Some(child_dropped));
                let _ = child_started.send(());
                let _ = child_wait.await;
            });
        });

        let (blocking_release, blocking_wait) = std::sync::mpsc::channel();
        let (blocking_started, blocking_running) = tokio::sync::oneshot::channel();
        // No ambient operation or physical scope exists at this transport entry point.
        let blocking = runtime.spawn_blocking(move || {
            let _ = blocking_started.send(());
            let _ = blocking_wait.recv();
        });
        blocking_running.await.unwrap();
        drop(blocking); // A started callback continues despite abort-on-drop.

        let (native_release, native_wait) = std::sync::mpsc::channel();
        let (native_started, native_running) = tokio::sync::oneshot::channel();
        let parent = runtime.spawn(async move {
            let child = datafusion::common::runtime::SpawnedTask::spawn_blocking(move || {
                let _ = native_started.send(());
                let _ = native_wait.recv();
            });
            let _ = child.await;
        });
        native_running.await.unwrap();
        drop(parent); // DataFusion's tracer must retain the uncancellable child.

        let close_runtime = runtime.clone();
        let mut close = tokio::spawn(async move { close_runtime.close_diagnostics().await });
        async fn pending<T>(task: &mut tokio::task::JoinHandle<T>) {
            assert!(
                tokio::time::timeout(std::time::Duration::from_millis(25), task)
                    .await
                    .is_err()
            );
        }
        pending(&mut close).await;
        parent_release.send(()).unwrap();
        child_running.await.unwrap();
        child_release.send(()).unwrap();
        child_gone.await.unwrap();
        pending(&mut close).await;
        blocking_release.send(()).unwrap();
        pending(&mut close).await;
        native_release.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(10), close)
            .await
            .expect("native physical drain")
            .unwrap()?;
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[test]
    fn kernel_filesystem_progresses_with_saturated_native_pool() -> Result<()> {
        const CHILD: &str = "LIBENR_KERNEL_SINGLE_BLOCKING_ORACLE";
        if std::env::var_os(CHILD).is_none() {
            let mut child = std::process::Command::new(std::env::current_exe()?)
                .args([
                    "--exact",
                    "kernel_runtime::tests::kernel_filesystem_progresses_with_saturated_native_pool",
                    "--nocapture",
                ])
                .env(CHILD, "1")
                .spawn()?;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
            loop {
                if let Some(status) = child.try_wait()? {
                    assert!(status.success(), "native kernel filesystem child: {status}");
                    return Ok(());
                }
                if std::time::Instant::now() >= deadline {
                    child.kill()?;
                    child.wait()?;
                    panic!("native kernel filesystem work exhausted its blocking pool");
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        }
        use crate::native_delta::{DeltaStore, StorageContract};
        use arrow::{
            array::Int64Array,
            datatypes::{DataType, Field, Schema},
            record_batch::RecordBatch,
        };

        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(
            root.path(),
            crate::runtime::QueryLimits {
                concurrency: 1,
                partitions: 1,
                native: enrichment_core::config::NativeQueryConfig {
                    blocking_threads: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        let work = runtime.clone();
        runtime.bootstrap(async move {
            eprintln!("kernel filesystem oracle: create");
            let store = DeltaStore::new(&root.path().join("tables"), work.clone())?;
            let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
            let contract = StorageContract::new(schema.clone())?;
            let table = store.create("kernel_io", &contract, false).await?;
            eprintln!("kernel filesystem oracle: append");
            let batch =
                RecordBatch::try_new(schema, vec![Arc::new(Int64Array::from(vec![41_i64]))])?;
            let table = store
                .append(
                    table,
                    &contract,
                    crate::native_catalog::batch(&store.session(), "kernel_runtime", batch)?,
                    vec![],
                )
                .await?;
            let expected = table.version();
            eprintln!("kernel filesystem oracle: checkpoint");
            deltalake::protocol::checkpoints::create_checkpoint(&table, None)
                .await
                .map_err(|error| datafusion::error::DataFusionError::External(Box::new(error)))?;
            drop(table);
            eprintln!("kernel filesystem oracle: load");
            // Real snapshot replay occupies the sole native blocking thread and needs
            // local directory/read operations. Sharing its pool would deadlock here.
            // Keep the checkpoint as the only source for this exact version.
            let log = root.path().join("tables/kernel_io/_delta_log");
            for entry in std::fs::read_dir(&log)? {
                let path = entry?.path();
                if path
                    .extension()
                    .is_some_and(|extension| extension == "json")
                {
                    std::fs::remove_file(path)?;
                }
            }
            let loaded = store.load("kernel_io", expected).await?;
            assert_eq!(loaded.version(), expected);
            eprintln!("kernel filesystem oracle: scan");
            let output = work
                .execute(
                    store
                        .session()
                        .read_table(store.provider(&loaded, &contract).await?)?,
                )
                .await?;
            assert_eq!(output.rows, 1);
            assert_eq!(
                output.batches[0]
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .value(0),
                41
            );
            eprintln!("kernel filesystem oracle: session engine storage");
            let engine = deltalake::delta_datafusion::engine::DataFusionEngine::new_from_context(
                store.session().task_ctx(),
            );
            let url = url::Url::from_file_path(log.join("_last_checkpoint")).unwrap();
            let bytes = work
                .spawn_blocking(move || {
                    engine
                        .storage_handler()
                        .read_files(vec![(url, None)])?
                        .collect::<DeltaResult<Vec<_>>>()
                })
                .await
                .unwrap()
                .map_err(|error| datafusion::error::DataFusionError::External(Box::new(error)))?;
            assert_eq!(bytes.len(), 1);
            assert!(!bytes[0].is_empty());
            eprintln!("kernel filesystem oracle: drain");
            work.close_diagnostics().await
        })?
    }

    #[tokio::test]
    async fn kernel_tasks_preserve_context_on_one_owned_worker() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(
            root.path(),
            crate::runtime::QueryLimits {
                concurrency: 1,
                ..Default::default()
            },
        )?;
        let tasks = KernelTasks {
            handle: runtime.executor_handle(),
            executor: runtime.executor_owner(),
        };
        let input = Arc::new(());
        let input_witness = Arc::downgrade(&input);
        runtime
            .job_operation(
                "kernel-owner".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "fixture.kernel".into(),
                    request_digest: "a".repeat(64),
                    policy_digest: "b".repeat(64),
                },
                std::time::Duration::from_secs(30),
                async {
                    let expected = runtime.executor_owner().io_handle().id();
                    // Exercise the current-thread transport caller and nested native callback.
                    let nested = tasks.clone();
                    let result = tasks.block_on(async move {
                        nested.block_on(async {
                            (crate::runtime::operation_id(), Handle::current().id())
                        })
                    });
                    assert_eq!(result, (Some("kernel-owner".into()), expected));
                    let (send, receive) = tokio::sync::oneshot::channel();
                    let (release, wait) = tokio::sync::oneshot::channel();
                    crate::task_context::InputContext::owned(input)
                        .scope(async {
                            tasks.spawn(async move {
                                let _ = send
                                    .send((crate::runtime::operation_id(), Handle::current().id()));
                                let _ = wait.await;
                            });
                        })
                        .await;
                    assert_eq!(
                        receive.await.unwrap(),
                        (Some("kernel-owner".into()), expected)
                    );
                    assert!(
                        input_witness.upgrade().is_some(),
                        "kernel stream owns its private input"
                    );
                    release.send(()).unwrap();
                    let result = tasks
                        .spawn_blocking(|| (crate::runtime::operation_id(), Handle::current().id()))
                        .await
                        .unwrap();
                    assert_eq!(
                        result,
                        (Some("kernel-owner".into()), runtime.executor_handle().id())
                    );
                },
            )
            .await;
        runtime.close_diagnostics().await?;
        assert!(input_witness.upgrade().is_none());
        Ok(())
    }
}
