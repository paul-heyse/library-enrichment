# `datafusion_functions_window::row_number`

Crate `datafusion-functions-window` · 3 public items · structured records in [`model/datafusion_functions_window.row_number.json`](../model/datafusion_functions_window.row_number.json)

## row_number

`function` · `datafusion_functions_window::row_number::row_number`

Also reachable as `datafusion_functions_window::expr_fn::row_number`

```rust
fn row_number() -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.row_number.row_number.md).


Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`RowNumber` user-defined window function.

Returns a unique row number for each row in window partition beginning at 1.

---

## row_number_udwf

`function` · `datafusion_functions_window::row_number::row_number_udwf`

```rust
fn row_number_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.row_number.row_number_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`row_number`].

Returns a unique row number for each row in window partition beginning at 1.

---

## RowNumber

`struct` · `datafusion_functions_window::row_number::RowNumber`

```rust
struct RowNumber
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
fn sort_options(&self) -> Option<SortOptions>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.row_number.RowNumber.md).


row_number expression

---
