# `datafusion_physical_plan::limit`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.limit.json`](../model/datafusion_physical_plan.limit.json)

## GlobalLimitExec

`struct` · `datafusion_physical_plan::limit::GlobalLimitExec`

```rust
struct GlobalLimitExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn fetch(&self) -> Option<usize>
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, skip: usize, fetch: Option<usize>) -> Self
fn required_ordering(&self) -> &Option<LexOrdering>
fn set_required_ordering(&mut self, required_ordering: Option<LexOrdering>)
fn skip(&self) -> usize
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Limit execution plan

---

## LimitStream

`struct` · `datafusion_physical_plan::limit::LimitStream`

```rust
struct LimitStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Methods** (1)

```rust
fn new(input: SendableRecordBatchStream, skip: usize, fetch: Option<usize>, baseline_metrics: BaselineMetrics) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A Limit stream skips `skip` rows, and then fetch up to `fetch` rows.

---

## LocalLimitExec

`struct` · `datafusion_physical_plan::limit::LocalLimitExec`

```rust
struct LocalLimitExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn fetch(&self) -> usize
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, fetch: usize) -> Self
fn required_ordering(&self) -> &Option<LexOrdering>
fn set_required_ordering(&mut self, required_ordering: Option<LexOrdering>)
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

LocalLimitExec applies a limit to a single partition

---
