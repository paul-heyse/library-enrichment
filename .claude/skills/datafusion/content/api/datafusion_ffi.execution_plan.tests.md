# `datafusion_ffi::execution_plan::tests`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.execution_plan.tests.json`](../model/datafusion_ffi.execution_plan.tests.json)

## EmptyExec

`struct` · `datafusion_ffi::execution_plan::tests::EmptyExec`

```rust
struct EmptyExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (5)

```rust
fn new(schema: arrow::datatypes::SchemaRef) -> Self
fn with_dynamic_expressions(self, dynamic_expressions: Vec<Arc<dyn PhysicalExpr>>) -> Self
fn with_expressions(self, expressions: Vec<Arc<dyn PhysicalExpr>>) -> Self
fn with_metrics(self, metrics: MetricsSet) -> Self
fn with_statistics(self, statistics: Statistics) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, _t: DisplayFormatType, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.execution_plan.tests.EmptyExec.md).


---
