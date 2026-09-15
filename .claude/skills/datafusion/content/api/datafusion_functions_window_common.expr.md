# `datafusion_functions_window_common::expr`

Crate `datafusion-functions-window-common` · 1 public items · structured records in [`model/datafusion_functions_window_common.expr.json`](../model/datafusion_functions_window_common.expr.json)

## ExpressionArgs

`struct` · `datafusion_functions_window_common::expr::ExpressionArgs`

Also reachable as `datafusion_expr::function::ExpressionArgs`

```rust
struct ExpressionArgs<'a>
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn input_exprs(&self) -> &'a [Arc<dyn PhysicalExpr>]
fn input_fields(&self) -> &'a [FieldRef]
fn new(input_exprs: &'a [Arc<dyn PhysicalExpr>], input_fields: &'a [FieldRef]) -> Self
```

Arguments passed to user-defined window function

---
