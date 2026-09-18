# `datafusion_physical_expr::expressions::is_not_null`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.is_not_null.json`](../model/datafusion_physical_expr.expressions.is_not_null.json)

## is_not_null

`function` · `datafusion_physical_expr::expressions::is_not_null::is_not_null`

Also reachable as `datafusion_physical_expr::expressions::is_not_null`, `datafusion_physical_plan::execution_plan::expressions::is_not_null`, `datafusion_physical_plan::expressions::is_not_null`

```rust
fn is_not_null(arg: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.is_not_null.is_not_null.md).


Create an IS NOT NULL expression

---

## IsNotNullExpr

`struct` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr`

Also reachable as `datafusion_physical_expr::expressions::IsNotNullExpr`, `datafusion_physical_plan::execution_plan::expressions::IsNotNullExpr`, `datafusion_physical_plan::expressions::IsNotNullExpr`

```rust
struct IsNotNullExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn arg(&self) -> &Arc<dyn PhysicalExpr>
fn new(arg: Arc<dyn PhysicalExpr>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.is_not_null.IsNotNullExpr.md).


IS NOT NULL expression

---
