# `datafusion_functions_window::nth_value`

Crate `datafusion-functions-window` · 9 public items · structured records in [`model/datafusion_functions_window.nth_value.json`](../model/datafusion_functions_window.nth_value.json)

## NthValueKind

`enum` · `datafusion_functions_window::nth_value::NthValueKind`

```rust
enum NthValueKind
```

**Variants**: `First`, `Last`, `Nth`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.NthValueKind.md).


Tag to differentiate special use cases of the NTH_VALUE built-in window function.

---

## first_value

`function` · `datafusion_functions_window::nth_value::first_value`

Also reachable as `datafusion_functions_window::expr_fn::first_value`

```rust
fn first_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.first_value.md).


Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`First` user-defined window function.

Returns the first value in the window frame

---

## first_value_udwf

`function` · `datafusion_functions_window::nth_value::first_value_udwf`

```rust
fn first_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.first_value_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`first_value`].

Returns the first value in the window frame

---

## last_value

`function` · `datafusion_functions_window::nth_value::last_value`

Also reachable as `datafusion_functions_window::expr_fn::last_value`

```rust
fn last_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.last_value.md).


Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Last` user-defined window function.

Returns the last value in the window frame

---

## last_value_udwf

`function` · `datafusion_functions_window::nth_value::last_value_udwf`

```rust
fn last_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.last_value_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`last_value`].

Returns the last value in the window frame

---

## nth_value

`function` · `datafusion_functions_window::nth_value::nth_value`

Also reachable as `datafusion_functions_window::expr_fn::nth_value`

```rust
fn nth_value(arg: datafusion_expr::Expr, n: i64) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.nth_value.md).


Create an expression to represent the `nth_value` window function

---

## nth_value_udwf

`function` · `datafusion_functions_window::nth_value::nth_value_udwf`

```rust
fn nth_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.nth_value_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`nth_value`].

Returns the nth value in the window frame

---

## NthValue

`struct` · `datafusion_functions_window::nth_value::NthValue`

```rust
struct NthValue
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn first() -> Self
fn kind(&self) -> &NthValueKind
fn last() -> Self
fn new(kind: NthValueKind) -> Self
fn nth() -> Self
```

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn reverse_expr(&self) -> ReversedUDWF
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.NthValue.md).


---

## NthValueState

`struct` · `datafusion_functions_window::nth_value::NthValueState`

```rust
struct NthValueState
```

**Fields**: `finalized_result`, `kind`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.nth_value.NthValueState.md).


---
