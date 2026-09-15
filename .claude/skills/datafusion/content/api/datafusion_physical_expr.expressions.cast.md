# `datafusion_physical_expr::expressions::cast`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.cast.json`](../model/datafusion_physical_expr.expressions.cast.json)

## cast

`function` · `datafusion_physical_expr::expressions::cast::cast`

Also reachable as `datafusion_physical_expr::expressions::cast`, `datafusion_physical_plan::execution_plan::expressions::cast`, `datafusion_physical_plan::expressions::cast`

```rust
fn cast(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, cast_type: arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Return a PhysicalExpression representing `expr` casted to
`cast_type`, if any casting is needed.

Note that such casts may lose type information

---

## CastExpr

`struct` · `datafusion_physical_expr::expressions::cast::CastExpr`

Also reachable as `datafusion_physical_expr::expressions::CastExpr`, `datafusion_physical_plan::execution_plan::expressions::CastExpr`, `datafusion_physical_plan::expressions::CastExpr`

```rust
struct CastExpr
```

**Fields**: `expr`

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (13)

```rust
fn cast_options(&self) -> &CastOptions<'static>
fn cast_type(&self) -> &DataType
fn check_bigger_cast(cast_type: &DataType, src: &DataType) -> bool
fn expr(&self) -> &Arc<dyn PhysicalExpr>
fn has_explicit_metadata(&self) -> bool
fn has_explicit_nullability(&self) -> bool
fn is_bigger_cast(&self, src: &DataType) -> bool
fn new(expr: Arc<dyn PhysicalExpr>, cast_type: DataType, cast_options: Option<CastOptions<'static>>) -> Self
fn new_with_target_field(expr: Arc<dyn PhysicalExpr>, target_field: FieldRef, cast_options: Option<CastOptions<'static>>) -> Self
fn target_field(&self) -> &FieldRef
fn target_metadata(&self) -> Option<&HashMap<String, String>>
fn target_nullable(&self) -> Option<bool>
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
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

CAST expression casts an expression to a specific data type and returns a runtime error on invalid cast

---
