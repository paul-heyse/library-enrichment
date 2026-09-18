# `datafusion_physical_expr::expressions::lambda`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.lambda.json`](../model/datafusion_physical_expr.expressions.lambda.json)

## lambda

`function` · `datafusion_physical_expr::expressions::lambda::lambda`

Also reachable as `datafusion_physical_expr::expressions::lambda`, `datafusion_physical_plan::execution_plan::expressions::lambda`, `datafusion_physical_plan::expressions::lambda`

```rust
fn lambda(params: impl IntoIterator<Item = impl Into<String>>, body: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.lambda.lambda.md).


Create a lambda expression.

---

## LambdaExpr

`struct` · `datafusion_physical_expr::expressions::lambda::LambdaExpr`

Also reachable as `datafusion_physical_expr::expressions::LambdaExpr`, `datafusion_physical_plan::execution_plan::expressions::LambdaExpr`, `datafusion_physical_plan::expressions::LambdaExpr`

```rust
struct LambdaExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn body(&self) -> &Arc<dyn PhysicalExpr>
fn params(&self) -> &[String]
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn try_new(params: Vec<String>, body: Arc<dyn PhysicalExpr>) -> Result<Self>
fn used_param_indices(&self) -> &[usize]
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
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.lambda.LambdaExpr.md).


Represents a lambda with the given parameters names and body

---
