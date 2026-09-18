# `datafusion_physical_plan::async_func`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.async_func.json`](../model/datafusion_physical_plan.async_func.json)

## AsyncFuncExec

`struct` · `datafusion_physical_plan::async_func::AsyncFuncExec`

```rust
struct AsyncFuncExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn async_exprs(&self) -> &[Arc<AsyncFuncExpr>]
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(async_exprs: Vec<Arc<AsyncFuncExpr>>, input: Arc<dyn ExecutionPlan>) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.async_func.AsyncFuncExec.md).


This structure evaluates a set of async expressions on a record
batch producing a new record batch

The schema of the output of the AsyncFuncExec is:
Input columns followed by one column for each async expression

---

## AsyncMapper

`struct` · `datafusion_physical_plan::async_func::AsyncMapper`

```rust
struct AsyncMapper
```

**Fields**: `async_exprs`

**Derives**: Debug

**Methods** (6)

```rust
fn find_references(&mut self, physical_expr: &Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<()>
fn is_empty(&self) -> bool
fn map_expr(&self, expr: Arc<dyn PhysicalExpr>) -> Transformed<Arc<dyn PhysicalExpr>>
fn new(num_input_columns: usize) -> Self
fn next_column_name(&self) -> String
fn output_column(&self, idx: usize) -> Arc<dyn PhysicalExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.async_func.AsyncMapper.md).


Maps async_expressions to new columns

The output of the async functions are appended, in order, to the end of the input schema

---
