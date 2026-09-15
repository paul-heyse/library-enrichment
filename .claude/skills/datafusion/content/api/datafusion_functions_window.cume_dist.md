# `datafusion_functions_window::cume_dist`

Crate `datafusion-functions-window` · 3 public items · structured records in [`model/datafusion_functions_window.cume_dist.json`](../model/datafusion_functions_window.cume_dist.json)

## cume_dist

`function` · `datafusion_functions_window::cume_dist::cume_dist`

Also reachable as `datafusion_functions_window::expr_fn::cume_dist`

```rust
fn cume_dist() -> datafusion_expr::Expr
```

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`CumeDist` user-defined window function.

Calculates the cumulative distribution of a value in a group of values.

---

## cume_dist_udwf

`function` · `datafusion_functions_window::cume_dist::cume_dist_udwf`

```rust
fn cume_dist_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`cume_dist`].

Calculates the cumulative distribution of a value in a group of values.

---

## CumeDist

`struct` · `datafusion_functions_window::cume_dist::CumeDist`

```rust
struct CumeDist
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
```

CumeDist calculates the cume_dist in the window function with order by

---
