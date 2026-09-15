# `datafusion_physical_plan::coop`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.coop.json`](../model/datafusion_physical_plan.coop.json)

## cooperative

`function` · `datafusion_physical_plan::coop::cooperative`

```rust
fn cooperative<T>(stream: T) -> CooperativeStream<T> where T: RecordBatchStream + Unpin + Send + 'static
```

Creates a [`CooperativeStream`] wrapper around the given [`RecordBatchStream`].
This wrapper collaborates with the Tokio cooperative scheduler by consuming a unit of
scheduling budget for each returned record batch.

---

## make_cooperative

`function` · `datafusion_physical_plan::coop::make_cooperative`

```rust
fn make_cooperative(stream: SendableRecordBatchStream) -> SendableRecordBatchStream
```

Wraps a `SendableRecordBatchStream` inside a [`CooperativeStream`] to enable cooperative multitasking.
Since `SendableRecordBatchStream` is a `dyn RecordBatchStream` this requires the use of dynamic
method dispatch.
When the stream type is statically known, consider use the generic [`cooperative`] function
to allow static method dispatch.

---

## CooperativeExec

`struct` · `datafusion_physical_plan::coop::CooperativeExec`

```rust
struct CooperativeExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, _t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, task_ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn name(&self) -> &str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> Arc<Schema>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

An execution plan decorator that enables cooperative multitasking.
It wraps the streams produced by its input execution plan using the [`make_cooperative`] function,
which makes the stream participate in Tokio cooperative scheduling.

---

## CooperativeStream

`struct` · `datafusion_physical_plan::coop::CooperativeStream`

```rust
struct CooperativeStream<T> where T: RecordBatchStream + Unpin
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Methods** (1)

```rust
fn new(inner: T) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> Arc<Schema>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A stream that passes record batches through unchanged while cooperating with the Tokio runtime.
It consumes cooperative scheduling budget for each returned [`RecordBatch`],
allowing other tasks to execute when the budget is exhausted.

See the [module level documentation](crate::coop) for an in-depth discussion.

---
