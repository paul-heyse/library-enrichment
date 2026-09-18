# `datafusion_physical_expr::expressions::like`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.like.json`](../model/datafusion_physical_expr.expressions.like.json)

## like

`function` · `datafusion_physical_expr::expressions::like::like`

Also reachable as `datafusion_physical_expr::expressions::like`, `datafusion_physical_plan::execution_plan::expressions::like`, `datafusion_physical_plan::expressions::like`

```rust
fn like(negated: bool, case_insensitive: bool, expr: std::sync::Arc<dyn PhysicalExpr>, pattern: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.like.like.md).


Create a like expression, erroring if the argument types are not compatible.

---

## LikeExpr

`struct` · `datafusion_physical_expr::expressions::like::LikeExpr`

Also reachable as `datafusion_physical_expr::expressions::LikeExpr`, `datafusion_physical_plan::execution_plan::expressions::LikeExpr`, `datafusion_physical_plan::expressions::LikeExpr`

```rust
struct LikeExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn case_insensitive(&self) -> bool
fn expr(&self) -> &Arc<dyn PhysicalExpr>
fn negated(&self) -> bool
fn new(negated: bool, case_insensitive: bool, expr: Arc<dyn PhysicalExpr>, pattern: Arc<dyn PhysicalExpr>) -> Self
fn pattern(&self) -> &Arc<dyn PhysicalExpr>
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
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.like.LikeExpr.md).


---
