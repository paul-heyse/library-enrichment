# `datafusion_physical_expr::expressions::in_list`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.in_list.json`](../model/datafusion_physical_expr.expressions.in_list.json)

## in_list

`function` · `datafusion_physical_expr::expressions::in_list::in_list`

Also reachable as `datafusion_physical_expr::expressions::in_list`, `datafusion_physical_plan::execution_plan::expressions::in_list`, `datafusion_physical_plan::expressions::in_list`

```rust
fn in_list(expr: std::sync::Arc<dyn PhysicalExpr>, list: Vec<std::sync::Arc<dyn PhysicalExpr>>, negated: &bool, schema: &Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Creates a unary expression InList

---

## InListExpr

`struct` · `datafusion_physical_expr::expressions::in_list::InListExpr`

Also reachable as `datafusion_physical_expr::expressions::InListExpr`, `datafusion_physical_plan::execution_plan::expressions::InListExpr`, `datafusion_physical_plan::expressions::InListExpr`

```rust
struct InListExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn expr(&self) -> &Arc<dyn PhysicalExpr>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn list(&self) -> &[Arc<dyn PhysicalExpr>]
fn negated(&self) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn try_new(expr: Arc<dyn PhysicalExpr>, list: Vec<Arc<dyn PhysicalExpr>>, negated: bool, schema: &Schema) -> Result<Self>
fn try_new_from_array(expr: Arc<dyn PhysicalExpr>, array: ArrayRef, negated: bool, schema: &Schema) -> Result<Self>
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

InList

---
