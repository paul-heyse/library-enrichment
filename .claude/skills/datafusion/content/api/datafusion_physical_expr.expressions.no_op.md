# `datafusion_physical_expr::expressions::no_op`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.expressions.no_op.json`](../model/datafusion_physical_expr.expressions.no_op.json)

## NoOp

`struct` · `datafusion_physical_expr::expressions::no_op::NoOp`

Also reachable as `datafusion_physical_expr::expressions::NoOp`, `datafusion_physical_plan::execution_plan::expressions::NoOp`, `datafusion_physical_plan::expressions::NoOp`

```rust
struct NoOp
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
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
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

A place holder expression, can not be evaluated.

Used in some cases where an `Arc<dyn PhysicalExpr>` is needed, such as `children()`

---
