# ExecutionPlan

`datafusion_physical_plan::execution_plan::ExecutionPlan`

```rust
trait ExecutionPlan: Any + Debug + DisplayAs + Send + Sync
```

Also reachable as `datafusion::physical_plan::ExecutionPlan`, `datafusion_physical_plan::ExecutionPlan`

Prose: [`api/datafusion_physical_plan.execution_plan.md`](../api/datafusion_physical_plan.execution_plan.md#executionplan) · records: [`model/datafusion_physical_plan.execution_plan.json`](../model/datafusion_physical_plan.execution_plan.json)

## Required

Every implementation must supply these.

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn name(&self) -> &str
fn properties(&self) -> &Arc<PlanProperties>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
fn downcast_delegate(&self) -> Option<&dyn ExecutionPlan>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn repartitioned(&self, _target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn static_name() -> &'static str where Self: Sized
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_pushdown_sort(&self, _order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, _projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, _ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

## Implementors (47)

Read one before writing your own.

- `datafusion_datasource::sink::DataSinkExec`
- `datafusion_datasource::source::DataSourceExec`
- `datafusion_ffi::execution_plan::ForeignExecutionPlan`
- `datafusion_ffi::execution_plan::tests::EmptyExec`
- `datafusion_physical_optimizer::output_requirements::OutputRequirementExec`
- `datafusion_physical_plan::aggregates::AggregateExec`
- `datafusion_physical_plan::analyze::AnalyzeExec`
- `datafusion_physical_plan::async_func::AsyncFuncExec`
- `datafusion_physical_plan::buffer::BufferExec`
- `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec`
- `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec`
- `datafusion_physical_plan::coop::CooperativeExec`
- `datafusion_physical_plan::empty::EmptyExec`
- `datafusion_physical_plan::explain::ExplainExec`
- `datafusion_physical_plan::filter::FilterExec`
- `datafusion_physical_plan::joins::cross_join::CrossJoinExec`
- `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec`
- `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec`
- `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec`
- `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec`
- `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec`
- `datafusion_physical_plan::limit::GlobalLimitExec`
- `datafusion_physical_plan::limit::LocalLimitExec`
- `datafusion_physical_plan::memory::LazyMemoryExec`
- `datafusion_physical_plan::placeholder_row::PlaceholderRowExec`
- `datafusion_physical_plan::projection::ProjectionExec`
- `datafusion_physical_plan::recursive_query::RecursiveQueryExec`
- `datafusion_physical_plan::repartition::RepartitionExec`
- `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec`
- `datafusion_physical_plan::sorts::partial_sort::PartialSortExec`
- `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec`
- `datafusion_physical_plan::sorts::sort::SortExec`
- `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec`
- `datafusion_physical_plan::streaming::StreamingTableExec`
- `datafusion_physical_plan::test::TestMemoryExec`
- `datafusion_physical_plan::test::exec::BarrierExec`
- `datafusion_physical_plan::test::exec::BlockingExec`
- `datafusion_physical_plan::test::exec::ErrorExec`
- `datafusion_physical_plan::test::exec::MockExec`
- `datafusion_physical_plan::test::exec::PanicExec`
- `datafusion_physical_plan::test::exec::StatisticsExec`
- `datafusion_physical_plan::union::InterleaveExec`
- `datafusion_physical_plan::union::UnionExec`
- `datafusion_physical_plan::unnest::UnnestExec`
- `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec`
- `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec`
- `datafusion_physical_plan::work_table::WorkTableExec`

## Demonstrated by 4 upstream example(s)

- [`corpus/examples/custom_data_source/custom_datasource.rs`](../corpus/examples/custom_data_source/custom_datasource.rs)
- [`corpus/examples/execution_monitoring/memory_pool_execution_plan.rs`](../corpus/examples/execution_monitoring/memory_pool_execution_plan.rs)
- [`corpus/examples/proto/composed_extension_codec.rs`](../corpus/examples/proto/composed_extension_codec.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Documentation

Represent nodes in the DataFusion Physical Plan.

Calling [`execute`] produces an `async` [`SendableRecordBatchStream`] of
[`RecordBatch`] that incrementally computes a partition of the
`ExecutionPlan`'s output from its input. See [`Partitioning`] for more
details on partitioning.

Methods such as [`Self::schema`] and [`Self::properties`] communicate
properties of the output to the DataFusion optimizer, and methods such as
[`required_input_distribution`] and [`required_input_ordering`] express
requirements of the `ExecutionPlan` from its input.

[`ExecutionPlan`] can be displayed in a simplified form using the
return value from [`displayable`] in addition to the (normally
quite verbose) `Debug` output.

[`execute`]: ExecutionPlan::execute
[`required_input_distribution`]: ExecutionPlan::required_input_distribution
[`required_input_ordering`]: ExecutionPlan::required_input_ordering

# Examples

See [`datafusion-examples`] for examples, including
[`memory_pool_execution_plan.rs`] which shows how to implement a custom
`ExecutionPlan` with memory tracking and spilling support.

[`datafusion-examples`]: https://github.com/apache/datafusion/tree/main/datafusion-examples
[`memory_pool_execution_plan.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/execution_monitoring/memory_pool_execution_plan.rs
