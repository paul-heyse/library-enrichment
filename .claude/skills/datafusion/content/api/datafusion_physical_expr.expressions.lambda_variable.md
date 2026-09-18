# `datafusion_physical_expr::expressions::lambda_variable`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.lambda_variable.json`](../model/datafusion_physical_expr.expressions.lambda_variable.json)

## lambda_variable

`function` · `datafusion_physical_expr::expressions::lambda_variable::lambda_variable`

Also reachable as `datafusion_physical_expr::expressions::lambda_variable`, `datafusion_physical_plan::execution_plan::expressions::lambda_variable`, `datafusion_physical_plan::expressions::lambda_variable`

```rust
fn lambda_variable(name: &str, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.lambda_variable.lambda_variable.md).


Create a lambda variable expression

---

## LambdaVariable

`struct` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable`

Also reachable as `datafusion_physical_expr::expressions::LambdaVariable`, `datafusion_physical_plan::execution_plan::expressions::LambdaVariable`, `datafusion_physical_plan::expressions::LambdaVariable`

```rust
struct LambdaVariable
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn field(&self) -> &FieldRef
fn index(&self) -> usize
fn name(&self) -> &str
fn new(index: usize, field: FieldRef) -> Self
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
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.lambda_variable.LambdaVariable.md).


Represents the lambda variable with a given index and field

---
