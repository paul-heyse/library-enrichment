# `datafusion_physical_expr::expressions::literal`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.literal.json`](../model/datafusion_physical_expr.expressions.literal.json)

## lit

`function` · `datafusion_physical_expr::expressions::literal::lit`

Also reachable as `datafusion_physical_expr::expressions::lit`, `datafusion_physical_plan::execution_plan::expressions::lit`, `datafusion_physical_plan::expressions::lit`

```rust
fn lit<T: datafusion_expr::Literal>(value: T) -> std::sync::Arc<dyn PhysicalExpr>
```

Create a literal expression

---

## Literal

`struct` · `datafusion_physical_expr::expressions::literal::Literal`

Also reachable as `datafusion_physical_expr::expressions::Literal`, `datafusion_physical_plan::execution_plan::expressions::Literal`, `datafusion_physical_plan::expressions::Literal`

```rust
struct Literal
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn new(value: ScalarValue) -> Self
fn new_with_metadata(value: ScalarValue, metadata: Option<FieldMetadata>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn value(&self) -> &ScalarValue
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
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn placement(&self) -> ExpressionPlacement
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Represents a literal value

---
