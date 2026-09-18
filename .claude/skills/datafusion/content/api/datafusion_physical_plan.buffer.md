# `datafusion_physical_plan::buffer`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.buffer.json`](../model/datafusion_physical_plan.buffer.json)

## BufferExec

`struct` · `datafusion_physical_plan::buffer::BufferExec`

```rust
struct BufferExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn capacity(&self) -> usize
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, capacity: usize) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.buffer.BufferExec.md).


WARNING: EXPERIMENTAL

Decouples production and consumption of record batches with an internal queue per partition,
eagerly filling up the capacity of the queues even before any message is requested.

```text
            ┌───────────────────────────┐
            │        BufferExec         │
            │                           │
            │┌────── Partition 0 ──────┐│
            ││            ┌────┐ ┌────┐││       ┌────┐
──background poll────────▶│    │ │    ├┼┼───────▶    │
            ││            └────┘ └────┘││       └────┘
            │└─────────────────────────┘│
            │┌────── Partition 1 ──────┐│
            ││     ┌────┐ ┌────┐ ┌────┐││       ┌────┐
──background poll─▶│    │ │    │ │    ├┼┼───────▶    │
            ││     └────┘ └────┘ └────┘││       └────┘
            │└─────────────────────────┘│
            │                           │
            │           ...             │
            │                           │
            │┌────── Partition N ──────┐│
            ││                   ┌────┐││       ┌────┐
──background poll───────────────▶│    ├┼┼───────▶    │
            ││                   └────┘││       └────┘
            │└─────────────────────────┘│
            └───────────────────────────┘
```

The capacity is provided in bytes, and for each buffered record batch it will take into account
the size reported by [RecordBatch::get_array_memory_size].

If a single record batch exceeds the maximum capacity set in the `capacity` argument, it's still
allowed to pass in order to not deadlock the buffer.

This is useful for operators that conditionally start polling one of their children only after
other child has finished, allowing to perform some early work and accumulating batches in
memory so that they can be served immediately when requested.

---

## MemoryBufferedStream

`struct` · `datafusion_physical_plan::buffer::MemoryBufferedStream`

```rust
struct MemoryBufferedStream<T: SizedMessage>
```

**Implements**: `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (2)

```rust
fn messages_queued(&self) -> usize
fn new(input: impl Stream<Item = Result<T>> + Unpin + Send + 'static, capacity: usize, memory_reservation: MemoryReservation) -> Self
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.buffer.MemoryBufferedStream.md).


Decouples production and consumption of messages in a stream with an internal queue, eagerly
filling it up to the specified maximum capacity even before any message is requested.

Allows each message to have a different size, which is taken into account for determining if
the queue is full or not.

---

## SizedMessage

`trait` · `datafusion_physical_plan::buffer::SizedMessage`

```rust
trait SizedMessage
```

**Implementors** (1)

- `arrow_array::record_batch::RecordBatch`

**Methods** (1)

```rust
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.buffer.SizedMessage.md).


Represents anything that occupies a capacity in a [MemoryBufferedStream].

---
