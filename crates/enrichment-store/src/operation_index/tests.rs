use super::*;
use crate::runtime::QueryLimits;

async fn input(runtime: &QueryRuntime) -> DataFrame {
    runtime.session().sql("SELECT CAST(0 AS BIGINT UNSIGNED) AS plan, 'α' AS key, 'one' AS label UNION ALL SELECT CAST(0 AS BIGINT UNSIGNED), 'β', NULL UNION ALL SELECT CAST(0 AS BIGINT UNSIGNED), 'α', 'one'")
        .await.unwrap()
}
fn descriptor() -> enrichment_core::telemetry::OperationDescriptor {
    enrichment_core::telemetry::OperationDescriptor {
        method: "unit.materialization".into(),
        request_digest: "unit-request".into(),
        policy_digest: "unit-policy".into(),
    }
}
#[tokio::test]
async fn cache_planning_is_pure_and_count_replays_complete_base() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        root.path(),
        QueryLimits {
            concurrency: 1,
            ..Default::default()
        },
    )
    .unwrap();
    runtime
        .operation("cache-unit".into(), descriptor(), async {
            let frame = cache(
                &runtime,
                input(&runtime).await,
                QueryFamily::Intermediate(
                    enrichment_core::telemetry::MaterializationFamily::ComparisonKeys,
                ),
            )
            .await
            .unwrap();
            let plan = frame.logical_plan();
            let LogicalPlan::Extension(extension) = plan else {
                panic!("cache extension")
            };
            let binding = extension
                .node
                .as_any()
                .downcast_ref::<Materialization>()
                .unwrap()
                .binding
                .clone();
            frame.clone().create_physical_plan().await.unwrap();
            frame
                .clone()
                .explain(false, false)
                .unwrap()
                .collect()
                .await
                .unwrap();
            assert_eq!(binding.state.load(AtomicOrdering::Acquire), UNSTARTED);
            assert_eq!(
                runtime
                    .session()
                    .runtime_env()
                    .disk_manager
                    .used_disk_space(),
                0
            );
            let narrowed = frame
                .clone()
                .filter(datafusion::prelude::col("key").eq(datafusion::prelude::lit("β")))
                .unwrap()
                .select_columns(&["key"])
                .unwrap()
                .limit(0, Some(1))
                .unwrap();
            let rows = runtime.execute_family(narrowed, None).await.unwrap();
            assert_eq!(rows.rows, 1);
            assert_eq!(count(&runtime, frame.clone()).await.unwrap(), 3);
            assert_eq!(binding.state.load(AtomicOrdering::Acquire), READY);
            let fills = binding
                .metrics
                .clone_inner()
                .sum_by_name("cache_fills")
                .unwrap()
                .as_usize();
            assert_eq!(fills, 1);
            drop((binding, frame));
            assert_eq!(
                runtime
                    .session()
                    .runtime_env()
                    .disk_manager
                    .used_disk_space(),
                0
            );
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}

fn binding(frame: &DataFrame) -> Arc<Binding> {
    let LogicalPlan::Extension(extension) = frame.logical_plan() else {
        panic!("cache extension")
    };
    extension
        .node
        .as_any()
        .downcast_ref::<Materialization>()
        .unwrap()
        .binding
        .clone()
}
fn family() -> QueryFamily {
    QueryFamily::Intermediate(enrichment_core::telemetry::MaterializationFamily::ComparisonKeys)
}

#[tokio::test]
async fn factory_refuses_unbound_volatile_mutable_and_wrong_family_inputs() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    assert!(
        cache(&runtime, input(&runtime).await, family())
            .await
            .is_err()
    );
    runtime.operation("cache-refusal".into(), descriptor(), async {
        assert!(input(&runtime).await.cache().await.is_err());
        assert!(cache(&runtime, input(&runtime).await, QueryFamily::Count).await.is_err());
        let volatile = runtime.session().sql("SELECT CAST(0 AS BIGINT UNSIGNED) AS plan, CAST(random() AS VARCHAR) AS key, 'x' AS label").await.unwrap();
        assert!(cache(&runtime, volatile, family()).await.is_err());
        let frame = input(&runtime).await;
        let batches = frame.collect().await.unwrap();
        let mutable = runtime.session().read_table(Arc::new(datafusion::datasource::MemTable::try_new(
            batches[0].schema(), vec![batches],
        ).unwrap())).unwrap();
        assert!(cache(&runtime, mutable.clone(), family()).await.is_err());
        crate::leases::initialize(root.path()).unwrap();
        let lease = crate::leases::shared(root.path()).unwrap();
        let hidden_mutable = Arc::new(crate::leases::LeasedProvider::new(mutable.into_view(), lease.clone()));
        let hidden_mutable = runtime.session().read_table(hidden_mutable).unwrap();
        assert!(cache(&runtime, hidden_mutable, family()).await.is_err());
        let volatile = runtime.session().sql("SELECT CAST(0 AS BIGINT UNSIGNED) AS plan, CAST(random() AS VARCHAR) AS key, 'x' AS label").await.unwrap();
        let hidden_volatile = Arc::new(crate::leases::LeasedProvider::new(volatile.into_view(), lease));
        let hidden_volatile = runtime.session().read_table(hidden_volatile).unwrap();
        assert!(cache(&runtime, hidden_volatile, family()).await.is_err());
        let wrong = runtime.session().sql("SELECT 1 AS wrong").await.unwrap();
        assert!(cache(&runtime, wrong, family()).await.is_err());
    }).await.unwrap();
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn immutable_arrow_ingress_survives_constraint_and_lease_wrappers() {
    let root = tempfile::tempdir().unwrap();
    crate::leases::initialize(root.path()).unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    runtime
        .operation("immutable-ingress".into(), descriptor(), async {
            let source = input(&runtime).await.collect().await.unwrap();
            let schema = source[0].schema();
            let batch = arrow::compute::concat_batches(&schema, &source).unwrap();
            let ingress = Arc::new(
                crate::admitted_provider::AdmittedProvider::from_batch(
                    batch,
                    &runtime.session().runtime_env().memory_pool,
                )
                .unwrap(),
            );
            let constrained = Arc::new(crate::admitted_provider::AdmittedProvider::new(
                ingress,
                datafusion::common::Constraints::new_unverified(vec![]),
            ));
            let leased = Arc::new(crate::leases::LeasedProvider::new(
                constrained,
                crate::leases::shared(root.path()).unwrap(),
            ));
            let frame = runtime.session().read_table(leased).unwrap();
            let cached = cache(&runtime, frame, family()).await.unwrap();
            assert_eq!(count(&runtime, cached).await.unwrap(), 3);
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn operation_identity_and_rewrite_admission_are_exact() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let cached = runtime
        .operation("same-text".into(), descriptor(), async {
            let cached = cache(&runtime, input(&runtime).await, family())
                .await
                .unwrap();
            let value = binding(&cached);
            let node = Materialization {
                binding: value.clone(),
                admitted: true,
            };
            assert!(
                UserDefinedLogicalNodeCore::with_exprs_and_inputs(&node, vec![], vec![]).is_ok()
            );
            assert!(UserDefinedLogicalNodeCore::inputs(&node).is_empty());
            assert!(
                UserDefinedLogicalNodeCore::with_exprs_and_inputs(
                    &node,
                    vec![],
                    vec![value.input.clone()]
                )
                .is_err(),
                "even an equal base cannot become a rewritable consumer child"
            );
            let changed = DataFrame::new(runtime.session().state(), value.input.clone())
                .limit(0, Some(1))
                .unwrap()
                .into_parts()
                .1;
            assert!(
                UserDefinedLogicalNodeCore::with_exprs_and_inputs(&node, vec![], vec![changed])
                    .is_err()
            );
            let changed_policy = datafusion::prelude::SessionContext::new();
            assert!(value.require(&changed_policy.state()).is_err());
            let no_planner = datafusion::physical_planner::DefaultPhysicalPlanner::default();
            assert!(
                no_planner
                    .create_physical_plan(cached.logical_plan(), &runtime.session().state())
                    .await
                    .is_err()
            );
            cached
        })
        .await
        .unwrap();
    runtime
        .operation("same-text".into(), descriptor(), async {
            assert!(
                cached.create_physical_plan().await.is_err(),
                "text equality cannot revive a dead operation"
            );
        })
        .await
        .unwrap();
    drop(cached);
    runtime.close_diagnostics().await.unwrap();
}

#[derive(Debug)]
struct ControlledInput {
    input: Arc<dyn ExecutionPlan>,
    starts: Arc<tokio::sync::Semaphore>,
    proceed: Arc<tokio::sync::Semaphore>,
    failure: bool,
}
impl DisplayAs for ControlledInput {
    fn fmt_as(&self, _: DisplayFormatType, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ControlledUnitInput")
    }
}
impl ExecutionPlan for ControlledInput {
    fn name(&self) -> &str {
        "ControlledUnitInput"
    }
    fn properties(&self) -> &Arc<PlanProperties> {
        self.input.properties()
    }
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![&self.input]
    }
    fn apply_expressions(
        &self,
        _: &mut dyn FnMut(
            &Arc<dyn datafusion::physical_expr::PhysicalExpr>,
        ) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }
    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        assert_eq!(children.len(), 1);
        Ok(Arc::new(Self {
            input: children[0].clone(),
            starts: self.starts.clone(),
            proceed: self.proceed.clone(),
            failure: self.failure,
        }))
    }
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let starts = self.starts.clone();
        let proceed = self.proceed.clone();
        let input = self.input.clone();
        let failure = self.failure;
        Ok(Box::pin(RecordBatchStreamAdapter::new(
            self.schema(),
            futures::stream::once(async move {
                starts.add_permits(1);
                proceed.acquire().await.unwrap().forget();
                if failure {
                    return Err(DataFusionError::Execution("unit upstream failure".into()));
                }
                input.execute(partition, context)
            })
            .try_flatten(),
        )))
    }
}
async fn controlled(
    frame: &DataFrame,
    failure: bool,
) -> (
    Arc<dyn ExecutionPlan>,
    Arc<tokio::sync::Semaphore>,
    Arc<tokio::sync::Semaphore>,
) {
    let actual = frame.create_physical_plan().await.unwrap();
    let starts = Arc::new(tokio::sync::Semaphore::new(0));
    let proceed = Arc::new(tokio::sync::Semaphore::new(0));
    let input = Arc::new(ControlledInput {
        input: actual.children()[0].clone(),
        starts: starts.clone(),
        proceed: proceed.clone(),
        failure,
    });
    (
        Arc::new(MaterializationExec::new(input, binding(frame)).unwrap()),
        starts,
        proceed,
    )
}

#[tokio::test]
async fn concurrent_physical_readers_share_fill_and_abandoned_waiter_does_not_cancel_it() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    runtime
        .operation("shared".into(), descriptor(), async {
            let frame = cache(&runtime, input(&runtime).await, family())
                .await
                .unwrap();
            let (first, starts, proceed) = controlled(&frame, false).await;
            // Separately lower a clone: its child is never polled because the first owns this fill.
            let second = frame.clone().create_physical_plan().await.unwrap();
            let mut abandoned = first.execute(0, runtime.session().task_ctx()).unwrap();
            let mut wait = Box::pin(abandoned.try_next());
            assert!(futures::poll!(&mut wait).is_pending());
            tokio::time::timeout(std::time::Duration::from_secs(5), starts.acquire())
                .await
                .unwrap()
                .unwrap()
                .forget();
            drop(wait);
            drop(abandoned);
            assert_eq!(binding(&frame).state.load(AtomicOrdering::Acquire), FILLING);
            let second = second.execute(0, runtime.session().task_ctx()).unwrap();
            proceed.add_permits(16);
            let batches = second.try_collect::<Vec<_>>().await.unwrap();
            assert_eq!(
                batches.iter().map(|batch| batch.num_rows()).sum::<usize>(),
                3
            );
            assert_eq!(
                binding(&frame)
                    .metrics
                    .clone_inner()
                    .sum_by_name("cache_fills")
                    .unwrap()
                    .as_usize(),
                1
            );
            assert_eq!(count(&runtime, frame).await.unwrap(), 3);
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn upstream_failure_is_shared_terminal_and_never_retried_or_empty() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    runtime
        .operation("failed".into(), descriptor(), async {
            let frame = cache(&runtime, input(&runtime).await, family())
                .await
                .unwrap();
            let (first, _, proceed) = controlled(&frame, true).await;
            proceed.add_permits(16);
            let error = first
                .execute(0, runtime.session().task_ctx())
                .unwrap()
                .try_collect::<Vec<_>>()
                .await
                .unwrap_err();
            assert!(error.to_string().contains("unit upstream failure"));
            assert_eq!(binding(&frame).state.load(AtomicOrdering::Acquire), FAILED);
            let error = count(&runtime, frame.clone()).await.unwrap_err();
            assert!(error.to_string().contains("unit upstream failure"));
            assert_eq!(
                binding(&frame)
                    .metrics
                    .clone_inner()
                    .sum_by_name("cache_fills")
                    .unwrap()
                    .as_usize(),
                1
            );
            assert_eq!(
                runtime
                    .session()
                    .runtime_env()
                    .disk_manager
                    .used_disk_space(),
                0
            );
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn owner_cancellation_stops_a_fill_that_has_no_current_waiter() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let value = runtime
        .operation("cancelled".into(), descriptor(), async {
            let frame = cache(&runtime, input(&runtime).await, family())
                .await
                .unwrap();
            let (first, starts, _) = controlled(&frame, false).await;
            let mut stream = first.execute(0, runtime.session().task_ctx()).unwrap();
            let mut pending = Box::pin(stream.try_next());
            assert!(futures::poll!(&mut pending).is_pending());
            tokio::time::timeout(std::time::Duration::from_secs(5), starts.acquire())
                .await
                .unwrap()
                .unwrap()
                .forget();
            binding(&frame)
        })
        .await
        .unwrap();
    let outcome = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        value.fill.get().unwrap().clone(),
    )
    .await
    .unwrap();
    assert!(outcome.is_err());
    assert_eq!(value.state.load(AtomicOrdering::Acquire), CANCELLED);
    drop(value);
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn native_reader_holds_spill_and_reservation_after_plan_drop() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    runtime
        .operation("retained".into(), descriptor(), async {
            let frame = cache(&runtime, input(&runtime).await, family())
                .await
                .unwrap();
            let physical = frame.create_physical_plan().await.unwrap();
            assert_eq!(physical.properties().partitioning.partition_count(), 1);
            assert!(physical.properties().output_ordering().is_none());
            let mut stream = physical.execute(0, runtime.session().task_ctx()).unwrap();
            let batch = stream.try_next().await.unwrap().unwrap();
            assert!(batch.num_rows() > 0);
            drop((physical, frame));
            let env = runtime.session().runtime_env();
            assert!(env.disk_manager.used_disk_space() > 0);
            assert!(env.memory_pool.reserved() >= 128 * 1024);
            drop(stream);
            assert_eq!(env.disk_manager.used_disk_space(), 0);
            let held = batch.slice(0, 1);
            drop(batch);
            let with_output = env.memory_pool.reserved();
            assert!(with_output > 0, "emitted Arrow buffers retain payment");
            drop(held);
            assert!(
                env.memory_pool.reserved() < with_output,
                "the final Arrow slice releases its own charge"
            );
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn empty_fill_and_views_share_one_native_relation() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    runtime
        .operation("empty".into(), descriptor(), async {
            let frame = cache(
                &runtime,
                input(&runtime)
                    .await
                    .filter(datafusion::prelude::lit(false))
                    .unwrap(),
                family(),
            )
            .await
            .unwrap();
            let session = runtime.session();
            crate::native_catalog::work(&session, "unit_cached", frame.clone().into_view())
                .unwrap();
            assert_eq!(
                count(&runtime, session.table("unit_cached").await.unwrap())
                    .await
                    .unwrap(),
                0
            );
            assert_eq!(
                count(&runtime, session.table("unit_cached").await.unwrap())
                    .await
                    .unwrap(),
                0
            );
            assert_eq!(binding(&frame).state.load(AtomicOrdering::Acquire), READY);
            assert_eq!(
                binding(&frame)
                    .metrics
                    .clone_inner()
                    .sum_by_name("cache_fills")
                    .unwrap()
                    .as_usize(),
                1
            );
            assert_eq!(
                runtime
                    .session()
                    .runtime_env()
                    .disk_manager
                    .used_disk_space(),
                0
            );
        })
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}
