# `datafusion_physical_expr::higher_order_function`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.higher_order_function.json`](../model/datafusion_physical_expr.higher_order_function.json)

## HigherOrderFunctionExpr

`struct` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr`

Also reachable as `datafusion::physical_expr::HigherOrderFunctionExpr`, `datafusion_physical_expr::HigherOrderFunctionExpr`

```rust
struct HigherOrderFunctionExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
fn config_options(&self) -> &ConfigOptions
fn fun(&self) -> &HigherOrderUDF
fn name(&self) -> &str
fn nullable(&self) -> bool
fn return_type(&self) -> &DataType
fn try_new_with_schema(fun: Arc<HigherOrderUDF>, args: Vec<Arc<dyn PhysicalExpr>>, schema: &Schema, config_options: Arc<ConfigOptions>) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
fn is_volatile_node(&self) -> bool
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.higher_order_function.HigherOrderFunctionExpr.md).


Physical expression of a higher order function

---
