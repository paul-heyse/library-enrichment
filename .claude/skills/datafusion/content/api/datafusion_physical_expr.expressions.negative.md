# `datafusion_physical_expr::expressions::negative`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.negative.json`](../model/datafusion_physical_expr.expressions.negative.json)

## negative

`function` · `datafusion_physical_expr::expressions::negative::negative`

Also reachable as `datafusion_physical_expr::expressions::negative`, `datafusion_physical_plan::execution_plan::expressions::negative`, `datafusion_physical_plan::expressions::negative`

```rust
fn negative(arg: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Creates a unary expression NEGATIVE

# Errors

This function errors when the argument's type is not signed numeric

---

## NegativeExpr

`struct` · `datafusion_physical_expr::expressions::negative::NegativeExpr`

Also reachable as `datafusion_physical_expr::expressions::NegativeExpr`, `datafusion_physical_plan::execution_plan::expressions::NegativeExpr`, `datafusion_physical_plan::expressions::NegativeExpr`

```rust
struct NegativeExpr
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
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Negative expression

---
