# `datafusion_physical_expr::async_scalar_function`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.async_scalar_function.json`](../model/datafusion_physical_expr.async_scalar_function.json)

## AsyncFuncExpr

`struct` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr`

```rust
struct AsyncFuncExpr
```

**Fields**: `name`, `func`

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn field(&self, _input_schema: &Schema) -> Result<Field>
fn ideal_batch_size(&self) -> Result<Option<usize>>
async fn invoke_with_args(&self, batch: &RecordBatch, config_options: Arc<ConfigOptions>) -> Result<ColumnarValue>
fn name(&self) -> &str
fn try_new(name: impl Into<String>, func: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.async_scalar_function.AsyncFuncExpr.md).


Wrapper around a scalar function that can be evaluated asynchronously

---
