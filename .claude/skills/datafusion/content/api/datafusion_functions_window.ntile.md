# `datafusion_functions_window::ntile`

Crate `datafusion-functions-window` · 3 public items · structured records in [`model/datafusion_functions_window.ntile.json`](../model/datafusion_functions_window.ntile.json)

## ntile

`function` · `datafusion_functions_window::ntile::ntile`

Also reachable as `datafusion_functions_window::expr_fn::ntile`

```rust
fn ntile(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.ntile.ntile.md).


Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Ntile` user-defined window function.

Integer ranging from 1 to the argument value, dividing the partition as equally as possible.

---

## ntile_udwf

`function` · `datafusion_functions_window::ntile::ntile_udwf`

```rust
fn ntile_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.ntile.ntile_udwf.md).


Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`ntile`].

Integer ranging from 1 to the argument value, dividing the partition as equally as possible.

---

## Ntile

`struct` · `datafusion_functions_window::ntile::Ntile`

```rust
struct Ntile
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
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window.ntile.Ntile.md).


---
