# `datafusion_physical_plan::execution_plan`

Crate `datafusion-physical-plan` · 27 public items · structured records in [`model/datafusion_physical_plan.execution_plan.json`](../model/datafusion_physical_plan.execution_plan.json)

## Boundedness

`enum` · `datafusion_physical_plan::execution_plan::Boundedness`

```rust
enum Boundedness
```

**Variants**: `Bounded`, `Unbounded`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_unbounded(&self) -> bool
```

Represents whether a stream of data **generated** by an operator is bounded (finite)
or unbounded (infinite).

This is used to determine whether an execution plan will eventually complete
processing all its data (bounded) or could potentially run forever (unbounded).

For unbounded streams, it also tracks whether the operator requires finite memory
to process the stream or if memory usage could grow unbounded.

Boundedness of the output stream is based on the boundedness of the input stream and the nature of
the operator. For example, limit or topk with fetch operator can convert an unbounded stream to a bounded stream.

---

## CardinalityEffect

`enum` · `datafusion_physical_plan::execution_plan::CardinalityEffect`

```rust
enum CardinalityEffect
```

**Variants**: `Unknown`, `Equal`, `LowerEqual`, `GreaterEqual`

Indicates the effect an execution plan operator will have on the cardinality
of its input stream

---

## ChildrenPropertiesMode

`enum` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode`

Also reachable as `datafusion::physical_plan::ChildrenPropertiesMode`, `datafusion_physical_plan::ChildrenPropertiesMode`

```rust
enum ChildrenPropertiesMode
```

**Variants**: `Keep`, `Recompute`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Indicates whether the plan properties of the new children must be recomputed.

Part of [`ReplaceChildrenOptions`].

---

## EmissionType

`enum` · `datafusion_physical_plan::execution_plan::EmissionType`

```rust
enum EmissionType
```

**Variants**: `Incremental`, `Final`, `Both`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Represents how an operator emits its output records.

This is used to determine whether an operator emits records incrementally as they arrive,
only emits a final result at the end, or can do both. Note that it generates the output -- record batch with `batch_size` rows
but it may still buffer data internally until it has enough data to emit a record batch or the source is exhausted.

For example, in the following plan:
```text
  SortExec [EmissionType::Final]
    |_ on: [col1 ASC]
    FilterExec [EmissionType::Incremental]
      |_ pred: col2 > 100
      DataSourceExec [EmissionType::Incremental]
        |_ file: "data.csv"
```
- DataSourceExec emits records incrementally as it reads from the file
- FilterExec processes and emits filtered records incrementally as they arrive
- SortExec must wait for all input records before it can emit the sorted result,
  since it needs to see all values to determine their final order

Left joins can emit both incrementally and finally:
- Incrementally emit matches as they are found
- Finally emit non-matches after all input is processed

---

## EvaluationType

`enum` · `datafusion_physical_plan::execution_plan::EvaluationType`

```rust
enum EvaluationType
```

**Variants**: `Lazy`, `Eager`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Represents how an operator's stream drives [`RecordBatch`] production
relative to downstream demand.

This is execution-topology metadata for optimizers. It distinguishes streams
whose batch production is driven directly by downstream calls to
`Stream::poll_next` from streams that may also drive input or output
production independently, such as by spawning tasks or buffering batches
ahead of demand.

---

## InvariantLevel

`enum` · `datafusion_physical_plan::execution_plan::InvariantLevel`

```rust
enum InvariantLevel
```

**Variants**: `Always`, `Executable`

**Derives**: Clone, Copy

[`ExecutionPlan`] Invariant Level

What set of assertions ([Invariant]s)  holds for a particular `ExecutionPlan`

[Invariant]: https://en.wikipedia.org/wiki/Invariant_(mathematics)#Invariants_in_computer_science

---

## SchedulingType

`enum` · `datafusion_physical_plan::execution_plan::SchedulingType`

```rust
enum SchedulingType
```

**Variants**: `NonCooperative`, `Cooperative`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Represents whether an operator's `Stream` has been implemented to actively cooperate with the
Tokio scheduler or not. Please refer to the [`coop`](crate::coop) module for more details.

---

## apply_expression_roots

`function` · `datafusion_physical_plan::execution_plan::apply_expression_roots`

Also reachable as `datafusion::physical_plan::apply_expression_roots`, `datafusion_physical_plan::apply_expression_roots`

```rust
fn apply_expression_roots<I>(roots: I, f: &mut dyn FnMut(&std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::TreeNodeRecursion>) -> datafusion_common::Result<datafusion_common::tree_node::TreeNodeRecursion> where I: IntoIterator, I::Item: AsPhysicalExprRef
```

Applies `f` to a shallow sequence of physical expression roots.

[`TreeNodeRecursion::Stop`] stops iteration and is returned immediately.
[`TreeNodeRecursion::Jump`] is normalized to [`TreeNodeRecursion::Continue`]
because this function does not visit expression children.

---

## check_default_invariants

`function` · `datafusion_physical_plan::execution_plan::check_default_invariants`

```rust
fn check_default_invariants<P: ExecutionPlan + ?Sized>(plan: &P, check: InvariantLevel) -> datafusion_common::Result<(), datafusion_common::DataFusionError>
```

Checks a set of invariants that apply to all ExecutionPlan implementations.
Returns an error if the given node does not conform.

---

## check_not_null_constraints

`function` · `datafusion_physical_plan::execution_plan::check_not_null_constraints`

```rust
fn check_not_null_constraints(batch: arrow::array::RecordBatch, column_indices: &Vec<usize>) -> datafusion_common::Result<arrow::array::RecordBatch>
```

Checks a `RecordBatch` for `not null` constraints on specified columns.

# Arguments

* `batch` - The `RecordBatch` to be checked
* `column_indices` - A vector of column indices that should be checked for
  `not null` constraints.

# Returns

* `Result<RecordBatch>` - The original `RecordBatch` if all constraints are met

This function iterates over the specified column indices and ensures that none
of the columns contain null values. If any column contains null values, an error
is returned.

---

## collect

`function` · `datafusion_physical_plan::execution_plan::collect`

Also reachable as `datafusion::physical_plan::collect`, `datafusion_physical_plan::collect`

```rust
async fn collect(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>>
```

Execute the [ExecutionPlan] and collect the results in memory

---

## collect_partitioned

`function` · `datafusion_physical_plan::execution_plan::collect_partitioned`

Also reachable as `datafusion::physical_plan::collect_partitioned`, `datafusion_physical_plan::collect_partitioned`

```rust
async fn collect_partitioned(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<Vec<arrow::array::RecordBatch>>>
```

Execute the [ExecutionPlan] and collect the results in memory

---

## displayable

`function` · `datafusion_physical_plan::execution_plan::displayable`

Also reachable as `datafusion::physical_plan::displayable`, `datafusion_physical_plan::displayable`

```rust
fn displayable(plan: &dyn ExecutionPlan) -> display::DisplayableExecutionPlan<'_>
```

Return a [`DisplayableExecutionPlan`] wrapper around an
[`ExecutionPlan`] which can be displayed in various easier to
understand ways.

See examples on [`DisplayableExecutionPlan`]

---

## execute_input_stream

`function` · `datafusion_physical_plan::execution_plan::execute_input_stream`

Also reachable as `datafusion::physical_plan::execute_input_stream`, `datafusion_physical_plan::execute_input_stream`

```rust
fn execute_input_stream(input: std::sync::Arc<dyn ExecutionPlan>, sink_schema: arrow::datatypes::SchemaRef, partition: usize, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream>
```

Executes an input stream and ensures that the resulting stream adheres to
the `not null` constraints specified in the `sink_schema`.

# Arguments

* `input` - An execution plan
* `sink_schema` - The schema to be applied to the output stream
* `partition` - The partition index to be executed
* `context` - The task context

# Returns

* `Result<SendableRecordBatchStream>` - A stream of `RecordBatch`es if successful

This function first executes the given input plan for the specified partition
and context. It then checks if there are any columns in the input that might
violate the `not null` constraints specified in the `sink_schema`. If there are
such columns, it wraps the resulting stream to enforce the `not null` constraints
by invoking the [`check_not_null_constraints`] function on each batch of the stream.

---

## execute_stream

`function` · `datafusion_physical_plan::execution_plan::execute_stream`

Also reachable as `datafusion::physical_plan::execute_stream`, `datafusion_physical_plan::execute_stream`

```rust
fn execute_stream(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream>
```

Execute the [ExecutionPlan] and return a single stream of `RecordBatch`es.

See [collect] to buffer the `RecordBatch`es in memory.

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

---

## execute_stream_partitioned

`function` · `datafusion_physical_plan::execution_plan::execute_stream_partitioned`

Also reachable as `datafusion::physical_plan::execute_stream_partitioned`, `datafusion_physical_plan::execute_stream_partitioned`

```rust
fn execute_stream_partitioned(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<SendableRecordBatchStream>>
```

Execute the [ExecutionPlan] and return a vec with one stream per output
partition

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

---

## get_plan_string

`function` · `datafusion_physical_plan::execution_plan::get_plan_string`

Also reachable as `datafusion::physical_plan::get_plan_string`, `datafusion_physical_plan::get_plan_string`

```rust
fn get_plan_string(plan: &std::sync::Arc<dyn ExecutionPlan>) -> Vec<String>
```

Utility function yielding a string representation of the given [`ExecutionPlan`].

---

## has_same_children_properties

`function` · `datafusion_physical_plan::execution_plan::has_same_children_properties`

```rust
fn has_same_children_properties(plan: &dyn ExecutionPlan, children: &[std::sync::Arc<dyn ExecutionPlan>]) -> datafusion_common::Result<bool>
```

Check if the `plan` children has the same properties as passed `children`.
In this case plan can avoid self properties re-computation when its children
replace is requested.
The size of `children` must be equal to the size of `ExecutionPlan::children()`.

---

## need_data_exchange

`function` · `datafusion_physical_plan::execution_plan::need_data_exchange`

```rust
fn need_data_exchange(plan: std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Indicate whether a data exchange is needed for the input of `plan`.

This identifies physical operators that redistribute child partitions or
gather multiple child partitions into one output partition:

1. RepartitionExec for non-round-robin repartitioning
2. CoalescePartitionsExec for collapsing multiple partitions into one without ordering guarantee
3. SortPreservingMergeExec for collapsing multiple sorted partitions into one with ordering guarantee

---

## replace_children_if_necessary

`function` · `datafusion_physical_plan::execution_plan::replace_children_if_necessary`

Also reachable as `datafusion::physical_plan::replace_children_if_necessary`, `datafusion_physical_plan::replace_children_if_necessary`

```rust
fn replace_children_if_necessary(plan: std::sync::Arc<dyn ExecutionPlan>, children: Vec<std::sync::Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Returns a plan with the given children, skipping as much work as possible.

This helper is the single entry point for "rebuild a plan from new
children" and applies three layers of short-circuits, from cheapest to
most expensive:

1. **Same child pointers** — if every `children[i]` is `Arc::ptr_eq` to the
   corresponding existing child, the original `plan` is returned
   unchanged (no allocation, no [`ExecutionPlan::replace_children`]
   call).
2. **Same child properties** — if the children's `PlanProperties` Arcs
   match (via [`has_same_children_properties`]), the plan's own
   `PlanProperties` cache can be reused. This calls
   [`ExecutionPlan::replace_children`] with [`ChildrenPropertiesMode::Keep`],
   which swaps the child pointers without recomputing `PlanProperties`.
3. **Full recompute** — otherwise, delegate to
   [`ExecutionPlan::replace_children`] with [`ChildrenPropertiesMode::Recompute`],
   which recomputes `PlanProperties` from scratch.

The size of `children` must be equal to the size of `ExecutionPlan::children()`.

---

## reset_plan_states

`function` · `datafusion_physical_plan::execution_plan::reset_plan_states`

```rust
fn reset_plan_states(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Make plan ready to be re-executed returning its clone with state reset for all nodes.

Some plans will change their internal states after execution, making them unable to be executed again.
This function uses [`ExecutionPlan::reset_state`] to reset any internal state within the plan.

An example is `CrossJoinExec`, which loads the left table into memory and stores it in the plan.
However, if the data of the left table is derived from the work table, it will become outdated
as the work table changes. When the next iteration executes this plan again, we must clear the left table.

# Limitations

While this function enables plan reuse, it does not allow the same plan to be executed if it (OR):

* uses dynamic filters,
* represents a recursive query.

---

## with_new_children_if_necessary

`function` · `datafusion_physical_plan::execution_plan::with_new_children_if_necessary`

> **Deprecated** — since 55.0.0: Use `replace_children_if_necessary`

Also reachable as `datafusion::physical_plan::with_new_children_if_necessary`, `datafusion_physical_plan::with_new_children_if_necessary`

```rust
fn with_new_children_if_necessary(plan: std::sync::Arc<dyn ExecutionPlan>, children: Vec<std::sync::Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

---

## PlanProperties

`struct` · `datafusion_physical_plan::execution_plan::PlanProperties`

Also reachable as `datafusion::physical_plan::PlanProperties`, `datafusion_physical_plan::PlanProperties`

```rust
struct PlanProperties
```

**Fields**: `eq_properties`, `partitioning`, `emission_type`, `boundedness`, `evaluation_type`, `scheduling_type`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (13)

```rust
fn equivalence_properties(&self) -> &EquivalenceProperties
fn new(eq_properties: EquivalenceProperties, partitioning: Partitioning, emission_type: EmissionType, boundedness: Boundedness) -> Self
fn output_ordering(&self) -> Option<&LexOrdering>
fn output_partitioning(&self) -> &Partitioning
fn set_constraints(&mut self, constraints: Constraints)
fn set_eq_properties(&mut self, eq_properties: EquivalenceProperties)
fn with_boundedness(self, boundedness: Boundedness) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_emission_type(self, emission_type: EmissionType) -> Self
fn with_eq_properties(self, eq_properties: EquivalenceProperties) -> Self
fn with_evaluation_type(self, drive_type: EvaluationType) -> Self
fn with_partitioning(self, partitioning: Partitioning) -> Self
fn with_scheduling_type(self, scheduling_type: SchedulingType) -> Self
```

Stores plan properties used in query optimization.

Serves as a cache for these properties, which are often
expensive to compute.

---

## ReplaceChildrenOptions

`struct` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions`

Also reachable as `datafusion::physical_plan::ReplaceChildrenOptions`, `datafusion_physical_plan::ReplaceChildrenOptions`

```rust
struct ReplaceChildrenOptions
```

**Fields**: `children_properties`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn new(children_properties: ChildrenPropertiesMode) -> Self
```

Options for [`ExecutionPlan::replace_children`]

---

## AsPhysicalExprRef

`trait` · `datafusion_physical_plan::execution_plan::AsPhysicalExprRef`

Also reachable as `datafusion::physical_plan::AsPhysicalExprRef`, `datafusion_physical_plan::AsPhysicalExprRef`

```rust
trait AsPhysicalExprRef
```

**Implementors** (2)

- `alloc::sync::Arc`
- `datafusion_physical_expr::projection::ProjectionExpr`

**Methods** (1)

```rust
fn as_physical_expr_ref(&self) -> &Arc<dyn PhysicalExpr>
```

Allows a type to be treated as a reference to an
[`Arc<dyn PhysicalExpr>`].

Used by [`apply_expression_roots`].

---

## ExecutionPlan

`trait` · `datafusion_physical_plan::execution_plan::ExecutionPlan`

Also reachable as `datafusion::physical_plan::ExecutionPlan`, `datafusion_physical_plan::ExecutionPlan`

```rust
trait ExecutionPlan: Any + Debug + DisplayAs + Send + Sync
```

**Implementors** (47)

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

**Methods** (35)

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn downcast_delegate(&self) -> Option<&dyn ExecutionPlan>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn properties(&self) -> &Arc<PlanProperties>
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
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

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

---

## ExecutionPlanProperties

`trait` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties`

Also reachable as `datafusion::physical_plan::ExecutionPlanProperties`, `datafusion_physical_plan::ExecutionPlanProperties`

```rust
trait ExecutionPlanProperties
```

**Implementors** (1)

- `alloc::sync::Arc`

**Methods** (5)

```rust
fn boundedness(&self) -> Boundedness
fn equivalence_properties(&self) -> &EquivalenceProperties
fn output_ordering(&self) -> Option<&LexOrdering>
fn output_partitioning(&self) -> &Partitioning
fn pipeline_behavior(&self) -> EmissionType
```

Extension trait provides an easy API to fetch various properties of
[`ExecutionPlan`] objects based on [`ExecutionPlan::properties`].

---
