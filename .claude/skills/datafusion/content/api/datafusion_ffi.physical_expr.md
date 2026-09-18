# `datafusion_ffi::physical_expr`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.physical_expr.json`](../model/datafusion_ffi.physical_expr.json)

## FFI_PhysicalExpr

`struct` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr`

```rust
struct FFI_PhysicalExpr
```

**Fields**: `data_type`, `nullable`, `evaluate`, `return_field`, `evaluate_selection`, `children`, `new_with_children`, `evaluate_bounds`, `propagate_constraints`, `evaluate_statistics`, `propagate_statistics`, `get_properties`, `fmt_sql`, `snapshot`, `snapshot_generation`, `is_volatile_node`, `expression_id`, `display`, `hash`, `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(expr: Arc<dyn PhysicalExpr>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.FFI_PhysicalExpr.md).


---

## ForeignPhysicalExpr

`struct` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr`

```rust
struct ForeignPhysicalExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq, Send, Sync

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
fn evaluate_selection(&self, batch: &RecordBatch, selection: &BooleanArray) -> Result<ColumnarValue>
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
fn expression_id(&self) -> Option<u64>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
fn is_volatile_node(&self) -> bool
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn propagate_statistics(&self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
fn snapshot_generation(&self) -> u64
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.ForeignPhysicalExpr.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalExpr to interact with the expression.

---
