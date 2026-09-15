# `datafusion_physical_expr::expressions::dynamic_filters`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.expressions.dynamic_filters.json`](../model/datafusion_physical_expr.expressions.dynamic_filters.json)

## DynamicFilterPhysicalExpr

`struct` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr`

Also reachable as `datafusion_physical_expr::expressions::DynamicFilterPhysicalExpr`, `datafusion_physical_plan::execution_plan::expressions::DynamicFilterPhysicalExpr`, `datafusion_physical_plan::expressions::DynamicFilterPhysicalExpr`

```rust
struct DynamicFilterPhysicalExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn current(&self) -> Result<Arc<dyn PhysicalExpr>>
fn is_used(&Arc<self>) -> bool
fn mark_complete(&self)
fn new(children: Vec<Arc<dyn PhysicalExpr>>, inner: Arc<dyn PhysicalExpr>) -> Self
fn try_from_proto(proto: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn update(&self, new_expr: Arc<dyn PhysicalExpr>) -> Result<()>
async fn wait_complete(&self)
async fn wait_update(&self)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &arrow::record_batch::RecordBatch) -> Result<ColumnarValue>
fn expression_id(&self) -> Option<u64>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
fn snapshot_generation(&self) -> u64
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

A dynamic [`PhysicalExpr`] that can be updated by anyone with a reference to it.

Any `ExecutionPlan` that uses this expression and holds a reference to it internally should probably also
implement `ExecutionPlan::reset_state` to remain compatible with recursive queries and other situations where
the same `ExecutionPlan` is reused with different data.

For more background, please also see the [Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]

[Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]: https://datafusion.apache.org/blog/2025/09/10/dynamic-filters

---
