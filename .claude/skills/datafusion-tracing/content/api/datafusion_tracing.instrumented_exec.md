# `datafusion_tracing::instrumented_exec`

Crate `datafusion-tracing` · 4 public items · structured records in [`model/datafusion_tracing.instrumented_exec.json`](../model/datafusion_tracing.instrumented_exec.json)

## ExecutionRecorders

`struct` · `datafusion_tracing::instrumented_exec::ExecutionRecorders`

```rust
struct ExecutionRecorders
```

**Methods** (7)

```rust
fn cancel_stream(&Arc<self>, partition: usize)
fn finish_stream(&Arc<self>)
fn is_same_execution(&self, parent_span_id: Option<&Id>, context: &Arc<TaskContext>) -> bool
fn new(slot: Weak<Mutex<Vec<Arc<ExecutionRecorders>>>>, parent_span_id: Option<Id>, context: Arc<TaskContext>, partition: usize, node_recorder: NodeRecorder, metrics_recorder: Option<MetricsRecorder>, preview_recorder: Option<PreviewRecorder>) -> Self
fn release_stream(&Arc<self>, canceled_partition: Option<usize>)
fn span(&self) -> Span
fn try_reserve_partition(&self, partition: usize) -> bool
```

---

## ExecutionRecordingStream

`struct` · `datafusion_tracing::instrumented_exec::ExecutionRecordingStream`

```rust
struct ExecutionRecordingStream
```

**Implements**: `core::ops::drop::Drop`, `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (3)

```rust
fn new(inner: SendableRecordBatchStream, recorders: Arc<ExecutionRecorders>) -> Self
fn project<'pin>(_pin_project::__private::Pin<&'pin mut self>) -> __ExecutionRecordingStreamProjection<'pin>
fn project_ref<'pin>(_pin_project::__private::Pin<&'pin self>) -> __ExecutionRecordingStreamProjectionRef<'pin>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

---

## InstrumentedExec

`struct` · `datafusion_tracing::instrumented_exec::InstrumentedExec`

```rust
struct InstrumentedExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (9)

```rust
fn create_populated_span(&self) -> Span
fn execution_recording_stream(&self, inner_stream: SendableRecordBatchStream, recorders: Arc<ExecutionRecorders>) -> SendableRecordBatchStream
fn is_instrumented(plan: &dyn ExecutionPlan) -> bool
fn metrics_recording_stream(&self, inner_stream: SendableRecordBatchStream, recorder: Arc<MetricsRecorder>) -> SendableRecordBatchStream
fn new(inner: Arc<dyn ExecutionPlan>, span_create_fn: Arc<dyn Fn() -> tracing::Span + Send + Sync>, options: &InstrumentationOptions) -> InstrumentedExec
fn node_recording_stream(&self, inner_stream: SendableRecordBatchStream, recorder: Arc<NodeRecorder>) -> SendableRecordBatchStream
fn preview_recording_stream(&self, inner_stream: SendableRecordBatchStream, recorder: Arc<PreviewRecorder>, partition: usize) -> SendableRecordBatchStream
fn reserve_recorders(&self, context: Arc<TaskContext>, partition: usize) -> Arc<ExecutionRecorders>
fn with_new_inner(&self, inner: Arc<dyn ExecutionPlan>) -> Arc<dyn ExecutionPlan>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, format: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn downcast_delegate(&self) -> Option<&dyn ExecutionPlan>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn static_name() -> &'static str
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_state(&self, state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

An [`ExecutionPlan`] wrapper that instruments execution with tracing spans and metrics recording.

---

## SpanCreateFn

`type_alias` · `datafusion_tracing::instrumented_exec::SpanCreateFn`

```rust
type SpanCreateFn = dyn Fn() -> tracing::Span + Send + Sync
```

Type alias for a function that creates a tracing span.

---
