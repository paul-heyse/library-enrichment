# `datafusion_physical_expr::expressions::unknown_column`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.expressions.unknown_column.json`](../model/datafusion_physical_expr.expressions.unknown_column.json)

## UnKnownColumn

`struct` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn`

Also reachable as `datafusion_physical_expr::expressions::UnKnownColumn`, `datafusion_physical_plan::execution_plan::expressions::UnKnownColumn`, `datafusion_physical_plan::expressions::UnKnownColumn`

```rust
struct UnKnownColumn
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn name(&self) -> &str
fn new(name: &str) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.unknown_column.UnKnownColumn.md).


---
