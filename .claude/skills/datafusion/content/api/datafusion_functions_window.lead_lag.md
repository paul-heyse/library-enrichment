# `datafusion_functions_window::lead_lag`

Crate `datafusion-functions-window` · 6 public items · structured records in [`model/datafusion_functions_window.lead_lag.json`](../model/datafusion_functions_window.lead_lag.json)

## WindowShiftKind

`enum` · `datafusion_functions_window::lead_lag::WindowShiftKind`

```rust
enum WindowShiftKind
```

**Variants**: `Lag`, `Lead`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.WindowShiftKind.md).


---

## lag

`function` · `datafusion_functions_window::lead_lag::lag`

Also reachable as `datafusion_functions_window::expr_fn::lag`

```rust
fn lag(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.lag.md).


Create an expression to represent the `lag` window function

returns value evaluated at the row that is offset rows before the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null

---

## lag_udwf

`function` · `datafusion_functions_window::lead_lag::lag_udwf`

```rust
fn lag_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.lag_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lag`].

Returns the row value that precedes the current row by a specified offset within partition. If no such row exists, then returns the default value.

---

## lead

`function` · `datafusion_functions_window::lead_lag::lead`

Also reachable as `datafusion_functions_window::expr_fn::lead`

```rust
fn lead(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.lead.md).


Create an expression to represent the `lead` window function

returns value evaluated at the row that is offset rows after the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null

---

## lead_udwf

`function` · `datafusion_functions_window::lead_lag::lead_udwf`

```rust
fn lead_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.lead_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lead`].

Returns the value from a row that follows the current row by a specified offset within the partition. If no such row exists, then returns the default value.

---

## WindowShift

`struct` · `datafusion_functions_window::lead_lag::WindowShift`

```rust
struct WindowShift
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn kind(&self) -> &WindowShiftKind
fn lag() -> Self
fn lead() -> Self
```

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn reverse_expr(&self) -> ReversedUDWF
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.lead_lag.WindowShift.md).


window shift expression

---
