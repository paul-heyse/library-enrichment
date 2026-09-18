# `datafusion_physical_expr::expressions::not`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.not.json`](../model/datafusion_physical_expr.expressions.not.json)

## not

`function` · `datafusion_physical_expr::expressions::not::not`

Also reachable as `datafusion_physical_expr::expressions::not`, `datafusion_physical_plan::execution_plan::expressions::not`, `datafusion_physical_plan::expressions::not`

```rust
fn not(arg: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.not.not.md).


Creates a unary expression NOT

---

## NotExpr

`struct` · `datafusion_physical_expr::expressions::not::NotExpr`

Also reachable as `datafusion_physical_expr::expressions::NotExpr`, `datafusion_physical_plan::execution_plan::expressions::NotExpr`, `datafusion_physical_plan::expressions::NotExpr`

```rust
struct NotExpr
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
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn propagate_statistics(&self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.not.NotExpr.md).


Not expression

---
