# `datafusion_physical_expr::expressions::try_cast`

Crate `datafusion-physical-expr` · 3 public items · structured records in [`model/datafusion_physical_expr.expressions.try_cast.json`](../model/datafusion_physical_expr.expressions.try_cast.json)

## try_cast

`function` · `datafusion_physical_expr::expressions::try_cast::try_cast`

Also reachable as `datafusion_physical_expr::expressions::try_cast`, `datafusion_physical_plan::execution_plan::expressions::try_cast`, `datafusion_physical_plan::expressions::try_cast`

```rust
fn try_cast(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, cast_type: arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Return a PhysicalExpression representing `expr` casted to
`cast_type`, if any casting is needed.

Note that such casts may lose type information

---

## try_cast_with_target_field

`function` · `datafusion_physical_expr::expressions::try_cast::try_cast_with_target_field`

Also reachable as `datafusion_physical_expr::expressions::try_cast_with_target_field`, `datafusion_physical_plan::execution_plan::expressions::try_cast_with_target_field`, `datafusion_physical_plan::expressions::try_cast_with_target_field`

```rust
fn try_cast_with_target_field(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, target_field: &arrow::datatypes::FieldRef) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Return a PhysicalExpression representing `expr` casted to `target_field`,
preserving any explicit field semantics such as metadata.

TRY_CAST results are always nullable since failed casts return NULL.

If the input expression already has the same data type, the target field
has no explicit metadata constraints, and the source has no extension
metadata to strip, the original expression is returned unchanged.

---

## TryCastExpr

`struct` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr`

Also reachable as `datafusion_physical_expr::expressions::TryCastExpr`, `datafusion_physical_plan::execution_plan::expressions::TryCastExpr`, `datafusion_physical_plan::expressions::TryCastExpr`

```rust
struct TryCastExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn cast_type(&self) -> &DataType
fn expr(&self) -> &Arc<dyn PhysicalExpr>
fn new(expr: Arc<dyn PhysicalExpr>, cast_type: DataType) -> Self
fn new_with_target_field(expr: Arc<dyn PhysicalExpr>, target_field: FieldRef) -> Self
fn target_field(&self) -> &FieldRef
fn target_metadata(&self) -> Option<&HashMap<String, String>>
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
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

TRY_CAST expression casts an expression to a specific data type and returns NULL on invalid cast

---
