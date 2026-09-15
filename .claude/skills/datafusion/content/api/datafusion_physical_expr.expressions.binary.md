# `datafusion_physical_expr::expressions::binary`

Crate `datafusion-physical-expr` · 3 public items · structured records in [`model/datafusion_physical_expr.expressions.binary.json`](../model/datafusion_physical_expr.expressions.binary.json)

## binary

`function` · `datafusion_physical_expr::expressions::binary::binary`

Also reachable as `datafusion_physical_expr::expressions::binary`, `datafusion_physical_plan::execution_plan::expressions::binary`, `datafusion_physical_plan::expressions::binary`

```rust
fn binary(lhs: std::sync::Arc<dyn PhysicalExpr>, op: datafusion_expr::Operator, rhs: std::sync::Arc<dyn PhysicalExpr>, _input_schema: &Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Create a binary expression whose arguments are correctly coerced.
This function errors if it is not possible to coerce the arguments
to computational types supported by the operator.

---

## similar_to

`function` · `datafusion_physical_expr::expressions::binary::similar_to`

Also reachable as `datafusion_physical_expr::expressions::similar_to`, `datafusion_physical_plan::execution_plan::expressions::similar_to`, `datafusion_physical_plan::expressions::similar_to`

```rust
fn similar_to(negated: bool, case_insensitive: bool, expr: std::sync::Arc<dyn PhysicalExpr>, pattern: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Create a similar to expression

---

## BinaryExpr

`struct` · `datafusion_physical_expr::expressions::binary::BinaryExpr`

Also reachable as `datafusion_physical_expr::expressions::BinaryExpr`, `datafusion_physical_plan::execution_plan::expressions::BinaryExpr`, `datafusion_physical_plan::expressions::BinaryExpr`

```rust
struct BinaryExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn left(&self) -> &Arc<dyn PhysicalExpr>
fn new(left: Arc<dyn PhysicalExpr>, op: Operator, right: Arc<dyn PhysicalExpr>) -> Self
fn op(&self) -> &Operator
fn right(&self) -> &Arc<dyn PhysicalExpr>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn with_fail_on_overflow(self, fail_on_overflow: bool) -> Self
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
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Binary expression

---
