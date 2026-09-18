# `datafusion_physical_plan::filter`

Crate `datafusion-physical-plan` · 6 public items · structured records in [`model/datafusion_physical_plan.filter.json`](../model/datafusion_physical_plan.filter.json)

## batch_filter

`function` · `datafusion_physical_plan::filter::batch_filter`

```rust
fn batch_filter(batch: &arrow::record_batch::RecordBatch, predicate: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<arrow::record_batch::RecordBatch>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.batch_filter.md).


---

## collect_columns_from_predicate

`function` · `datafusion_physical_plan::filter::collect_columns_from_predicate`

> **Deprecated** — since 51.0.0: This function will be internal in the future

```rust
fn collect_columns_from_predicate(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> EqualAndNonEqual<'_>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.collect_columns_from_predicate.md).


Return the equals Column-Pairs and Non-equals Column-Pairs

---

## FilterExec

`struct` · `datafusion_physical_plan::filter::FilterExec`

```rust
struct FilterExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`, `datafusion_physical_plan::projection::EmbeddedProjection`

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn batch_size(&self) -> usize
fn default_selectivity(&self) -> u8
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn predicate(&self) -> &Arc<dyn PhysicalExpr>
fn projection(&self) -> &Option<ProjectionRef>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(predicate: Arc<dyn PhysicalExpr>, input: Arc<dyn ExecutionPlan>) -> Result<Self>
fn with_batch_size(&self, batch_size: usize) -> Result<Self>
fn with_default_selectivity(self, default_selectivity: u8) -> Result<Self, DataFusionError>
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, fetch: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::projection::EmbeddedProjection`**

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.FilterExec.md).


FilterExec evaluates a boolean predicate against all input batches to determine which rows to
include in its output batches.

---

## FilterExecBuilder

`struct` · `datafusion_physical_plan::filter::FilterExecBuilder`

```rust
struct FilterExecBuilder
```

**Implements**: `core::convert::From`

**Methods** (9)

```rust
fn apply_projection(self, projection: Option<Vec<usize>>) -> Result<Self>
fn apply_projection_by_ref(self, projection: Option<&ProjectionRef>) -> Result<Self>
fn build(self) -> Result<FilterExec>
fn new(predicate: Arc<dyn PhysicalExpr>, input: Arc<dyn ExecutionPlan>) -> Self
fn with_batch_size(self, batch_size: usize) -> Self
fn with_default_selectivity(self, default_selectivity: u8) -> Self
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_input(self, input: Arc<dyn ExecutionPlan>) -> Self
fn with_predicate(self, predicate: Arc<dyn PhysicalExpr>) -> Self
```

**via `core::convert::From`**

```rust
fn from(exec: &FilterExec) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.FilterExecBuilder.md).


Builder for [`FilterExec`] to set optional parameters

---

## EqualAndNonEqual

`type_alias` · `datafusion_physical_plan::filter::EqualAndNonEqual`

```rust
type EqualAndNonEqual<'a> = (Vec<PhysicalExprPairRef<'a>>, Vec<PhysicalExprPairRef<'a>>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.EqualAndNonEqual.md).


The equals Column-Pairs and Non-equals Column-Pairs in the Predicates

---

## PhysicalExprPairRef

`type_alias` · `datafusion_physical_plan::filter::PhysicalExprPairRef`

```rust
type PhysicalExprPairRef<'a> = (&'a std::sync::Arc<dyn PhysicalExpr>, &'a std::sync::Arc<dyn PhysicalExpr>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.filter.PhysicalExprPairRef.md).


Pair of `Arc<dyn PhysicalExpr>`s

---
